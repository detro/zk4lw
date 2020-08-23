#!/usr/bin/env bash

SUPPORTED_VERSIONS=(3.4 3.5 3.6)

for ZK_VERSION in ${SUPPORTED_VERSIONS[@]}; do
  echo "Pulling latest zookeeper:${ZK_VERSION}"
  docker pull zookeeper:${ZK_VERSION}
  echo ""
done

echo "Cleaning up"
docker image prune
echo ""