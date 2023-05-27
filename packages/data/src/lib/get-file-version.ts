import { findPattern } from "../utils/find-pattern";

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

  const decoder = new TextDecoder("utf16le");
  return decoder.decode(file.subarray(start, end));
};
