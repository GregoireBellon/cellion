import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig(({ mode }) => {
  const isDev = mode === "development";
  return {
    plugins: [react()],
    server: {
      port: 8080,
      proxy: {
        "/api": {
          target: isDev ? "http://localhost:5000" : "http://api:5000",
          changeOrigin: isDev ? true : false,
          secure: isDev ? false : true,
          rewrite: (path) => path.replace(/^\/api/, ""),
        },
      },
    },
  };
});
