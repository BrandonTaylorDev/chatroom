<script setup lang="ts">
  import type { Message } from '~/types/Message';

  type Props = {
    messages?: Message[]
  }

  const props = withDefaults(defineProps<Props>(), {
    messages: () => []
  });
  const messageListRef = ref<HTMLDivElement | null>(null);

  watch(props.messages, async () => {
    await nextTick();
    if (messageListRef.value) {
      messageListRef.value.scrollTop = messageListRef.value.scrollHeight;
    }
  });
</script>

<template>
  <div
    ref="messageListRef"
    class="bg-red-200 flex flex-col overflow-y-scroll gap-4 text-slate-950 dark:text-slate-50"
  >
    <div
      v-for="message in props.messages"
      :key="message.id"
      class="p-4"
    >
      <div class="flex flex-col">
        <div class="text-sm">
          {{ message.author }}
        </div>

        <div class="text-xs text-slate-300 dark:text-slate-500">
          {{ new Date(message.createdAt).toLocaleString() }}
        </div>
      </div>
      <div>
        {{ message.text }}
      </div>
    </div>
  </div>
</template>
