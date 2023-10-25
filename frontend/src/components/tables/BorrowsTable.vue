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
    header: "Available",
    accessor: "available",
  },
  {
    header: "APY",
    accessor: "apy",
  },
  {
    header: "",
    accessor: "action",
  },
] as TableColumn[];

const tableData = [
  {
    asset: "CORE",
    available: formatAssetAmount(8990211.88),
    available_usd: formatUSDAmount(8990211.88),
    apy: formatPctValue(20.96),
  },
  {
    asset: "BTC",
    available: formatAssetAmount(8990211.88),
    available_usd: formatUSDAmount(8990211.88),
    apy: formatPctValue(20.96),
  },
  {
    asset: "ETH",
    available: formatAssetAmount(8990211.88),
    available_usd: formatUSDAmount(8990211.88),
    apy: formatPctValue(20.96),
  },
  {
    asset: "USDC",
    available: formatAssetAmount(8990211.88),
    available_usd: formatUSDAmount(8990211.88),
    apy: formatPctValue(20.96),
  },
  {
    asset: "ALGO",
    available: formatAssetAmount(8990211.88),
    available_usd: formatUSDAmount(8990211.88),
    apy: formatPctValue(20.96),
  },
];

const assetToBorrow = ref({ name: "", decimals: 0 } as any);

function onBorrow(asset: string) {
  assetToBorrow.value = { name: asset, decimals: 6 };
  const dialog = document.getElementById(modalsID.BORROW);
  if (dialog)
    (dialog as any).showModal();
  else
    console.error("Modal could not be opened");
}
</script>

<template>
  <BorrowModal :asset="assetToBorrow" />
  <Card title="Available to Borrow">
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
      <template #available="row">
        <div>
          <p>{{ row.available }}</p>
          <p class="text-sm opacity-50">
            {{ row.available_usd }}
          </p>
        </div>
      </template>
      <template #action="row">
        <button class="btn btn-primary float-right text-xs" :onclick="() => onBorrow(row.asset)">
          Borrow
        </button>
      </template>
    </BaseTable>
  </card>
</template>
