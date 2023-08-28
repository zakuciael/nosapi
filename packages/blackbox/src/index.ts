export type { IFingerprint, IFingerprintRequest } from "./schema/FingerprintSchema";
export { safeValidateFingerprint, validateFingerprint } from "./utils/validateFingerprint";
export { createFingerprint, createBlackbox, createEncryptedBlackbox } from "./Blackbox";

export { decode, encode } from "./Blackbox/encoding";
export { encrypt, decrypt } from "./Blackbox/encryption";
