import { assetsData } from "~/config";
import type { AssetInfoResponse } from "~/sdk";
import type { BasicAsset } from "~/types";

export function rawAssetToBasic(
  asset: AssetInfoResponse,
  userBalance: number,
  price: number) {
  return {
    name: assetsData[asset.denom].name,
    icon: assetsData[asset.denom].icon,
    decimals: asset.assetConfig.decimals,
    price,
    balance: userBalance / asset.precision,
    balanceUSD: (userBalance / asset.precision) * price,
    precision: asset.precision,
  } as BasicAsset;
}
