/* eslint-disable id-length */
import { encode as blackboxEncode } from "./Blackbox/encoding";
import { encrypt as blackboxEncrypt } from "./Blackbox/encryption";
import type { DateISOString, IFingerprint, IFingerprintRequest } from "./schema/FingerprintSchema";
import randomNumber from "./utils/randomNumber";
import { validateFingerprint } from "./utils/validateFingerprint";

function getServerTimeInMS(): DateISOString {
  const d = new Date();
  const dt = d.toUTCString(); // "Sat, 03 Jun 2023 12:58:09 GMT"
  return new Date(dt).toISOString() as DateISOString; // "2023-06-03T12:58:09.000Z"
}

/**
 * Prepare fingerprint to use.
 * (Add missing fields, compute d, dP, dF, dW, dC, creation, serverTimeInMS)
 *
 * @param fingerprint - Fingerprint object (for example: from config)
 * @returns Fingerprint object
 */
export function createFingerprint(fingerprint: IFingerprint): IFingerprint {
  const fpParsed = validateFingerprint(fingerprint);
  const fp: IFingerprint = {
    ...fpParsed,
    // dP: randomNumber(0, 50),
    // dF: randomNumber(0, 50),
    // dW: randomNumber(0, 50),
    // dC: randomNumber(0, 50),
    creation: new Date().toISOString() as DateISOString,
    serverTimeInMS: getServerTimeInMS(),
  };
  // fp.d = fp.dP + fp.dF + fp.dW + fp.dC + randomNumber(0, 10);
  fp.d = randomNumber(10, 350);
  fp.vector = Buffer.from(fp.vector).toString("base64");
  return fp;
}

/**
 * Create blackbox from fingerprint. tra:abcd...
 *
 * @param fingerprint - Fingerprint object
 * @param request - Fingerprint request object (optional)
 * @returns encoded blackbox
 */
export function createBlackbox(fingerprint: IFingerprint, request?: IFingerprintRequest): string {
  const fp = validateFingerprint(fingerprint);
  fp.request = request === undefined ? null : request;
  return blackboxEncode(fp);
}

/**
 * Create encrypted blackbox from fingerprint.
 *
 * @param fingerprint - Fingerprint object
 * @param gsId - gsId from config
 * @param accountId - accountId from config
 * @param installationId - installationId from config
 * @returns encrypted blackbox
 */
export function createEncryptedBlackbox(
  fingerprint: IFingerprint,
  gsId: string,
  accountId: string,
  installationId: string,
): string {
  const fp = validateFingerprint(fingerprint);
  const delimIndex = gsId.lastIndexOf("-");
  const session = gsId.slice(0, delimIndex);

  const request = {
    features: [randomNumber(1, 0xfffffffe - 1)],
    installation: installationId,
    session,
  };
  const blackbox = createBlackbox(fp, request);
  return blackboxEncrypt(blackbox, gsId, accountId);
}

export default {
  createFingerprint,
  createBlackbox,
  createEncryptedBlackbox,
};
