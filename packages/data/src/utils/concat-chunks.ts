export const concatChunks = (...chunks: Uint8Array[]) => {
  const size = chunks.reduce((acc, val) => acc + val.byteLength, 0);
  const concatChunk = new Uint8Array(size);

  let offset = 0;
  for (const arr of chunks) {
    concatChunk.set(arr, offset);
    offset += arr.byteLength;
  }

  return concatChunk;
};
