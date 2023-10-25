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
  <DepositModal :asset="assetToDeposit" />
  <Card title="Lending">
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
        <button class="btn btn-primary float-right text-xs" :onclick="() => onDeposit(row.asset)">
          Deposit
        </button>
      </template>
    </BaseTable>
  </Card>
</template>
