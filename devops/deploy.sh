source .env.testnet

./devops/build.sh

RES=$(cored tx wasm store artifacts/oracle.wasm \
    --from wallet --gas auto --gas-adjustment 1.3 -y -b sync \
    --output json --log_format json --trace --log_level trace $COREUM_NODE_ARGS $COREUM_CHAIN_ID_ARGS)
echo $RES    
# CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[-1].value')
# echo $CODE_ID

sleep 1

RES=$(cored tx wasm store artifacts/llama_finance.wasm \
    --from wallet --gas auto --gas-adjustment 1.3 -y -b sync \
    --output json --log_format json --trace --log_level trace $COREUM_NODE_ARGS $COREUM_CHAIN_ID_ARGS)
echo $RES
# CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[-1].value')
# echo $CODE_ID
