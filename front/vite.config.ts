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
        ...(isDev && {
          "/api": {
            target: "http://localhost:5000",
            changeOrigin: true,
            secure: false,
            rewrite: (path) => path.replace(/^\/api/, ""),
          },
        }),
      },
    },
    // https://github.com/vitejs/vite/discussions/9440#discussioncomment-5913798
    build: {
      rollupOptions: {
        output: {
          manualChunks(id) {
            if (id.includes("node_modules")) {
              return id
                .toString()
                .split("node_modules/")[1]
                .split("/")[0]
                .toString();
            }
          },
        },
      },
    },
    preview: {
      proxy: {
        "/api": {
          target: "http://localhost:5000",
          changeOrigin: true,
          secure: false,
          rewrite: (path) => path.replace(/^\/api/, ""),
        },
      },
    },
  };
});
