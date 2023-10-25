<script setup lang="ts">
import { BadgeCheckIcon } from "@heroicons/vue/outline";
import { modalsID } from "~/config";

defineProps({
  txnHash: {
    type: String,
    required: true,
  },
});

function onClose() {
  const dialog = document.getElementById(modalsID.TXN_SUCCESS);
  (dialog as any).close();
}
</script>

<template>
  <BaseModal
    :id="modalsID.TXN_SUCCESS"
    title="Success!"
    btn-text="Close"
    @submit="onClose"
  >
    <div class="text-xl text-success">
      Your transaction has been successfully processed.
    </div>
    <div class="flex flex-col items-center mt-2">
      <BadgeCheckIcon class="text-success w-32" />
      <a
        :href="`https://explorer.testnet-1.coreum.dev/coreum/transactions/${txnHash}`"
        target="_blank"
        rel="noopener noreferrer"
        class="text-accent underline"
      >
        View on explorer
      </a>
    </div>
  </BaseModal>
</template>
