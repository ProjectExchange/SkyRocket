#!/bin/sh
#
# This simple bash script utilizes the swagger-codegen container to generate an
# API-client from the backend documentation.
#
local_dir="$(readlink -f $(dirname $0))"

if command -v podman >/dev/null 2>&1
then
  cmd="podman run --security-opt label=disable"
else
  cmd="docker run"
fi

$cmd --rm --network host \
  -v "${local_dir}":/local \
  -v "${local_dir}/../lib/ng-api-client":/out \
  swaggerapi/swagger-codegen-cli-v3 generate \
    -i http://localhost:8000/v1/openapi.json \
    -l typescript-angular \
    -o /out \
    -c /local/options.json
