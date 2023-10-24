<script setup lang="ts">
onBeforeMount(() => {
  accountStore.init();
});
</script>

<template>
  <RouterView />
</template>
