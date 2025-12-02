<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";

defineProps<{
  themes: Array<{ name: string; label: string }>;
  currentTheme: string;
}>();

defineEmits<{
  'update:currentTheme': [theme: string];
}>();
</script>

<template>
  <div class="mb-6">
    <div class="grid grid-cols-2 gap-2 mt-2">
      <Button
        v-for="theme in themes"
        :key="theme.name"
        @click="$emit('update:currentTheme', theme.name)"
        :variant="currentTheme === theme.name ? 'default' : 'outline'"
        class="transition-all"
      >
        {{ theme.label }}
      </Button>
    </div>
  </div>
</template>

