<script lang="ts" setup>
import type { TableColumn } from "~/types";
import { formatPctValue, formatUSDAmount } from "~/utils";

const columns = [
  {
    header: "Asset",
    accessor: "asset",
  },
  {
    header: "Total Deposited",
    accessor: "total_deposited",
  },
  {
    header: "Total Borrowed",
    accessor: "total_borrowed",
  },
  {
    header: "Deposit APY",
    accessor: "deposit_apy",
  },
  {
    header: "Borrow APY",
    accessor: "borrow_apy",
  },
] as TableColumn[];

const tableData = [
  {
    asset: "CORE",
    total_deposited: formatUSDAmount(8990211.88),
    total_borrowed: formatUSDAmount(8990211.88),
    deposit_apy: formatPctValue(20.96),
    borrow_apy: formatPctValue(31.31),
  },
  {
    asset: "BTC",
    total_deposited: formatUSDAmount(123456789.7),
    total_borrowed: formatUSDAmount(123456789.7),
    deposit_apy: formatPctValue(0.96),
    borrow_apy: formatPctValue(5.31),
  },
  {
    asset: "ETH",
    total_deposited: formatUSDAmount(643245.18),
    total_borrowed: formatUSDAmount(643245.18),
    deposit_apy: formatPctValue(1.8),
    borrow_apy: formatPctValue(2.41),
  },
  {
    asset: "USDC",
    total_deposited: formatUSDAmount(573280.53),
    total_borrowed: formatUSDAmount(573280.53),
    deposit_apy: formatPctValue(10.26),
    borrow_apy: formatPctValue(31.31),
  },
  {
    asset: "ALGO",
    total_deposited: formatUSDAmount(3262.3),
    total_borrowed: formatUSDAmount(3262.3),
    deposit_apy: formatPctValue(20.96),
    borrow_apy: formatPctValue(31.31),
  },
];

const tvl = ref(575843280.53);
</script>

<template>
  <Card title="Assets">
    <template #top-right>
      <p class="font-bold">
        TVL
      </p>
      <p>{{ formatUSDAmount(tvl) }}</p>
    </template>
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
    </BaseTable>
  </Card>
</template>
