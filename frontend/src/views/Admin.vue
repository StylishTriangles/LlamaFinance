<script setup lang="ts">
import { contractAddresses } from '~/config';
import { Oracle, Finance } from '~/sdk';
// import { coin } from "@cosmjs/stargate";

const denom = ref('');

const price = ref('');

const decimals = ref('');
const target_utilization = ref('');
const min_rate = ref('');
const optimal_rate = ref('');
const max_rate = ref('');

async function addAsset() {
  if (accountStore.walletAddress && accountStore.signingClient) {
    let oracle = new Oracle(accountStore.signingClient, accountStore.walletAddress, contractAddresses.ORACLE_ADDRESS);
    let res2 = await oracle.addSymbol(denom.value);
    console.log(res2);
  }
}

async function setPrice() {
  if (accountStore.walletAddress && accountStore.signingClient) {
    let oracle = new Oracle(accountStore.signingClient, accountStore.walletAddress, contractAddresses.ORACLE_ADDRESS);
    let res2 = await oracle.setPrice(denom.value, price.value);
    console.log(res2);
  }
}

async function updateAsset() {
    if (accountStore.walletAddress && accountStore.signingClient) {
        let finance = new Finance(accountStore.signingClient, accountStore.walletAddress, contractAddresses.FINANCE_ADDRESS);
        let res2 = await finance.updateAsset(
            denom.value, 
            decimals.value, 
            target_utilization.value, 
            min_rate.value, 
            optimal_rate.value, 
            max_rate.value
        );
        console.log(res2);
    }
}

async function logPrices() {
  if (accountStore.walletAddress && accountStore.signingClient) {
    let oracle = new Oracle(accountStore.signingClient, accountStore.walletAddress, contractAddresses.ORACLE_ADDRESS);
    let res = await oracle.getPrices();
    console.log(res);
  }
}
</script>

<template>
  <div class="px-[1rem] md:px-[4rem]">
    <div>
        <a class="text-3xl">Oracle</a><br>
        <a class="text-xl" style="margin-left: 10px;">Add Asset</a><br>
        <!-- <form style="margin-left: 10px;"> -->
        <div style="margin-left: 20px;">
            <label for="denom">Asset denom:</label><br>
            <input v-model="denom" type="text"><br>
            <button @click="addAsset">ADD</button>
        </div>
        <!-- </form> -->
        <a class="text-xl" style="margin-left: 10px;">Set Price</a><br>
        <div style="margin-left: 20px;">
            <label for="denom">Asset denom:</label><br>
            <input v-model="denom" type="text"><br>
            <label for="price">Price:</label><br>
            <input v-model="price" type="text"><br>
            <button @click="setPrice">SET</button>
        </div>
        <a class="text-3xl">Finance</a><br>
        <a class="text-xl" style="margin-left: 10px;">Update Asset</a><br>
        <div style="margin-left: 20px;">
            <label for="denom">Asset denom:</label><br>
            <input v-model="denom" type="text"><br>
            <label for="decimals">Decimals:</label><br>
            <input v-model="decimals" type="text"><br>
            <label for="target_utilization">Target Utilization (%):</label><br>
            <input v-model="target_utilization" type="text"><br>
            <label for="min_rate">Rates (%):</label><br>
            <input v-model="min_rate" type="text">
            <input v-model="optimal_rate" type="text">
            <input v-model="max_rate" type="text"><br>
            <button @click="updateAsset">UPDATE</button>
        </div>

    </div>
    
  </div>
  <button @click="logPrices">Log Prices</button>
</template>
