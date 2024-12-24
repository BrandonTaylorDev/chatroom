<script setup lang="ts">

  type Props = {
    modelValue?: string
    label?: string
  }
  
  const props = withDefaults(defineProps<Props>(), {
    modelValue: '',
    label: '',
  });

  const emit = defineEmits([ 'update:modelValue' ]);
  const id = useId();
</script>

<template>
  <div>
    <div class="relative">
      <label
        :for="id"
        :class="[
          ''
        ]"
      >
        {{ label }}
      </label>

      <input
        :id="id"
        class=""
        type="text"
        :value="modelValue"
        @input="$event => emit('update:modelValue', ($event.target as HTMLInputElement).value)"
      />
    </div>
  </div>
</template>
