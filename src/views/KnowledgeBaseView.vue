<template>
  <div class="d-flex flex-column vh-100 bg-light" id="wrapper">
    <div class="d-flex flex-grow-1 overflow-hidden">
      <div class="sidebar p-4 d-flex flex-column justify-content-between flex-shrink-0" style="width: 280px;">
        <div>
          <div class="logo h4 fw-bold mb-5 text-primary">stack-underflow</div>
          <div class="list-group list-group-flush bg-transparent">
            <RouterLink to="/user" class="list-group-item list-group-item-action bg-transparent">Mis Tickets</RouterLink>
            <RouterLink to="/knowledge-base" class="list-group-item list-group-item-action bg-transparent active">Base de Conocimiento</RouterLink>
          </div>
        </div>
        <div>
          <div class="small text-muted mb-2 text-capitalize">Perfil: <strong class="text-dark">{{ store.currentUser?.fullName || 'Usuario' }}</strong></div>
          <button @click="logout" class="btn btn-outline-danger w-100">Cerrar Sesión</button>
        </div>
      </div>

      <div class="container-fluid p-5 d-flex flex-column overflow-auto">
        <div class="mb-4 pb-2 border-bottom">
          <h1 class="h3 m-0 fw-bold">Ayuda y Tutoriales</h1>
          <p class="text-muted mt-2">Resuelve tus dudas rápidas sin esperar a un técnico.</p>
        </div>

        <div class="row g-4 mb-5">
          <div class="col-12">
            
            <div class="card p-3 mb-4">
              <div class="card-header bg-transparent border-0 fw-bold fs-5">Redes e Internet</div>
              <div class="card-body">
                
                <div class="mb-3 p-2 border-bottom">
                  <div class="fw-bold text-primary accordion-header" @click="toggle('ans1')">&#9656; ¿Qué es una red y por qué no tengo sistema?</div>
                  <div class="mt-2 text-secondary px-3 accordion-content" v-show="openTab === 'ans1'">
                    <p>Una red es simplemente un grupo de computadoras conectadas entre sí para compartir información (como carpetas compartidas o impresoras). Si "no tienes sistema", usualmente significa que tu computadora perdió la conexión con el servidor principal.</p>
                    <p><strong>Solución rápida:</strong> Revisa si puedes entrar a páginas web normales (como Google). Si tampoco cargan, es tu internet.</p>
                  </div>
                </div>

                <div class="mb-3 p-2 border-bottom">
                  <div class="fw-bold text-primary accordion-header" @click="toggle('ans2')">&#9656; ¿Qué cable necesito para conectarme a internet?</div>
                  <div class="mt-2 text-secondary px-3 accordion-content" v-show="openTab === 'ans2'">
                    <p>El cable estándar se llama <strong>Ethernet (o RJ45)</strong>. Es parecido a un cable de teléfono fijo, pero la punta es más gruesa y suele ser de color azul, gris o amarillo.</p>
                    <p>Debe conectarse en el puerto que tiene luces parpadeantes (verde/naranja) en la parte trasera de tu PC o lateral de tu laptop.</p>
                  </div>
                </div>

                <div class="mb-3 p-2 border-bottom">
                  <div class="fw-bold text-primary accordion-header" @click="toggle('ans3')">&#9656; Diferencia entre WiFi y Cable</div>
                  <div class="mt-2 text-secondary px-3 accordion-content" v-show="openTab === 'ans3'">
                    <p>El <strong>Cable</strong> es más rápido y estable, ideal para trabajar en la oficina. El <strong>WiFi</strong> es inalámbrico, pero puede fallar si estás lejos del módem o hay muchas paredes. Siempre preferimos que uses cable para evitar desconexiones.</p>
                  </div>
                </div>

              </div>
            </div>

            <div class="card p-3">
              <div class="card-header bg-transparent border-0 fw-bold fs-5">Tutoriales Comunes</div>
              <div class="card-body">
                
                <div class="mb-3 p-2 border-bottom">
                  <div class="fw-bold text-primary accordion-header" @click="toggle('ans4')">&#9656; ¿Cómo instalar Google Chrome en una PC nueva?</div>
                  <div class="mt-2 text-secondary px-3 accordion-content" v-show="openTab === 'ans4'">
                    <ol style="margin-left: 20px;">
                      <li>Abre el navegador que viene por defecto (Edge en Windows o Safari en Mac).</li>
                      <li>En la barra de direcciones escribe: <code>google.com/chrome</code></li>
                      <li>Haz clic en el botón azul grande que dice <strong>"Descargar Chrome"</strong>.</li>
                      <li>Se descargará un archivo. Haz clic sobre él para abrirlo.</li>
                      <li>Espera a que termine la instalación automática. ¡Listo!</li>
                    </ol>
                  </div>
                </div>

                <div class="mb-3 p-2 border-bottom">
                  <div class="fw-bold text-primary accordion-header" @click="toggle('ans5')">&#9656; Olvidé mi contraseña, ¿cómo la restablezco?</div>
                  <div class="mt-2 text-secondary px-3 accordion-content" v-show="openTab === 'ans5'">
                    <p>No necesitas crear un ticket para esto. Sigue estos pasos:</p>
                    <ol style="margin-left: 20px;">
                      <li>En la pantalla de inicio de sesión, busca el enlace pequeño que dice <strong>"¿Olvidaste tu contraseña?"</strong>.</li>
                      <li>Ingresa tu correo electrónico corporativo.</li>
                      <li>Revisa tu bandeja de entrada (y la carpeta de Spam). Recibirás un enlace seguro.</li>
                      <li>Haz clic en el enlace y escribe tu nueva contraseña dos veces.</li>
                    </ol>
                  </div>
                </div>

                <div class="mb-3 p-2 border-bottom">
                  <div class="fw-bold text-primary accordion-header" @click="toggle('ans6')">&#9656; Mi impresora no imprime</div>
                  <div class="mt-2 text-secondary px-3 accordion-content" v-show="openTab === 'ans6'">
                    <ul>
                      <li>Revisa que la impresora esté encendida (luz verde).</li>
                      <li>Revisa que tenga papel en la bandeja.</li>
                      <li>Verifica que el cable USB esté bien conectado a tu computadora.</li>
                      <li>Si nada funciona, intenta reiniciar la impresora (apagar y prender).</li>
                    </ul>
                  </div>
                </div>

              </div>
            </div>

          </div>
        </div>

        <div class="py-3 mt-auto text-center w-100 border-top text-muted">
            <p class="m-0 small">&copy; 2026 stack-underflow System. Todos los derechos reservados.</p>
            <div class="mt-2 w3c-badges">
                <a href="https://validator.w3.org/nu/?doc=referer" target="_blank">
                    <img src="https://www.w3.org/Icons/valid-html5" alt="Valid HTML5" height="31" width="88">
                </a>
                <a href="https://jigsaw.w3.org/css-validator/check/referer" target="_blank">
                    <img src="https://jigsaw.w3.org/css-validator/images/vcss-blue" alt="CSS Valido!" height="31" width="88">
                </a>
            </div>
        </div>

      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { store } from '../store'

const router = useRouter()
const openTab = ref(null)

const toggle = (id) => {
  openTab.value = openTab.value === id ? null : id
}

const logout = () => {
  store.logout()
  router.push('/')
}
</script>

<style scoped>
.accordion-header { 
  cursor: pointer; 
  user-select: none;
  transition: color 0.2s;
}
.accordion-header:hover {
  color: #0a58ca !important;
}
.w3c-badges a { 
  display: inline-block; 
  margin: 0 5px; 
  text-decoration: none; 
  border: none; 
}
.w3c-badges img { 
  border: 0; 
  vertical-align: middle; 
}
</style>