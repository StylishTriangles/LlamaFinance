source .env.testnet

INIT="{\"oracle\": \"$ORACLE_ADDRESS\"}"
cored tx wasm instantiate $FINANCE_CODE_ID "$INIT" --from wallet --label "Llama Oracle" -y --no-admin $COREUM_NODE_ARGS $COREUM_CHAIN_ID_ARGS

