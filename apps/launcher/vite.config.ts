import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import path from 'path';

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [vue()],
    clearScreen: false,
    resolve: {
        alias: {
            '@shared': path.resolve(__dirname, '../../libs/shared-frontend/src'),
        },
    },
    server: {
        port: 1420,
        strictPort: true,
        host: false,
        watch: {
            ignored: ["**/src-tauri/**"],
        },
    },
});
