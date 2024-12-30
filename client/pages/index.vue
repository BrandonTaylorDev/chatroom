<script setup lang="ts">
  import type { Message } from '~/types/Message';

  definePageMeta({
    middleware: 'enforce-auth'
  });

  const messages = ref([] as Message[]);
  let socket: WebSocket | null = null;

  onMounted(() => {

    // Initialize WebSocket connection
    socket = new WebSocket('ws://127.0.0.1:9002');

    // Listen for incoming messages
    socket.addEventListener('message', (event) => {
      const receivedMessage: Message = JSON.parse(event.data);
      messages.value.push(receivedMessage);
    });

    // Handle connection close
    socket.addEventListener('close', () => {
      console.log('WebSocket connection closed');
    });

    // Handle errors
    socket.addEventListener('error', (error) => {
      console.error('WebSocket error:', error);
    });
  });

  onUnmounted(() => {

    // Close WebSocket connection when component is unmounted
    if (socket) {
      socket.close();
      socket = null;
    }
  });

  const sendMessage = (msg: string) => {
    if (socket) {
      socket.send(JSON.stringify({
        author: '',
        text: msg,
        createdAt: new Date().getTime()
      } as Message));
    }
  }
</script>

<template>
  <div class="flex flex-col h-dvh w-full">
    <nav class="flex flex-row justify-between p-4 bg-slate-900 text-slate-100">
      <div class="grid place-items-center">
        <span class="select-none">
          Chat Room
        </span>
      </div>

      <!-- spacer -->
      <div class="flex-1"></div>

      <div>
        <ul>
          <li class="rounded-full hover:bg-slate-800 w-12 h-12  flex justify-center items-center">
            <NuxtLink to="/logout">
              <icon name="mdi:logout" size="1.5em" mode="svg" />
            </NuxtLink>
          </li>
        </ul>
      </div>
    </nav>

    <div class="flex-1 w-full">
      <MessageList :messages="messages" />
    </div>

    <div class="p-4 bg-slate-900">
      <FormSendMessage @submit="msg => sendMessage(msg)" />
    </div>
  </div>
</template>
