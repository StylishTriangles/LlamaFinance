import { reactive } from "vue";
import { GasPrice, defaultRegistryTypes } from "@cosmjs/stargate";
import { SigningCosmWasmClient, wasmTypes } from "@cosmjs/cosmwasm-stargate";
import type { GeneratedType } from "@cosmjs/proto-signing";
import { Registry } from "@cosmjs/proto-signing";
import { coreumRegistryTypes } from "../coreum/tx";
import { connectKeplr } from "~/services/keplr";
import { appConfig } from "~/config";
import { getFromLocalStorage } from "~/utils";

const PUBLIC_RPC_ENDPOINT = appConfig.NEXT_PUBLIC_CHAIN_RPC_ENDPOINT || "";
const PUBLIC_CHAIN_ID = appConfig.NEXT_PUBLIC_CHAIN_ID;
const GAS_PRICE = appConfig.NEXT_PUBLIC_GAS_PRICE || "";

class AccountStore {
  walletAddress = null as string | null;
  signingClient = null as SigningCosmWasmClient | null;
  loading = false;
  error = null as any;

  init() {
    const wasConnected = getFromLocalStorage("wasConnected", false);
    if (wasConnected)
      this.connectWallet();
  }

  async connectWallet() {
    this.loading = true;
    try {
      await connectKeplr();

      // enable website to access keplr
      await (window as any).keplr.enable(PUBLIC_CHAIN_ID);

      // get offline signer for signing txs
      const offlineSigner = await (window as any).getOfflineSigner(
        PUBLIC_CHAIN_ID,
      );

      // register default and custom messages
      const registryTypes: ReadonlyArray<[string, GeneratedType]> = [
        ...defaultRegistryTypes,
        ...coreumRegistryTypes,
        ...wasmTypes,
      ];
      const registry = new Registry(registryTypes);

      // signing client
      this.signingClient = await SigningCosmWasmClient.connectWithSigner(
        PUBLIC_RPC_ENDPOINT,
        offlineSigner,
        {
          registry,
          gasPrice: GasPrice.fromString(GAS_PRICE),
        },
      );

      // get user address
      const [{ address }] = await offlineSigner.getAccounts();
      this.walletAddress = address;
      this.loading = false;
      localStorage.setItem("wasConnected", "true"); // In case of refreshing the page, the user will be automatically connected
    } catch (error: any) {
      console.error(error);
      this.error = error;
      this.loading = false;
    }
  }

  async getUserBalance(tokenName: string) {
    if (!this.signingClient || !this.walletAddress)
      return 0;
    const balanceInfo = await this.signingClient.getBalance(this.walletAddress, tokenName);
    if (balanceInfo)
      return Number(balanceInfo.amount);
    else
      return 0;
  }

  disconnect() {
    if (this.signingClient)
      this.signingClient.disconnect();
    this.walletAddress = null;
    this.signingClient = null;
    this.loading = false;
    localStorage.removeItem("wasConnected");
  }
}

export const accountStore = reactive(new AccountStore()) as any as AccountStore;
