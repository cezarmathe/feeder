#!/usr/bin/env bash

docker run --rm -d -p 8000:8000 feeder:$1

CONTAINER_ID=$?

sleep 5

docker kill ${CONTAINER_ID}
