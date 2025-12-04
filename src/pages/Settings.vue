<script setup lang="ts">
import { ref, computed, onMounted, inject, type Ref } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import { ask } from '@tauri-apps/plugin-dialog';
import { openUrl } from '@tauri-apps/plugin-opener';
import { useUpdater } from '../composables/useUpdater';
import ToastNotification from '../components/ToastNotification.vue';
import ThemeSelector from '../components/ThemeSelector.vue';
import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { Separator } from "@/components/ui/separator";

const router = useRouter();

// Inject theme and functions from App.vue
const themes = inject<Array<{ name: string; label: string }>>('themes', []);
const currentTheme = inject<Ref<string>>('currentTheme', ref('slate'));
const setTheme = inject<(theme: string) => void>('setTheme', () => { });
const exportAliases = inject<() => Promise<void>>('exportAliases', async () => { });
const importAliases = inject<() => Promise<void>>('importAliases', async () => { });

const { status, checkForUpdate, installUpdate } = useUpdater();
const checkingUpdate = ref(false);
const installingUpdate = ref(false);
const showToast = ref(false);
const toastMessage = ref('');

// Gist sync state
const gistToken = ref('');
const gistId = ref<string | null>(null);
const showTokenInput = ref(false);
const syncingToGist = ref(false);
const syncingFromGist = ref(false);

const updateButtonText = computed(() => {
  if (checkingUpdate.value) return 'Checking...';
  if (installingUpdate.value) return `Installing... ${status.value.progress}%`;
  if (status.value.available) return `Update to v${status.value.version}`;
  return 'Check for Updates';
});

async function loadGistConfig() {
  try {
    const token = await invoke<string | null>('get_gist_token');
    const id = await invoke<string | null>('get_gist_id');
    gistToken.value = token || '';
    gistId.value = id;
    showTokenInput.value = !token;
  } catch (error) {
    console.error('Failed to load Gist config:', error);
  }
}

async function saveGistToken() {
  if (!gistToken.value.trim()) {
    toastMessage.value = 'Please enter a GitHub token';
    showToast.value = true;
    setTimeout(() => { showToast.value = false; }, 3000);
    return;
  }

  try {
    await invoke('set_gist_token', { token: gistToken.value.trim() });
    showTokenInput.value = false;
    toastMessage.value = 'GitHub token saved successfully';
    showToast.value = true;
    setTimeout(() => { showToast.value = false; }, 3000);
    await loadGistConfig();
  } catch (error: any) {
    toastMessage.value = error || 'Failed to save token';
    showToast.value = true;
    setTimeout(() => { showToast.value = false; }, 3000);
  }
}

async function handleSyncToGist() {
  syncingToGist.value = true;
  showToast.value = false;

  try {
    await invoke('sync_aliases_to_gist', { content: await invoke<string>('export_aliases') });
    await loadGistConfig();
    toastMessage.value = 'Successfully synced to GitHub Gist';
    showToast.value = true;
    setTimeout(() => { showToast.value = false; }, 3000);
  } catch (error: any) {
    toastMessage.value = error || 'Failed to sync to Gist';
    showToast.value = true;
    setTimeout(() => { showToast.value = false; }, 5000);
  } finally {
    syncingToGist.value = false;
  }
}

