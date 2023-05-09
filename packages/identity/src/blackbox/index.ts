import axios from "axios";
import { BlackboxEncoding } from "./blackbox_encoding.js";
import { BlackboxEncryption } from "./blackbox_encryption.js";

const SERVER_FILE_GAME1_FILE = "https://gameforge.com/tra/game1.js";
const VECTOR_LENGTH = 100;

function random_ascii_character() {
  return String.fromCodePoint(Math.trunc(0x20 + Math.random() * (0x7e - 0x20)));
}

function generateVector() {
  const vector_content = Array.from(Array.from({ length: VECTOR_LENGTH }), random_ascii_character).join("");
  const time = Date.now();
  return `${vector_content} ${time}`;
}

async function get_server_date() {
  const res = await axios.get(SERVER_FILE_GAME1_FILE);
  return new Date(res.headers.date).toISOString();
}

function updateVector(vector: string) {
  const delim_index = vector.lastIndexOf(" ");
  const vec_content_array = vector.slice(0, Math.max(0, delim_index));
  const vec_time_array = vector.slice(Math.max(0, delim_index + 1));
  const current_time = Date.now();

  const vec_content = vec_content_array.split("");
  const vec_time = Number.parseInt(vec_time_array, 10);

  if (current_time > vec_time + 1_000) {
    vec_content.shift();
    vec_content.push(random_ascii_character());

    return `${vec_content.join("")} ${current_time}`;
  }

  return vector;
}

function randomInt(min: number, max: number) {
  return Math.floor(Math.random() * (max - min + 1)) + min;
}

async function createFingerprint(identity) {
  const fingerprint = {
    ...identity.fingerprint,
    dP: randomInt(1, 50), // timing browser_info, platform_info, perms_media_audio
    dF: randomInt(1, 50), // intalled fonts fignerprint timing
    dW: randomInt(1, 50), // webgl fingerprint timing
    dC: randomInt(1, 50), // canvas fignerprint timing
    creation: new Date().toISOString(),
    serverTimeInMS: await get_server_date(),
  };

  fingerprint.d = fingerprint.dP + fingerprint.dF + fingerprint.dW + fingerprint.dC + randomInt(1, 50);
  fingerprint.vector = Buffer.from(fingerprint.vector).toString("base64");
  return fingerprint;
}

type FingerprintRequest = {
  features: number[];
  installation: string;
  session: string;
};

function createBlackbox(fingerprint, request: FingerprintRequest) {
  fingerprint.request = request;
  return BlackboxEncoding.encode(fingerprint);
}

function createEncryptedBlackbox(fingerprint, gs_id, accountId: string, installationId: string): FingerprintRequest {
  const delim_index = gs_id.lastIndexOf("-");
  const session = gs_id.slice(0, Math.max(0, delim_index));

  const request = {
    features: [randomInt(1, 0xfffffffe - 1)],
    installation: installationId,
    session,
  };

  const blackbox = createBlackbox(fingerprint, request);
  return BlackboxEncryption.encrypt(blackbox, gs_id, accountId);
}

export const Blackbox = {
  createFingerprint,
  createBlackbox,
  createEncryptedBlackbox,
  generateVector,
  updateVector,
};
