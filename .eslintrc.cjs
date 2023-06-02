/** @type {import("eslint").Linter.Config} */
const config = {
  root: true,
  extends: ["neon/common", "neon/node", "neon/typescript", "neon/prettier"],
  parserOptions: {
    project: ["tsconfig.eslint.json", "apps/*/tsconfig.eslint.json", "packages/*/tsconfig.eslint.json"],
  },
  ignorePatterns: ["**/dist/*"],
  overrides: [
    {
      rules: {
        "tsdoc/syntax": ["off"],
        "@typescript-eslint/consistent-type-imports": [
          "error",
          {
            prefer: "type-imports",
            fixStyle: "inline-type-imports",
          },
        ],
      },
      files: ["*.ts", "*.tsx"],
    },
  ],
};

module.exports = config;
