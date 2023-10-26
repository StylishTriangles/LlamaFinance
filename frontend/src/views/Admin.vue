<!-- Not visible for basic users, used for setting up assets -->
<script setup lang="ts">
import { contractAddresses } from "~/config";
import { Finance, Oracle } from "~/sdk";

const state = reactive({
  denom: "",
  price: "",
  precision: "",
  decimals: "",
  targetUtilization: "",
  minRate: "",
  optimalRate: "",
  maxRate: "",
});

async function addAsset() {
  if (accountStore.walletAddress && accountStore.signingClient) {
    const oracle = new Oracle(
      accountStore.signingClient,
      accountStore.walletAddress,
      contractAddresses.ORACLE_ADDRESS,
    );
    const res2 = await oracle.addSymbol(state.denom);
    console.log(res2);
  }
}

async function setPrice() {
  if (accountStore.walletAddress && accountStore.signingClient) {
    const oracle = new Oracle(
      accountStore.signingClient,
      accountStore.walletAddress,
      contractAddresses.ORACLE_ADDRESS,
    );
    const res2 = await oracle.setPrice(
      state.denom,
      Number(state.price),
      Number(state.precision),
    );
    console.log(res2);
  }
}

async function updateAsset() {
  if (accountStore.walletAddress && accountStore.signingClient) {
    const finance = new Finance(
      accountStore.signingClient,
      accountStore.walletAddress,
      contractAddresses.FINANCE_ADDRESS,
      new Oracle(
        accountStore.signingClient,
        accountStore.walletAddress,
        contractAddresses.ORACLE_ADDRESS,
      ),
    );
    const res2 = await finance.updateAsset(
      state.denom,
      Number(state.decimals),
      Number(state.targetUtilization),
      Number(state.minRate),
      Number(state.optimalRate),
      Number(state.maxRate),
    );
    console.log(res2);
  }
}

async function logPrices() {
  if (accountStore.walletAddress && accountStore.signingClient) {
    const oracle = new Oracle(
      accountStore.signingClient,
      accountStore.walletAddress,
      contractAddresses.ORACLE_ADDRESS,
    );
    const res = await oracle.getPrices();
    console.log(res);
  }
}
</script>

<template>
  <div class="px-[1rem] md:px-[4rem]">
    <Card title="Oracle">
      <p class="text-xl">
        Add Asset
      </p>
      <div class="ml-10 flex-col">
        <label class="label">
          <span class="label-text text-neutral-content">Asset denom</span>
        </label>
        <input
          v-model="state.denom"
          type="text"
          class="input input-bordered rounded-lg input-primary bg-opacity-20 text-neutral-content"
          autocomplete="off"
        >
        <button class="btn btn-primary ml-2" @click="addAsset">
          ADD
        </button>
      </div>

      <p class="text-xl" style="margin-left: 10px">
        Set Price
      </p>
      <div class="ml-10 flex-col">
        <label class="label">
          <span class="label-text text-neutral-content">Asset denom</span>
        </label>
        <input
          v-model="state.denom"
          type="text"
          class="input input-bordered rounded-lg input-primary bg-opacity-20 text-neutral-content"
          autocomplete="off"
        >
        <label class="label">
          <span class="label-text text-neutral-content">Price</span>
        </label>
        <input
          v-model="state.price"
          type="text"
          class="input input-bordered rounded-lg input-primary bg-opacity-20 text-neutral-content"
          autocomplete="off"
        >
        <label class="label">
          <span class="label-text text-neutral-content">Precision</span>
        </label>
        <input
          v-model="state.precision"
          type="text"
          class="input input-bordered rounded-lg input-primary bg-opacity-20 text-neutral-content"
          autocomplete="off"
        >

        <button class="btn btn-primary ml-2" @click="setPrice">
          SET
        </button>
      </div>
    </Card>

    <Card title="Finance">
      <p class="text-xl" style="margin-left: 10px">
        Update Asset
      </p>
      <div class="ml-10 flex-col">
        <label class="label">
          <span class="label-text text-neutral-content">Asset denom</span>
        </label>
        <input
          v-model="state.denom"
          type="text"
          class="input input-bordered rounded-lg input-primary bg-opacity-20 text-neutral-content"
          autocomplete="off"
        >
        <label class="label">
          <span class="label-text text-neutral-content">Decimals</span>
        </label>
        <input
          v-model="state.decimals"
          type="text"
          class="input input-bordered rounded-lg input-primary bg-opacity-20 text-neutral-content"
          autocomplete="off"
        >
        <label class="label">
          <span class="label-text text-neutral-content">Target Utilization (%):</span>
        </label>
        <input
          v-model="state.targetUtilization"
          type="text"
          class="input input-bordered rounded-lg input-primary bg-opacity-20 text-neutral-content"
          autocomplete="off"
        >
        <label class="label">
          <span class="label-text text-neutral-content">Rates (%):</span>
        </label>
        <input
          v-model="state.minRate"
          type="text"
          class="input input-bordered mr-2 rounded-lg input-primary bg-opacity-20 text-neutral-content"
          placeholder="Min Rate"
          autocomplete="off"
        >
        <input
          v-model="state.optimalRate"
          type="text"
          class="input input-bordered mr-2 rounded-lg input-primary bg-opacity-20 text-neutral-content"
          placeholder="Optimal Rate"
          autocomplete="off"
        >
        <input
          v-model="state.maxRate"
          type="text"
          class="input input-bordered rounded-lg input-primary bg-opacity-20 text-neutral-content"
          placeholder="Max Rate"
          autocomplete="off"
        >
        <button class="btn btn-primary ml-2" @click="updateAsset">
          Update
        </button>
      </div>
    </Card>
    <div class="w-full flex justify-center">
      <button class="btn btn-primary mt-10" @click="logPrices">
        Log Prices
      </button>
    </div>
  </div>
</template>
