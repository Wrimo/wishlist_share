import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte()],
  build: {
    rollupOptions: {
      onwarn: (warning, handler) => {
        const { code, frame } = warning;
        if (code === "anchor-is-valid" || code === "a11y-autofocus") {
          return;
        }
        if (code === "css-unused-selector" && frame && frame.includes("shape")) {
          return;
        }
        handler(warning);
      }
    }
  },
  test: {
    include: ['src/**/*.{test,spec}.{js,ts}']
  }
})
