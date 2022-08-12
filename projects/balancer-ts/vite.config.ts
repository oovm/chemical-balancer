import {defineConfig} from 'vite';

export default defineConfig({
    build: {
        lib: {
            entry: 'src/index.ts',
            name: 'ChemicalBalancer',
            fileName: (format) => `chemical-balancer.${format}.js`,
        },
    },
    plugins: [],

});