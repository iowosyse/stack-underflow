<template>
  <div class="d-flex flex-column align-items-center justify-content-center vh-100 bg-light">
    <div class="login-box p-5 shadow-sm bg-white mb-auto mt-auto">
      <h2 class="text-center mb-4 fw-bold">stack-underflow</h2>
      <form @submit.prevent="handleLogin">
        <div class="mb-3">
          <label for="user" class="form-label small text-muted">Usuario</label>
          <input type="text" id="user" class="form-control" v-model="username">
        </div>
        <div class="mb-3">
          <label for="pass" class="form-label small text-muted">Contraseña</label>
          <input type="password" id="pass" class="form-control" v-model="password">
        </div>
        <button type="submit" class="btn btn-dark w-100 py-2 rounded-0">INGRESAR</button>
      </form>
      <div class="mt-3 text-center text-muted small">
        <p class="mb-1">Pistas: admin/admin o root/root</p>
        <RouterLink to="/forgot-password" class="text-danger text-decoration-none">Olvidé mi contraseña</RouterLink>
      </div>
    </div>

    <div class="py-3 mt-auto text-center w-100 border-top border-secondary-subtle bg-white">
      <div class="container small text-muted">
        <p class="m-0">&copy; 2026 stack-underflow. Todos los derechos reservados.</p>
        <div class="mt-2 w3c-badges">
          <a href="https://validator.w3.org/nu/?doc=referer">
            <img src="https://www.w3.org/Icons/valid-html5" alt="Valid HTML5" height="31" width="88">
          </a>
          <a href="https://jigsaw.w3.org/css-validator/check/referer">
            <img src="https://jigsaw.w3.org/css-validator/images/vcss-blue" alt="CSS Valido!" height="31" width="88">
          </a>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const username = ref('')
const password = ref('')

const handleLogin = () => {
  if (username.value === 'admin' && password.value === 'admin') {
    router.push('/admin')
  } else if (username.value === 'root' && password.value === 'root') {
    router.push('/user')
  } else {
    alert("Credenciales incorrectas")
  }
}
</script>