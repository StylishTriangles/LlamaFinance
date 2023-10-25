<script lang="ts" setup>
defineProps({
  id: {
    type: String,
    required: true,
  },
  title: {
    type: String,
    required: true,
  },
  btnText: {
    type: String,
    default: "Submit",
  },
  isLoading: {
    type: Boolean,
    default: false,
  },
});

const emit = defineEmits(["submit"]);

function onSubmit() {
  emit("submit");
}
</script>

<template>
  <dialog :id="id" class="modal shadow-xl backdrop-blur">
    <div class="modal-box bg-neutral text-neutral-content">
      <h1 class="font-bold text-2xl mb-6">
        {{ title }}
      </h1>

      <slot />

      <div class="w-full mt-10 flex justify-center">
        <button class="w-1/2 p-2 btn btn-primary" @click="onSubmit">
          <span v-if="isLoading" class="loading loading-spinner loading-md" />
          {{ btnText }}
        </button>
      </div>
    </div>

    <!-- A trick that's closing the dialog when clicked outside of it -->
    <form method="dialog" class="modal-backdrop opacity-100">
      <button class="outline-none cursor-default" :style="{ background: 'transparent' }">
        close
      </button>
    </form>
  </dialog>
</template>
