<script lang="ts" setup>
import { type PropType } from "vue";
import type { TableColumn } from "~/types";

defineProps({
  columns: {
    type: Array as PropType<TableColumn[]>,
    required: true,
  },
  data: {
    type: Array as PropType<any[]>,
    required: true,
  },
  isLoading: {
    type: Boolean,
    default: false,
  },
  noDataMessage: {
    type: String,
    default: "No data found",
  },
});
</script>

<template>
  <div class="overflow-x-auto">
    <table class="min-w-full table text-left">
      <thead>
        <tr>
          <th
            v-for="col in columns"
            :key="col.accessor"
            class="pl-2 pb-2 border-b text-lg lg:text-xl border-secondary"
          >
            {{ col.header }}
          </th>
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="(row, rowId) in data"
          :key="rowId"
        >
          <th
            v-for="col in columns"
            :key="col.accessor + rowId"
            class="pl-2 py-3 font-normal text-base lg:text-lg"
            :class="rowId % 2 === 1 && 'bg-secondary/10'"
          >
            <slot :name="col.accessor" v-bind="row">
              {{ row[col.accessor] }}
            </slot>
          </th>
        </tr>
      </tbody>
    </table>
  </div>

  <!-- <div class="overflow-x-auto">
    <div
      class="grid"
      :style="{ gridTemplateColumns: columnWidths }"
    >
      <div
        v-for="col in columns"
        :key="col.accessor"
        class="mb-2 select-none whitespace-nowrap border-b border-t border-secondary px-4 py-3 text-base font-medium lg:text-lg"
      >
        <span>{{ col.header }}</span>
      </div>

      <div v-if="isLoading" class="col-span-full loading loading-spinner mx-auto" />
      <div v-else-if="data.length === 0" class="col-span-full px-2 text-center">
        <p class="py-10 text-base tracking-wider">
          {{ noDataMessage }}
        </p>
      </div>

      <div
        v-for="(row, rowId) in data"
        v-else
        :key="rowId"
        class="relative col-span-full mx-2 my-2"
      >
        <div
          class="relative grid items-center rounded-[15px] text-sm transition-all ease-in"
          :style="{ gridTemplateColumns: columnWidths }"
        >
          <div
            v-for="(col) in columns"
            :key="col.accessor"
            class="flex h-full items-center break-words py-2 px-4"
          >
            <slot :name="col.accessor" v-bind="row">
              {{ row[col.accessor] }}
            </slot>
          </div>
        </div>
      </div>
    </div>
  </div> -->
</template>
