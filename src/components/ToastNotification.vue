<script setup lang="ts">
defineProps<{
  show: boolean;
  message?: string;
}>();

defineEmits<{
  close: [];
}>();
</script>

<template>
  <div v-if="show" class="toast toast-center z-50">
    <div class="alert alert-info shadow-lg transition-all">
      <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current h-6 w-6" fill="none"
        viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
          d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
      <div>
        <h3 class="font-bold">{{ message || 'Notification' }}</h3>
      </div>
      <button @click="$emit('close')" class="btn btn-sm btn-circle btn-ghost">✕</button>
    </div>
  </div>
</template>

