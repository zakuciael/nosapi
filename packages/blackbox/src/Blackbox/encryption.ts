import { sha512 } from "../utils/sha512";
import { xor } from "../utils/xor";

function createEncryptionKey(gsId: string, accountId: string) {
  return Buffer.from(sha512(`${gsId}-${accountId}`));
}

/**
 * Encrypt blackbox using gsId and accountId as encryption key.
 *
 * @param blackbox - JSON string with blackbox
 * @param gsId - gsId from config
 * @param accountId - accountId from config
 * @returns  Encrypted blackbox
 */
export function encrypt(blackbox: string, gsId: string, accountId: string): string {
  const key = createEncryptionKey(gsId, accountId);
  const encryptedBlackbox = xor(Buffer.from(blackbox), key);
  return encryptedBlackbox.toString("base64");
}

/**
 * Decrypt blackbox using gsId and accountId as encryption key.
 *
 * @param encryptedBlackbox - Encrypted blackbox
 * @param gsId - gsId from config
 * @param accountId - accountId from config
 * @returns JSON string with blackbox
 */
export function decrypt(encryptedBlackbox: string, gsId: string, accountId: string): string {
  const key = createEncryptionKey(gsId, accountId);
  const decryptedBlackbox = xor(Buffer.from(encryptedBlackbox, "base64"), key);
  return decryptedBlackbox.toString();
}

export default {
    encrypt,
    decrypt,
}