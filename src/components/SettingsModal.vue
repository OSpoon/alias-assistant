<script setup lang="ts">
import { ref, computed } from 'vue';
import { ask } from '@tauri-apps/plugin-dialog';
import { useUpdater } from '../composables/useUpdater';
import ToastNotification from './ToastNotification.vue';
import ThemeSelector from './ThemeSelector.vue';
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription, DialogFooter } from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import { Separator } from "@/components/ui/separator";

defineProps<{
  themes: Array<{ name: string; label: string }>;
  currentTheme: string;
}>();

defineEmits<{
  'update:currentTheme': [theme: string];
  export: [];
  import: [];
}>();

const open = ref(false);
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

function handleOpen() {
  open.value = true;
}

function handleClose() {
  open.value = false;
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

  // Confirm installation
  const confirmed = await ask(
    `A new version (v${status.value.version}) is available. The app will restart after installation. Continue?`,
    {
      title: "Install Update",
      kind: "info",
    }
  );

  if (!confirmed) {
    return;
  }

  installingUpdate.value = true;
  showToast.value = false;
  
  try {
    // downloadAndInstall will automatically restart the app after installation
    // On Windows, the app will exit automatically before installation
    // On macOS/Linux, the app will restart automatically after installation
    await installUpdate();
    
    // If we reach here (which is unlikely on most platforms),
    // the update installation completed and restart should happen automatically
    toastMessage.value = 'Update installed successfully. Restarting...';
    showToast.value = true;
    
    // Give a brief moment for the restart process to begin
    // Most platforms will restart automatically, so this is just a fallback message
    setTimeout(() => {
      // If we're still here after a delay, something might be wrong
      // But typically the app will have restarted by now
    }, 1000);
  } catch (error: any) {
    toastMessage.value = error.message || 'Failed to install update';
    showToast.value = true;
    setTimeout(() => {
      showToast.value = false;
    }, 5000);
    installingUpdate.value = false;
    status.value.downloading = false;
    status.value.progress = 0;
  }
}

defineExpose({
  open: handleOpen,
  close: handleClose,
});
</script>

<template>
  <Dialog v-model:open="open">
    <DialogContent class="max-w-md">
      <DialogHeader>
        <DialogTitle>Settings</DialogTitle>
        <DialogDescription>
          Configure theme, updates, and manage your aliases.
        </DialogDescription>
      </DialogHeader>
      
      <ThemeSelector
        :themes="themes"
        :current-theme="currentTheme"
        @update:current-theme="$emit('update:currentTheme', $event)"
      />

      <Separator />

      <div class="mb-6">
        <Label class="font-semibold mb-2 block">Updates</Label>
        <div class="flex flex-col gap-2 mt-2">
          <Button
            @click="handleInstallUpdate"
            :disabled="checkingUpdate || installingUpdate"
            :variant="status.available ? 'default' : 'outline'"
            class="transition-all hover:scale-105"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
            {{ updateButtonText }}
          </Button>
          <div v-if="status.currentVersion" class="text-xs text-muted-foreground mt-1">
            Current version: v{{ status.currentVersion }}
          </div>
          <div v-if="status.error" class="text-xs text-destructive mt-1">
            {{ status.error }}
          </div>
        </div>
      </div>

      <Separator />

      <div class="mb-6">
        <Label class="font-semibold mb-2 block">Alias Management</Label>
        <div class="flex flex-col gap-2 mt-2">
          <Button
            @click="$emit('export')"
            variant="outline"
            class="transition-all hover:scale-105"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
            Export Aliases
          </Button>
          <Button
            @click="$emit('import')"
            variant="outline"
            class="transition-all hover:scale-105"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
            </svg>
            Import Aliases
          </Button>
        </div>
      </div>

      <DialogFooter>
        <Button type="button" variant="outline" @click="handleClose">
          Close
        </Button>
      </DialogFooter>
    </DialogContent>
    
    <Teleport to="body">
      <ToastNotification
        :show="showToast"
        :message="toastMessage"
        @close="showToast = false"
      />
    </Teleport>
  </Dialog>
</template>

