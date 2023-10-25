<script setup lang="ts">
import { formatUSDAmount } from "~/utils";

const totalCollateralized = ref(843280.53); // TODO
const totalBorrowed = ref(1234.53); // TODO
const visibleYourTable = ref("COLLATERAL");
</script>

<template>
  <div class="px-[1rem] md:px-[4rem]">
    <Card title="">
      <template #top-left>
        <div class="flex flex-col sm:flex-row sm:divide-x gap-x-4">
          <h2
            class="card-title text-base sm:text-xl md:text-3xl cursor-pointer transition-all duration-200 hover:opacity-100 hover:text-accent/80"
            :class="visibleYourTable === 'COLLATERAL' && 'text-accent'"
            @click="visibleYourTable = 'COLLATERAL'"
          >
            Your Collaterals
          </h2>
          <h2
            class="card-title sm:pl-4 text-base sm:text-xl md:text-3xl cursor-pointer transition-all duration-200 hover:opacity-100 hover:text-accent/80"
            :class="visibleYourTable === 'BORROW' && 'text-accent'"
            @click="visibleYourTable = 'BORROW'"
          >
            Your Borrows
          </h2>
        </div>
      </template>

      <template
        v-if="(
          visibleYourTable === 'COLLATERAL' && totalCollateralized > 0)
          || (visibleYourTable === 'BORROW' && totalBorrowed > 0
          )"
        #top-right
      >
        <template v-if="visibleYourTable === 'COLLATERAL'">
          <p class="font-bold text-base sm:text-xl md:text-3xl">
            Total Collateralized
          </p>
          <p>{{ formatUSDAmount(totalCollateralized) }}</p>
        </template>
        <template v-else>
          <p class="font-bold text-base sm:text-xl md:text-3xl">
            Total Borrowed
          </p>
          <p>{{ formatUSDAmount(totalBorrowed) }}</p>
        </template>
      </template>

      <YourCollateralsTable v-if="visibleYourTable === 'COLLATERAL'" />
      <YourBorrowsTable v-else />
    </Card>

    <div class="flex gap-x-4 lg:flex-row flex-col">
      <CollateralTable />
      <BorrowsTable />
    </div>
  </div>
</template>
