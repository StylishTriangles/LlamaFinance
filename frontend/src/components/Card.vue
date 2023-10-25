<script lang="ts" setup>
defineProps({
  title: {
    type: String,
    required: true,
  },
});
</script>

<template>
  <div class="card bg-neutral w-full text-neutral-content rounded-xl mt-12 lg:mt-26 shadow-2xl">
    <div class="card-body">
      <div class="flex items-start justify-between mb-10">
        <h2 class="card-title text-xl md:text-3xl">
          {{ title }}
        </h2>
        <div class="text-right text-base md:text-xl font-medium">
          <slot name="top-right" />
        </div>
      </div>
      <slot />
    </div>
  </div>
</template>
