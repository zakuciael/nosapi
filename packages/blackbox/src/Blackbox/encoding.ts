/* eslint-disable id-length */
/* eslint-disable unicorn/prefer-code-point */
import { FingerprintFields } from "../schema/FingerprintSchema";
import type { IFingerprint } from "../schema/FingerprintSchema";

const BLACKBOX_FIELDS = FingerprintFields;

/**
 * Convert fingerprint to blackbox
 *
 * @param fp - Fingerprint object to convert
 * @returns JSON string with blackbox
 */
function generateCorrectBlackbox(fp: IFingerprint): string {
  const blackbox: (boolean | number | string)[] = [];

  for (const field of BLACKBOX_FIELDS) {
    // @ts-expect-error: TS doesn't know that field is a key of IFingerprint
    if (fp[field] === undefined) {
      throw new Error(`Missing fingerprint field ${field}`);
    }

    // @ts-expect-error: TS doesn't know that field is a key of IFingerprint
    blackbox.push(fp[field]);
  }

  return JSON.stringify(blackbox);
}

/**
 * Encode fingerprint to blackbox.
 * This value is send using "iovation" endpoint.
 * Note: This value is not computed using accountId, gsid or installationId. Only fingerprint is used.
 *
 * @param fp - Fingerprint object to encode
 * @returns Encoded blackbox
 */
export function encode(fp: IFingerprint): string {
  const jsonObj = generateCorrectBlackbox(fp);
  const uriEncoded = encodeURIComponent(jsonObj);

  let result = uriEncoded[0] as string;
  for (let i = 1; i < uriEncoded.length; ++i) {
    const a = result.charCodeAt(i - 1);
    const b = uriEncoded.charCodeAt(i);
    const c = String.fromCharCode((a + b) % 0x100);

    result += c;
  }

  const blackbox = Buffer.from(result, "latin1").toString("base64");
  return "tra:" + blackbox.replaceAll("/", "_").replaceAll("+", "-").replaceAll("=", "");
}

/**
 * Decode blackbox to fingerprint.
 *
 * @param encodedBlackbox - Encoded blackbox
 * @returns Fingerprint object
 */
export function decode(encodedBlackbox: string) {
  const blackbox = encodedBlackbox.replaceAll("tra:", "").replaceAll("_", "/").replaceAll("-", "+");
  const gfEncoded = Buffer.from(blackbox, "base64");

  let uriEncoded = String.fromCharCode(gfEncoded[0] as number);
  for (let i = 1; i < gfEncoded.length; ++i) {
    const b = gfEncoded[i - 1] as number;
    let a = gfEncoded[i] as number;

    if (a < b) {
      a += 0x100;
    }

    const c = String.fromCharCode(a - b);
    uriEncoded += c;
  }

  // const fingerprintStr = decodeURIComponent(uriEncoded.toString("latin1"));
  const fingerprintStr = decodeURIComponent(uriEncoded);
  const fingerprintArray = JSON.parse(fingerprintStr);
  const fingerprint = {};

  if (fingerprintArray.length !== BLACKBOX_FIELDS.length) {
    throw new Error(
      `Incomplete blackbox (You have ${fingerprintArray.length} fields, but you need ${BLACKBOX_FIELDS.length})`,
    );
  }

  // eslint-disable-next-line unicorn/no-for-loop
  for (let i = 0; i < BLACKBOX_FIELDS.length; i++) {
    // @ts-expect-error: TS doesn't know that BLACKBOX_FIELDS[i] is a key of IFingerprint
    fingerprint[BLACKBOX_FIELDS[i]] = fingerprintArray[i];
  }

  return fingerprint as IFingerprint;
}

export default {
  encode,
  decode,
};
