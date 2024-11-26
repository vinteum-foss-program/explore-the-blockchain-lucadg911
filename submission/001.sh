#!/bin/bash
USER="-rpcuser=user_106"
PASSWORD="-rpcpassword=4wjt7NoqJHld"
IP="-rpcconnect=84.247.182.145"

HEIGHT=654321

BLOCK_HASH=$(bitcoin-cli $USER $PASSWORD $IP getblockhash "$HEIGHT")
echo "$BLOCK_HASH"