/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ['./src/**/*.{html,rs}', 'index.html'],

	theme: {
		extend: {
		  fontFamily: {
        'jost-sans': ['"Jost"', 'sans-serif'],
      },
      colors: {
        primary: '#8B4A4A',
        secondary: '#4A8B8B',
        scrollbar: {
          DEFAULT: '#FFFFFF',
          track: '#FFFFFF',
          thumb: '#8B4A4A',
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
