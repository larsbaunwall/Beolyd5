import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue()],

  // Tauri dev/build should load from root; GitHub Pages simulator stays under /Beolyd5/.
  base: process.env.TAURI_ENV_PLATFORM ? "/" : "/Beolyd5/",

  // Vite options tailored for Tauri development
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,

  server: {
    // 2. tauri expects a fixed port, fail if that port is not available
    port: 1421,
    strictPort: true,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
});
