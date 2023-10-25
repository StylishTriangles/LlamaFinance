import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";

interface PriceResponse {
    symbol: string
    price: string
    precision: string
}

class PriceInfo {
    symbol: string
    price_raw: BigInt
    precision: BigInt
    price: number

    constructor(symbol: string, price: string, precision: string) {
        this.symbol = symbol;
        this.price_raw = BigInt(price);
        this.precision = BigInt(precision);
        this.price = Number(price) / Number(precision);
    }
}

export class Oracle {
    walletAddress: string;
    contractAddress: string;
    client: SigningCosmWasmClient

    constructor(client: SigningCosmWasmClient, walletAddress: string, contractAddress: string) {
        this.walletAddress = walletAddress;
        this.contractAddress = contractAddress;
        this.client = client;
    }

    async addSymbol(symbol: string) {
        let msg = {
            addSymbol: {
                symbol: symbol
            }
        };
        return await this.client.execute(
            this.walletAddress,
            this.contractAddress,
            msg,
            "auto"
        );
    }

    async setPrice(symbol: string, price: number, precision: number) {
        let msg = {
            setPrice: {
                symbol,
                price: (price * precision).toFixed(0),
                precision: precision.toFixed(0)
            }
        };
        return await this.client.execute(
            this.walletAddress,
            this.contractAddress,
            msg,
            "auto"
        );
    }

    async getPrice(symbol: string): Promise<PriceInfo> {
        let ret: PriceResponse  = await this.client.queryContractSmart(
            this.contractAddress,
            {
                price: {
                    symbol: symbol
                }
            }
        )
        
        return new PriceInfo(ret.symbol, ret.price, ret.precision);
    }

    async getPrices(): Promise<Record<string, PriceInfo>> {
        let ret: Array<PriceResponse> = await this.client.queryContractSmart(
            this.contractAddress,
            {
                prices: {}
            }
        )

        // Convert to a map of PriceInfo
        let prices: Record<string, PriceInfo> = {};

        for (let i = 0; i < ret.length; i++) {
            let price = ret[i];
            prices[price.symbol] = new PriceInfo(price.symbol, price.price, price.precision);
        }

        return prices;
    }
}