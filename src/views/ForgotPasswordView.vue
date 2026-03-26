<template>
  <div class="d-flex flex-column align-items-center justify-content-center vh-100 user-bg">
    <div class="glass-user p-5 text-center shadow-lg" style="width: 100%; max-width: 450px;">
      <h3 class="mb-3 fw-bold text-accent">Recuperar Acceso</h3>
      <p class="small mb-4 opacity-75" style="font-family: 'Inter', sans-serif;">Ingresa tu correo institucional para recibir un enlace de restablecimiento seguro.</p>
      
      <form @submit.prevent="handleRecover" class="text-start">
        
        <div class="mb-4">
          <label class="form-label">Correo Institucional</label>
          <input type="email" class="form-control py-2" v-model="email" placeholder="usuario@morelia.tecnm.mx" required>
          
          <div v-if="email && !isEmailValid" class="text-danger mt-2" style="font-size: 0.75rem; font-family: 'Inter', sans-serif; font-weight: 600;">
            Ingresa un correo válido (ej. tu_nombre@morelia.tecnm.mx)
          </div>
        </div>

        <div v-if="successMessage" class="alert alert-success small text-center fw-bold py-2 px-3" style="border-radius: var(--radius-xs); font-family: 'Inter', sans-serif; background: rgba(16, 185, 129, 0.15); border: 1px solid var(--accent); color: var(--accent);">
          {{ successMessage }}
        </div>

        <button v-else type="submit" class="btn btn-primary w-100 py-2 rounded-pill mb-3" :disabled="!isEmailValid">
          Enviar Enlace
        </button>
        
        <RouterLink to="/" class="btn btn-outline-dark w-100 py-2 rounded-pill border-0 text-center d-block" style="text-decoration: none; background: rgba(0,0,0,0.05);">
          ← Volver al Login
        </RouterLink>
      </form>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'

const email = ref('')
const successMessage = ref('')

// Validación mediante Expresión Regular (Regex)
const isEmailValid = computed(() => {
  if (!email.value) return false
  
  // Exige: Texto (letras, números, puntos) + @ + morelia.tecnm.mx exacto
  const emailRegex = /^[a-zA-Z0-9._%+-]+@morelia\.tecnm\.mx$/i
  
  // .test() compara lo que escribe el usuario con la regla matemática
  return emailRegex.test(email.value.trim())
})

const handleRecover = () => {
  successMessage.value = "Si el correo existe, hemos enviado las instrucciones."
  email.value = ''
}
</script>