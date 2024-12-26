<script setup lang="ts">
  const emit = defineEmits([ 'submit' ]);
  const user = ref('');
  const pass = ref('');

  const submit = () => {
    if (user.value === '' || pass.value === '') return;
    emit('submit', {
      username: user.value,
      password: pass.value
    });
    user.value = '';
    pass.value = '';
  }
</script>

<template>
  <form class="w-full flex flex-col gap-4" @submit.prevent="submit">
    <div class="flex flex-col gap-4">
      <p>
        Enter a username and a password.
      </p>

      <FormElementTextfield
        class="flex-1"
        label="Username"
        v-model="user"
      />

      <FormElementTextfield
        class="flex-1"
        label="Password"
        v-model="pass"
        type="password"
      />
    </div>
    <ButtonLogin
      type="submit"
    />
  </form>
</template>