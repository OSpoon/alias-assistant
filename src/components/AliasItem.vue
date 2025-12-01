<script setup lang="ts">
import type { Alias } from '../types/alias';
import { Button } from "@/components/ui/button";

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
    class="flex justify-between items-center p-3 bg-card rounded-lg hover:bg-accent transition-all duration-300 shadow-sm hover:shadow-md opacity-0 animate-[fadeIn_0.4s_ease-out_forwards] cursor-pointer"
    :style="{ animationDelay: `${index * 50}ms` }"
    :title="alias.command"
  >
    <div class="flex flex-col flex-1 min-w-0">
      <code class="text-primary font-semibold text-base">{{ alias.name }}</code>
      <code class="text-sm text-muted-foreground mt-1 block truncate">{{ alias.command }}</code>
    </div>
    <div class="flex flex-col gap-2" @click.stop>
      <Button @click="$emit('openTerminal', alias.name)" variant="outline" size="sm" class="transition-all hover:scale-105">
        Terminal
      </Button>
      <Button @click="$emit('delete', alias.name)" variant="destructive" size="sm" class="transition-all hover:scale-105">
        Delete
      </Button>
    </div>
  </div>
</template>

