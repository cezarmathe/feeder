#!/usr/bin/env bash

CONTAINER_ID=$(docker run --rm -d -p 8000:8000 feeder:$1)

sleep 5

docker kill ${CONTAINER_ID}
