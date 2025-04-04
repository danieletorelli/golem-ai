#/bin/bash

set -euo pipefail

ID=${1?}

golem-cli app deploy

golem-cli worker delete golem-ai:input-analyzer/golem-ai-input-analyzer-${ID} || true
golem-cli worker new golem-ai:input-analyzer/golem-ai-input-analyzer-${ID} --env OPENAI_API_KEY="${OPENAI_API_KEY}"

golem-cli api deployment delete localhost:9006 || true
golem-cli api definition delete --id golem-ai --version 0.0.2 || true

golem-cli api definition new api-definition.yaml
golem-cli api deployment deploy golem-ai/0.0.2 --host=localhost:9006