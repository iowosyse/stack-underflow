// composables/useTheme.js
// Singleton: todos los componentes comparten el mismo estado de tema.

import { ref } from 'vue'

const isDark  = ref(false)
let   _inited = false

function _apply(dark) {
  document.documentElement.setAttribute('data-theme', dark ? 'dark' : 'light')
  try { localStorage.setItem('su-theme', dark ? 'dark' : 'light') } catch (_) {}
}

export function useTheme() {
  /**
   * Llama init() en onMounted() de cada vista raíz.
   * Sólo ejecuta lógica real en el primer mount.
   */
  function init() {
    if (_inited) return
    _inited = true
    let saved = null
    try { saved = localStorage.getItem('su-theme') } catch (_) {}
    const prefersDark = window.matchMedia?.('(prefers-color-scheme: dark)').matches ?? false
    isDark.value = saved ? saved === 'dark' : prefersDark
    _apply(isDark.value)
  }

  function toggle() {
    isDark.value = !isDark.value
    _apply(isDark.value)
  }

  return { isDark, toggle, init }
}
