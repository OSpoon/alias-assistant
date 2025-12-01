<script setup lang="ts">
import { Teleport } from "vue";
import { Button } from "@/components/ui/button";

defineProps<{
  show: boolean;
  message?: string;
}>();

defineEmits<{
  close: [];
}>();
</script>

<template>
  <Teleport to="body">
    <div v-if="show" class="fixed top-4 left-1/2 transform -translate-x-1/2 z-50">
      <div class="bg-primary text-primary-foreground shadow-lg rounded-lg p-4 flex items-center gap-3 min-w-[300px] max-w-md transition-all">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 flex-shrink-0" fill="none"
          viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
            d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        <div class="flex-1">
          <h3 class="font-bold">{{ message || 'Notification' }}</h3>
        </div>
        <Button @click="$emit('close')" variant="ghost" size="icon-sm" class="text-primary-foreground hover:bg-primary-foreground/20">
          ✕
        </Button>
      </div>
    </div>
  </Teleport>
</template>

