import { ReadableStream } from "node:stream/web";
import { concatChunks } from "./concat-chunks";

export const getReadableChunkStream = (stream: ReadableStream<Uint8Array>, chunkSize: number) => {
  const reader = stream.getReader();

  return new ReadableStream<Uint8Array>({
    start: async (controller) => {
      let buffer = new Uint8Array(0);
      let result = await reader.read();

      while (!result.done) {
        const { value } = result;
        let chunk = concatChunks(buffer, value);

        while (chunk.byteLength >= chunkSize) {
          controller.enqueue(chunk.slice(0, chunkSize));
          chunk = chunk.slice(chunkSize);
        }

        buffer = chunk;
        result = await reader.read();
      }

      if (buffer.byteLength > 0) controller.enqueue(buffer);
      controller.close();
    },
  });
};
