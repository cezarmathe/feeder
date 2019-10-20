#!/usr/bin/env bash

function killer() {
    sleep 1
    local container_id
    container_id=$(docker container ls --format "{{.ID}}")
    echo "Container id: ${container_id}"
    sleep 5
    docker kill "${container_id}"
}
killer &


if ! docker run --rm -p 8000:8000 feeder:"$1"
then
    exit 0
fi
exit 1
