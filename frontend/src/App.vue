<script setup lang="ts">
onBeforeMount(() => {
  accountStore.init();
});
</script>

<template>
  <ModalsController />
  <RouterView />
</template>
