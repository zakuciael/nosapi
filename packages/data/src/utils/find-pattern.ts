/**
 * Finds offset on which the specified pattern starts/ends,
 *
 * @internal
 * @param buffer - Buffer in which the offset should be searched.
 * @param pattern - The pattern that should be used to find the offset.
 * @param options - Options customizing the behavior of this method.
 * @param options.after_pattern -
 * Specifies if the offset should point to the beginning or the end of the found pattern.
 * @returns The offset pointing to where the pattern begins/ends or undefined if the pattern was not found.
 */
export const findPattern = (
  buffer: Uint8Array,
  pattern: Uint8Array,
  options: { after_pattern?: boolean },
) => {
  const { after_pattern } = { after_pattern: true, ...options };

  const compare = (bufOffset: number): boolean => {
    for (const [patternOffset, element] of pattern.entries()) {
      if (buffer[bufOffset + patternOffset] === element) continue;
      return false;
    }

    return true;
  };

  for (let offset = 0; offset < buffer.length; offset++)
    if (compare(offset)) return after_pattern ? offset + pattern.byteLength : offset;

  return undefined;
};
