const { iconsPlugin, getIconCollections } = require('@egoist/tailwindcss-icons')

/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{ts,tsx}'],
  theme: {
    extend: {},
  },
  plugins: [
    // tailwind-icon
    // [こちらのiconを使用できるらしい](https://icones.js.org/)
    // [導入参考](https://zenn.dev/hayato94087/articles/1abcb002d1e254)
    iconsPlugin({
      collections: getIconCollections(['ic', 'solar']),
    }),
  ],
}
