<script setup lang="ts">
import { contractAddresses } from '~/config';
import { Oracle } from '~/sdk';

// import { Oracle } from '~/sdk';
async function onClick() {
  if (accountStore.walletAddress && accountStore.signingClient) {
    let oracle = new Oracle(accountStore.signingClient, accountStore.walletAddress, contractAddresses.ORACLE_ADDRESS);
    let price = await oracle.getPrices();
    console.log(price);
  }
  
}
</script>

<template>
  <div class="px-[1rem] md:px-[4rem]">
    <AssetsTable />
  </div>
  <button @click="onClick">OMEGALOL</button>
</template>
