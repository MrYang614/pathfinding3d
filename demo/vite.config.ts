import { resolve } from 'node:path';
import { defineConfig } from 'vite';
import topLevelAwait from 'vite-plugin-top-level-await';
import wasm from 'vite-plugin-wasm';

export default defineConfig({
	build: {
		target: 'esnext',
		rollupOptions: {
			input: {
				main: resolve(__dirname, 'index.html'),
				benchmark: resolve(__dirname, 'benchmark.html'),
			},
		},
	},
	optimizeDeps: {
		exclude: ['three_pathfinding_3d_wasm'],
	},
	plugins: [wasm(), topLevelAwait()],
});
