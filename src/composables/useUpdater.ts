import { ref } from 'vue';
import { check, type Update, type DownloadEvent } from '@tauri-apps/plugin-updater';

export interface UpdateStatus {
  available: boolean;
  currentVersion: string;
  version: string | null;
  downloading: boolean;
  progress: number;
  error: string | null;
}

export function useUpdater() {
  const status = ref<UpdateStatus>({
    available: false,
    currentVersion: '',
    version: null,
    downloading: false,
    progress: 0,
    error: null,
  });

  let updater: Update | null = null;

  async function checkForUpdate() {
    try {
      status.value.error = null;
      const result = await check();
      
      if (result) {
        updater = result;
        status.value.available = true;
        status.value.version = result.version;
        status.value.currentVersion = result.currentVersion;
        return true;
      } else {
        status.value.available = false;
        status.value.version = null;
        return false;
      }
    } catch (error: any) {
      status.value.error = error.message || 'Failed to check for updates';
      status.value.available = false;
      return false;
    }
  }

  async function installUpdate() {
    if (!updater) {
      throw new Error('No update available');
    }

    try {
      status.value.downloading = true;
      status.value.error = null;
      status.value.progress = 0;
      
      // Download and install with progress tracking
      // Note: downloadAndInstall will automatically relaunch the app after installation
      // On Windows: app exits before installation
      // On macOS/Linux: app restarts automatically after installation
      await updater.downloadAndInstall((event: DownloadEvent) => {
        if (event.event === 'Started') {
          status.value.progress = 5;
        } else if (event.event === 'Progress') {
          // Increment progress gradually to show activity
          // The actual progress depends on the download size
          if (event.data?.chunkLength) {
            status.value.progress = Math.min(status.value.progress + 2, 95);
          }
        } else if (event.event === 'Finished') {
          status.value.progress = 100;
          // At this point, installation is complete
          // The app will restart automatically on macOS/Linux
          // On Windows, the app already exited before installation
        }
      });
      
      // If we reach here (unlikely on most platforms),
      // the installation completed and restart should happen automatically
      // Give a brief moment for the restart process
      await new Promise(resolve => setTimeout(resolve, 500));
    } catch (error: any) {
      status.value.error = error.message || 'Failed to install update';
      status.value.downloading = false;
      status.value.progress = 0;
      throw error;
    }
  }

  return {
    status,
    checkForUpdate,
    installUpdate,
  };
}

