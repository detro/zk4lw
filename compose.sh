#!/usr/bin/env bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
COMPOSE_DIR="${SCRIPT_DIR}/fixtures/docker-compose"
COMPOSE_DATA_DIR="${COMPOSE_DIR}/.data"

! which docker-compose >/dev/null && echo "ERROR: Requires Docker Compose" && exit 1

if [[ $# -lt 2 ]] || [[ $# -gt 3 ]]; then
  echo "Description:"
  echo "  Spins up and shuts down dependencies for this application, using Docker Compose."
  echo "  This is useful for local development/testing."
  echo ""
  echo "Usage:"
  echo "  $(basename ${BASH_SOURCE}) <TYPE: ensemble|standalone> <ACTION: up|down> [OPTION: attach|clean]"
  echo ""
  echo "Options:"
  echo "  attach    After launch, place Docker Compose logs in foreground (only for 'up' action)"
  echo "  clean     After shutdown, delete any data directory (only for 'down' action)"
  echo ""
  exit 1
fi

TYPE=${1}
ACTION=${2}
OPTION=${3}
COMPOSE_FILE="${COMPOSE_DIR}/${TYPE}.yml"

[[ ! -f ${COMPOSE_FILE} ]] && echo "ERROR: Unknown compose type" && exit 1

if [[ ${ACTION} == "up" ]]; then

  if [[ ${OPTION} == "attach" ]]; then
    docker-compose -f ${COMPOSE_FILE} up
  else
    docker-compose -f ${COMPOSE_FILE} up --detach
  fi

elif [[ ${ACTION} == "down" ]]; then
  docker-compose -f ${COMPOSE_FILE} down --remove-orphans --volumes

  if [[ ${OPTION} == "clean" ]]; then
    rm -r ${COMPOSE_DATA_DIR}
  fi

else
  echo "Error: unknown action '${COMPOSE_ACTION}'"
  exit 1
fi
