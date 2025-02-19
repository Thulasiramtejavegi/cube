#!/bin/bash

# Update package list
sudo apt update -y

# Install Docker if not installed
if ! command -v docker &> /dev/null; then
    sudo apt install -y docker.io
    sudo systemctl start docker
    sudo systemctl enable docker
fi

# Run Cube.js container
docker run -d --name cubejs-container -p 4000:4000 -p 15432:15432 \
    -v $PWD:/cube/conf -e CUBEJS_DEV_MODE=true cubejs/cube

echo "Started Cube.js container."
