<template>
  <div class="d-flex flex-column align-items-center justify-content-center vh-100 bg-light">
    <div class="login-box p-5 shadow-sm bg-white mb-auto mt-auto">
      <h2 class="text-center mb-4 fw-bold">stack-underflow</h2>
      <form @submit.prevent="handleLogin">
        <div class="mb-3">
          <label class="form-label small text-muted">Usuario (Primer Nombre)</label>
          <input type="text" class="form-control" v-model="username" placeholder="ej. candido">
        </div>
        <div class="mb-3">
          <label class="form-label small text-muted">Contraseña (Apellidos)</label>
          <input type="password" class="form-control" v-model="password" placeholder="ej. ortega martinez">
        </div>
        <button type="submit" class="btn btn-dark w-100 py-2 rounded-0">INGRESAR</button>
      </form>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { store } from '../store'

const router = useRouter()
const username = ref('')
const password = ref('')

const handleLogin = () => {
  const role = store.login(username.value, password.value)
  if (role === 'admin') router.push('/admin')
  else if (role === 'user') router.push('/user')
  else alert("Credenciales incorrectas. (Recuerda: sin acentos)")
}
</script>