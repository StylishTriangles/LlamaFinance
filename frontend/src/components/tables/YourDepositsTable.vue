<script setup lang="ts">
import { emitter } from "~/main";
import type { BasicAsset, TableColumn } from "~/types";
import { formatAssetAmount, formatPctValue, formatUSDAmount, rawAssetToBasic } from "~/utils";

const columns = [
  {
    header: "Asset",
    accessor: "asset",
  },
  {
    header: "Amount",
    accessor: "amount_deposited",
  },
  {
    header: "Wallet Balance",
    accessor: "balance",
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
const totalDeposited = ref(0);
const isLoading = ref(true);

onBeforeMount(async () => {
  const rawData = await accountStore.financeSDK!.getAssetsInfoArray();
  const rawUserData = await accountStore.financeSDK!.getUserAssetsInfo(accountStore.walletAddress!);
  const data = [];
  let total = 0;
  for (const asset of rawData) {
    const userBalance = await accountStore.getUserBalance(asset.denom);
    const price = asset.price_per_unit * asset.precision;
    const userDeposit = rawUserData.get(asset.denom)!.lAssetAmount;
    if (userDeposit === 0)
      continue;

    data.push({
      asset: rawAssetToBasic(asset, userBalance, price),
      balance: formatAssetAmount(userBalance / asset.precision),
      balance_usd: formatUSDAmount((userBalance / asset.precision) * price),
      apr: formatPctValue(asset.apr * asset.totalBorrow / (asset.totalDeposit || 1)),
      amout_deposited_raw: userDeposit,
      amount_deposited: formatAssetAmount(userDeposit),
      amount_deposited_usd: formatUSDAmount(userDeposit * price),
    });
    total += userDeposit * price;
  }
  tableData.value = data;
  totalDeposited.value = total;

  isLoading.value = false;
});

function onDeposit(asset: BasicAsset) {
  emitter.emit("open-deposit-modal", asset);
}
function onWithdraw(asset: BasicAsset, available: number, totalDeposited: number, aprPct: string) {
  console.log({ asset, available, totalDeposited });
  emitter.emit("open-withdraw-modal", { asset, available, totalDeposited, aprPct });
}
</script>

<template>
  <Card title="Your Deposits">
    <template v-if="totalDeposited > 0" #top-right>
      <p class="font-bold">
        Total Deposited
      </p>
      <p>{{ formatUSDAmount(totalDeposited) }}</p>
    </template>

    <BaseTable
      :columns="columns"
      :data="tableData"
      :is-loading="isLoading"
      no-data-message="No deposits yet"
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
      <template #amount_deposited="row">
        <div>
          <p>{{ row.amount_deposited }}</p>
          <p class="text-sm opacity-50">
            {{ row.amount_deposited_usd }}
          </p>
        </div>
      </template>
      <template #action="row">
        <div class="flex justify-end gap-x-2">
          <button
            class="btn btn-primary text-xs"
            :onclick="() => onDeposit(row.asset)"
          >
            Deposit
          </button>
          <button
            class="btn btn-primary btn-outline text-xs"
            :onclick="() => onWithdraw(row.asset, row.amout_deposited_raw, Number(row.amount_deposited_usd.slice(1)), row.apr)"
          >
            Withdraw
          </button>
        </div>
      </template>
    </BaseTable>
  </Card>
</template>
