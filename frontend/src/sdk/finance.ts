import type { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { coin } from "@cosmjs/stargate";
import type { Oracle } from "./oracle";

const APR_PRECISION = 10_000;
const MAX_LTV = 0.7;
const MAINTANANCE_MARGIN = 0.1;

interface UserAssetInfo {
  collateral: string;
  borrowAmount: string;
  lAssetAmount: string;
  cumulativeInterest: string;
}

interface AssetConfig {
  targetUtilizationRateBps: number;
  decimals: number;
  minRate: number;
  optimalRate: number;
  maxRate: number;
}
interface AssetInfo {
  denom: string;
  apr: string;
  totalDeposit: string;
  totalBorrow: string;
  totalLAsset: string;
  totalCollateral: string;
  cumulativeInterest: string;
  assetConfig: AssetConfig;
  timestamp: number;
}

export class UserAssetInfoResponse {
  denom: string;
  price_per_unit: number;
  precision: number; // Asset decimals
  collateral: number;
  collateralUSD: number;
  borrowAmount: number;
  borrowAmountUSD: number;
  lAssetAmount: number;
  cumulativeInterest: number;

  constructor(uai: UserAssetInfo, denom: string, decimals: number, price_per_unit: number) {
    const precision = 10 ** decimals;
    this.denom = denom;
    this.price_per_unit = price_per_unit;
    this.precision = precision;
    this.collateral = Number(uai.collateral) / precision;
    this.collateralUSD = this.collateral * price_per_unit * precision;
    this.borrowAmount = Number(uai.borrowAmount) / precision;
    this.borrowAmountUSD = this.borrowAmount * price_per_unit * precision;
    this.lAssetAmount = Number(uai.lAssetAmount) / precision;
    this.cumulativeInterest = Number(uai.cumulativeInterest) / 2 ** 64;
  }
}

export class AssetInfoResponse {
  denom: string;
  apr: number;
  price_per_unit: number;
  precision: number; // The precision of the assets decimals -> ex. USDC has 1000000
  totalDeposit: number;
  totalDepositUSD: number;
  totalBorrow: number;
  totalBorrowUSD: number;
  totalLAsset: number;
  totalCollateral: number;
  totalCollateralUSD: number;
  cumulativeInterest: number;
  assetConfig: AssetConfig;
  timestamp: number;

  constructor(ai: AssetInfo, price_per_unit: number) {
    const precision = 10 ** ai.assetConfig.decimals;

    this.denom = ai.denom;
    this.apr = Number(ai.apr) * 100 / APR_PRECISION;
    this.price_per_unit = price_per_unit;
    this.precision = precision;
    this.totalDeposit = Number(ai.totalDeposit) / precision;
    this.totalDepositUSD = this.totalDeposit * price_per_unit * precision;
    this.totalBorrow = Number(ai.totalBorrow) / precision;
    this.totalBorrowUSD = this.totalBorrow * price_per_unit * precision;
    this.totalLAsset = Number(ai.totalLAsset) / precision;
    this.totalCollateral = Number(ai.totalCollateral) / precision;
    this.totalCollateralUSD = this.totalCollateral * price_per_unit * precision;
    this.cumulativeInterest = Number(ai.cumulativeInterest) / 2 ** 64;
    this.assetConfig = ai.assetConfig;
    this.timestamp = ai.timestamp;
  }
}

export class Finance {
  walletAddress: string;
  contractAddress: string;
  client: SigningCosmWasmClient;
  oracle: Oracle;

  constructor(client: SigningCosmWasmClient, walletAddress: string, contractAddress: string, oracle: Oracle) {
    this.walletAddress = walletAddress;
    this.contractAddress = contractAddress;
    this.client = client;
    this.oracle = oracle;
  }

  async deposit(denom: string, amount: number | string) {
    const dep = coin(amount, denom);
    return await this.client.execute(
      this.walletAddress,
      this.contractAddress,
      {
        deposit: {},
      },
      "auto",
      undefined,
      [dep],
    );
  }

  async withdraw(denom: string, amount: number | string) {
    return await this.client.execute(
      this.walletAddress,
      this.contractAddress,
      {
        withdraw: {
          denom,
          amount: amount.toString(),
        },
      },
      "auto",
    );
  }

  async depositCollateral(denom: string, amount: number | string) {
    const dep = coin(amount, denom);
    return await this.client.execute(
      this.walletAddress,
      this.contractAddress,
      {
        depositCollateral: {},
      },
      "auto",
      undefined,
      [dep],
    );
  }

  async withdrawCollateral(denom: string, amount: number | string) {
    return await this.client.execute(
      this.walletAddress,
      this.contractAddress,
      {
        withdrawCollateral: {
          denom,
          amount: amount.toString(),
        },
      },
      "auto",
    );
  }

  async borrow(denom: string, amount: number | string) {
    return await this.client.execute(
      this.walletAddress,
      this.contractAddress,
      {
        borrow: {
          denom,
          amount: amount.toString(),
        },
      },
      "auto",
    );
  }

  async repay(denom: string, amount: number | string) {
    const dep = coin(amount, denom);
    return await this.client.execute(
      this.walletAddress,
      this.contractAddress,
      {
        repay: {
          denom,
          amount: amount.toString(),
        },
      },
      "auto",
      undefined,
      [dep],
    );
  }

  async updateAsset(denom: string, decimals: number, target_utilization_rate_bps: number, min_rate: number, optimal_rate: number, max_rate: number) {
    // multiply by 100 to convert percentages to BPS
    const msg = {
      updateAsset: {
        denom,
        decimals,
        target_utilization_rate_bps: Math.floor(target_utilization_rate_bps * 100),
        min_rate: Math.floor(min_rate * 100),
        optimal_rate: Math.floor(optimal_rate * 100),
        max_rate: Math.floor(max_rate * 100),
      },
    };
    console.log(msg);
    return await this.client.execute(
      this.walletAddress,
      this.contractAddress,
      msg,
      "auto",
    );
  }

  // Queries
  async getAssets() {
    const res: Array<string> = await this.client.queryContractSmart(this.contractAddress, { assets: {} });
    return res;
  }

  async getUserAssetsInfo(user: string): Promise<Map<string, UserAssetInfoResponse>> {
    const assetsInfo = await this.getAssetsInfoArray();
    const prices = await this.oracle.getPrices();

    const userAssetsInfo: Array<UserAssetInfo> = await this.client.queryContractSmart(
      this.contractAddress,
      {
        userAssetsInfo: {
          user,
        },
      },
    );

    const ret = new Map<string, UserAssetInfoResponse>();
    for (let i = 0; i < assetsInfo.length; i++)
      ret.set(
        assetsInfo[i].denom,
        new UserAssetInfoResponse(
          userAssetsInfo[i],
          assetsInfo[i].denom,
          assetsInfo[i].assetConfig.decimals,
          prices[assetsInfo[i].denom].price,
        ),
      );

    return ret;
  }

  async getAssetInfo(denom: string) {
    const res: AssetInfo = await this.client.queryContractSmart(this.contractAddress, { assetInfo: { denom } });
    const price = await this.oracle.getPrice(denom);
    return new AssetInfoResponse(res, price.price);
  }

  async getAssetsInfo(): Promise<Map<string, AssetInfoResponse>> {
    const prices = await this.oracle.getPrices();
    const res: Array<AssetInfo> = await this.client.queryContractSmart(this.contractAddress, { assetsInfo: {} });
    const ret = new Map<string, AssetInfoResponse>();
    for (const ai of res)
      ret.set(ai.denom, new AssetInfoResponse(ai, prices[ai.denom].price));

    return ret;
  }

  async getAssetsInfoArray(): Promise<Array<AssetInfoResponse>> {
    const prices = await this.oracle.getPrices();
    const res: Array<AssetInfo> = await this.client.queryContractSmart(this.contractAddress, { assetsInfo: {} });
    return res.map(ai => new AssetInfoResponse(ai, prices[ai.denom].price));
  }

  getLTV(data: Map<string, UserAssetInfoResponse>) {
    let totalCollateralUSD = 0;
    let totalBorrowUSD = 0;
    for (const uai of data.values()) {
      totalCollateralUSD += uai.collateralUSD;
      totalBorrowUSD += uai.borrowAmountUSD;
    }
    return totalBorrowUSD / totalCollateralUSD;
  }

  /// Positive delta means that collateral was added, negative means removed
  getLTVafter(data: Map<string, UserAssetInfoResponse>, denom: string, delta: number) {
    let totalCollateralUSD = 0;
    let totalBorrowUSD = 0;
    for (const uai of data.values()) {
      if (uai.denom === denom)
        totalCollateralUSD += (uai.collateral + delta) * uai.price_per_unit * uai.precision;
      else
        totalCollateralUSD += uai.collateralUSD;
      totalBorrowUSD += uai.borrowAmountUSD;
    }
    if (totalCollateralUSD < 1e-6)
      return 0;

    return totalBorrowUSD / totalCollateralUSD;
  }

  getLiquidationMargin(ltv: number) {
    return 1 - ltv / MAX_LTV;
  }

  getMaxLoanAmount(data: Map<string, UserAssetInfoResponse>, denom: string) {
    const ai = data.get(denom);
    if (!ai)
      return 0;
    let totalCollateralUSD = 0;
    let totalBorrowUSD = 0;
    for (const uai of data.values()) {
      totalCollateralUSD += uai.collateralUSD;
      totalBorrowUSD += uai.borrowAmountUSD;
    }

    const ltv = totalBorrowUSD / totalCollateralUSD;
    const liquidationMargin = this.getLiquidationMargin(ltv) - MAINTANANCE_MARGIN;
    const maxLoanAmountUSD = totalCollateralUSD * liquidationMargin;
    return maxLoanAmountUSD / ai.price_per_unit / ai.precision;
  }
}
