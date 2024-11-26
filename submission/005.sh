#!/bin/bash
USER="-rpcuser=user_106"
PASSWORD="-rpcpassword=4wjt7NoqJHld"
IP="-rpcconnect=84.247.182.145"
TXID="37d966a263350fe747f1c606b159987545844a493dd38d84b070027a895c4517"

HEX_TRANSACTION=$(bitcoin-cli $USER $PASSWORD $IP getrawtransaction $TXID)
TRANSACTION=$(bitcoin-cli $USER $PASSWORD $IP decoderawtransaction "$HEX_TRANSACTION")
PUB_KEYS=$(echo "$TRANSACTION" | jq -r '.vin[] | .txinwitness[1]')

PUB_KEYS_ARRAY=($(echo "$PUB_KEYS"))

PUB_KEY1="${PUB_KEYS_ARRAY[0]}"
PUB_KEY2="${PUB_KEYS_ARRAY[1]}"
PUB_KEY3="${PUB_KEYS_ARRAY[2]}"
PUB_KEY4="${PUB_KEYS_ARRAY[3]}"

MULTISIG=$(bitcoin-cli $USER $PASSWORD $IP createmultisig 1 "[\"$PUB_KEY1\", \"$PUB_KEY2\", \"$PUB_KEY3\", \"$PUB_KEY4\"]")
P2SH_ADDRESS=$(echo "$MULTISIG" | jq -r '.address')

echo "$P2SH_ADDRESS"