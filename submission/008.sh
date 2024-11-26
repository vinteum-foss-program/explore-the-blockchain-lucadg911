#!/bin/bash
USER="-rpcuser=user_106"
PASSWORD="-rpcpassword=4wjt7NoqJHld"
IP="-rpcconnect=84.247.182.145"

TXID="e5969add849689854ac7f28e45628b89f7454b83e9699e551ce14b6f90c86163"

TRANSACTION=$(bitcoin-cli $USER $PASSWORD $IP getrawtransaction $TXID true)
TXINWITNESS=$(echo "$TRANSACTION" | jq -r '.vin[0].txinwitness[2]')
PUB_KEY1=$(echo "$TXINWITNESS" | cut -c 5-70)
echo "$PUB_KEY1"