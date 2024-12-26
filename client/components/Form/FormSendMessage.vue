<script setup lang="ts">
  const emit = defineEmits([ 'submit' ]);
  const text = ref('');

  const submit = () => {
    if (text.value === '') return;
    emit('submit', text.value);
    text.value = '';
  }
</script>

<template>
  <form class="w-full flex flex-col sm:flex-row gap-4" @submit.prevent="submit">
    <FormElementTextfield
      class="flex-1"
      label="Message"
      v-model="text"
    />
    <ButtonSendMessage
      type="submit"
    />
  </form>
</template>