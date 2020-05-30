#!/usr/bin/env bash

DATA_DIR=".data"
SUPPORTED_VERSIONS=(3.4 3.5 3.6)
REQUIRE_CLIENT_PORT_SUFFIX=(3.5 3.6)

COMPOSE_ABS_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
COMPOSE_ABS_DATA_DIR="${COMPOSE_ABS_DIR}/${DATA_DIR}"

! which docker-compose >/dev/null && echo "ERROR: Requires Docker Compose" && exit 1

if [[ $# -lt 3 ]] || [[ $# -gt 4 ]]; then
  echo "Description:"
  echo "  Spins up and shuts down dependencies for this application, using Docker Compose."
  echo "  This is useful for local development/testing."
  echo ""
  echo "Usage:"
  echo "  $(basename ${BASH_SOURCE}) <VERSION: 3.4|3.5|3.6> <TYPE: ensemble|standalone> <ACTION: up|down> [OPTION: attach|clean]"
  echo ""
  echo "Options:"
  echo "  attach    After launch, place Docker Compose logs in foreground (only for 'up' action)"
  echo "  clean     After shutdown, delete any data directory (only for 'down' action)"
  echo ""
  exit 1
fi

VERSION=${1}
TYPE=${2}
ACTION=${3}
OPTION=${4}
COMPOSE_FILE="${COMPOSE_ABS_DIR}/${TYPE}.yml"

# --------------------------------------------------------------- VALIDATE INPUT
[[ ! -f ${COMPOSE_FILE} ]] && echo "ERROR: Unknown compose setup: ${COMPOSE_FILE}" && exit 1
[[ ! "${SUPPORTED_VERSIONS[@]}" =~ "${VERSION}" ]] && echo "ERROR: Unsupported version: ${VERSION}" && exit 1

CLIENT_PORT_SUFFIX=""
[[ "${REQUIRE_CLIENT_PORT_SUFFIX[@]}" =~ "${VERSION}" ]] && CLIENT_PORT_SUFFIX=";2181"

# ---------------------------------------------------------------------- EXECUTE
if [[ ${ACTION} == "up" ]]; then
  if [[ ${OPTION} == "attach" ]]; then
    ZK_VERSION=${VERSION} \
    ZK_LOCAL_DATA_DIR=${DATA_DIR} \
    ZK_CLIENT_PORT_SUFFIX=${CLIENT_PORT_SUFFIX} \
        docker-compose -f ${COMPOSE_FILE} up
  else
    ZK_VERSION=${VERSION} \
    ZK_LOCAL_DATA_DIR=${DATA_DIR} \
    ZK_CLIENT_PORT_SUFFIX=${CLIENT_PORT_SUFFIX} \
        docker-compose -f ${COMPOSE_FILE} up --detach
  fi
elif [[ ${ACTION} == "down" ]]; then
  ZK_VERSION=${VERSION} \
  ZK_LOCAL_DATA_DIR=${DATA_DIR} \
  ZK_CLIENT_PORT_SUFFIX=${CLIENT_PORT_SUFFIX} \
    docker-compose -f ${COMPOSE_FILE} down --remove-orphans --volumes

  if [[ ${OPTION} == "clean" ]]; then
    rm -r ${COMPOSE_ABS_DATA_DIR}
  fi
else
  echo "Error: unknown action '${ACTION}'"
  exit 1
fi
