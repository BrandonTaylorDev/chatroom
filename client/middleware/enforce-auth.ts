export default defineNuxtRouteMiddleware(async () => {
  
  type Result = {
    success: boolean;
    action: string;
  }

  const res: Result = await $fetch('/api/login/validate', {
    method: 'POST',
    headers: useRequestHeaders(['cookie'])
  });
  
  if (!res.success) {
    return navigateTo('/login');
  }
});
