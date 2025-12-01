<script setup lang="ts">
import { computed } from 'vue';
import type { Alias } from '../types/alias';
import AliasItem from './AliasItem.vue';

const props = defineProps<{
  aliases: Alias[];
  searchQuery: string;
}>();

defineEmits<{
  copy: [aliasName: string];
  openTerminal: [aliasName: string];
  delete: [aliasName: string];
}>();

const filteredAliases = computed(() => {
  if (!props.searchQuery.trim()) {
    return props.aliases;
  }
  const query = props.searchQuery.toLowerCase();
  return props.aliases.filter(
    (alias) =>
      alias.name.toLowerCase().includes(query) ||
      alias.command.toLowerCase().includes(query)
  );
});
</script>

<template>
  <div class="bg-muted/50 flex-1 flex flex-col min-h-0 rounded-lg p-4">
    <div class="flex flex-col overflow-y-auto min-h-0 flex-1 scroll-smooth">
      <div class="space-y-2 flex flex-col">
        <div v-if="filteredAliases.length === 0"
          class="text-center text-muted-foreground flex-1 flex items-center justify-center">
          <span v-if="searchQuery.trim()">
            No aliases found matching "{{ searchQuery }}"
          </span>
          <span v-else>
            No aliases found. Add one using the button below!
          </span>
        </div>
        <AliasItem
          v-for="(alias, index) in filteredAliases"
          :key="alias ? alias.name : ''"
          :alias="alias"
          :index="index"
          @copy="$emit('copy', $event)"
          @open-terminal="$emit('openTerminal', $event)"
          @delete="$emit('delete', $event)"
        />
      </div>
    </div>
  </div>
</template>

