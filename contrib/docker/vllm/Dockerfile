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

# vLLM Docker setup for pgmoneta_mcp
#
# Provides the vLLM OpenAI-compatible server.
#
# Build:
#   docker build -t pgmoneta-mcp-vllm .
#
# Run:
#   docker compose up -d

FROM vllm/vllm-openai:latest

# We simply expose the default vLLM port
EXPOSE 8000
