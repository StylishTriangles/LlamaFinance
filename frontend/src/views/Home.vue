<script setup lang="ts">
</script>

<template>
  <div class="px-[1rem] md:px-[4rem]">
    <AssetsTable v-if="accountStore.walletAddress" />
  </div>
</template>
