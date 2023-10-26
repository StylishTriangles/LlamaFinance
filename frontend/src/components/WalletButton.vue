<script setup lang="ts">
import {
  ClipboardCopyIcon,
  ExternalLinkIcon,
  LogoutIcon,
} from "@heroicons/vue/outline";
import { appConfig } from "~/config";
import { abbreviate } from "~/utils";

const explorerLink = computed(() => {
  if (appConfig.NEXT_PUBLIC_CHAIN_NAME === "Coreum Testnet")
    return `https://explorer.testnet-1.coreum.dev/coreum/accounts/${accountStore.walletAddress}`;
  return `https://explorer.coreum.com/coreum/accounts/${accountStore.walletAddress}`;
});

const copied = ref(false);

async function onAddressCopy() {
  await navigator.clipboard.writeText(accountStore.walletAddress!);
  copied.value = true;

  setTimeout(() => {
    copied.value = false;
  }, 5 * 1000); // allow to copy again after 5 seconds
}

async function onConnect() {
  await accountStore.connectWallet();
}

function onDisconnect() {
  accountStore.disconnect();
}
</script>

<template>
  <button
    v-if="!accountStore.walletAddress"
    class="btn-accent btn"
    @click="onConnect"
  >
    <span v-if="accountStore.loading" class="loading loading-spinner" />
    <span v-else>Connect wallet</span>
  </button>
  <div v-else class="dropdown dropdown-end">
    <label tabindex="0" class="btn-accent btn">{{
      abbreviate(accountStore.walletAddress)
    }}</label>
    <ul
      tabindex="0"
      class="dropdown-content z-[1] menu mt-1 p-2 shadow bg-accent text-accent-content rounded-box w-52"
    >
      <li>
        <div class="flex" @click="onAddressCopy">
          <ClipboardCopyIcon class="w-4 shrink-0" />
          <span v-if="copied">Copied!</span>
          <span v-else>Copy Address</span>
        </div>
      </li>
      <li>
        <a
          :href="explorerLink"
          class="flex"
          target="_blank"
          rel="noopener noreferrer"
        >
          <ExternalLinkIcon class="w-4 shrink-0" />
          <span>Explorer</span>
        </a>
      </li>
      <hr class="my-1">
      <li>
        <div class="flex" @click="onDisconnect">
          <LogoutIcon class="w-4 shrink-0" />
          <span>Disconnect</span>
        </div>
      </li>
    </ul>
  </div>
</template>
