import { type ReadableStream } from "node:stream/web";
import { concatChunks } from "../utils/concat-chunks";
import { findPattern } from "../utils/find-pattern";
import { getReadableChunkStream } from "../utils/get-readable-chunk-stream";

const SEARCH_PATTERN = new Uint8Array([
  0x46, 0x00, 0x69, 0x00, 0x6c, 0x00, 0x65, 0x00, 0x56, 0x00, 0x65, 0x00, 0x72, 0x00, 0x73, 0x00, 0x69, 0x00,
  0x6f, 0x00, 0x6e, 0x00, 0x00, 0x00, 0x00, 0x00,
]);

/**
 * Get the file version from the specified buffer.
 *
 * @param file - Buffer containing PE executable file
 * @returns The file version or undefined if the version could not be found.
 * @example Getting the file version of a file loaded from filesystem
 * ```ts
 * import { readFile } from "node:fs/promises";
 * import { getFileVersion } from "@nosapi/data";
 *
 * const file = await readFile("./NostaleClientX.exe");
 * const fileVersion = getFileVersion(file);
 * console.log(fileVersion) // 0.9.3.3191
 * ```
 */
export const getFileVersion = (file: Uint8Array) => {
  const start = findPattern(file, SEARCH_PATTERN, true);
  let end = file.length;

  if (start === -1) return undefined;

  for (let offset = start; offset < file.length; offset += 2) {
    if (file[offset] === 0x00 && file[offset + 1] === 0x00) {
      end = offset;
      break;
    }
  }

  return new TextDecoder("utf-16le").decode(file.subarray(start, end));
};

export const getFileVersionStream = async (stream: ReadableStream<Uint8Array>) => {
  const chunkSize = SEARCH_PATTERN.byteLength + 0x4;
  const chunkStream = getReadableChunkStream(stream, chunkSize);
  const reader = chunkStream.getReader();

  let result = await reader.read();
  let buffer = new Uint8Array(0);
  let data: Uint8Array | undefined;

  while (!result.done) {
    const { value } = result;

    if (data === undefined) {
      const chunk = concatChunks(buffer, value);
      const start = findPattern(chunk, SEARCH_PATTERN, true);

      if (start === -1) {
        buffer = chunk;

        if (buffer.byteLength >= chunkSize * 3) buffer = buffer.slice(chunkSize);
      } else {
        data = chunk.slice(start);
        buffer = new Uint8Array(0);
      }
    } else {
      let end = value.byteLength;

      for (let offset = 0; offset < value.byteLength; offset += 2) {
        if (value[offset] === 0x00 && value[offset + 1] === 0x00) {
          end = offset;
          break;
        }
      }

      data = concatChunks(data, value.slice(0, end));
      break;
    }

    result = await reader.read();
  }

  return data === undefined ? undefined : new TextDecoder("utf-16le").decode(data);
};
