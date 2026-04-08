# Ollama

Docker setup for running [Ollama](https://ollama.com) as a backend for pgmoneta MCP.

## Quick start

```
docker compose up -d
```

On first launch, the container will download the default model (`llama3.1:8b`).
This takes a few minutes depending on your connection. Subsequent starts are instant
because models are stored in a persistent Docker volume.

## Configuration & Setups

To avoid disk space issues (`ENOSPC`) with large AI models, you should define a custom docker volume or bind mount point for the model downloads if your root partition is small.
By default, models are persisted in a Docker volume named `ollama`. You can override this in `docker-compose.yml` to point to a high-capacity drive (e.g., `/mnt/ai/ollama:/root/.ollama`).

We define three standard setups based on your hardware capabilities. Start the container with the respective environment variable:

**Small Setup** (Laptop friendly; ~4GB RAM/Disk):
```sh
OLLAMA_MODEL=llama3.2:3b docker compose up -d
```

**Best Setup** (Recommended balance; ~8GB RAM/Disk):
```sh
OLLAMA_MODEL=granite-code:8b docker compose up -d
```

**Full Setup** (Workstation; ~40GB+ RAM/Disk):
```sh
OLLAMA_MODEL=llama3.1:70b docker compose up -d
```

## pgmoneta MCP configuration

Add to your `pgmoneta-mcp.conf`:

```ini
[llm]
provider = ollama
endpoint = http://localhost:11434
model = llama3.1
max_tool_rounds = 10
```

## Verify

```
curl http://localhost:11434/
```

Should print `Ollama is running`.

## GPU support

For NVIDIA GPU acceleration on Linux, add the following to the `ollama` service
in `docker-compose.yml`:

```yaml
    deploy:
      resources:
        reservations:
          devices:
            - driver: nvidia
              count: all
              capabilities: [gpu]
```
