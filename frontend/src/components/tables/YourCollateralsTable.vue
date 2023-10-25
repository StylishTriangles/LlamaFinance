<script setup lang="ts">
import { emitter } from "~/main";
import type { TableColumn } from "~/types";
import { formatAssetAmount, formatPctValue, formatUSDAmount } from "~/utils";

const columns = [
  {
    header: "Asset",
    accessor: "asset",
  },
  {
    header: "Amount",
    accessor: "amount_added",
  },
  {
    header: "Wallet Balance",
    accessor: "balance",
  },
  {
    header: "Collateral APY",
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
    balance: formatAssetAmount(8990211.88),
    balance_usd: formatUSDAmount(8990211.88),
    amount_added: formatAssetAmount(8990211.88),
    amount_added_usd: formatUSDAmount(8990211.88),
    apy: formatPctValue(20.96),
  },
  {
    asset: "ETH",
    balance: formatAssetAmount(8990211.88),
    balance_usd: formatUSDAmount(8990211.88),
    amount_added: formatAssetAmount(8990211.88),
    amount_added_usd: formatUSDAmount(8990211.88),
    apy: formatPctValue(20.96),
  },
];

function onCollateralize(asset: string) {
  emitter.emit("open-collateral-modal", { name: asset, decimals: 6 });
}
</script>

<template>
  <BaseTable
    :columns="columns"
    :data="tableData"
    no-data-message="No collaterals yet"
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
    <template #amount_added="row">
      <div>
        <p>{{ row.amount_added }}</p>
        <p class="text-sm opacity-50">
          {{ row.amount_added_usd }}
        </p>
      </div>
    </template>
    <template #action="row">
      <button class="btn btn-primary float-right text-xs" :onclick="() => onCollateralize(row.asset)">
        Collateralize
      </button>
    </template>
  </BaseTable>
</template>
