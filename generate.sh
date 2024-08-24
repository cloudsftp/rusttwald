#!/usr/bin/env sh

java -jar openapi-generator-cli.jar \
    generate \
    -g rust \
    -i api.json \
    --skip-validate-spec
