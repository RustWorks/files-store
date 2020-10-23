module.exports = {
  mount: {
    public: '/',
    src: '/assets',
  },
  plugins: [
    '@snowpack/plugin-svelte',
    '@snowpack/plugin-dotenv',
    '@snowpack/plugin-typescript',
    [
      '@snowpack/plugin-run-script',
      {cmd: 'svelte-check --output human', watch: '$1 --watch', output: 'stream'},
    ],
    "@snowpack/plugin-optimize"
  ],
  install: [],
  installOptions: {},
  devOptions: {
    port: 5555
  },
  buildOptions: {
    clean: true,
  },
  proxy: {},
  alias: {},
};
