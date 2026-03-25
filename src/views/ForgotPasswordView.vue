<template>
  <div class="bg-light d-flex flex-column align-items-center justify-content-center vh-100">
    <div class="login-box p-5 shadow-sm bg-white mb-auto mt-auto">
      <h3 class="text-center mb-3 fw-bold">Recuperar Acceso</h3>
      <p class="text-muted small text-center mb-4">Ingresa tus datos para recibir un enlace de restablecimiento.</p>
      
      <form @submit.prevent="handleRecover">
        <div class="mb-3">
          <label for="employeeId" class="form-label small fw-bold text-uppercase">ID de Empleado</label>
          <input type="text" id="employeeId" class="form-control" v-model="employeeId">
        </div>
        <div class="mb-3">
          <label for="email" class="form-label small fw-bold text-uppercase">Correo Corporativo</label>
          <input type="email" id="email" class="form-control" v-model="email">
        </div>
        <button type="submit" class="btn btn-primary w-100 py-2 rounded-0 mb-2">ENVIAR ENLACE</button>
        <RouterLink to="/" class="btn btn-outline-secondary w-100 py-2 rounded-0">VOLVER AL LOGIN</RouterLink>
      </form>
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
const employeeId = ref('')
const email = ref('')

const handleRecover = () => {
  if (employeeId.value && email.value) {
    alert("Si los datos son correctos, recibirás un correo en los próximos minutos.")
    router.push('/')
  } else {
    alert("Por favor llena todos los campos.")
  }
}
</script>

<style scoped>
/* Usamos el CSS externo exclusivamente para esta vista */
@import url('https://cdn.jsdelivr.net/gh/iowosyse/estilos@cssP2/su-style.css');
</style>