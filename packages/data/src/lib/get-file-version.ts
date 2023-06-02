import { findPattern } from "../utils/find-pattern";

const SEARCH_PATTERN = new Uint8Array([
  0x46, 0x00, 0x69, 0x00, 0x6c, 0x00, 0x65, 0x00, 0x56, 0x00, 0x65, 0x00, 0x72, 0x00, 0x73, 0x00, 0x69, 0x00,
  0x6f, 0x00, 0x6e, 0x00, 0x00, 0x00, 0x00, 0x00,
]);

/**
 * Get the file version from the specified uint8 array.
 *
 * @param file - Buffer containing PE executable file
 * @returns The file version or undefined if the version could not be found/processed.
 * @example Getting the file version from a local file
 * import { readFile } from "node:fs/promises";
 * import { getFileVersion } from "@nosapi/data";
 *
 * const file = await readFile("./NostaleClientX.exe");
 * const fileVersion = getFileVersion(file);
 * console.log(fileVersion) // 0.9.3.3191
 */
export const getFileVersion = (file: Uint8Array) => {
  const start = findPattern(file, SEARCH_PATTERN, { after_pattern: true });
  if (start === undefined) return undefined;

  for (let index = start; index < file.byteLength; index += 2)
    if (file[index] === 0x00 && file[index + 1] === 0x00)
      return new TextDecoder("utf-16le").decode(file.subarray(start, index));

  return undefined;
};
