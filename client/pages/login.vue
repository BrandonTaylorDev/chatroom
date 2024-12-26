<script setup lang="ts">

  type FormResponse = {
    username: string,
    password: string
  }

  type LoginResponse = {
    success: boolean
    message?: string
    action?: 'login' | 'register'
  }

  const login = async ({ username, password }: FormResponse) => {
    const res: LoginResponse = await $fetch('/api/login', {
      method: 'POST',
      body: {
        username,
        password
      }
    });

    if (res.success) {
      navigateTo('/');
      return
    }

    console.error(res.message || 'Unknown error');
  }
</script>

<template>
  <div class="text-slate-950 dark:text-slate-50 p-8 absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-full max-w-xl bg-slate-300 dark:bg-slate-900 rounded-xl">
    <FormLogin @submit="login" />
  </div>
</template>
