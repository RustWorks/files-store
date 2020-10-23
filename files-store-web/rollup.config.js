import svelte from 'rollup-plugin-svelte'
import resolve from '@rollup/plugin-node-resolve'
import commonjs from '@rollup/plugin-commonjs'
import { terser } from 'rollup-plugin-terser'
import sveltePreprocess from 'svelte-preprocess'
import typescript from '@rollup/plugin-typescript'

const production = !process.env.ROLLUP_WATCH

export default {
  input: 'src/index.ts',
  output: {
    sourcemap: true,
    format: 'iife',
    name: 'app',
    file: 'static/assets/index.js'
  },
  plugins: [
    svelte({
      dev: !production,
      css: css => {
        css.write('index.css');
      },
      preprocess: sveltePreprocess(),
    }),
    resolve({
      browser: true,
      dedupe: ['svelte']
    }),
    commonjs(),
    typescript({ sourceMap: true }),
    terser()
  ],
  watch: {
    clearScreen: false
  }
}
