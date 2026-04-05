# Copyright (C) 2026 The pgmoneta community
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.

# llama.cpp Docker setup for pgmoneta_mcp
#
# Provides the llama-server runtime with a default GGUF model
# downloaded on first startup. Models are persisted via a Docker volume.
#
# Build:
#   docker build -t pgmoneta-mcp-llama-cpp .
#
# Run:
#   docker compose up -d

FROM ghcr.io/ggml-org/llama.cpp:server

# Install curl for model downloads
RUN apt-get update && apt-get install -y --no-install-recommends curl ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

ENV MODEL_URL=https://huggingface.co/bartowski/Meta-Llama-3.1-8B-Instruct-GGUF/resolve/main/Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf
ENV MODEL_FILE=Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf
ENV CTX_SIZE=8192

EXPOSE 8080

ENTRYPOINT ["/entrypoint.sh"]
