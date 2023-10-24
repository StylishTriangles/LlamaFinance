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
</script>

<template>
  <div class="card bg-neutral text-neutral-content rounded-xl mt-12 lg:mt-32 shadow-2xl">
    <div class="card-body">
      <div class="flex justify-between mb-10">
        <h2 class="card-title text-3xl">
          Assets
        </h2>
        <div class="text-right text-xl font-medium">
          <p class="font-bold">
            TVL
          </p>
          <p>{{ formatUSDAmount(575843280.53) }}</p>
        </div>
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
      </BaseTable>
    </div>
  </div>
</template>
