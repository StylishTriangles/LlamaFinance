<script lang="ts" setup>
import type { BasicAsset } from "~/types";
import { formatAssetAmount, formatUSDAmount } from "~/utils";

const props = defineProps({
  value: {
    // https://forum.vuejs.org/t/how-to-define-a-nullable-prop/117642/4
    type: null as unknown as PropType<string | number | null>,
    default: null,
  },
  asset: {
    type: Object as PropType<BasicAsset>,
    default: () => {},
  },
  usdValue: {
    type: Number,
    default: 0,
  },
  max: {
    type: Number,
    default: 0,
  },
  error: {
    type: String,
    default: "",
  },
});

const emit = defineEmits(["input"]);
const inputValue = ref(props.value);

function setMax() {
  inputValue.value = props.max;
  emit("input", props.max);
}

function onKeyPress(e: KeyboardEvent) {
  const allowedKeys = /[0-9]|\.|,/;
  if (
    !allowedKeys.test(e.key)
    || ((e.key === "." || e.key === ",")
      && inputValue.value!.toString().includes("."))
  )
    e.preventDefault();
}

function onInput() {
  const toEmit = inputValue
    .value!.toString()
    .replace(",", ".")
    .replaceAll(",", "")
    .replaceAll(" ", "")
    .replace(/\..*/, c => "." + c.replace(/\./g, () => ""));
  // replace first comma with dot and then reomve all other commas and whitespaces and in the end remove all dots except the first one
  // it's ugly but also the only way to make it work on mobiles since preventDefault() cannot be triggered there like on desktop
  if (Number(toEmit) || Number(toEmit) === 0)
    emit("input", toEmit);
}
</script>

<template>
  <div class="relative form-control w-full">
    <label class="label">
      <span class="label-text text-neutral-content">Amount</span>
    </label>
    <div class="relative">
      <input
        v-model="inputValue"
        type="text"
        class="input input-bordered pr-20 rounded-lg w-full input-primary bg-opacity-20 text-neutral-content"
        :class="error ? 'input-error' : 'input-primary'"
        pattern="/[0-9]|\.|,/"
        inputmode="decimal"
        lang="en-US"
        placeholder="0.00"
        autocomplete="off"
        @keypress="onKeyPress"
        @input="onInput"
      >
      <p
        class="pointer-events-none absolute right-2 text-lg flex items-center sm:text-2xl"
        :style="{ top: '50%', transform: 'translateY(-50%)' }"
      >
        <img :src="asset.icon" class="w-5 h-5 flex-shrink-0">
        <span class="text-base text-neutral-content ml-1">{{ asset.name }}</span>
      </p>
    </div>
    <label v-if="error" class="label">
      <span class="label-text-alt text-error">{{ error }}</span>
    </label>
    <label class="label">
      <span class="label-text-alt text-neutral-content">{{ formatUSDAmount(usdValue) }}</span>
      <span class="label-text-alt text-neutral-content">
        <span class="font-bold mr-2">Total balance: {{ formatAssetAmount(max) }} </span>
        <span class="text-accent cursor-pointer" @click="setMax">MAX</span>
      </span>
    </label>
  </div>
</template>
