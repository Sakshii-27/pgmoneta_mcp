
## vLLM

[vLLM](https://github.com/vllm-project/vllm) is a high-throughput and memory-efficient inference and serving engine for LLMs. It is heavily utilized in production environments providing state-of-the-art serving throughput using PagedAttention.

Because vLLM natively exposes an **OpenAI-compatible server API** (`/v1/chat/completions`), it integrates perfectly with **pgmoneta_mcp** as a backend provider.

### Install

vLLM is a Python package. It is highly recommended to install it in an isolated virtual environment (or via Docker) to avoid dependency conflicts. 

For a basic setup on Rocky Linux 10, you can use pip:

```sh
pip install vllm
```

For advanced installation methods (such as Docker or building from source), refer to the [official vLLM installation guide](https://docs.vllm.ai/en/latest/getting_started/installation.html).

### Download models

vLLM does not require you to manually hunt for model files. It automatically pulls standard Hugging Face (Safetensor) model weights at runtime.

You simply specify the Hugging Face repository ID (e.g., `ibm-granite/granite-3.0-8b-instruct`).

### Storage Management

vLLM utilizes the standard Hugging Face cache directory (`~/.cache/huggingface`). Set the `HF_HOME` environment variable to a large mounted drive to prevent disk space exhaustion:

```sh
export HF_HOME=/mnt/ai/huggingface
```

> [!NOTE]
> vLLM loads raw unquantized or 16-bit weight SafeTensors by default. Its RAM/VRAM requirements and disk storage needs are therefore **significantly higher** than GGUF equivalents (like llama.cpp or Ollama) unless you explicitly use models pre-quantized in AWQ/GPTQ formats.

### Start the server

Start the vLLM server by pointing the `openai.api_server` entrypoint to your desired model. vLLM will automatically download the model weights to the Hugging Face cache if they are not already present.

**Small setup** (Laptop friendly, ~8GB RAM req):
```sh
HF_HOME=/mnt/ai/huggingface python -m vllm.entrypoints.openai.api_server \
  --model meta-llama/Llama-3.2-3B-Instruct \
  --port 8000
```

**Best setup** (Recommended, ~16GB RAM req):
```sh
HF_HOME=/mnt/ai/huggingface python -m vllm.entrypoints.openai.api_server \
  --model ibm-granite/granite-3.0-8b-instruct \
  --port 8000
```

**Full setup** (Workstation only):
```sh
HF_HOME=/mnt/ai/huggingface python -m vllm.entrypoints.openai.api_server \
  --model meta-llama/Meta-Llama-3.1-70B-Instruct \
  --port 8000 \
  --tensor-parallel-size 4
```

The default endpoint will be `http://localhost:8000`.

### Configure pgmoneta_mcp

Add or update the `[llm]` section in `pgmoneta-mcp.conf`:

```ini
[llm]
provider = vllm
endpoint = http://localhost:8000
model = ibm-granite/granite-3.0-8b-instruct
max_tool_rounds = 10
```

### Quick verification

Confirm the server is running by querying the models endpoint:

```sh
curl http://localhost:8000/v1/models
```

Start **pgmoneta_mcp**:

```sh
pgmoneta-mcp-server -c pgmoneta-mcp.conf -u pgmoneta-mcp-users.conf
```

Open your MCP client and ask a question about your backups to verify the end-to-end setup.
