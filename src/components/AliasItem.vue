<script setup lang="ts">
import type { Alias } from '../types/alias';

defineProps<{
  alias: Alias;
  index: number;
}>();

defineEmits<{
  copy: [aliasName: string];
  openTerminal: [aliasName: string];
  delete: [aliasName: string];
}>();
</script>

<template>
  <div
    v-if="alias && alias.name"
    @click="$emit('copy', alias.name)"
    class="flex justify-between items-center p-3 bg-base-100 rounded-lg hover:bg-base-300 transition-all duration-300 shadow-sm hover:shadow-md opacity-0 animate-[fadeIn_0.4s_ease-out_forwards] cursor-pointer"
    :style="{ animationDelay: `${index * 50}ms` }"
  >
    <div class="flex flex-col flex-1 min-w-0">
      <code class="text-primary font-semibold text-base">{{ alias.name }}</code>
      <div class="tooltip tooltip-top" :data-tip="alias.command">
        <code class="text-sm text-base-content/70 mt-1 block truncate">{{ alias.command }}</code>
      </div>
    </div>
    <div class="flex flex-col gap-2" @click.stop>
      <button @click="$emit('openTerminal', alias.name)" class="btn btn-info btn-xs transition-all hover:scale-105 rounded-lg">
        Terminal
      </button>
      <button @click="$emit('delete', alias.name)" class="btn btn-error btn-xs transition-all hover:scale-105 rounded-lg">
        Delete
      </button>
    </div>
  </div>
</template>

