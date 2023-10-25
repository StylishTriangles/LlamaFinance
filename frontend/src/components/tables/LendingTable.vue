<script lang="ts" setup>
import { modalsID } from "~/config";
import type { TableColumn } from "~/types";
import { formatAssetAmount, formatPctValue, formatUSDAmount } from "~/utils";

const columns = [
  {
    header: "Asset",
    accessor: "asset",
  },
  {
    header: "Wallet Balance",
    accessor: "balance",
  },
  {
    header: "APY",
    accessor: "apy",
  },
  {
    header: "Total Deposited",
    accessor: "total_deposited",
  },
  {
    header: "",
    accessor: "action",
  },
] as TableColumn[];

const tableData = [
  {
    asset: "CORE",
    balance: formatAssetAmount(8990211.88),
    balance_usd: formatUSDAmount(8990211.88),
    total_deposited: formatAssetAmount(8990211.88),
    total_deposited_usd: formatUSDAmount(8990211.88),
    apy: formatPctValue(20.96),
  },
  {
    asset: "BTC",
    balance: formatAssetAmount(8990211.88),
    balance_usd: formatUSDAmount(8990211.88),
    total_deposited: formatAssetAmount(8990211.88),
    total_deposited_usd: formatUSDAmount(8990211.88),
    apy: formatPctValue(20.96),
  },
  {
    asset: "ETH",
    balance: formatAssetAmount(8990211.88),
    balance_usd: formatUSDAmount(8990211.88),
    total_deposited: formatAssetAmount(8990211.88),
    total_deposited_usd: formatUSDAmount(8990211.88),
    apy: formatPctValue(20.96),
  },
  {
    asset: "USDC",
    balance: formatAssetAmount(8990211.88),
    balance_usd: formatUSDAmount(8990211.88),
    total_deposited: formatAssetAmount(8990211.88),
    total_deposited_usd: formatUSDAmount(8990211.88),
    apy: formatPctValue(20.96),
  },
  {
    asset: "ALGO",
    balance: formatAssetAmount(8990211.88),
    balance_usd: formatUSDAmount(8990211.88),
    total_deposited: formatAssetAmount(8990211.88),
    total_deposited_usd: formatUSDAmount(8990211.88),
    apy: formatPctValue(20.96),
  },
];

const assetToDeposit = ref({ name: "", decimals: 0 } as any);

function onDeposit(asset: string) {
  assetToDeposit.value = { name: asset, decimals: 6 };
  const dialog = document.getElementById(modalsID.DEPOSIT);
  if (dialog)
    (dialog as any).showModal();
  else
    console.error("Modal could not be opened");
}
</script>

<template>
  <DepositModal v-if="assetToDeposit" :asset="assetToDeposit" />
  <div class="card bg-neutral text-neutral-content rounded-xl mt-12 lg:mt-32 shadow-2xl">
    <div class="card-body">
      <div class="flex justify-between mb-10">
        <h2 class="card-title text-xl md:text-3xl">
          Lending
        </h2>
      </div>
      <BaseTable
        :columns="columns"
        :data="tableData"
      >
        <template #asset="row">
          <div class="flex gap-x-2 items-center">
            <img src="https://assets.pact.fi/currencies/MainNet/386192725.image" class="w-4">
            <p>{{ row.asset }}</p>
          </div>
        </template>
        <template #balance="row">
          <div>
            <p>{{ row.balance }}</p>
            <p class="text-sm opacity-50">
              {{ row.balance_usd }}
            </p>
          </div>
        </template>
        <template #total_deposited="row">
          <div>
            <p>{{ row.total_deposited }}</p>
            <p class="text-sm opacity-50">
              {{ row.total_deposited_usd }}
            </p>
          </div>
        </template>
        <template #action="row">
          <button class="btn btn-primary text-xs" :onclick="() => onDeposit(row.asset)">
            Deposit
          </button>
        </template>
      </BaseTable>
    </div>
  </div>
</template>
