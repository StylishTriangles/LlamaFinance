<script setup lang="ts">
import ThemeChange from "./ThemeChange/index.vue";

defineOptions({
  name: "Navbar",
});
</script>

<template>
  <div
    class="sticky top-0 z-30 flex h-16 w-full justify-center text-base-content opacity-90 backdrop-blur transition-all duration-100"
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
            class="inline-flex text-lg text-primary transition-all duration-200 md:text-3xl"
          >
            <span class="text-accent">L<span class="lowercase">lama</span></span>
            <span class="text-base-content">F<span class="lowercase">inance</span></span>
          </div>
        </RouterLink>
      </div>
      <div>
        <ThemeChange />
      </div>
    </nav>
  </div>
</template>
