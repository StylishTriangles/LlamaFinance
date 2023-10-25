source .env.testnet

USDT="musdt-testcore12d9e2fhlyw9khswzzrq3sakkmvrsms55n6zqff"
BTC="sats-testcore12d9e2fhlyw9khswzzrq3sakkmvrsms55n6zqff"
ADMIN="testcore12d9e2fhlyw9khswzzrq3sakkmvrsms55n6zqff"
PATRYK="testcore1dq3wt07r5pu83kj5l0du25nq5t9adm36tum3nm"
PABLO="testcore1rvasq87skm2vkhqlnyfenh3yuvgqg6ch6zk3n8"

cored tx bank send $ADMIN $PATRYK 100000000000$USDT --from wallet -y $COREUM_NODE_ARGS $COREUM_CHAIN_ID_ARGS
cored tx bank send $ADMIN $PATRYK 100000000$BTC --from wallet -y $COREUM_NODE_ARGS $COREUM_CHAIN_ID_ARGS
# cored tx assetft mint 1000000000$BTC --from wallet --recipient $PATRYK -y $COREUM_NODE_ARGS $COREUM_CHAIN_ID_ARGS


cored tx bank send $ADMIN $PABLO 100000000000$USDT --from wallet -y $COREUM_NODE_ARGS $COREUM_CHAIN_ID_ARGS
cored tx bank send $ADMIN $PABLO 100000000$BTC --from wallet -y $COREUM_NODE_ARGS $COREUM_CHAIN_ID_ARGS

# cored tx assetft mint 1000000000000$USDC --from wallet --recipient $PABLO -y $COREUM_NODE_ARGS $COREUM_CHAIN_ID_ARGS
# cored tx assetft mint 1000000000$BTC --from wallet --recipient $PABLO -y $COREUM_NODE_ARGS $COREUM_CHAIN_ID_ARGS