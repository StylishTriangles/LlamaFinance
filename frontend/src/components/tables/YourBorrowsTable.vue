<script setup lang="ts">
import { emitter } from "~/main";
import type { BasicAsset, TableColumn } from "~/types";
import { formatAssetAmount, formatPctValue, formatUSDAmount, rawAssetToBasic } from "~/utils";

const emit = defineEmits(["borrowCalced"]);

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
    header: "Borrow APR",
    accessor: "apr",
  },
  {
    header: "",
    accessor: "action",
  },
] as TableColumn[];

const tableData = ref([] as any[]);
const totalBorrow = ref(0);
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
    const userBorrow = rawUserData.get(asset.denom)!.borrowAmount;
    if (userBorrow === 0)
      continue;

    data.push({
      asset: rawAssetToBasic(asset, userBalance, price),
      balance: formatAssetAmount(userBalance / asset.precision),
      balance_usd: formatUSDAmount((userBalance / asset.precision) * price),
      amount_borrowed: formatAssetAmount(userBorrow),
      amount_borrowed_usd: formatUSDAmount(userBorrow * price),
      apr: formatPctValue(asset.apr),
    });
    total += userBorrow * price;
  }
  tableData.value = data;
  totalBorrow.value = total;
  emit("borrowCalced", total);

  isLoading.value = false;
}

function onBorrow(asset: BasicAsset) {
  emitter.emit("open-borrow-modal", asset);
}
function onRepay(asset: BasicAsset) {
  emitter.emit("open-repay-modal", asset);
}
</script>

<template>
  <BaseTable
    :columns="columns"
    :data="tableData"
    :is-loading="isLoading"
    no-data-message="No borrows yet"
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
