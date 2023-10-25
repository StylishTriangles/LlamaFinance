<script lang="ts" setup>
import { ExclamationIcon } from "@heroicons/vue/outline";
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
  depositedBalance: 10,
});

const balanceLeft = computed(() =>
  state.depositedBalance - state.assetUsdValue,
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
  const dialog = document.getElementById(modalsID.WITHDRAW);
  (dialog as any).close();
}
</script>

<template>
  <BaseModal
    :id="modalsID.WITHDRAW"
    :title="`Withdraw ${asset.name}`"
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
        You will no longer earn the following APY on the amount you withdraw:
      </p>
    </div>
    <div class="flex mb-1 text-sm w-full justify-between">
      <span class="opacity-80">
        Interest APY
      </span>
      <span class="font-medium">
        {{ formatPctValue(interestAPY) }}
      </span>
    </div>
  </BaseModal>
</template>
