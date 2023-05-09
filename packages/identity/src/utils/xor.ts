/* eslint-disable id-length */
/* eslint-disable unicorn/no-for-loop */
/* eslint-disable unicorn/prefer-code-point */

export function xorEncrypt(data: string, key: string): string {
  const result: string[] = [];
  for (let i = 0; i < data.length; i++) {
    const byte = data[i] as string;
    const key_idx = i % key.length;

    const char =
      // @ts-expect-error key_idx have correct value
      byte.charCodeAt(0) ^ key[key_idx].charCodeAt(0) ^ key[key.length - key_idx - 1].charCodeAt(0);
    result.push(String.fromCharCode(char));
  }

  return result.join("");
}
