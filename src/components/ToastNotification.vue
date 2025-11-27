<script setup lang="ts">
defineProps<{
  show: boolean;
}>();

defineEmits<{
  close: [];
}>();
</script>

<template>
  <div v-if="show" class="toast toast-center z-50">
    <div class="alert alert-success shadow-lg transition-all">
      <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current h-6 w-6" fill="none"
        viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
          d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
      <div>
        <h3 class="font-bold">Action successful!</h3>
        <div class="text-xs">To apply changes, reload your shell or open a new terminal.</div>
      </div>
      <button @click="$emit('close')" class="btn btn-sm btn-circle btn-ghost">✕</button>
    </div>
  </div>
</template>

