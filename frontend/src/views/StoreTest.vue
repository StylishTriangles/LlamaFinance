<script setup lang="ts">
import { testStore } from "~/store";
</script>

<template>
  <div class="flex flex-col items-center gap-20 py-20">
    <p class="text-xl font-medium uppercase tracking-wide">
      Counter:
    </p>
    <p class="text-4xl font-bold uppercase">
      {{ testStore.counter }}
    </p>
    <button type="button" class="btn-primary btn" @click="testStore.increment()">
      increment
    </button>
  </div>
</template>
