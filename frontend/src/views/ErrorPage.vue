<script setup lang="ts">
import { useRouter } from "vue-router";

const router = useRouter();
</script>

<template>
  <div class="flex flex-col items-center gap-20 py-20">
    <div class="card w-96">
      <div class="card-body">
        <h2 class="card-title">
          Error 404
        </h2>
        <p>We're sorry, something is not right, page not found!</p>
        <div class="card-actions justify-end">
          <button class="btn-primary btn" @click="router.back()">
            Back!
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
