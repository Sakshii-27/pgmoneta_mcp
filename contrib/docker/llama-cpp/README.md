# llama.cpp

Docker setup for running [llama.cpp](https://github.com/ggml-org/llama.cpp) (using `llama-server`) as a backend for pgmoneta MCP.

## Quick start

```
docker compose up -d
```

On first launch, the container will download the default model (`Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf`).
This takes a few minutes depending on your connection. Subsequent starts are instant
because models are stored in a persistent Docker volume.

## Configuration

To use a different model, set the URL and filename environment variables:

```
MODEL_URL=https://huggingface.co/Qwen/Qwen2.5-7B-Instruct-GGUF/resolve/main/qwen2.5-7b-instruct-q4_k_m.gguf \
MODEL_FILE=qwen2.5-7b-instruct-q4_k_m.gguf \
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
