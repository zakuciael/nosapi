import { describe, it } from "vitest";
import GameforgeApi from "../src/GameforgeApi";

describe("GameforgeApi", async () => {
  it("check environment variables", ({ expect }) => {
    expect(process.env.VITE_GF_EMAIL).toBeDefined();
    expect(process.env.VITE_GF_PASSWORD).toBeDefined();
  });

  it("should login", async ({ expect }) => {
    const installationId = "41f3670d-6e6e-4ac0-9c03-080d4f58487c";
    const gfapi = new GameforgeApi({ installationId });

    expect(await gfapi.auth(process.env.VITE_GF_EMAIL!, process.env.VITE_GF_PASSWORD!)).toBe(true);
  });
});
