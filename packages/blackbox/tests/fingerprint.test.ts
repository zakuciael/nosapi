/* eslint-disable unicorn/numeric-separators-style */
/* eslint-disable id-length */
import { expect, test } from "vitest";
import { createBlackbox, createEncryptedBlackbox, createFingerprint, safeValidateFingerprint } from "../src";
import type { IFingerprint } from "../src";

// hey, this is created using ChatGPT. If you want use it its 110% propably you will be BANNED!
const exampleFingerprint: IFingerprint = {
  v: 1,
  tz: "Europe/Warsaw",
  dnt: false,
  product: "Firefox",
  osType: "Linux",
  app: "Firefox",
  vendor: "Google Inc.",
  cookies: true,
  mem: 8,
  con: 4,
  lang: "pl-PL",
  plugins: "Shockwave Flash",
  gpu: "ANGLE (Intel(R) HD Graphics 4000 Direct3D11 vs_5_0 ps_5_0)",
  fonts: "Arial",
  audioC: "0",
  analyser: "0",
  width: 1920,
  height: 1080,
  depth: 24,
  lStore: true,
  sStore: true,
  video: "0",
  audio: "0",
  media: "0",
  permissions: "0",
  audioFP: 0,
  webglFP: "0",
  canvasFP: 0,
  dP: 0,
  dF: 0,
  dW: 0,
  dC: 0,
  creation: "2021-08-31T12:00:00.000Z",
  uuid: "0",
  d: 0,
  osVersion: "0",
  vector: "0",
  userAgent: "Mozilla/5.0 (X11; Linux x86_64; rv:91.0) Gecko/20100101 Firefox/91.0",
  serverTimeInMS: "2021-08-31T12:00:00.000Z",
};

function uuidv4() {
  // eslint-disable-next-line unicorn/prefer-string-replace-all
  return "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, (c) => {
    const r = Math.trunc(Math.random() * 16);
    const v = c === "x" ? r : (r & 0x3) | 0x8;
    return v.toString(16);
  });
}

function genGsid() {
  const session = uuidv4();
  const num = (Math.floor(Math.random() * 9999) + 1).toString();
  return `${session}-${num.padStart(4, "0")}`;
}

test("Fingerprint zod", () => {
  const success = safeValidateFingerprint(exampleFingerprint);
  expect(success).toBe(true);
});

test("Prepare to Send iovation request", () => {
  const fingerprint = createFingerprint(exampleFingerprint);
  createBlackbox(fingerprint);
  expect(true).toBe(true);
});

test("Prepare to Send thin/codes request", () => {
  const accountId = uuidv4();
  const installationId = uuidv4();
  const gsid = genGsid();

  const fingerprint = createFingerprint(exampleFingerprint);
  createEncryptedBlackbox(fingerprint, gsid, accountId, installationId);
  expect(true).toBe(true);
});
