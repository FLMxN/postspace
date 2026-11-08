import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'

// Relative assets work on both a custom domain and /<repository>/ on Pages.
export default defineConfig({ base: './', plugins: [react(), tailwindcss()] })
