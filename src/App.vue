<script setup lang="ts">
import { provide } from "vue";
import { RouterView } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import { ask, save, open } from "@tauri-apps/plugin-dialog";
import { writeTextFile, readTextFile } from "@tauri-apps/plugin-fs";
import { useTheme } from "./composables/useTheme";

// Initialize theme
const { themes, currentTheme, setTheme } = useTheme();

// Export aliases function
async function exportAliases() {
  try {
    const content = await invoke<string>("export_aliases");
    
    const filePath = await save({
      defaultPath: "aliases.sh",
      filters: [
        {
          name: "Shell Script",
          extensions: ["sh"],
        },
        {
          name: "All Files",
          extensions: ["*"],
        },
      ],
    });

    if (filePath) {
      await writeTextFile(filePath, content);
    }
  } catch (error) {
    console.error("Failed to export aliases:", error);
    alert(`Failed to export aliases: ${error}`);
  }
}

// Import aliases function
async function importAliases() {
  try {
    const filePath = await open({
      multiple: false,
      filters: [
        {
          name: "Shell Script",
          extensions: ["sh"],
        },
        {
          name: "All Files",
          extensions: ["*"],
        },
      ],
    });

    if (filePath) {
      const path = Array.isArray(filePath) ? filePath[0] : filePath;
      const content = await readTextFile(path);
      
      const confirmed = await ask(
        "This will merge imported aliases with existing ones. Aliases with the same name will be replaced. Continue?",
        {
          title: "Import Aliases",
          kind: "warning",
        }
      );

      if (confirmed) {
        await invoke("import_aliases_from_content", { content });
      }
    }
  } catch (error) {
    console.error("Failed to import aliases:", error);
    alert(`Failed to import aliases: ${error}`);
  }
}

// Provide theme and functions to child components
provide('themes', themes);
provide('currentTheme', currentTheme);
provide('setTheme', setTheme);
provide('exportAliases', exportAliases);
provide('importAliases', importAliases);
</script>

<template>
  <div class="h-screen flex flex-col bg-background overflow-hidden">
    <main class="flex flex-col flex-1 p-4 min-h-0">
      <RouterView v-slot="{ Component, route }">
        <Transition name="fade" mode="out-in">
          <component :is="Component" :key="route.path" />
        </Transition>
      </RouterView>
    </main>
  </div>
</template>

<style>
@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* Simple fade transition */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.25s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
