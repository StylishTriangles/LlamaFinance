<script lang="ts" setup>
import { ExclamationIcon } from "@heroicons/vue/outline";
import { modalsID } from "~/config";
import { emitter } from "~/main";
import type { BasicAsset } from "~/types";
import { formatPctValue, formatUSDAmount, validateInput } from "~/utils";

const props = defineProps({
  asset: {
    type: Object as PropType<BasicAsset>,
    required: true,
  },
});

const state = reactive({
  assetAmount: "",
  assetUsdValue: 0,
  error: "",
  isLoading: false,
  maxBalance: 0,
  depositedBalance: 0,
  apr: 0,
});

const balanceLeft = computed(() =>
  Math.max(state.depositedBalance - state.assetUsdValue, 0),
);

onBeforeMount(async () => {
  const rawData = await accountStore.financeSDK!.getAssetsInfo();
  const assetData = rawData.get(props.asset.denom)!;
  state.maxBalance = assetData.totalDeposit;
  state.depositedBalance = assetData.totalDepositUSD;
  state.apr = assetData.apr * assetData.totalBorrow / (assetData.totalDeposit || 1);
  state.isLoading = false;
});

function onInputChange(value: string) {
  state.assetAmount = value;
  state.assetUsdValue = Number(value) * props.asset.price;

  state.error = validateInput(
    value,
    state.maxBalance,
    props.asset.decimals,
    props.asset.name,
  );
}

async function onSubmit() {
  if (state.error || state.isLoading)
    return;

  state.isLoading = true;
  try {
    const res = await accountStore.financeSDK!.withdraw(
      props.asset.denom,
      Number(state.assetAmount) * props.asset.precision,
    );

    emitter.emit("txn-success", res.transactionHash);
  } catch (e) {
    console.error(e);
  }
  state.isLoading = false;
}
</script>

<template>
  <BaseModal
    :id="modalsID.WITHDRAW"
    :title="`Withdraw ${asset.name}`"
    :is-loading="state.isLoading"
    @submit="onSubmit"
  >
    <NumberInput
      :value="state.assetAmount"
      :usd-value="state.assetUsdValue"
      :asset="asset"
      :max="state.maxBalance"
      :error="state.error"
      @input="onInputChange"
    />

    <hr class="my-4 opacity-50">

    <div class="flex mb-1 text-sm w-full justify-between">
      <span class="opacity-80">
        Deposit balance
      </span>
      <span class="font-medium">
        <span v-if="balanceLeft >= 0">{{ formatUSDAmount(balanceLeft) }}</span>
        <span v-else>-</span>
      </span>
    </div>

    <div class="flex items-center mt-4">
      <ExclamationIcon class="text-warning w-8" />
      <p class="ml-3 pl-3 py-1 border-l border-l-warning text-xs ">
        You will no longer earn the following APR on the amount you withdraw:
      </p>
    </div>
    <div class="flex mb-1 text-sm w-full justify-between">
      <span class="opacity-80">
        Interest APR
      </span>
      <span class="font-medium">
        {{ formatPctValue(state.apr) }}
      </span>
    </div>
  </BaseModal>
</template>
