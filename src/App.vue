<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { ask, save, open } from "@tauri-apps/plugin-dialog";
import { writeTextFile, readTextFile } from "@tauri-apps/plugin-fs";
import type { Alias } from "./types/alias";
import ToastNotification from "./components/ToastNotification.vue";
import SearchBar from "./components/SearchBar.vue";
import AliasList from "./components/AliasList.vue";
import AddAliasModal from "./components/AddAliasModal.vue";
import SettingsModal from "./components/SettingsModal.vue";

const aliases = ref<Alias[]>([]);
const showReloadHint = ref(false);
const addAliasModal = ref<InstanceType<typeof AddAliasModal> | null>(null);
const configModal = ref<InstanceType<typeof SettingsModal> | null>(null);
const searchQuery = ref("");

// Theme management
const themes = [
  { name: "light", label: "Light" },
  { name: "dark", label: "Dark" },
  { name: "forest", label: "Forest" },
  { name: "synthwave", label: "Synthwave" },
  { name: "dracula", label: "Dracula" },
];

const currentTheme = ref<string>("forest");

async function fetchAliases() {
  try {
    const result = await invoke("get_aliases");
    console.log("Aliases fetched:", result);
    aliases.value = result as Alias[];
  } catch (error) {
    console.error("Failed to fetch aliases:", error);
  }
}

async function handleAddAlias(name: string, command: string) {
  try {
    await invoke("add_alias", { name, command });
    showReloadHint.value = true;
    setTimeout(() => { showReloadHint.value = false; }, 2000);
    await fetchAliases();
  } catch (error) {
    console.error("Failed to add alias:", error);
    alert(`Failed to add alias: ${error}`);
  }
}

async function deleteAlias(name: string) {
  const confirmed = await ask(
    `Are you sure you want to delete the alias "${name}"?`,
    {
      title: "Delete Alias",
    }
  );
  if (!confirmed) {
    return;
  }
  try {
    await invoke("delete_alias", { name });
    showReloadHint.value = true;
    setTimeout(() => { showReloadHint.value = false; }, 2000);
    await fetchAliases();
  } catch (error) {
    console.error("Failed to delete alias:", error);
  }
}

async function openTerminal(aliasName: string) {
  try {
    await invoke("open_terminal", { aliasName });
  } catch (error) {
    console.error("Failed to open terminal:", error);
    alert(`Failed to open terminal: ${error}`);
  }
}

async function copyAliasName(aliasName: string) {
  try {
    await invoke("copy_alias_name", { aliasName });
  } catch (error) {
    console.error("Failed to copy alias name:", error);
    alert(`Failed to copy alias name: ${error}`);
  }
}

// Theme functions
function setTheme(theme: string) {
  currentTheme.value = theme;
  const html = document.documentElement;
  html.setAttribute("data-theme", theme);
  localStorage.setItem("alias-assistant-theme", theme);
}

function loadTheme() {
  const savedTheme = localStorage.getItem("alias-assistant-theme") || "forest";
  setTheme(savedTheme);
}

// Export aliases
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

// Import aliases
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
        await fetchAliases();
      }
    }
  } catch (error) {
    console.error("Failed to import aliases:", error);
    alert(`Failed to import aliases: ${error}`);
  }
}

onMounted(async () => {
  loadTheme();
  await fetchAliases();
  try {
    await invoke("ensure_sourcing_is_setup");
  } catch (error) {
    console.error("Failed to ensure sourcing setup:", error);
    alert(`Setup failed: ${error}`);
  }
});
</script>

<template>
  <div :data-theme="currentTheme" class="h-screen flex flex-col bg-base-100 overflow-hidden">
    <main class="flex flex-col flex-1 p-4 min-h-0">
      <ToastNotification :show="showReloadHint" @close="showReloadHint = false" />

      <div class="flex justify-between items-center mb-4">
        <h1 class="text-3xl font-bold text-primary">Alias Assistant ✨</h1>
        <button 
          @click="configModal?.open()" 
          class="btn btn-ghost btn-circle"
          title="Settings"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
          </svg>
        </button>
      </div>

      <SearchBar v-model="searchQuery" />

      <AliasList
        :aliases="aliases"
        :search-query="searchQuery"
        @copy="copyAliasName"
        @open-terminal="openTerminal"
        @delete="deleteAlias"
      />

      <!-- FAB to open modal -->
      <div class="fixed bottom-8 right-8">
        <button class="btn btn-primary btn-circle btn-lg shadow-lg hover:shadow-xl transition-all hover:scale-110" @click="addAliasModal?.open()">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
        </button>
      </div>

      <AddAliasModal ref="addAliasModal" @submit="handleAddAlias" />

      <SettingsModal
        ref="configModal"
        :themes="themes"
        :current-theme="currentTheme"
        @update:current-theme="setTheme"
        @export="exportAliases"
        @import="importAliases"
      />
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
</style>
