<script lang="ts" setup>
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
  isLoading: true,
  maxBalance: props.asset.balance,
  depositedBalance: 0,
  apr: 0,
});

const totalBalance = computed(() =>
  state.depositedBalance + state.assetUsdValue,
);

onBeforeMount(async () => {
  const rawData = await accountStore.financeSDK!.getAssetsInfo();
  const assetData = rawData.get(props.asset.denom)!;
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
    const res = await accountStore.financeSDK!.deposit(
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
    :id="modalsID.DEPOSIT"
    :title="`Deposit ${asset.name}`"
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
        {{ formatUSDAmount(totalBalance) }}
      </span>
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
