export const findNullTermination = (
  buf: Uint8Array,
  offset: number,
  default_value: number | undefined = undefined,
) => {
  for (let index = offset; index < buf.byteLength; index += 2) {
    if (buf[index] === 0x00 && buf[index + 1] === 0x00) {
      return index;
    }
  }

  return default_value;
};
