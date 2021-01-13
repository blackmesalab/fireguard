#!/usr/bin/env bash

if [ $# -ne 6 ]; then
  echo "usage: $0 <git repository> <repository name> <username> <peername>"
  echo "example: $0 https://github.com/blackmesalab/novanet novanet bigo rpi-home"
  exit 1
fi

REPOSITORY=$1
REPONAME=$2
USERNAME=$3
PEERNAME=$4
PRIVKEY=$5

fireguard repo clone -r "${REPOSITORY}"
fireguard wg -r "${REPONAME}" render -u "${USERNAME}" -p "${PEERNAME}" -P "${PRIVKEY}"
fireguard wg -r "${REPONAME}" up
fireguard proxy -r "${REPONAME}" up
# fireguard dns -r ${REPONAME} up
# fireguard quagga -r "${REPONAME}" up
