<script setup lang="ts">
  import type { Message } from '~/types/Message';

  const messages = ref([
    
  ] as Message[]);
</script>

<template>
  <div class="flex flex-col h-dvh w-full">
    <div class="flex-1 w-full">
      <MessageList :messages="messages" />
    </div>

    <div class="p-4 bg-slate-900">
      <FormSendMessage />
    </div>
  </div>
</template>
