#!/bin/bash

NET_NAME=$1
DB_PATH=$2

if [ -z $NET_NAME ];then
    NET_NAME="calla"
fi

if [ -z $DB_PATH ];then
    DB_PATH=`pwd`/db/tick
fi

if [ $NET_NAME = "calla" ];then
    CHAIN_SPEC=`pwd`/chainspec/calla-raw.json
fi

echo -e "Starting boot tick node...\n   network is ${NET_NAME}\n   chainspec at ${CHAIN_SPEC}\n   save data at ${DB_PATH}"


./target/release/tolak-node \
  --base-path ${DB_PATH} \
  --chain ${CHAIN_SPEC} \
  --port 30444 \
  --ws-port 9944 \
  --rpc-port 9994 \
  --validator \
  --rpc-methods Unsafe \
  --name tick \
  --node-key `cat .secure/.tick-node-key`

sleep 5s

# insert babe & grandpa key
# curl http://127.0.0.1:9994 -H "Content-Type:application/json;charset=utf-8" -d ".secure/.secure-tick-babe-json"
# curl http://127.0.0.1:9994 -H "Content-Type:application/json;charset=utf-8" -d ".secure/.secure-tick-grandpa-json"

