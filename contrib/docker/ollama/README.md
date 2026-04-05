# Ollama

Docker setup for running [Ollama](https://ollama.com) as a backend for pgmoneta MCP.

## Quick start

```
docker compose up -d
```

On first launch, the container will download the default model (`llama3.1:8b`).
This takes a few minutes depending on your connection. Subsequent starts are instant
because models are stored in a persistent Docker volume.

## Configuration

Set the `OLLAMA_MODEL` environment variable to use a different model:

```
OLLAMA_MODEL=qwen2.5:7b docker compose up -d
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
