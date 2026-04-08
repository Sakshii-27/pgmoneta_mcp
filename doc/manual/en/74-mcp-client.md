# pgmoneta-mcp-client

## Client Configuration File

The interactive client reads connection settings from a dedicated INI conf file.

Default path:

```text
/etc/pgmoneta-mcp/pgmoneta-mcp-client.conf
```

Expected format:

```ini
[pgmoneta_mcp_client]
url = http://localhost:8000/mcp
timeout = 30

[llm]
provider = ollama
endpoint = http://localhost:11434
model = qwen2.5:3b
max_tool_rounds = 10
```

The file must contain a `[pgmoneta_mcp_client]` section and may optionally include an
`[llm]` section using the same keys as `pgmoneta-mcp-server.conf`.

### `[pgmoneta_mcp_client]`

| Key | Required | Description |
| :--- | :--- | :--- |
| `url` | Yes | Full MCP endpoint used by the client. This should point to the server's `/mcp` route, for example `http://localhost:8000/mcp`. |
| `timeout` | No | Connection and request timeout in seconds. Defaults to `30` when omitted. |

### `[llm]`

| Key | Required | Description |
| :--- | :--- | :--- |
| `provider` | Yes | LLM backend. Supported values match the server configuration: `ollama`, `llama.cpp`, `ramalama`, and `vllm`. |
| `endpoint` | Yes | Base URL of the LLM server. |
| `model` | Yes | Model name or ID to use for tool selection. |
| `max_tool_rounds` | No | Accepted for compatibility with the server's `[llm]` block. Defaults to `10`. |

Example with comments:

```ini
[pgmoneta_mcp_client]
# Full MCP endpoint used by the client
url = http://localhost:8000/mcp

# Timeout in seconds for connect, list_tools, and call_tool operations
timeout = 30

[llm]
# Optional LLM configuration for natural-language tool execution
provider = ollama
endpoint = http://localhost:11434
model = qwen2.5:3b
max_tool_rounds = 10
```

## Interactive Shell

Start the client:

```bash
./pgmoneta-mcp-client
./pgmoneta-mcp-client --conf <path_to_client_conf> --users <path_to_users_conf>
```

The prompt uses the selected username and configured MCP URL:

```text
admin@localhost:8000/mcp$ 
```

## Commands

```text
/help                 Show basic usage
/user                 Switch to user mode (default)
/developer            Switch to developer mode
/tools                List available tools
/exit or /quit        Exit the client
```

The client uses `url` from `pgmoneta-mcp-client.conf` as the MCP endpoint to
connect to, derives the tool `server` argument from that endpoint's host name,
and injects `username` from the users file passed with `-u` / `--users`. If the
users file contains multiple admin usernames, the client asks you to choose one
at startup. For any remaining parameters, the client prompts from the tool
schema. Required fields must be filled in, while optional fields can be skipped
by pressing Enter.

The client starts in `/user` mode. In this mode it accepts natural-language
requests. If an `[llm]` section is present, it sends the current `/tools`
definitions to the configured LLM, asks it to choose the best matching tool,
and then executes that tool with the generated JSON arguments. For example,
`List backups on primary server` maps to `list_backups {"server":"primary"}`
before execution.

Use `/developer` to switch to developer mode. In this mode the input must be an
explicit tool call such as `list_backups {"server":"primary"}`, and the client
prints the full JSON response instead of the human-readable translation used in
user mode.

The shell uses readline-style editing, so standard history and cursor shortcuts
such as Arrow Up / Down, Home / End, Ctrl+A / E, Ctrl+B / F, Ctrl+R, Ctrl+U / K,
and Ctrl+Y work directly in the input prompt. Slash commands support Tab
completion, so typing `/ex` and pressing Tab completes to `/exit`. Command
history is loaded from and saved to `~/.pgmoneta-mcp/pgmoneta-mcp-client.history`,
and the client keeps at most the latest 1000 entries. Tool errors are printed in
the session and do not terminate the client. When a tool response is JSON, the
client pretty-prints it and translates known pgmoneta fields such as file sizes,
LSNs, compression, encryption, command codes, and error codes into more
readable values.

Examples:

```bash
admin@localhost:8000/mcp$ /user
admin@localhost:8000/mcp$ List backups on primary server
admin@localhost:8000/mcp$ /developer
admin@localhost:8000/mcp$ list_backups {"server":"primary"}
```
