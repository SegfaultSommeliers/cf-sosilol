import { defineConfig } from "vite";

export default defineConfig({
    publicDir: "frontend/public",
    input: {
        "js/main": "frontend/js/main.js",
        "css/main": "frontend/css/main.css",
    },
    build: {
        rolldownOptions: {
            output: {
                entryFileNames: "assets/[name].js",
                assetFileNames: (asset) =>
                    asset.name?.endsWith(".css")
                        ? "assets/css/main.css"
                        : "assets/[name]-[hash][extname]",
            },
        },
    },
});
