#!/bin/bash

NET_NAME=$1
DB_PATH=$2

if [ -z $NET_NAME ];then
    NET_NAME="calla"
fi

if [ -z $DB_PATH ];then
    DB_PATH=`pwd`/db/trick
fi

if [ $NET_NAME = "calla" ];then
    CHAIN_SPEC=`pwd`/chainspec/calla-raw.json
fi

echo -e "Starting boot trick node...\n   network is ${NET_NAME}\n   chainspec at ${CHAIN_SPEC}\n   save data at ${DB_PATH}"


./target/release/tolak-node \
  --base-path ${DB_PATH} \
  --chain ${CHAIN_SPEC} \
  --port 30555 \
  --ws-port 9955 \
  --rpc-port 9995 \
  --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
  --validator \
  --rpc-methods Unsafe \
  --name trick \
  --node-key `cat .secure/.trick-node-key` \
  --bootnodes /ip4/127.0.0.1/tcp/30444/p2p/`cat ./tick-peerid`

sleep 5s

# insert babe & grandpa key
# curl http://127.0.0.1:9995 -H "Content-Type:application/json;charset=utf-8" -d ".secure/.secure-trick-babe-json"
# curl http://127.0.0.1:9995 -H "Content-Type:application/json;charset=utf-8" -d ".secure/.secure-trick-grandpa-json"
