<script setup lang="ts">
import { modalsID } from "~/config";
import type { TableColumn } from "~/types";
import { formatAssetAmount, formatPctValue, formatUSDAmount, openModal } from "~/utils";

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
    asset: "ETH",
    balance: formatAssetAmount(8990211.88),
    balance_usd: formatUSDAmount(8990211.88),
    amount_borrowed: formatAssetAmount(8990211.88),
    amount_borrowed_usd: formatUSDAmount(8990211.88),
    apy: formatPctValue(20.96),
  },
];

const assetToBorrow = ref({ name: "", decimals: 0 } as any);

function onBorrow(asset: string) {
  assetToBorrow.value = { name: asset, decimals: 6 };
  openModal(modalsID.BORROW);
}
</script>

<template>
  <CollateralModal :asset="assetToBorrow" />
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
      <button class="btn btn-primary float-right text-xs" :onclick="() => onBorrow(row.asset)">
        Borrow
      </button>
    </template>
  </BaseTable>
</template>
