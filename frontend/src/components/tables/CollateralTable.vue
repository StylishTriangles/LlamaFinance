<script lang="ts" setup>
import { emitter } from "~/main";
import type { BasicAsset, TableColumn } from "~/types";
import { formatAssetAmount, formatUSDAmount, rawAssetToBasic } from "~/utils";

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

    data.push({
      asset: rawAssetToBasic(asset, userBalance, price),
      balance: formatAssetAmount(userBalance / asset.precision),
      balance_usd: formatUSDAmount((userBalance / asset.precision) * price),
    });
  }

  tableData.value = data;
  isLoading.value = false;
});

function onCollateralize(asset: BasicAsset) {
  emitter.emit("open-collateral-modal", asset);
}
</script>

<template>
  <Card title="Available to Collateralize">
    <BaseTable
      :columns="columns"
      :data="tableData"
      :is-loading="isLoading"
    >
      <template #asset="row">
        <div class="flex gap-x-2 items-center">
          <img :src="row.asset.icon" class="w-4">
          <p>{{ row.asset.name }}</p>
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
      <template #action="row">
        <button
          class="btn btn-primary float-right text-xs"
          @click="() => onCollateralize(row.asset)"
        >
          Collateralize
        </button>
      </template>
    </BaseTable>
  </card>
</template>
