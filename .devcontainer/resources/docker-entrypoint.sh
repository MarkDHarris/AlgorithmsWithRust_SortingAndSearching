#!/bin/bash
set -e

# Start the Docker daemon if no socket is mounted
if [ ! -S /var/run/docker.sock ]; then
    echo "Starting Docker daemon..."
    dockerd-entrypoint.sh &

    # Wait for Docker daemon to be ready
    timeout=60
    while ! docker info >/dev/null 2>&1; do
        sleep 1
        timeout=$((timeout - 1))
        if [ $timeout -eq 0 ]; then
            echo "Docker daemon failed to start."
            exit 1
        fi
    done
fi

exec "$@"
