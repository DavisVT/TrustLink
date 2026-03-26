import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  base: "/TrustLink/", // GitHub Pages base path
  define: {
    global: "globalThis",
  },
});
