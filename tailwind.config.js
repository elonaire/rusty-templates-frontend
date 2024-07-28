/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ['./src/**/*.{html,rs}', 'index.html'],

	theme: {
		extend: {
      colors: {
        primary: '#C43A1F',
        secondary: '#C41F57'
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
