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
  isLoading: false,
  maxBalance: props.asset.balance,
  colSoFar: 0,
  liqMargin: 20.1,
  ltv: 2.1,
});

const collateralBalance = computed(() =>
  state.colSoFar + state.assetUsdValue,
);
const newLiqMargin = computed(() =>
  15.24, // TODO
);
const newLTV = computed(() =>
  3.24, // TODO
);

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
    const res = await accountStore.financeSDK!.depositCollateral(
      props.asset.denom,
      Number(state.assetAmount) * props.asset.precision,
    );

    emitter.emit("txn-success", res.transactionHash);
  } catch (e) {
    console.error(e);
  }
  state.isLoading = false;

  // const dialog = document.getElementById(modalsID.COLLATERAL);
  // (dialog as any).close();
}
</script>

<template>
  <BaseModal
    :id="modalsID.COLLATERAL"
    :title="`Collateralize ${asset.name}`"
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
        Collateral balance
      </span>
      <span class="font-medium">
        {{ formatUSDAmount(collateralBalance) }}
      </span>
    </div>
    <div class="flex mb-1 text-sm w-full justify-between">
      <span class="opacity-80">
        Liquidation margin
      </span>
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
