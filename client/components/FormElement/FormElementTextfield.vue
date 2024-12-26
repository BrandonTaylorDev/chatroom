<script setup lang="ts">

  type Props = {
    modelValue?: string
    label?: string
    type?: 'text' | 'password'
  }
  
  const props = withDefaults(defineProps<Props>(), {
    modelValue: '',
    label: '',
    type: 'text'
  });

  const emit = defineEmits([ 'update:modelValue' ]);
  const id = useId();
  const hasFocusOrValue = ref(false);
</script>

<template>
  <div class="flex rounded-full bg-slate-50  overflow-hidden">
    <div class="grid place-items-center w-12 text-slate-50 dark:text-slate-950">
      <icon name="mdi:message-outline" size="1.5em" mode="svg" />
    </div>

    <div class="relative flex-1">
      <label
        :for="id"
        :class="[
          'absolute left-0 text-slate-50 dark:text-slate-950 transition-all duration-300',
          hasFocusOrValue
            ? 'text-xs top-0'
            : 'top-1/2 -translate-y-1/2'
        ]"
      >
        {{ label }}
      </label>

      <input
        :id="id"
        :class="[
          'text-slate-50 dark:text-slate-950 h-12 w-full outline-none pe-4 bg-inherit',
          props.label
            ? 'pt-2'
            : null
        ]"
        :type="type"
        autocomplete="off"
        :value="modelValue"
        @input="$event => emit('update:modelValue', ($event.target as HTMLInputElement).value)"
        @focusin="hasFocusOrValue = true"
        @focusout="props.modelValue !== '' ? hasFocusOrValue = true : hasFocusOrValue = false"
      />
    </div>
  </div>
</template>
