import daisyui from "./assets/daisyui.mjs";

module.exports = {
  content: [
    "./templates/**/*.html",
    "./src/**/*.rs",
  ],
  theme: {
    extend: {},
  },
  plugins: [
    [require("@tailwindcss/forms"), require("@tailwindcss/typography"), require(daisyui)],
  ],
};
