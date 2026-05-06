<template>
  <div class="d-flex flex-column align-items-center justify-content-center vh-100 user-bg">

    <button @click="toggle" class="theme-toggle theme-toggle--float">
      <span class="theme-toggle-icon">{{ isDark ? '☀' : '☾' }}</span>
      <span class="theme-toggle-label">{{ isDark ? 'Claro' : 'Oscuro' }}</span>
    </button>

    <div class="glass-user p-5 text-center auth-card">

      <div class="mb-5">
        <h1 class="logo logo-auth mb-2 text-accent">stack-underflow</h1>
        <div class="auth-divider"></div>
        <p class="auth-subtitle mt-3 mb-0">Sistema de Gestión de Tickets</p>
      </div>

      <form @submit.prevent="handleLogin" class="text-start">
        <div class="mb-3">
          <label class="form-label">Usuario</label>
          <input type="text" class="form-control" v-model="username"
                 placeholder="primer nombre, sin acentos">
        </div>
        <div class="mb-4">
          <label class="form-label">Contraseña</label>
          <input type="password" class="form-control" v-model="password"
                 placeholder="apellidos, sin acentos">
        </div>
        <button type="submit" class="btn btn-dark w-100 py-2 rounded-pill mb-3" :disabled="cargando">
          {{ cargando ? 'Verificando...' : 'Ingresar al Sistema' }}
        </button>
        <RouterLink to="/forgot-password" class="btn btn-outline-dark w-100 py-2 rounded-pill">
          Olvidé mi Contraseña
        </RouterLink>
      </form>

    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
// Dependiendo de tu arquitectura, puedes mantener el store o quitarlo si solo usarás localStorage
// import { store } from '../store' 
import { useTheme } from '../composables/useTheme'

const router   = useRouter()
const username = ref('')
const password = ref('')
const cargando = ref(false) // Estado para bloquear el botón mientras se valida
const { isDark, toggle, init } = useTheme()

onMounted(() => init())

const handleLogin = async () => {
  // Validación básica del frontend
  if (!username.value || !password.value) {
    alert("Por favor ingresa usuario y contraseña.");
    return;
  }

  cargando.value = true;

  try {
    // 1. Construir el correo esperado por el backend (ej. alberto.montoya@tecnm.mx)
    const userLimpio = username.value.trim().toLowerCase();
    const passLimpia = password.value.trim().toLowerCase().replace(/\s/g, '');
    const emailBackend = `${userLimpio}.${passLimpia}@tecnm.mx`;

    // 2. Formatear la contraseña para el SHA256 de Postgres (Primera mayúscula, ej. "Montoya")
    const passOriginal = password.value.trim();
    const passParaBackend = passOriginal.charAt(0).toUpperCase() + passOriginal.slice(1).toLowerCase();

    // 3. Petición AJAX al proxy de Vite -> Servidor Rust
    const respuesta = await fetch('/api/login', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ 
        email: emailBackend, 
        password: passParaBackend 
      })
    });

    if (respuesta.ok) {
      // 4. Extraer los datos del JSON (id, rol, token)
      const datos = await respuesta.json();
      
      // 5. Conservar sesión en el navegador
      localStorage.setItem('token', datos.token);
      localStorage.setItem('rol', datos.rol);
      localStorage.setItem('usuario_id', datos.id);

      // 6. Redirección basada en el ENUM de la base de datos
      if (datos.rol === 'administrador') {
        router.push('/admin');
      } else if (datos.rol === 'cliente') {
        router.push('/user');
      } else {
        router.push('/');
      }
      
    } else if (respuesta.status === 401) {
      alert("Credenciales incorrectas. Revisa tu usuario y contraseña.");
    } else {
      alert("Error interno en la base de datos.");
    }
  } catch (error) {
    console.error("Fallo de red:", error);
    alert("No se pudo establecer conexión con el servidor.");
  } finally {
    cargando.value = false;
  }
}
</script>
