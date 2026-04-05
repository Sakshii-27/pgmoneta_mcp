#!/bin/bash
#
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
#
# Entrypoint for the llama.cpp server container.
#
# Downloads the default GGUF model on first run,
# then starts llama-server.

set -e

MODEL_DIR="/models"
MODEL_PATH="${MODEL_DIR}/${MODEL_FILE}"

# Download model if not already present
if [ ! -f "${MODEL_PATH}" ]; then
    echo "Downloading model: ${MODEL_FILE}"
    echo "This may take several minutes..."
    curl -L -o "${MODEL_PATH}" "${MODEL_URL}"
    echo "Download complete."
else
    echo "Model ${MODEL_FILE} already available."
fi

echo "Starting llama-server..."
exec llama-server \
    --model "${MODEL_PATH}" \
    --host 0.0.0.0 \
    --port 8080 \
    --ctx-size "${CTX_SIZE}"
