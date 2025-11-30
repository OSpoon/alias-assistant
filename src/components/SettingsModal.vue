<script setup lang="ts">
import { ref, computed, Teleport } from 'vue';
import ThemeSelector from './ThemeSelector.vue';
import { useUpdater } from '../composables/useUpdater';
import ToastNotification from './ToastNotification.vue';

defineProps<{
  themes: Array<{ name: string; label: string }>;
  currentTheme: string;
}>();

defineEmits<{
  'update:currentTheme': [theme: string];
  export: [];
  import: [];
}>();

const modalRef = ref<HTMLDialogElement | null>(null);
const { status, checkForUpdate, installUpdate } = useUpdater();
const checkingUpdate = ref(false);
const installingUpdate = ref(false);
const showToast = ref(false);
const toastMessage = ref('');

const updateButtonText = computed(() => {
  if (checkingUpdate.value) return 'Checking...';
  if (installingUpdate.value) return `Installing... ${status.value.progress}%`;
  if (status.value.available) return `Update to v${status.value.version}`;
  return 'Check for Updates';
});

function open() {
  modalRef.value?.showModal();
}

function close() {
  modalRef.value?.close();
}

async function handleCheckUpdate() {
  checkingUpdate.value = true;
  showToast.value = false;
  
  try {
    const hasUpdate = await checkForUpdate();
    if (hasUpdate) {
      toastMessage.value = `Update available: v${status.value.version}`;
    } else {
      toastMessage.value = 'You are using the latest version';
    }
    showToast.value = true;
    setTimeout(() => {
      showToast.value = false;
    }, 3000);
  } catch (error: any) {
    toastMessage.value = error.message || 'Failed to check for updates';
    showToast.value = true;
    setTimeout(() => {
      showToast.value = false;
    }, 3000);
  } finally {
    checkingUpdate.value = false;
  }
}

async function handleInstallUpdate() {
  if (!status.value.available) {
    await handleCheckUpdate();
    return;
  }

  installingUpdate.value = true;
  showToast.value = false;
  
  try {
    await installUpdate();
    // If install succeeds, the app will relaunch, so we won't reach here
  } catch (error: any) {
    toastMessage.value = error.message || 'Failed to install update';
    showToast.value = true;
    setTimeout(() => {
      showToast.value = false;
    }, 3000);
    installingUpdate.value = false;
  }
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
          <span class="label-text font-semibold">Updates</span>
        </label>
        <div class="flex flex-col gap-2 mt-2">
          <button
            @click="handleInstallUpdate"
            :disabled="checkingUpdate || installingUpdate"
            :class="[
              'btn transition-all hover:scale-105 rounded-lg',
              status.available ? 'btn-primary' : 'btn-outline btn-info'
            ]"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
            {{ updateButtonText }}
          </button>
          <div v-if="status.currentVersion" class="text-xs text-base-content/60 mt-1">
            Current version: v{{ status.currentVersion }}
          </div>
          <div v-if="status.error" class="text-xs text-error mt-1">
            {{ status.error }}
          </div>
        </div>
      </div>

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
    
    <Teleport to="body">
      <ToastNotification
        :show="showToast"
        :message="toastMessage"
        @close="showToast = false"
      />
    </Teleport>
  </dialog>
</template>

