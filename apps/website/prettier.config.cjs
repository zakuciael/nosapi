/** @typedef  {import("prettier").Config} PrettierConfig*/
/** @typedef  {{ tailwindConfig: string }} TailwindConfig*/

/** @type {PrettierConfig | TailwindConfig} **/
const config = {
  ...require("../../prettier.config.cjs"),
  plugins: [require.resolve("prettier-plugin-tailwindcss")],
  pluginSearchDirs: false,
  tailwindConfig: "./tailwind.config.ts",
};

module.exports = config;
