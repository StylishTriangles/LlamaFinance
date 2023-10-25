<script lang="ts" setup>
import { modalsID } from "~/config";
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
  maxBalance: props.asset.balance,
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
  state.assetUsdValue = Number(value) * props.asset.price;

  state.error = validateInput(
    value,
    state.maxBalance,
    props.asset.decimals,
    props.asset.name,
  );
}

async function onSubmit() {
  if (state.error)
    return;

  const res = await accountStore.financeSDK!.deposit(
    props.asset.name,
    Number(state.assetAmount) * props.asset.precision,
  );
  console.log(res);

  // const dialog = document.getElementById(modalsID.DEPOSIT);
  // (dialog as any).close();
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
      <span class="opacity-80">
        Deposit balance
      </span>
      <span class="font-medium">
        {{ formatUSDAmount(totalBalance) }}
      </span>
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
