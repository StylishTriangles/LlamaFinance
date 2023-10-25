source .env.testnet

INIT="{\"admin\": \"$ADMIN\"}"
cored tx wasm instantiate $ORACLE_CODE_ID "$INIT" --from wallet --label "Llama Oracle" -y --no-admin $COREUM_NODE_ARGS $COREUM_CHAIN_ID_ARGS

