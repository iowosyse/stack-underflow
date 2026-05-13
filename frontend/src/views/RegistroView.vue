<template>
  <div class="d-flex flex-column align-items-center justify-content-center min-vh-100 py-5 user-bg">

    <button @click="toggle" class="theme-toggle theme-toggle--float">
      <span class="theme-toggle-icon">{{ isDark ? '☀' : '☾' }}</span>
      <span class="theme-toggle-label">{{ isDark ? 'Claro' : 'Oscuro' }}</span>
    </button>

    <div class="glass-user p-5 text-center auth-card auth-card--wide">

      <div class="mb-5">
        <h1 class="logo logo-auth mb-2 text-accent">stack-underflow</h1>
        <div class="auth-divider"></div>
        <p class="auth-subtitle mt-3 mb-0">Registra tu empresa</p>
      </div>

      <div v-if="exito" class="alert-success-custom mb-4">
        {{ mensajeExito }}
        <RouterLink to="/" class="d-block mt-2 fw-semibold">Ir al Login →</RouterLink>
      </div>

      <form v-else @submit.prevent="handleRegistro" class="text-start">

        <p class="form-section-label mb-3">Datos de la empresa</p>

        <div class="mb-3">
          <label class="form-label">Nombre de la empresa</label>
          <input type="text" class="form-control" v-model="nombreEmpresa"
                 placeholder="Ej. Acme Corporation" required>
        </div>

        <div class="mb-4">
          <label class="form-label">Dominio institucional</label>
          <div class="input-group">
            <span class="input-group-text">@</span>
            <input type="text" class="form-control" v-model="dominio"
                   placeholder="acme.com" required
                   @input="dominio = dominio.trim().toLowerCase()">
          </div>
          <div class="form-text opacity-55">
            Solo se podrán crear usuarios con correos de este dominio.
          </div>
        </div>

        <p class="form-section-label mb-3">Administrador de la cuenta</p>

        <div class="mb-3">
          <label class="form-label">Nombre completo</label>
          <input type="text" class="form-control" v-model="nombreAdmin"
                 placeholder="Ej. Ana García" required>
        </div>

        <div class="mb-3">
          <label class="form-label">Correo del administrador</label>
          <input type="email" class="form-control" v-model="emailAdmin"
                 placeholder="admin@acme.com" required
                 :class="{ 'is-invalid': emailAdmin && dominio && !emailValido }">
          <div v-if="emailAdmin && dominio && !emailValido" class="invalid-feedback">
            El correo debe terminar en @{{ dominio }}
          </div>
        </div>

        <div class="mb-4">
          <label class="form-label">Contraseña</label>
          <input type="password" class="form-control" v-model="password"
                 placeholder="Mínimo 8 caracteres" required minlength="8">
        </div>

        <!-- Cloudflare Turnstile -->
        <div class="mb-4 d-flex justify-content-center">
          <div
            class="cf-turnstile"
            :data-sitekey="sitekey"
            data-theme="auto"
            data-callback="onTurnstileSuccess"
          ></div>
        </div>

        <div v-if="errorMsg" class="alert-error-custom mb-3">{{ errorMsg }}</div>

        <button type="submit" class="btn btn-dark w-100 py-2 rounded-pill mb-3"
                :disabled="enviando || !captchaToken || !emailValido || !dominio">
          {{ enviando ? 'Registrando...' : 'Crear cuenta' }}
        </button>

        <RouterLink to="/" class="btn btn-outline-dark w-100 py-2 rounded-pill">
          ← Ya tengo cuenta
        </RouterLink>

      </form>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useTheme } from '../composables/useTheme'

const { isDark, toggle, init } = useTheme()

const nombreEmpresa = ref('')
const dominio       = ref('')
const nombreAdmin   = ref('')
const emailAdmin    = ref('')
const password      = ref('')
const captchaToken  = ref('')
const enviando      = ref(false)
const errorMsg      = ref('')
const exito         = ref(false)
const mensajeExito  = ref('')

// Clave de sitio de prueba de Cloudflare (siempre pasa en desarrollo)
// Reemplazar por la clave real al desplegar: https://dash.cloudflare.com/
const sitekey = '0x4AAAAAADOBqPghHrl756s6'

const emailValido = computed(() => {
  if (!emailAdmin.value || !dominio.value) return true
  return emailAdmin.value.trim().toLowerCase().endsWith(`@${dominio.value}`)
})

// El widget Turnstile llama a esta función global al completarse
window.onTurnstileSuccess = (token) => {
  captchaToken.value = token
}

onMounted(() => {
  init()
  // Cargar script de Turnstile si aún no está
  if (!document.getElementById('cf-turnstile-script')) {
    const script = document.createElement('script')
    script.id = 'cf-turnstile-script'
    script.src = 'https://challenges.cloudflare.com/turnstile/v0/api.js'
    script.async = true
    script.defer = true
    document.head.appendChild(script)
  }
})

const handleRegistro = async () => {
  errorMsg.value = ''

  if (!captchaToken.value) {
    errorMsg.value = 'Por favor completa el CAPTCHA.'
    return
  }
  if (!emailValido.value) {
    errorMsg.value = `El correo debe terminar en @${dominio.value}`
    return
  }

  enviando.value = true

  try {
    const resp = await fetch('/api/empresas/registro', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        nombre_empresa: nombreEmpresa.value.trim(),
        dominio: dominio.value,
        nombre_admin: nombreAdmin.value.trim(),
        email_admin: emailAdmin.value.trim().toLowerCase(),
        password_admin: password.value,
        captcha_token: captchaToken.value,
      })
    })

    const datos = await resp.json()

    if (resp.ok) {
      exito.value = true
      mensajeExito.value = datos.mensaje
    } else {
      errorMsg.value = datos || 'Error al registrar la empresa.'
      captchaToken.value = ''
      // Resetear widget de Turnstile si está disponible
      if (window.turnstile) window.turnstile.reset()
    }
  } catch {
    errorMsg.value = 'Error de conexión con el servidor.'
  } finally {
    enviando.value = false
  }
}
</script>

<style scoped>
.form-section-label {
  font-size: 0.75rem;
  font-weight: 600;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  opacity: 0.5;
}

.alert-success-custom {
  background: color-mix(in srgb, var(--color-accent) 12%, transparent);
  border: 1px solid color-mix(in srgb, var(--color-accent) 40%, transparent);
  border-radius: var(--radius-md);
  padding: 1rem;
  color: var(--color-accent);
  font-size: 0.9rem;
}

.alert-error-custom {
  background: color-mix(in srgb, #ef4444 10%, transparent);
  border: 1px solid color-mix(in srgb, #ef4444 35%, transparent);
  border-radius: var(--radius-md);
  padding: 0.75rem 1rem;
  color: #ef4444;
  font-size: 0.875rem;
}
</style>
