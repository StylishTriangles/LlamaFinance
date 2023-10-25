<script lang="ts" setup>
import { modalsID } from "~/config";
import { formatPctValue, formatUSDAmount, validateInput } from "~/utils";

const props = defineProps({
  asset: {
    type: Object,
    required: true,
  },
});

const state = reactive({
  assetAmount: "",
  assetUsdValue: 0,
  error: "",
  maxBalance: 9999,
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
const interestAPY = computed(() =>
  15.24, // TODO
);

function onInputChange(value: string) {
  state.assetAmount = value;
  state.assetUsdValue = Number(value) * 0.7;

  state.error = validateInput(
    value,
    state.maxBalance,
    props.asset.decimals,
    props.asset.name,
  );
}

function onSubmit() {
  if (state.error)
    return;

  console.log("Submit");
  const dialog = document.getElementById(modalsID.COLLATERAL);
  (dialog as any).close();
}
</script>

<template>
  <BaseModal :id="modalsID.COLLATERAL" :title="`Collateralize ${asset.name}`" @submit="onSubmit">
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
      <p class="opacity-80">
        Collateral balance
      </p>
      <p class="font-medium">
        {{ formatUSDAmount(collateralBalance) }}
      </p>
    </div>
    <div class="flex mb-1 text-sm w-full justify-between">
      <p class="opacity-80">
        Liquidation margin
      </p>
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
    <div class="flex mb-1 text-sm w-full justify-between">
      <p class="opacity-80">
        Interest APY
      </p>
      <p class="font-medium">
        {{ formatPctValue(interestAPY) }}
      </p>
    </div>
  </BaseModal>
</template>
