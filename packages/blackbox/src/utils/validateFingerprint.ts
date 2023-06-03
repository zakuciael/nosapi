import { FingerprintSchema } from "../schema/FingerprintSchema";
import type { IFingerprint } from "../schema/FingerprintSchema";

/**
 * Validate fingerprint (safe)
 *
 * @param fp - Fingerprint object to validate
 * @returns boolean
 */
export function safeValidateFingerprint(fp: IFingerprint): boolean {
  const { success } = FingerprintSchema.safeParse(fp);
  return success;
}

/**
 * Validate fingerprint
 *
 * @param fp - Fingerprint object to validate
 * @returns Fingerprint object
 * @throws ValidationError
 */
export function validateFingerprint(fp: IFingerprint) {
  return FingerprintSchema.parse(fp);
}
