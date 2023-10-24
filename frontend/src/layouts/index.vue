<script setup lang="ts">
import Navbar from "~/components/Navbar.vue";
</script>

<template>
  <div class="antialiased">
    <div class="min-h-screen bg-gradient-to-bl from-primary to-base-100">
      <Navbar />
      <RouterView />
    </div>
  </div>
</template>

<style scoped>

</style>
