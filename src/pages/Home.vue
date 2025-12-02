<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import { ask } from "@tauri-apps/plugin-dialog";
import type { Alias } from "../types/alias";
import ToastNotification from "../components/ToastNotification.vue";
import SearchBar from "../components/SearchBar.vue";
import AliasList from "../components/AliasList.vue";
import AddAliasModal from "../components/AddAliasModal.vue";
import { Button } from "@/components/ui/button";

const router = useRouter();

const aliases = ref<Alias[]>([]);
const showReloadHint = ref(false);
const toastMessage = ref('');
const addAliasModal = ref<InstanceType<typeof AddAliasModal> | null>(null);
const searchQuery = ref("");

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
    toastMessage.value = `Alias "${name}" added successfully`;
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
    toastMessage.value = `Alias "${name}" deleted successfully`;
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

onMounted(async () => {
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
  <div class="flex flex-col flex-1 min-h-0">
    <ToastNotification :show="showReloadHint" :message="toastMessage" @close="showReloadHint = false" />

    <div class="flex justify-between items-center mt-2 mb-6">
      <h1 class="text-3xl font-bold text-primary">Alias Assistant ✨</h1>
      <Button 
        @click="router.push('/settings')" 
        variant="ghost"
        size="icon"
        title="Settings"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
        </svg>
      </Button>
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
      <Button 
        class="size-14 rounded-full shadow-lg hover:shadow-xl transition-all hover:scale-110" 
        @click="addAliasModal?.open()"
        size="icon-lg"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
        </svg>
      </Button>
    </div>

    <AddAliasModal ref="addAliasModal" @submit="handleAddAlias" />
  </div>
</template>

