<script setup lang="ts"></script>

<template>
  <div class="px-[1rem] md:px-[4rem]">
    <template v-if="accountStore.walletAddress">
      <YourDepositsTable />
      <LendingTable />
    </template>
  </div>
</template>
