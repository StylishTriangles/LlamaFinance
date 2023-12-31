<script lang="ts" setup>
import { modalsID } from "~/config";
import { emitter } from "~/main";
import type { UserAssetInfoResponse } from "~/sdk";
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
  maxBalance: 0,
  colSoFar: 0,
  liqMargin: 0,
  ltv: 0,
});
const rawUserData = ref(null as null | Map<string, UserAssetInfoResponse>);

onBeforeMount(async () => {
  rawUserData.value = await accountStore.financeSDK!.getUserAssetsInfo(
    accountStore.walletAddress!,
  );
  state.colSoFar = rawUserData.value.get(props.asset.denom)!.collateralUSD;
  state.maxBalance = rawUserData.value.get(props.asset.denom)!.collateral;
  state.ltv = accountStore.financeSDK!.getLTV(rawUserData.value) * 100;
  state.liqMargin
    = accountStore.financeSDK!.getLiquidationMargin(state.ltv / 100) * 100;
  state.isLoading = false;
});

const collateralBalance = computed(() => state.colSoFar - state.assetUsdValue);
const newLTV = computed(() => {
  if (!rawUserData.value)
    return 0;
  return (
    accountStore.financeSDK!.getLTVafter(
      rawUserData.value,
      props.asset.denom,
      Number(state.assetAmount) * -1,
    ) * 100
  );
});
const newLiqMargin = computed(() => {
  if (!rawUserData.value)
    return 0;
  return (
    accountStore.financeSDK!.getLiquidationMargin(newLTV.value / 100) * 100
  );
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
    const res = await accountStore.financeSDK!.withdrawCollateral(
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
    :id="modalsID.REDUCE_COL"
    :title="`Reduce ${asset.name} collateral`"
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
      <span class="opacity-80"> Collateral balance </span>
      <span class="font-medium">
        <span v-if="collateralBalance >= 0">{{
          formatUSDAmount(collateralBalance)
        }}</span>
        <span v-else>-</span>
      </span>
    </div>
    <div class="flex mb-1 text-sm w-full justify-between">
      <span class="opacity-80"> Liquidation margin </span>
      <div class="flex gap-x-2 font-medium">
        <span :class="state.liqMargin >= 0 ? 'text-success' : 'text-error'">
          {{ formatPctValue(state.liqMargin) }}
        </span>
        <template v-if="state.assetUsdValue > 0">
          <span>→</span>
          <span :class="newLiqMargin >= 0 ? 'text-success' : 'text-error'">
            {{ formatPctValue(newLiqMargin) }}
          </span>
        </template>
      </div>
    </div>
    <div class="flex mb-1 text-sm w-full justify-between">
      <p class="opacity-80">
        LTV
      </p>
      <div class="flex gap-x-2 font-medium">
        <span>
          {{ formatPctValue(state.ltv) }}
        </span>
        <template v-if="state.assetUsdValue > 0">
          <span>→</span>
          <span>
            {{ formatPctValue(newLTV) }}
          </span>
        </template>
      </div>
    </div>
  </BaseModal>
</template>
