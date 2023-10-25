<script lang="ts" setup>
import { emitter } from "~/main";
import type { BasicAsset, TableColumn } from "~/types";
import { formatAssetAmount, formatPctValue, formatUSDAmount, rawAssetToBasic } from "~/utils";

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

const tableData = ref([] as any[]);

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
      apy: formatPctValue(asset.apr * asset.totalBorrow / (asset.totalDeposit || 1)),
      total_deposited: formatAssetAmount(asset.totalDeposit),
      total_deposited_usd: formatUSDAmount(asset.totalDepositUSD),
    });
  }

  tableData.value = data;
});

function onDeposit(asset: BasicAsset) {
  emitter.emit("open-deposit-modal", asset);
}
</script>

<template>
  <Card title="Lending">
    <BaseTable
      :columns="columns"
      :data="tableData"
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
