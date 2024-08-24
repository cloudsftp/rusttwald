#!/usr/bin/env sh

# get api spec:
# curl https://api.mittwald.de/v2/openapi.json > original-api.json

# filter api spec:
# https://github.com/Mermade/openapi-filter

# get openapi-generator-cli:
# wget https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/7.7.0/openapi-generator-cli-7.7.0.jar -O openapi-generator-cli.jar

java -jar openapi-generator-cli.jar \
    generate \
    -g rust \
    -i original-api.json \
    --skip-validate-spec
