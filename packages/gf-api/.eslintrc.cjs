/** @type {import("eslint").Linter.Config} */
const config = {
  extends: ["../../.eslintrc.cjs", "neon/edge", "neon/module", "neon/prettier"],
};

module.exports = config;
