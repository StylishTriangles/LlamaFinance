import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { coin } from "@cosmjs/stargate";


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
        return await this.client.execute(
            this.walletAddress,
            this.contractAddress,
            {
                updateAsset: {
                    denom,
                    decimals,
                    target_utilization_rate_bps,
                    min_rate,
                    optimal_rate,
                    max_rate
                }
            },
            "auto",
        );
    }
}
