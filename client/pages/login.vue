<script setup lang="ts">
  const v = ref('');
</script>

<template>
  <div class="text-slate-950 dark:text-slate-50">
    <FormElementTextfield v-model="v" />
    {{ v }}
  </div>
</template>
