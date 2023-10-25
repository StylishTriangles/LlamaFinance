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
    accessor: "amount_borrowed",
  },
  {
    header: "Wallet Balance",
    accessor: "balance",
  },
  {
    header: "Borrow APY",
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
    amount_borrowed: formatAssetAmount(8990211.88),
    amount_borrowed_usd: formatUSDAmount(8990211.88),
    apy: formatPctValue(20.96),
  },
  {
    asset: "ETHH",
    balance: formatAssetAmount(8990211.88),
    balance_usd: formatUSDAmount(8990211.88),
    amount_borrowed: formatAssetAmount(8990211.88),
    amount_borrowed_usd: formatUSDAmount(8990211.88),
    apy: formatPctValue(20.96),
  },
];

function onBorrow(asset: string) {
  emitter.emit("open-borrow-modal", { name: asset, decimals: 6 });
}
function onRepay(asset: string) {
  emitter.emit("open-repay-modal", { name: asset, decimals: 6 });
}
</script>

<template>
  <BaseTable
    :columns="columns"
    :data="tableData"
    no-data-message="No borrows yet"
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
    <template #amount_borrowed="row">
      <div>
        <p>{{ row.amount_borrowed }}</p>
        <p class="text-sm opacity-50">
          {{ row.amount_borrowed_usd }}
        </p>
      </div>
    </template>
    <template #action="row">
      <div class="flex justify-end gap-x-2">
        <button class="btn btn-primary text-xs" :onclick="() => onBorrow(row.asset)">
          Borrow
        </button>
        <button class="btn btn-primary btn-outline text-xs" :onclick="() => onRepay(row.asset)">
          Repay
        </button>
      </div>
    </template>
  </BaseTable>
</template>
