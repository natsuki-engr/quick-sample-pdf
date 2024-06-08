const { iconsPlugin, getIconCollections } = require('@egoist/tailwindcss-icons')

/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{js,ts,vue}"],
  theme: {
    extend: {},
  },
  plugins: [iconsPlugin({
    collections: getIconCollections(["bxs", "bx"]),
  })],
}

