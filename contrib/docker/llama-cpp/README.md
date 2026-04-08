# llama.cpp

Docker setup for running [llama.cpp](https://github.com/ggml-org/llama.cpp) (using `llama-server`) as a backend for pgmoneta MCP.

## Quick start

```
docker compose up -d
```

On first launch, the container will download the default model (`Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf`).
This takes a few minutes depending on your connection. Subsequent starts are instant
because models are stored in a persistent Docker volume.

## Configuration & Setups

To avoid disk space issues (`ENOSPC`) with large AI models, you should define a custom docker volume or bind mount point for the model downloads if your root partition is small. 
By default, models are persisted in a Docker volume named `models`. You can override this in `docker-compose.yml` to point to a high-capacity drive (e.g., `/mnt/ai/models:/models`).

We define three standard setups based on your hardware capabilities. Start the container with the respective environment variables:

**Small Setup** (Laptop friendly; ~4GB RAM/Disk):
```sh
MODEL_URL=https://huggingface.co/bartowski/Llama-3.2-3B-Instruct-GGUF/resolve/main/Llama-3.2-3B-Instruct-Q4_K_M.gguf \
MODEL_FILE=Llama-3.2-3B-Instruct-Q4_K_M.gguf \
docker compose up -d
```

**Best Setup** (Recommended balance; ~8GB RAM/Disk):
```sh
MODEL_URL=https://huggingface.co/bartowski/granite-3.0-8b-instruct-GGUF/resolve/main/granite-3.0-8b-instruct-Q4_K_M.gguf \
MODEL_FILE=granite-3.0-8b-instruct-Q4_K_M.gguf \
docker compose up -d
```

**Full Setup** (Workstation; ~40GB+ RAM/Disk):
```sh
MODEL_URL=https://huggingface.co/bartowski/Meta-Llama-3.1-70B-Instruct-GGUF/resolve/main/Meta-Llama-3.1-70B-Instruct-Q4_K_M.gguf \
MODEL_FILE=Meta-Llama-3.1-70B-Instruct-Q4_K_M.gguf \
docker compose up -d
```

## pgmoneta MCP configuration

Add to your `pgmoneta-mcp.conf`:

```ini
[llm]
provider = llama.cpp
endpoint = http://localhost:8080
model = Meta-Llama-3.1-8B-Instruct-Q4_K_M
max_tool_rounds = 10
```

## Verify

```
curl http://localhost:8080/health
```

Should return `{"status":"ok"}`.

## GPU support

The default image `ghcr.io/ggml-org/llama.cpp:server` is CPU-only.
To use an NVIDIA GPU, edit the `Dockerfile` to use `ghcr.io/ggml-org/llama.cpp:server-cuda` 
and add `--n-gpu-layers 99` to the `llama-server` command in `entrypoint.sh`.

Then add the nvidia runtime to `docker-compose.yml`:
```yaml
    deploy:
      resources:
        reservations:
          devices:
            - driver: nvidia
              count: all
              capabilities: [gpu]
```
