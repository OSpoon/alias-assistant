<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { ask } from "@tauri-apps/plugin-dialog";

interface Alias {
  name: string;
  command: string;
}

const aliases = ref<Alias[]>([]);
const newAliasName = ref("");
const newAliasCommand = ref("");
const showReloadHint = ref(false);
const addAliasModal = ref<HTMLDialogElement | null>(null);
const searchQuery = ref("");

const filteredAliases = computed(() => {
  if (!searchQuery.value.trim()) {
    return aliases.value;
  }
  const query = searchQuery.value.toLowerCase();
  return aliases.value.filter(
    (alias) =>
      alias.name.toLowerCase().includes(query) ||
      alias.command.toLowerCase().includes(query)
  );
});

async function fetchAliases() {
  try {
    const result = await invoke("get_aliases");
    console.log("Aliases fetched:", result); // Added console.log
    aliases.value = result as Alias[];
  } catch (error) {
    console.error("Failed to fetch aliases:", error);
  }
}

async function handleAddAlias() {
  if (!newAliasName.value || !newAliasCommand.value) {
    alert("Please enter both alias name and command.");
    return;
  }
  try {
    await invoke("add_alias", {
      name: newAliasName.value,
      command: newAliasCommand.value,
    });
    newAliasName.value = "";
    newAliasCommand.value = "";
    showReloadHint.value = true;
    setTimeout(() => { showReloadHint.value = false; }, 2000); // Auto-dismiss after 2 seconds
    addAliasModal.value?.close();
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
    setTimeout(() => { showReloadHint.value = false; }, 2000); // Auto-dismiss after 2 seconds
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
  <div data-theme="forest" class="h-screen flex flex-col bg-base-100 overflow-hidden">
    <main class="flex flex-col flex-1 p-4 min-h-0">

      <!-- Toast Notification -->
      <div v-if="showReloadHint" class="toast toast-center z-50">
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
          <button @click="showReloadHint = false" class="btn btn-sm btn-circle btn-ghost">✕</button>
        </div>
      </div>

      <h1 class="text-3xl font-bold text-primary mb-4">Alias Assistant ✨</h1>

      <!-- Search Bar -->
      <div class="mb-4">
        <fieldset class="form-control">
          <div class="relative">
            <input
              v-model="searchQuery"
              type="text"
              placeholder="Search aliases..."
              class="input input-bordered w-full pl-10 rounded-lg"
            />
            <svg xmlns="http://www.w3.org/2000/svg" class="absolute left-3 top-1/2 transform -translate-y-1/2 h-5 w-5 text-base-content/60 pointer-events-none z-10" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
            </svg>
          </div>
        </fieldset>
      </div>

      <!-- Alias List -->
      <div class="bg-base-200 flex-1 flex flex-col min-h-0 rounded-lg p-4">
        <div class="flex flex-col overflow-y-auto min-h-0 flex-1 scroll-smooth">
          <div class="space-y-2 flex flex-col">
            <div v-if="filteredAliases.length === 0"
              class="text-center text-base-content/60 flex-1 flex items-center justify-center">
              <span v-if="searchQuery.trim()">
                No aliases found matching "{{ searchQuery }}"
              </span>
              <span v-else>
                No aliases found. Add one using the button below!
              </span>
            </div>
            <template v-for="(alias, index) in filteredAliases" :key="alias ? alias.name : ''">
              <div v-if="alias && alias.name"
                class="flex justify-between items-center p-3 bg-base-100 rounded-lg hover:bg-base-300 transition-all duration-300 shadow-sm hover:shadow-md opacity-0 animate-[fadeIn_0.4s_ease-out_forwards]"
                :style="{ animationDelay: `${index * 50}ms` }">
                <div class="flex flex-col flex-1 min-w-0">
                  <code class="text-primary font-semibold text-base">{{ alias.name }}</code>
                  <div class="tooltip tooltip-top" :data-tip="alias.command">
                    <code class="text-sm text-base-content/70 mt-1 block truncate">{{ alias.command }}</code>
                  </div>
                </div>
                <div class="flex flex-col gap-2">
                  <button @click="openTerminal(alias.name)" class="btn btn-info btn-xs transition-all hover:scale-105 rounded-lg">Terminal</button>
                  <button @click="deleteAlias(alias.name)" class="btn btn-error btn-xs transition-all hover:scale-105 rounded-lg">Delete</button>
                </div>
              </div>
            </template>
          </div>
        </div>
      </div>

      <!-- FAB to open modal -->
      <div class="fixed bottom-8 right-8">
        <button class="btn btn-primary btn-circle btn-lg shadow-lg hover:shadow-xl transition-all hover:scale-110" @click="addAliasModal?.showModal()">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
        </button>
      </div>

      <!-- Add Alias Modal -->
      <dialog ref="addAliasModal" class="modal">
        <div class="modal-box rounded-lg">
          <h3 class="font-bold text-lg mb-4">Add New Alias</h3>
          <form @submit.prevent="handleAddAlias" class="flex flex-col gap-4">
            <fieldset class="fieldset">
              <label class="label">Alias Name</label>
              <input v-model="newAliasName" type="text" placeholder="e.g., ll" class="input input-bordered w-full rounded-lg"
                required />
              <label class="label">Command</label>
              <textarea v-model="newAliasCommand" placeholder="e.g., ls -la" 
                class="textarea textarea-bordered w-full rounded-lg min-h-[100px]" required></textarea>
            </fieldset>
            <div class="modal-action">
              <button type="button" class="btn transition-all hover:scale-105 rounded-lg" @click="addAliasModal?.close()">Cancel</button>
              <button type="submit" class="btn btn-primary transition-all hover:scale-105 rounded-lg">Add Alias</button>
            </div>
          </form>
        </div>
        <form method="dialog" class="modal-backdrop">
          <button>close</button>
        </form>
      </dialog>

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