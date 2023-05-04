const path = require("path");

/** @type {import("eslint").Linter.Config} */
const config = {
    root: true,
    overrides: [
        {
            extends: ["plugin:@typescript-eslint/recommended-requiring-type-checking"],
            files: ["*.ts", "*.tsx"],
        },
    ],
    parser: "@typescript-eslint/parser",
    parserOptions: {
        tsconfigRootDir: __dirname,
        project: ["./tsconfig.json", "./apps/*/tsconfig.json", "./packages/*/tsconfig.json"],
    },
    plugins: ["@typescript-eslint", "tailwindcss"],
    extends: [
        "next/core-web-vitals",
        "plugin:@typescript-eslint/recommended",
        "plugin:tailwindcss/recommended",
        "prettier",
    ],
    settings: {
        next: {
            rootDir: ["apps/website"],
        },
    },
    rules: {
        "@next/next/no-html-link-for-pages": ["off"],
        "@typescript-eslint/consistent-type-imports": [
            "warn",
            {
                prefer: "type-imports",
                fixStyle: "inline-type-imports",
            },
        ],
        "@typescript-eslint/no-unused-vars": [
            "warn",
            {
                vars: "all",
                args: "after-used",
                ignoreRestSiblings: false,
                argsIgnorePattern: "^_",
                varsIgnorePattern: "^_",
            },
        ],
        "import/no-anonymous-default-export": ["off"],
    },
};

module.exports = config;
