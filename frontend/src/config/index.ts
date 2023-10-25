export const appConfig = {
  NEXT_PUBLIC_CHAIN_ID: "coreum-testnet-1",
  NEXT_PUBLIC_CHAIN_NAME: "Coreum Testnet",
  NEXT_PUBLIC_CHAIN_BECH32_PREFIX: "testcore",
  NEXT_PUBLIC_CHAIN_RPC_ENDPOINT: "https://full-node.testnet-1.coreum.dev:26657/",
  NEXT_PUBLIC_CHAIN_REST_ENDPOINT: "https://full-node.testnet-1.coreum.dev:1317/",
  NEXT_PUBLIC_CHAIN_EXPLORER: "https://explorer.testnet-1.coreum.dev/",
  NEXT_PUBLIC_STAKING_DENOM: "utestcore",
  NEXT_PUBLIC_CHAIN_COIN_TYPE: "990",
  NEXT_PUBLIC_SITE_TITLE: "Coreum starter",
  NEXT_PUBLIC_SITE_ICON_URL: "/coreum.svg",
  NEXT_PUBLIC_GAS_PRICE: "0.0625utestcore",

  ADMIN: "testcore1snhqzau0mdhuqugpknac7udllhdpc462ea6ssx",
};

export const modalsID = {
  DEPOSIT: "deposit_modal",
  WITHDRAW: "withdraw_modal",
  BORROW: "borrow_modal",
  REPAY: "repay_modal",
  COLLATERAL: "collateral_modal",
  REDUCE_COL: "reduce_collateral_modal",
  TXN_SUCCESS: "txn_success",
};

export const contractAddresses = {
  ORACLE_ADDRESS: "testcore1g8m5knlt3ydkhcef9wpufe46u3h0jncywr420n6pcg8fqzma396sdhw532",
  FINANCE_ADDRESS: "testcore1tq05ygjv9d4p2ll5eq0afvmj70d6ww869jj6shx47ddk3jk4mq5s6mcuut",
};

export const assetsData = {
  "utestcore": {
    name: "TESTCORE",
    icon: "https://assets.coingecko.com/coins/images/24169/large/2b7Phdmz_400x400.png",
  },
  "musdt-testcore12d9e2fhlyw9khswzzrq3sakkmvrsms55n6zqff": {
    name: "USDT",
    icon: "https://icons.iconarchive.com/icons/cjdowner/cryptocurrency-flat/512/Tether-USDT-icon.png",
  },
  "sats-testcore12d9e2fhlyw9khswzzrq3sakkmvrsms55n6zqff": {
    name: "BTC",
    icon: "https://upload.wikimedia.org/wikipedia/commons/thumb/4/46/Bitcoin.svg/1200px-Bitcoin.svg.png",
  },
};
