<script setup lang="ts">
import { emitter } from "~/main";
import type { BasicAsset, TableColumn } from "~/types";
import { formatAssetAmount, formatUSDAmount, rawAssetToBasic } from "~/utils";

const emit = defineEmits(["collateralCalced"]);

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
    header: "",
    accessor: "action",
  },
] as TableColumn[];

const tableData = ref([] as any[]);
const totalCollateral = ref(0);
const isLoading = ref(true);

onBeforeMount(() => {
  assignData();
  emitter.on("txn-success", assignData);
});

async function assignData() {
  isLoading.value = true;
  const rawData = await accountStore.financeSDK!.getAssetsInfoArray();
  const rawUserData = await accountStore.financeSDK!.getUserAssetsInfo(accountStore.walletAddress!);
  const data = [];
  let total = 0;
  for (const asset of rawData) {
    const userBalance = await accountStore.getUserBalance(asset.denom);
    const price = asset.price_per_unit * asset.precision;
    const userCollateral = rawUserData.get(asset.denom)!.collateral;
    if (userCollateral === 0)
      continue;

    data.push({
      asset: rawAssetToBasic(asset, userBalance, price),
      balance: formatAssetAmount(userBalance / asset.precision),
      balance_usd: formatUSDAmount((userBalance / asset.precision) * price),
      amount_added: formatAssetAmount(userCollateral),
      amount_added_usd: formatUSDAmount(userCollateral * price),
    });
    total += userCollateral * price;
  }
  tableData.value = data;
  totalCollateral.value = total;
  emit("collateralCalced", total);

  isLoading.value = false;
}

function onCollateralize(asset: BasicAsset) {
  emitter.emit("open-collateral-modal", asset);
}
function onReduce(asset: BasicAsset) {
  emitter.emit("open-reduce-col-modal", asset);
}
</script>

<template>
  <BaseTable
    :columns="columns"
    :data="tableData"
    :is-loading="isLoading"
    no-data-message="No collaterals yet"
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
    <template #amount_added="row">
      <div>
        <p>{{ row.amount_added }}</p>
        <p class="text-sm opacity-50">
          {{ row.amount_added_usd }}
        </p>
      </div>
    </template>
    <template #action="row">
      <div class="flex justify-end gap-x-2">
        <button class="btn btn-primary text-xs" :onclick="() => onCollateralize(row.asset)">
          Collateralize
        </button>
        <button class="btn btn-primary btn-outline text-xs" :onclick="() => onReduce(row.asset)">
          Reduce
        </button>
      </div>
    </template>
  </BaseTable>
</template>
