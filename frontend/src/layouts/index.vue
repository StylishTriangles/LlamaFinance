<script setup lang="ts">
import Navbar from "~/components/Navbar.vue";
</script>

<template>
  <div class="antialiased">
    <div class="min-h-screen bg-gradient-to-bl from-primary to-base-100">
      <div class="mx-auto max-w-7xl pb-20">
        <Navbar />
        <RouterView v-if="accountStore.walletAddress" />
        <div v-else class="flex flex-col items-center gap-20 py-20 text-primary-content">
          <div class="card w-108">
            <div class="card-body">
              <h2 class="card-title text-4xl">
                Not Connected!
              </h2>
              <p class="text-xl">
                Please connect with your Keplr wallet in order to use LlamaFinance
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>

</style>
