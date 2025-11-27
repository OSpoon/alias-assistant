<script setup lang="ts">
import { ref } from 'vue';

const emit = defineEmits<{
  submit: [name: string, command: string];
}>();

const modalRef = ref<HTMLDialogElement | null>(null);
const newAliasName = ref('');
const newAliasCommand = ref('');

function open() {
  modalRef.value?.showModal();
}

function close() {
  modalRef.value?.close();
  newAliasName.value = '';
  newAliasCommand.value = '';
}

function handleSubmit() {
  if (!newAliasName.value || !newAliasCommand.value) {
    alert('Please enter both alias name and command.');
    return;
  }
  emit('submit', newAliasName.value, newAliasCommand.value);
  close();
}

defineExpose({
  open,
  close,
});
</script>

<template>
  <dialog ref="modalRef" class="modal">
    <div class="modal-box rounded-lg">
      <h3 class="font-bold text-lg mb-4">Add New Alias</h3>
      <form @submit.prevent="handleSubmit" class="flex flex-col gap-4" autocomplete="off">
        <fieldset class="fieldset">
          <label class="label">Alias Name</label>
          <input
            v-model="newAliasName"
            type="text"
            placeholder="e.g., ll"
            class="input input-bordered w-full rounded-lg"
            autocomplete="off"
            spellcheck="false"
            required
          />
          <label class="label">Command</label>
          <textarea
            v-model="newAliasCommand"
            placeholder="e.g., ls -la"
            class="textarea textarea-bordered w-full rounded-lg min-h-[100px]"
            autocomplete="off"
            spellcheck="false"
            required
          ></textarea>
        </fieldset>
        <div class="modal-action">
          <button type="button" class="btn transition-all hover:scale-105 rounded-lg" @click="close">
            Cancel
          </button>
          <button type="submit" class="btn btn-primary transition-all hover:scale-105 rounded-lg">
            Add Alias
          </button>
        </div>
      </form>
    </div>
    <form method="dialog" class="modal-backdrop">
      <button @click="close">close</button>
    </form>
  </dialog>
</template>

