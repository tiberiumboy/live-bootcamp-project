#!/bin/bash

# path to env file
ENV_FILE="./auth-service/.env"

# ensure env file exist
if ! [[ -f "$ENV_FILE" ]]; then
    echo "Error: .env file not found!"
    exit 1
fi

# read each line in the .env file (ignoring comments)
while IFS= read -r line; do
#skip blank lines and lines starting with #
    if [[ -n "$line" ]] && [[ "$line" != \#* ]]; then 
        key=$(echo "$line" | cut -d '=' -f1-)
        value=$(echo "$line" | cut -d '=' -f2-)
        export "$key=$value"
    fi
done < <(grep -v '^#' "$ENV_FILE")

# run docker commands
docker-compose build
docker-compose up