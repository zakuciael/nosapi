import { getFileVersion } from "@nosapi/data/src";
import { NextResponse } from "next/server";
import { ArrayBuffer as SparkMDArrayBuffer } from "spark-md5";
import { type FileData, gameforgeClient } from "~/data/gameforge";

export const runtime = "edge";

export const GET = async () => {
  const { entries: files } = await gameforgeClient("/index");

  const clients = files.reduce<[FileData, FileData]>((acc, fileData) => {
    switch (fileData.file) {
      case "NostaleClientX.exe":
        acc[0] = fileData as FileData;
        break;
      case "NostaleClient.exe":
        acc[1] = fileData as FileData;
        break;
    }

    return acc;

    // eslint-disable-next-line @typescript-eslint/prefer-reduce-type-parameter
  }, [] as unknown as [FileData, FileData]);

  const [clientX, clientGL] = (await Promise.all(
    clients.map(async (fileData) => gameforgeClient("/download", { path: fileData.path })),
  )) as [ArrayBuffer, ArrayBuffer];

  return NextResponse.json(
    {
      md5: {
        client: SparkMDArrayBuffer.hash(clientX),
        clientX: SparkMDArrayBuffer.hash(clientGL),
      },
      version: getFileVersion(new Uint8Array(clientX)),
    },
    {
      headers: {
        "Cache-Control": "s-maxage=3600",
      },
    },
  );
};
