<script setup lang="ts">
import { modalsID } from "~/config";
import type { TableColumn } from "~/types";
import { formatAssetAmount, formatPctValue, formatUSDAmount } from "~/utils";

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
    header: "APY",
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
    amount_deposited: formatAssetAmount(8990211.88),
    amount_deposited_usd: formatUSDAmount(8990211.88),
    apy: formatPctValue(20.96),
  },
  {
    asset: "ETH",
    balance: formatAssetAmount(8990211.88),
    balance_usd: formatUSDAmount(8990211.88),
    amount_deposited: formatAssetAmount(8990211.88),
    amount_deposited_usd: formatUSDAmount(8990211.88),
    apy: formatPctValue(20.96),
  },
];

const assetToWithdraw = ref({ name: "", decimals: 0 } as any);
const totalDeposited = ref(843280.53); // TODO

function onWithdraw(asset: string) {
  assetToWithdraw.value = { name: asset, decimals: 6 };
  const dialog = document.getElementById(modalsID.WITHDRAW);
  if (dialog)
    (dialog as any).showModal();
  else
    console.error("Modal could not be opened");
}
</script>

<template>
  <WithdrawModal :asset="assetToWithdraw" />
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
      no-data-message="No deposits yet"
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
      <template #amount_deposited="row">
        <div>
          <p>{{ row.amount_deposited }}</p>
          <p class="text-sm opacity-50">
            {{ row.amount_deposited_usd }}
          </p>
        </div>
      </template>
      <template #action="row">
        <button class="btn btn-primary btn-outline float-right text-xs" :onclick="() => onWithdraw(row.asset)">
          Withdraw
        </button>
      </template>
    </BaseTable>
  </Card>
</template>