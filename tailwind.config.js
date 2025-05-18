/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./src/**/*.{html,rs}",
        "./index.html"
    ],
    theme: {
        extend: {
            animation: {
                "fadein": "fadein 0.1s linear",
                "handwave": "handwave 2.5s linear infinite",
            },
            keyframes: {
                "fadein": {
                     "0%": {
                        opacity: 0,
                    },
                    "100%": {
                        opacity: 1,
                    }
                },
                "handwave": {
                    "0%": {
                        transform: "rotate(0)",
                    },
                    "10%": {
                        transform: "rotate(14deg)",
                    },
                    "20%": {
                        transform: "rotate(-8deg)",
                    },
                    "30%": {
                        transform: "rotate(14deg)",
                    },
                    "40%": {
                        transform: "rotate(-4deg)",
                    },
                    "50%": {
                        transform: "rotate(10deg)",
                    },
                    "60%": {
                        transform: "rotate(0)",
                    },
                    "100%": {
                        transform: "rotate(0)",
                    },
                },
            },
            fontFamily: {
                "Handwriting": ["Satisfy", "sans-serif"],
            },
        },
    },
    darkMode: "class",
    plugins: [],
}

