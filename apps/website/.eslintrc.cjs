/** @type {import("eslint").Linter.Config} */
const config = {
  extends: [
    "../../.eslintrc.cjs",
    "neon/react",
    "neon/next",
    "neon/edge",
    "plugin:tailwindcss/recommended",
    "neon/prettier",
  ],
  settings: {
    react: {
      version: "detect",
    },
  },
  rules: {
    "react/react-in-jsx-scope": 0,
    "react/jsx-filename-extension": [1, { extensions: [".tsx"] }],
  },
};

module.exports = config;
