<script setup lang="ts">
import { LogoutIcon } from "@heroicons/vue/outline";
import ThemeChange from "./ThemeChange/index.vue";
import { abbreviate } from "~/utils";

async function onConnect() {
  await accountStore.connectWallet();
}

function onDisconnect() {
  accountStore.disconnect();
}
</script>

<template>
  <div
    class="sticky mb-6 top-0 z-30 flex h-16 w-full justify-center text-base-content opacity-90 backdrop-blur transition-all duration-100"
  >
    <nav class="navbar w-full">
      <div class="flex flex-1 md:gap-1 lg:gap-2">
        <RouterLink
          to="/"
          aria-current="page"
          aria-label="Homepage"
          class="btn-ghost btn px-2"
        >
          <div
            class="inline-flex text-lg text-primary items-center transition-all duration-200 md:text-3xl"
          >
            <img alt="logo" src="pwa-192.png" class="w-10">
            <span class="text-accent">L<span class="lowercase">lama</span></span>
            <span class="text-base-content">F<span class="lowercase">inance</span></span>
          </div>
        </RouterLink>
      </div>
      <div class="flex gap-4">
        <ThemeChange />
        <button v-if="!accountStore.walletAddress" class="btn-accent btn" @click="onConnect">
          <span v-if="accountStore.loading" class="loading loading-spinner" />
          <span v-else>Connect wallet</span>
        </button>
        <button v-else class="btn-accent btn" @click="onDisconnect">
          {{ abbreviate(accountStore.walletAddress) }}
          <LogoutIcon class="stroke-current w-5" />
        </button>
      </div>
    </nav>
  </div>
</template>
