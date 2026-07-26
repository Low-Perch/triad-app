import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import { VitePWA } from "vite-plugin-pwa";

export default defineConfig({
  plugins: [
    svelte(),
    wasm(),
    topLevelAwait(),
    VitePWA({
      registerType: "autoUpdate",
      workbox: {
        globPatterns: ["**/*.{js,css,html,wasm,svg,png,ico}"],
        maximumFileSizeToCacheInBytes: 10 * 1024 * 1024,
        // Keep the SPA shell from swallowing the standalone download page
        navigateFallbackDenylist: [/^\/download/],
      },
      manifest: {
        name: "Triad",
        short_name: "Triad",
        description: "A daily word puzzle game",
        theme_color: "#121213",
        background_color: "#121213",
        display: "standalone",
        icons: [
          {
            src: "android-chrome-192x192.png",
            sizes: "192x192",
            type: "image/png",
          },
          {
            src: "android-chrome-512x512.png",
            sizes: "512x512",
            type: "image/png",
          },
          {
            src: "android-chrome-512x512.png",
            sizes: "512x512",
            type: "image/png",
            purpose: "maskable",
          },
        ],
      },
    }),
  ],

  resolve: {
    alias: {
      "$lib/bridge": new URL("./src/lib/bridge.web.ts", import.meta.url).pathname,
      "$lib/lifecycle": new URL("./src/lib/lifecycle.web.ts", import.meta.url).pathname,
      "triad-wasm": new URL("./wasm-pkg/triad_wasm.js", import.meta.url).pathname,
    },
  },

  build: {
    outDir: "dist-web",
    rollupOptions: {
      input: {
        main: new URL("./index.html", import.meta.url).pathname,
        download: new URL("./download.html", import.meta.url).pathname,
      },
    },
  },

  server: {
    port: 3000,
  },
});