async function handleSyncFromGist() {
  syncingFromGist.value = true;
  showToast.value = false;

  try {
    const content = await invoke<string>('sync_aliases_from_gist');
    const confirmed = await ask(
      'This will merge aliases from Gist with existing ones. Aliases with the same name will be replaced. Continue?',
      {
        title: 'Sync from Gist',
        kind: 'warning',
      }
    );

    if (confirmed) {
      await invoke('import_aliases_from_content', { content });
      toastMessage.value = 'Successfully synced from GitHub Gist';
      showToast.value = true;
      setTimeout(() => { showToast.value = false; }, 3000);
      // Navigate back to home to refresh aliases list
      router.push('/');
    }
  } catch (error: any) {
    toastMessage.value = error || 'Failed to sync from Gist';
    showToast.value = true;
    setTimeout(() => { showToast.value = false; }, 5000);
  } finally {
    syncingFromGist.value = false;
  }
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
    await installUpdate();
    toastMessage.value = 'Update installed successfully. Restarting...';
    showToast.value = true;
    setTimeout(() => {
      showToast.value = false;
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

onMounted(() => {
  loadGistConfig();
});
</script>

<template>
  <div class="flex flex-col flex-1 min-h-0">
    <div class="flex justify-between items-center mt-2 mb-6">
      <h1 class="text-3xl font-bold text-primary text-shadow-lg">Settings</h1>
      <Button @click="router.push('/')" variant="ghost" size="icon" title="Settings">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
        </svg>
      </Button>
    </div>

    <div class="flex-1 overflow-y-auto">
      <div class="max-w-2xl mx-auto space-y-6 px-4">
        <div class="opacity-0 animate-[fadeIn_0.4s_ease-out_forwards]" style="animation-delay: 0ms">
          <Label class="font-semibold mb-2 block text-base">Theme</Label>
          <ThemeSelector :themes="themes" :current-theme="currentTheme" @update:current-theme="setTheme" />
        </div>

        <Separator class="opacity-0 animate-[fadeIn_0.4s_ease-out_forwards]" style="animation-delay: 50ms" />

        <div class="opacity-0 animate-[fadeIn_0.4s_ease-out_forwards]" style="animation-delay: 100ms">
          <Label class="font-semibold mb-2 block text-base">Updates</Label>
          <div class="flex flex-col gap-2 mt-2">
            <Button @click="handleInstallUpdate" :disabled="checkingUpdate || installingUpdate"
              :variant="status.available ? 'default' : 'outline'" class="transition-all hover:scale-105">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
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

        <Separator class="opacity-0 animate-[fadeIn_0.4s_ease-out_forwards]" style="animation-delay: 150ms" />

        <div class="opacity-0 animate-[fadeIn_0.4s_ease-out_forwards]" style="animation-delay: 200ms">
          <Label class="font-semibold mb-2 block text-base">GitHub Gist Sync</Label>
          <p class="text-sm text-muted-foreground mb-4">
            Sync your aliases to GitHub Gist for backup and cross-device synchronization.
          </p>
          <div class="flex flex-col gap-4 mt-2">
            <div v-if="showTokenInput" class="flex flex-col gap-2">
              <div>
                <Label for="gist-token" class="text-sm mb-1 block">
                  GitHub Personal Access Token
                </Label>
                <Input id="gist-token" v-model="gistToken" type="password" placeholder="ghp_xxxxxxxxxxxx"
                  class="w-full" />
                <p class="text-xs text-muted-foreground mt-2">
                  Create a token at
                  <button
                    @click="openUrl('https://github.com/settings/tokens')"
                    class="text-primary underline hover:text-primary/80 cursor-pointer"
                  >
                    github.com/settings/tokens
                  </button>
                  with <code class="text-xs bg-muted px-1 rounded">gist</code> scope
                </p>
              </div>
              <Button @click="saveGistToken" variant="default" class="transition-all hover:scale-105">
                Save Token
              </Button>
            </div>
            <div v-else class="flex flex-col gap-2">
              <div v-if="gistId" class="text-sm text-muted-foreground mb-1">
                Gist ID: <code class="bg-muted px-2 py-1 rounded text-xs">{{ gistId }}</code>
              </div>
              <Button @click="showTokenInput = true" variant="outline" size="sm">
                Change Token
              </Button>
              <div class="flex gap-2">
                <Button @click="handleSyncToGist" :disabled="syncingToGist || syncingFromGist" variant="default"
                  class="flex-1 transition-all hover:scale-105">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24"
                    stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                      d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
                  </svg>
                  {{ syncingToGist ? 'Syncing...' : 'Sync to Gist' }}
                </Button>
                <Button @click="handleSyncFromGist" :disabled="syncingToGist || syncingFromGist" variant="outline"
                  class="flex-1 transition-all hover:scale-105">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24"
                    stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                      d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                  </svg>
                  {{ syncingFromGist ? 'Syncing...' : 'Sync from Gist' }}
                </Button>
              </div>
            </div>
          </div>
        </div>

        <Separator class="opacity-0 animate-[fadeIn_0.4s_ease-out_forwards]" style="animation-delay: 250ms" />

        <div class="opacity-0 animate-[fadeIn_0.4s_ease-out_forwards]" style="animation-delay: 300ms">
          <Label class="font-semibold mb-2 block text-base">Alias Management</Label>
          <p class="text-sm text-muted-foreground mb-4">
            Export or import your aliases to/from a file.
          </p>
          <div class="flex flex-col gap-2 mt-2">
            <Button @click="exportAliases" variant="outline" class="transition-all hover:scale-105">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
              </svg>
              Export Aliases
            </Button>
            <Button @click="importAliases" variant="outline" class="transition-all hover:scale-105">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24"
                stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
              </svg>
              Import Aliases
            </Button>
          </div>
        </div>
      </div>
    </div>

    <Teleport to="body">
      <ToastNotification :show="showToast" :message="toastMessage" @close="showToast = false" />
    </Teleport>
  </div>
</template>
