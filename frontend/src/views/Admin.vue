<script setup lang="ts">
import { contractAddresses } from '~/config';
import { Oracle } from '~/sdk';

const denom = ref('');

async function addAsset() {
  if (accountStore.walletAddress && accountStore.signingClient) {
    console.log(denom.value);
    let oracle = new Oracle(accountStore.signingClient, accountStore.walletAddress, contractAddresses.ORACLE_ADDRESS);
    let res = await oracle.addSymbol(denom);
    console.log(res);
  }
  
}
</script>

<template>
  <div class="px-[1rem] md:px-[4rem]">
    <div>
        <a class="text-3xl">Oracle</a><br>
        <a class="text-xl">Add Asset</a><br>
        <!-- <form style="margin-left: 10px;"> -->
            <label for="denom">Asset denom:</label><br>
            <input v-model="denom" type="text">
            <button @click="addAsset">ADD</button>
        <!-- </form> -->
    </div>
    
  </div>
  <button @click="onClick">OMEGALOL</button>
</template>
