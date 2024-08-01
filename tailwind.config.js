/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ['./src/**/*.{html,rs}', 'index.html'],

	theme: {
		extend: {
		  fontFamily: {
        'jost-sans': ['"Jost"', 'sans-serif'],
      },
      colors: {
        primary: '#C48D1F',
        secondary: '#A9C41E',
        scrollbar: {
          DEFAULT: '#FFFFFF',
          track: '#FFFFFF',
          thumb: '#C43A1F',
        },
      },
      height: {
        'screen-1/13': '7.692308svh',
        'screen-12/13': '92.307692svh',
        'screen-11/13': '80svh',
      },
		},
	},
	plugins: [
		require('@tailwindcss/typography'),
    require('@tailwindcss/forms'),
	],
};
