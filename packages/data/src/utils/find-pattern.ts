/**
 * Finds offset on which the specified pattern starts/ends,
 *
 * @internal
 * @param buf - Buffer in which the offset should be searched.
 * @param pattern - The pattern that should be used to find the offset.
 * @param end - Option specifying if the returned offset should be before the pattern (false) or after it (true).
 * @returns The offset pointing to the beginning of the pattern or the end of it.
 */
export const findPattern = (buf: Buffer, pattern: Buffer, end: boolean = false): number => {
  const compare = (bufOffset: number): boolean => {
    for (const [patternOffset, element] of pattern.entries()) {
      if (buf[bufOffset + patternOffset] === element) continue;
      return false;
    }

    return true;
  };

  for (let offset = 0; offset < buf.length; offset++)
    if (compare(offset)) return end ? offset + pattern.byteLength : offset;

  return -1;
};
