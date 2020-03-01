module.exports = {
    theme: {
        interFontFeatures: {
            default: ['calt', 'liga', 'kern'],
            numeric: ['tnum', 'salt', 'ss02']
        },
        fontSize: {
            xs: '0.75rem',
            sm: '0.875rem',
            base: '1rem',
            lg: '1.125rem',
            xl: '1.25rem',
            '2xl': '1.5rem',
            '3xl': '1.875rem',
            '4xl': '2.25rem',
            '5xl': '3rem',
            '6xl': '4rem',
            '7xl': '6rem',
            '8xl': '8rem',
            '9xl': '9rem',
            '10xl': '10rem'
        },
        extend: {
            colors: {
                gray: {
                    100: '#f5f5f5',
                    200: '#eeeeee',
                    300: '#e0e0e0',
                    400: '#bdbdbd',
                    500: '#9e9e9e',
                    600: '#757575',
                    700: '#616161',
                    800: '#424242',
                    900: '#212121',
                }
            }
        }
    },
    variants: {},
    plugins: [
        require('tailwindcss-font-inter')({
            importFontFace: true,
            disableUnusedFeatures: true,
        }),
    ]
};
