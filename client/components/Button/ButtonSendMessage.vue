<script setup lang="ts">
  type Props = {
    type?: 'button' | 'submit' | 'reset'
  }

  const props = withDefaults(defineProps<Props>(), {
    type: 'button'
  });
</script>

<template>
  <button
    :type="props.type"
    class="flex gap-2 justify-center items-center text-slate-50 bg-slate-500 hover:bg-slate-600 transition-colors duration-300 rounded-full px-4 py-2"
  >
    <icon name="mdi:send" size="1.5em" mode="svg" />
    Send Message
  </button>
</template>
