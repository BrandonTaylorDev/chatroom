<script setup lang="ts">
  const emit = defineEmits([ 'submit']);
</script>

<template>
  <form class="w-full flex gap-4" @submit.prevent="emit('submit')">
    <FormElementTextfield class="flex-1" label="Message" />
    <ButtonSendMessage />
  </form>
</template>