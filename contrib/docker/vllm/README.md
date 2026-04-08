# vLLM

Docker setup for running [vLLM](https://github.com/vllm-project/vllm) as a backend for pgmoneta MCP.
vLLM provides a high-throughput, OpenAI-compatible API server.

> [!IMPORTANT]
> **GPU Requirements**
> Running vLLM via Docker requires a Linux host environment with an **NVIDIA GPU** and the 
> [NVIDIA Container Toolkit](https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/latest/install-guide.html) installed.

## Quick start

```bash
docker compose up -d
```

The container mounts your local host's `~/.cache/huggingface` directory to persist downloaded model weights across restarts.

## Configuration & Setups

To avoid disk space issues (`ENOSPC`) with large AI models, you should define a custom docker volume or bind mount point for the model downloads if your root partition is small.
The container natively mounts your host's `~/.cache/huggingface` to persist weights. If your home partition is small, you should move this directory to a high-capacity drive or update the `docker-compose.yml` mount point.

We define three standard setups based on your hardware capabilities. Start the container with the respective environment variable:

**Small Setup** (Laptop friendly; ~8GB RAM/VRAM):
```bash
VLLM_MODEL=meta-llama/Llama-3.2-3B-Instruct docker compose up -d
```

**Best Setup** (Recommended balance; ~8GB RAM/Disk):
```bash
VLLM_MODEL=ibm-granite/granite-3.0-8b-instruct docker compose up -d
```

**Full Setup** (Workstation; ~40GB+ RAM/Disk):
```bash
VLLM_MODEL=meta-llama/Meta-Llama-3.1-70B-Instruct docker compose up -d
```

If you are using a gated model on Hugging Face (like Llama 3.1), you will also need to provide your Hugging Face API token:

```bash
HF_TOKEN=hf_your_token VLLM_MODEL=... docker compose up -d
```

## pgmoneta MCP configuration

vLLM provides an OpenAI-compatible API endpoint natively. Since pgmoneta MCP natively supports the OpenAI structural format via `llamafile`, you configure it as `provider = llama.cpp` or `provider = ramalama`:

```ini
[llm]
provider = llama.cpp
endpoint = http://localhost:8000/v1
model = meta-llama/Meta-Llama-3.1-8B-Instruct
max_tool_rounds = 10
```

## Verify

Check the container logs to see when the model has fully loaded into GPU memory:
```bash
docker compose logs -f
```

Once loaded, verify the server is responding to API queries:
```bash
curl http://localhost:8000/v1/models
```
