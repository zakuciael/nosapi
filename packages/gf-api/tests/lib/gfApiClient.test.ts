/* eslint-disable id-length */
import { describe, expect, it } from "vitest";
import { GfApiClient, getGameforgeClientVersion, GameforgeClientReleaseVersions } from "../../src";

describe("getGameforgeClientVersion", () => {
  it("should be able to get client version", async () => {
    const gfVersion = await getGameforgeClientVersion(GameforgeClientReleaseVersions.Final, true);
    /*
    {
      branch: '2-5-0',
      commitId: '0d41d52f',
      version: '2.5.0.1857',
    }
    */

    expect(gfVersion).toHaveProperty("branch");
    expect(gfVersion).toHaveProperty("commitId");
    expect(gfVersion).toHaveProperty("version");
  });
});

const dumbFingerprint = {
  v: 8,
  tz: "Europe/Warsaw",
  dnt: false,
  product: "Blink",
  osType: "Windows",
  app: "Chrome",
  vendor: "Google Inc.",
  mem: 8,
  con: 12,
  lang: "pl-PL,pl,en-US,en",
  plugins: "4f53ecbe5ed12ab4d8e11ba873c2f11161202b945cda18c2baa0c0354bb5f9a3",
  gpu: "Google Inc.,ANGLE (Intel(R) UHD Graphics Direct3D11 vs_5_0 ps_5_0)",
  fonts: "0c5c6fbbe3f88f5c256bf2b0c74a053e83011ba7e48f45baff5acbd6a5e7f6ff",
  audioC: "5eea53e7eace68c10f302cc933ad9af7aa1d00f202e8291fe49b9344f6974663",
  width: 1920,
  height: 1040,
  depth: 24,
  lStore: true,
  sStore: true,
  video: "6a7c8aa89bdeb078690fe8950eea2c39c5eca488bd7ee0a1d7ce6b5600da5f36",
  audio: "c0b73edf391fa02e5b8906ef5bc3ac2b34eb456687e4e0029125e93ba04c130a",
  media: "70e47f4f2c6aa530c0f0bea3df485bd2e5ed3e9b4157fc03761e093b31b42b85",
  permissions: "4efb01a9a42420041da0b72ec4c27f243aa0ac84a576f3009806c3b13614a9b6",
  audioFP: 124.0373434474659,
  webglFP: "a3a7d47f97432c9479f64519cfbfb9458d00f4ec2ffab13f36ab3a27833dd158",
  canvasFP: 1677110232,
  creation: "2023-08-28T14:28:11.552Z",
  uuid: "lsb57x4aus8omzjt1lvhryn5suf",
  d: 337,
  osVersion: "10",
  vector:
    "LEBPZzdZZC1yZVBfc1UmLzRSU205TlRYeSMlXVVxamIkX21zPDVfR057Jl5qOkh7IiYbXp5aVklWHVeNlh2JFJNKy1BS3AzMm5LNCAxN5XUXUxTVlBPHZbPyxNTyRibyloMDR7eFBjkzMjMyODkxMjE1",
  userAgent:
    "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/72.0.3626.121 Safari/537.36",
  serverTimeInMS: "2023-08-28T14:28:11.000Z",
  request: null,
};

describe("GfApiClient", () => {
  it("should be able to create instance and login", async () => {
    const api = new GfApiClient({
      certStore: null,
      clientVersion: await getGameforgeClientVersion(),
      installationId: "6901f592-887c-42be-bcbc-37225ffcd07b",
      fingerprint: dumbFingerprint,
    });

    expect(api).toHaveProperty("clientVersion");

    const authToken = await api.login("totaly.normal.player@gorlik.pl", "SuperSecretPassword1");
    expect(authToken).toBe("xd");
  });
});
