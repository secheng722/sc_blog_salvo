/** @type {import('tailwindcss').Config} */
import colors from 'tailwindcss/colors'
module.exports = {
  content: ["./template/*.html", "./html/*.html"],
  theme: {
    extend: {
      colors: {
        primary: colors.purple,
        gray: colors.gray,
      },
      maxWidth: {
        '150px': '150px',
      },
    },
  },
  plugins: [
    require('@tailwindcss/typography'),
  ],
}

