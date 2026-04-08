
## RamaLama

[RamaLama](https://ramalama.ai) is an open-source command-line interface and unified AI gateway that simplifies the deployment and inference of AI models using containerization (Podman or Docker). It provides an OpenAI-compatible REST API, making it easy to integrate with **pgmoneta_mcp**.

Using `RamaLama` with `pgmoneta-mcp` allows you to leverage various runtimes (like `llama.cpp` or `vLLM`) through a single, stable interface.

### Install

To use `RamaLama`, you need to install the CLI. On Fedora and other RPM-based distributions, it is available via:

``` sh
dnf install ramalama
```

For other platforms, follow the instructions at [ramalama.ai](https://ramalama.ai).

### Download models & Storage Management

RamaLama automatically handles pulling models from registries like Hugging Face or OCI.

By default, RamaLama pulls layers using container tooling and stores them in your system's temporary directory and container storage. To change the storage path to a larger drive (e.g., `/mnt/ai/ramalama`), use the `--store` flag:

**Small setup** (Laptop friendly):
```sh
ramalama --store /mnt/ai/ramalama pull llama3.2:3b
```

**Best setup** (Recommended):
```sh
ramalama --store /mnt/ai/ramalama pull granite-code:8b
```

**Full setup** (Workstation only):
```sh
ramalama --store /mnt/ai/ramalama pull llama3.1:70b
```

### Start the server

Start the RamaLama server using your chosen model. RamaLama will automatically run the model inside a container:

``` sh
ramalama --store /mnt/ai/ramalama serve granite-code:8b
```

The default endpoint will be `http://localhost:8080`.

### Configure pgmoneta_mcp

Add or update the `[llm]` section in `pgmoneta-mcp.conf`:

``` ini
[llm]
provider = ramalama
endpoint = http://localhost:8080
model = granite-3.0-8b-instruct
max_tool_rounds = 10
```

### Quick verification

Confirm the server is running:

``` sh
curl http://localhost:8080/v1/models
```

Start **pgmoneta_mcp**:

``` sh
pgmoneta-mcp-server -c pgmoneta-mcp.conf -u pgmoneta-mcp-users.conf
```

Open your MCP client and ask a question about your backups to verify end-to-end setup.