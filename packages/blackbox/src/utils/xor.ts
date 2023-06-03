/* eslint-disable id-length */
/* eslint-disable unicorn/no-for-loop */
export function xor(data: Buffer, key: Buffer) {
  const key_length = key.length;
  const result = [];

  for (let i = 0; i < data.length; ++i) {
    const wrapping_i = i % key_length;

    // @ts-expect-error: TS doesn't know that data[i] is a number
    const c = data[i] ^ key[wrapping_i] ^ key[key_length - wrapping_i - 1];
    result.push(c);
  }

  return Buffer.from(result);
}
