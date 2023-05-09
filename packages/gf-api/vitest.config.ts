import { defineConfig } from "vitest/config";
import dotenv from "dotenv";

dotenv.config();

export default defineConfig({
  test: {
    exclude: ["**/node_modules/", "**/dist/", ".idea/", ".vscode/", ".git/", ".cache/"],
    passWithNoTests: true,
  },
});
