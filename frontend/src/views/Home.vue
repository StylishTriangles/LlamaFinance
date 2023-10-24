<script setup lang="ts">
</script>

<template>
  <div class="hero mt-[-4rem] min-h-screen text-primary-content">
    <div
      class="hero-overlay place-items-center bg-gradient-to-bl from-primary to-base-100"
    />
    <div class="hero-content text-center">
      <div class="max-w-md">
        <h1 class="mb-5 text-5xl font-bold">
          Test app
        </h1>
        <p class="mb-5">
          Lorem ipsum dolor sit amet consectetur adipisicing elit. Quia voluptatem quos numquam laboriosam nostrum excepturi harum nemo quo odit a voluptatibus iste reiciendis officia eligendi suscipit, optio ipsum exercitationem rerum!
        </p>
        <RouterLink to="/store-test" class="btn-primary btn">
          Test Store
        </RouterLink>
      </div>
    </div>
  </div>
</template>

<style scoped></style>
