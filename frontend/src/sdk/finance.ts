import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { coin } from "@cosmjs/stargate";

interface UserAssetInfo {
    collateral: string,
    borrowAmount: string,
    lAssetAmount: string,
    cumulativeInterest: string,
}

interface AssetConfig {
    targetUtilizationRateBps: number,
    decimals: number,
    minRate: number,
    optimalRate: number,
    maxRate: number,
}
interface AssetInfo {
    denom: string,
    apr: string,
    totalDeposit: string,
    totalBorrow: string,
    totalLAsset: string,
    totalCollateral: string,
    cumulativeInterest: string,
    assetConfig: AssetConfig,
    timestamp: number,
}

export class UserAssetInfoResponse {
    collateral: number
    borrowAmount: number
    lAssetAmount: number
    cumulativeInterest: number

    constructor(uai: UserAssetInfo, decimals: number) {
        let precision = 10**decimals;
        this.collateral = Number(uai.collateral) / precision;
        this.borrowAmount = Number(uai.borrowAmount) / precision;
        this.lAssetAmount = Number(uai.lAssetAmount) / precision;
        this.cumulativeInterest = Number(uai.cumulativeInterest) / 2**64;
    }
}

export class AssetInfoResponse {
    denom: string
    apr: number
    totalDeposit: number
    totalBorrow: number
    totalLAsset: number
    totalCollateral: number
    cumulativeInterest: number
    assetConfig: AssetConfig
    timestamp: number

    constructor(ai: AssetInfo) {
        let precision = 10**ai.assetConfig.decimals;

        this.denom = ai.denom;
        this.apr = Number(ai.apr) / 2**64;
        this.totalDeposit = Number(ai.totalDeposit) / precision;
        this.totalBorrow = Number(ai.totalBorrow) / precision;
        this.totalLAsset = Number(ai.totalLAsset) / precision;
        this.totalCollateral = Number(ai.totalCollateral) / precision;
        this.cumulativeInterest = Number(ai.cumulativeInterest) / 2**64;
        this.assetConfig = ai.assetConfig;
        this.timestamp = ai.timestamp;
    }
}

export class Finance {
    walletAddress: string;
    contractAddress: string;
    client: SigningCosmWasmClient

    constructor(client: SigningCosmWasmClient, walletAddress: string, contractAddress: string) {
        this.walletAddress = walletAddress;
        this.contractAddress = contractAddress;
        this.client = client;
    }

    async deposit(denom: string, amount: number | string) {
        let dep = coin(amount, denom);
        return await this.client.execute(
            this.walletAddress,
            this.contractAddress,
            {
                deposit: {}
            },
            "auto",
            undefined,
            [dep]
        );
    }

    async withdraw(denom: string, amount: number | string) {
        return await this.client.execute(
            this.walletAddress,
            this.contractAddress,
            {
                withdraw: {
                    denom,
                    amount: amount.toString()
                }
            },
            "auto",
        );
    }

    async depositCollateral(denom: string, amount: number | string) {
        let dep = coin(amount, denom);
        return await this.client.execute(
            this.walletAddress,
            this.contractAddress,
            {
                depositCollateral: {}
            },
            "auto",
            undefined,
            [dep]
        );
    }

    async withdrawCollateral(denom: string, amount: number | string) {
        return await this.client.execute(
            this.walletAddress,
            this.contractAddress,
            {
                withdrawCollateral: {
                    denom,
                    amount: amount.toString()
                }
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
                    amount: amount.toString()
                }
            },
            "auto",
        );
    }

    async repay(denom: string, amount: number | string) {
        let dep = coin(amount, denom);
        return await this.client.execute(
            this.walletAddress,
            this.contractAddress,
            {
                repay: {
                    denom,
                    amount: amount.toString()
                }
            },
            "auto",
            undefined,
            [dep]
        );
    }

    async updateAsset(denom: string, decimals: number, target_utilization_rate_bps: number, min_rate: number, optimal_rate: number, max_rate: number) {
        // multiply by 100 to convert percentages to BPS
        let msg = {
            updateAsset: {
                denom,
                decimals,
                target_utilization_rate_bps: Math.floor(target_utilization_rate_bps * 100),
                min_rate: Math.floor(min_rate * 100),
                optimal_rate: Math.floor(optimal_rate * 100),
                max_rate: Math.floor(max_rate * 100),
            }
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
        let res: Array<string> = await this.client.queryContractSmart(this.contractAddress, { assets: {} });
        return res;
    }

    async getUserAssetsInfo(user: string): Promise<Map<string, UserAssetInfoResponse>> {
        let assetsInfo = await this.getAssetsInfoArray();

        let userAssetsInfo: Array<UserAssetInfo> = await this.client.queryContractSmart(
            this.contractAddress,
            {
                userAssetsInfo: {
                    user
                }
            }
        )

        let ret = new Map<string, UserAssetInfoResponse>();
        for (let i = 0; i < assetsInfo.length; i++) {
            ret.set(assetsInfo[i].denom, new UserAssetInfoResponse(userAssetsInfo[i], assetsInfo[i].assetConfig.decimals));
        }
        return ret;
    }

    async getAssetInfo(denom: string) {
        let res: AssetInfo = await this.client.queryContractSmart(this.contractAddress, { assetInfo: { denom } });
        return new AssetInfoResponse(res);
    }

    async getAssetsInfo(): Promise<Map<string, AssetInfoResponse>> {
        let res: Array<AssetInfo> = await this.client.queryContractSmart(this.contractAddress, { assetsInfo: {} });
        let ret = new Map<string, AssetInfoResponse>();
        for (let ai of res) {
            ret.set(ai.denom, new AssetInfoResponse(ai));
        }
        return ret;
    }

    async getAssetsInfoArray(): Promise<Array<AssetInfoResponse>> {
        let res: Array<AssetInfo> = await this.client.queryContractSmart(this.contractAddress, { assetsInfo: {} });
        return res.map(ai => new AssetInfoResponse(ai));
    }
}
