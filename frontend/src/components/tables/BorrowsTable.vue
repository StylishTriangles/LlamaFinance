<script lang="ts" setup>
import { emitter } from "~/main";
import type { BasicAsset, TableColumn } from "~/types";
import {
  formatAssetAmount,
  formatPctValue,
  formatUSDAmount,
  rawAssetToBasic,
} from "~/utils";

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
    header: "APR",
    accessor: "apr",
  },
  {
    header: "",
    accessor: "action",
  },
] as TableColumn[];

const tableData = ref([] as any[]);
const isLoading = ref(true);

onBeforeMount(async () => {
  const rawData = await accountStore.financeSDK!.getAssetsInfoArray();
  const data = [];
  for (const asset of rawData) {
    const userBalance = await accountStore.getUserBalance(asset.denom);
    const price = asset.price_per_unit * asset.precision;
    const available = Math.max(asset.totalDeposit - asset.totalBorrow, 0);

    data.push({
      asset: rawAssetToBasic(asset, userBalance, price),
      available: formatAssetAmount(available),
      available_usd: formatUSDAmount(available * price),
      apr: formatPctValue(asset.apr),
    });
  }

  tableData.value = data;
  isLoading.value = false;
});

function onBorrow(asset: BasicAsset) {
  emitter.emit("open-borrow-modal", asset);
}
</script>

<template>
  <Card title="Available to Borrow">
    <BaseTable :columns="columns" :data="tableData" :is-loading="isLoading">
      <template #asset="row">
        <div class="flex gap-x-2 items-center">
          <img :src="row.asset.icon" class="w-4">
          <p>{{ row.asset.name }}</p>
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
        <button
          class="btn btn-primary float-right text-xs"
          :onclick="() => onBorrow(row.asset)"
        >
          Borrow
        </button>
      </template>
    </BaseTable>
  </Card>
</template>
