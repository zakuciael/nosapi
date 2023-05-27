import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    exclude: [
      "**/node_modules/**/*",
      "**/dist/**/*",
      ".idea/**/*",
      ".vscode/**/*",
      ".git/**/*",
      ".cache/**/*",
    ],
    environmentMatchGlobs: [
      ["**/tests/**/*.edge.{test,spec}.{ts,tsx}", "edge-runtime"],
      ["**/tests/**/*.{test,spec}.{ts,tsx}", "node"],
    ],
    passWithNoTests: true,
  },
});
