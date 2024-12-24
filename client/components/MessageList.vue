<script setup lang="ts">
  import type { Message } from '~/types/Message';

  type Props = {
    messages?: Message[]
  }

  const props = withDefaults(defineProps<Props>(), {
    messages: () => []
  });
</script>

<template>
  <div class="h-[calc(100dvh-5rem)] flex flex-col overflow-y-scroll gap-4 text-slate-950 dark:text-slate-50">
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
