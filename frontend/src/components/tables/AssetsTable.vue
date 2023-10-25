<script lang="ts" setup>
import { assetsData } from "~/config";
import type { TableColumn } from "~/types";
import { formatAssetAmount, formatPctValue, formatUSDAmount } from "~/utils";

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
    header: "Deposit APR",
    accessor: "deposit_apr",
  },
  {
    header: "Borrow APR",
    accessor: "borrow_apr",
  },
] as TableColumn[];

const tableData = ref([] as any[]);
const isLoading = ref(true);
const tvl = ref(0);

onBeforeMount(async () => {
  const rawData = await accountStore.financeSDK!.getAssetsInfoArray();
  tableData.value = rawData.map(asset => ({
    asset: asset.denom,
    total_deposited: formatAssetAmount(asset.totalDeposit),
    total_deposited_usd: formatUSDAmount(asset.totalDepositUSD),
    total_borrowed: formatAssetAmount(asset.totalBorrow),
    total_borrowed_usd: formatUSDAmount(asset.totalBorrowUSD),
    deposit_apr: formatPctValue(asset.apr * asset.totalBorrow / (asset.totalDeposit || 1)),
    borrow_apr: formatPctValue(asset.apr),
  }));

  tvl.value = rawData.reduce((a, b) =>
    a + b.totalCollateralUSD + b.totalDepositUSD, 0);
  isLoading.value = false;
});
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
      :is-loading="isLoading"
    >
      <template #asset="row">
        <div class="flex gap-x-2 items-center">
          <img :src="assetsData[row.asset].icon" class="w-4">
          <p>{{ assetsData[row.asset].name }}</p>
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
      <template #total_borrowed="row">
        <div>
          <p>{{ row.total_borrowed }}</p>
          <p class="text-sm opacity-50">
            {{ row.total_borrowed_usd }}
          </p>
        </div>
      </template>
    </BaseTable>
  </Card>
</template>
