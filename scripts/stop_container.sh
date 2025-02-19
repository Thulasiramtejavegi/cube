#!/bin/bash

# Stop the Cube.js container if it's running
docker stop cubejs-container || true

# Remove the stopped container
docker rm cubejs-container || true

echo "Stopped and removed Cube.js container."
