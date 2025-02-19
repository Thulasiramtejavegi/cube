#!/bin/bash

# Update package list

# Run Cube.js container
docker run -d --name cubejs-container -p 4000:4000 -p 15432:15432 \
    -v $PWD:/cube/conf -e CUBEJS_DEV_MODE=true cubejs/cube

echo "Started Cube.js container."
