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
            class="px-2 pb-2 border-b text-lg lg:text-xl border-secondary"
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
            class="px-2 py-3 font-normal text-base lg:text-lg"
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
</template>
