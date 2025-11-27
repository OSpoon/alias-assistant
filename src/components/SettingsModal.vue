<script setup lang="ts">
import { ref } from 'vue';
import ThemeSelector from './ThemeSelector.vue';

defineProps<{
  themes: Array<{ name: string; label: string }>;
  currentTheme: string;
}>();

const emit = defineEmits<{
  'update:currentTheme': [theme: string];
  export: [];
  import: [];
}>();

const modalRef = ref<HTMLDialogElement | null>(null);

function open() {
  modalRef.value?.showModal();
}

function close() {
  modalRef.value?.close();
}

defineExpose({
  open,
  close,
});
</script>

<template>
  <dialog ref="modalRef" class="modal">
    <div class="modal-box rounded-lg">
      <h3 class="font-bold text-lg mb-4">Settings</h3>
      
      <ThemeSelector
        :themes="themes"
        :current-theme="currentTheme"
        @update:current-theme="$emit('update:currentTheme', $event)"
      />

      <div class="divider"></div>
      <div class="mb-6">
        <label class="label">
          <span class="label-text font-semibold">Alias Management</span>
        </label>
        <div class="flex flex-col gap-2 mt-2">
          <button
            @click="$emit('export')"
            class="btn btn-outline btn-info transition-all hover:scale-105 rounded-lg"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
            Export Aliases
          </button>
          <button
            @click="$emit('import')"
            class="btn btn-outline btn-success transition-all hover:scale-105 rounded-lg"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
            </svg>
            Import Aliases
          </button>
        </div>
      </div>

      <div class="modal-action">
        <button type="button" class="btn transition-all hover:scale-105 rounded-lg" @click="close">
          Close
        </button>
      </div>
    </div>
    <form method="dialog" class="modal-backdrop">
      <button @click="close">close</button>
    </form>
  </dialog>
</template>

