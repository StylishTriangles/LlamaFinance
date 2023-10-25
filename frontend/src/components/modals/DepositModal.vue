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
  depositedBalance: 0,
});

const totalBalance = computed(() =>
  state.depositedBalance + state.assetUsdValue,
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
  const dialog = document.getElementById(modalsID.DEPOSIT);
  (dialog as any).close();
}
</script>

<template>
  <BaseModal
    :id="modalsID.DEPOSIT"
    :title="`Deposit ${asset.name}`"
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
      <p class="opacity-80">
        Deposit balance
      </p>
      <p class="font-medium">
        {{ formatUSDAmount(totalBalance) }}
      </p>
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
