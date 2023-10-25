import type { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";

interface PriceResponse {
  symbol: string;
  price: string;
  precision: string;
}

class PriceInfo {
  symbol: string;
  price_raw: bigint;
  precision: bigint;
  price: number;

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
  client: SigningCosmWasmClient;

  constructor(client: SigningCosmWasmClient, walletAddress: string, contractAddress: string) {
    this.walletAddress = walletAddress;
    this.contractAddress = contractAddress;
    this.client = client;
  }

  async addSymbol(symbol: string) {
    const msg = {
      addSymbol: {
        symbol,
      },
    };
    return await this.client.execute(
      this.walletAddress,
      this.contractAddress,
      msg,
      "auto",
    );
  }

  async setPrice(symbol: string, price: number, precision: number) {
    const msg = {
      setPrice: {
        symbol,
        price: (price * precision).toFixed(0),
        precision: precision.toFixed(0),
      },
    };
    return await this.client.execute(
      this.walletAddress,
      this.contractAddress,
      msg,
      "auto",
    );
  }

  async getPrice(symbol: string): Promise<PriceInfo> {
    const ret: PriceResponse = await this.client.queryContractSmart(
      this.contractAddress,
      {
        price: {
          symbol,
        },
      },
    );

    return new PriceInfo(ret.symbol, ret.price, ret.precision);
  }

  async getPrices(): Promise<Record<string, PriceInfo>> {
    const ret: Array<PriceResponse> = await this.client.queryContractSmart(
      this.contractAddress,
      {
        prices: {},
      },
    );

    // Convert to a map of PriceInfo
    const prices: Record<string, PriceInfo> = {};

    for (let i = 0; i < ret.length; i++) {
      const price = ret[i];
      prices[price.symbol] = new PriceInfo(price.symbol, price.price, price.precision);
    }

    return prices;
  }
}
