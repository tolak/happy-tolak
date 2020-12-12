#!/bin/bash

NET_NAME=$1
DB_PATH=$2

if [ -z $NET_NAME ];then
    NET_NAME="calla"
fi

if [ -z $DB_PATH ];then
    DB_PATH=`pwd`/db/track
fi

if [ $NET_NAME = "calla" ];then
    CHAIN_SPEC=`pwd`/chainspec/calla-raw.json
fi

echo -e "Starting boot track node...\n   network is ${NET_NAME}\n   chainspec at ${CHAIN_SPEC}\n   save data at ${DB_PATH}"


./target/release/tolak-node \
  --base-path ${DB_PATH} \
  --chain ${CHAIN_SPEC} \
  --port 30666 \
  --ws-port 9966 \
  --rpc-port 9996 \
  --validator \
  --rpc-methods Unsafe \
  --name track \
  --node-key `cat .secure/.track-node-key` \
  --bootnodes /ip4/127.0.0.1/tcp/30444/p2p/`cat ./tick-peerid`

sleep 5s

# insert babe & grandpa key
# curl http://127.0.0.1:9996 -H "Content-Type:application/json;charset=utf-8" -d ".secure/.secure-track-babe-json"
# curl http://127.0.0.1:9996 -H "Content-Type:application/json;charset=utf-8" -d ".secure/.secure-track-grandpa-json"

