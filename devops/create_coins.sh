source .env.testnet

cored tx assetft issue USDt musdt 6 1000000000000 "Tether USD" --from wallet --features=burning,freezing,minting -y $COREUM_NODE_ARGS $COREUM_CHAIN_ID_ARGS
cored tx assetft issue BTC2 sats 8 3000000000 "Bitcoin" --from wallet --features=burning,freezing,minting -y $COREUM_NODE_ARGS $COREUM_CHAIN_ID_ARGS
