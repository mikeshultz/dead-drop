const path = require('path')
const sveltePreprocess = require('svelte-preprocess');

module.exports = {
  mode: 'production',
  entry: './src/main.ts',
  module: {
    rules: [
      /*{
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },*/
      {
        test: /\.(html|svelte)$/,
        use: {
          loader: 'svelte-loader',
          options: {
            preprocess: sveltePreprocess()
          }
        }
      },
      {
        // required to prevent errors from Svelte on Webpack 5+, omit on Webpack 4
        test: /node_modules\/svelte\/.*\.mjs$/,
        resolve: {
          fullySpecified: false
        }
      }
    ],
  },
  resolve: {
    // see below for an explanation
    alias: {
      svelte: path.resolve('node_modules', 'svelte/src/runtime') // Svelte 3: path.resolve('node_modules', 'svelte')
    },
    extensions: ['.mjs', '.ts', '.js', '.svelte'],
    mainFields: ['svelte', 'browser', 'module', 'main'],
    conditionNames: ['svelte', 'browser', 'import']
  },
  output: {
    filename: 'dead-drop.js',
    path: path.resolve(__dirname, 'static'),
  },
}
