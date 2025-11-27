<script setup lang="ts">
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
    <label class="label">
      <span class="label-text font-semibold">Theme</span>
    </label>
    <div class="grid grid-cols-2 gap-2 mt-2">
      <button
        v-for="theme in themes"
        :key="theme.name"
        @click="$emit('update:currentTheme', theme.name)"
        :class="[
          'btn btn-outline transition-all rounded-lg',
          currentTheme === theme.name ? 'btn-primary' : ''
        ]"
      >
        {{ theme.label }}
      </button>
    </div>
  </div>
</template>

