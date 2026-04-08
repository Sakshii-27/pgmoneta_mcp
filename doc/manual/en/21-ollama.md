
## Ollama

[Ollama](https://ollama.com) is the recommended way to get started with local LLM
support in **pgmoneta_mcp**. It provides a straightforward CLI, a local HTTP API,
and a simple model management workflow.

This section shows how to install Ollama, prepare a tool-capable model, and configure **pgmoneta_mcp** to use it.

### Install Ollama

Follow the installation instructions for your platform at
[ollama.com/download](https://ollama.com/download).

After installation, verify that the CLI is available:

``` sh
ollama --version
```

### Start the Ollama service

Ollama serves models over HTTP. Start it with:

``` sh
ollama serve
```

The default endpoint is:

``` text
http://localhost:11434
```

Check that it is reachable:

``` sh
curl http://localhost:11434/
```

If the service is up, the response should confirm that Ollama is running.

### Storage Management

By default, Ollama stores models in `/usr/share/ollama/.ollama/models` (or `~/.ollama/models` for user installs). To avoid filling up your root partition, redirect this directory using `OLLAMA_MODELS`:

```sh
export OLLAMA_MODELS=/mnt/ai/ollama/models
# For systemd service:
# systemctl set-environment OLLAMA_MODELS=/mnt/ai/ollama/models
# systemctl restart ollama
```

### Pull a model

Before using a model, download it locally. The model name here must match exactly what you configure in `pgmoneta-mcp.conf`. For example:

**Small setup** (Laptop friendly):
```sh
OLLAMA_MODELS=/mnt/ai/ollama/models ollama pull llama3.2:3b
```

**Best setup** (Recommended):
```sh
OLLAMA_MODELS=/mnt/ai/ollama/models ollama pull granite-code:8b
```

**Full setup** (Workstation only):
```sh
OLLAMA_MODELS=/mnt/ai/ollama/models ollama pull llama3.1:70b
```

**Verify downloaded models**

To see all models installed on your system:

``` sh
ollama list
```

This will show the model name, ID, size on disk, and when it was downloaded or last updated:

``` text
NAME               ID              SIZE    MODIFIED
qwen2.5:3b         cb8f3e4a7b82    1.9 GB  2 hours ago
llama3.1:latest    42182419e950    4.7 GB  3 days ago
```

The value in the `NAME` column is the model identifier you must set in your config. For example, if you see `qwen2.5:3b` in the list, your config should be:

``` ini
model = qwen2.5:3b
```

### Check tool support

For **pgmoneta_mcp**, the model must support tool calling.

To inspect a model:

``` sh
ollama show qwen2.5:3b
```

You can also query the API directly:

``` sh
curl -s http://localhost:11434/api/show \
  -d '{"model":"qwen2.5:3b"}'
```

Look for `tools` in the reported capabilities.

### Configure pgmoneta_mcp

Add or update the `[llm]` section in `pgmoneta-mcp.conf`:

``` ini
[llm]
provider = ollama
endpoint = http://localhost:11434
model = qwen2.5:3b
max_tool_rounds = 10
```
