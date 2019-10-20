#!/usr/bin/env bash

sed -i.bk "s/^LABEL com.cezarmathe.feeder-version=\"[0-9].[0-9].[0-9]\"/LABEL com.cezarmathe.feeder-version=\"${1}\"/" docker/Dockerfile
rm docker/Dockerfile.bk
