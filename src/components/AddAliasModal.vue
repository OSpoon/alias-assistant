<script setup lang="ts">
import { ref } from 'vue';
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription, DialogFooter } from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { Label } from "@/components/ui/label";

const emit = defineEmits<{
  submit: [name: string, command: string];
}>();

const open = ref(false);
const newAliasName = ref('');
const newAliasCommand = ref('');

function handleOpen() {
  open.value = true;
}

function handleClose() {
  open.value = false;
  newAliasName.value = '';
  newAliasCommand.value = '';
}

function handleSubmit() {
  if (!newAliasName.value || !newAliasCommand.value) {
    alert('Please enter both alias name and command.');
    return;
  }
  emit('submit', newAliasName.value, newAliasCommand.value);
  handleClose();
}

defineExpose({
  open: handleOpen,
  close: handleClose,
});
</script>

<template>
  <Dialog v-model:open="open">
    <DialogContent>
      <DialogHeader>
        <DialogTitle>Add New Alias</DialogTitle>
        <DialogDescription>
          Enter a new alias name and its corresponding command.
        </DialogDescription>
      </DialogHeader>
      <form @submit.prevent="handleSubmit" class="flex flex-col gap-4" autocomplete="off">
        <div class="flex flex-col gap-2">
          <Label for="alias-name">Alias Name</Label>
          <Input
            id="alias-name"
            v-model="newAliasName"
            type="text"
            placeholder="e.g., ll"
            autocomplete="off"
            spellcheck="false"
            required
          />
        </div>
        <div class="flex flex-col gap-2">
          <Label for="alias-command">Command</Label>
          <Textarea
            id="alias-command"
            v-model="newAliasCommand"
            placeholder="e.g., ls -la"
            class="min-h-[100px]"
            autocomplete="off"
            spellcheck="false"
            required
          />
        </div>
        <DialogFooter>
          <Button type="button" variant="outline" @click="handleClose">
            Cancel
          </Button>
          <Button type="submit">
            Add Alias
          </Button>
        </DialogFooter>
      </form>
    </DialogContent>
  </Dialog>
</template>

