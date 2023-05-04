/** @type {import("prettier").Config} **/
const config = {
    printWidth: 110,
    tabWidth: 4,
    endOfLine: "lf",
    semi: true,
    useTabs: false,
    singleQuote: false,
    bracketSpacing: true,
    trailingComma: "all",
    arrowParens: "always",
    quoteProps: "as-needed",
    overrides: [
        {
            files: ["{**/*,*}.json"],
            options: {
                tabWidth: 2,
            },
        },
    ],
};

module.exports = config;
