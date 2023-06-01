import { createSafeClient, initUntypeable, type RoutesFromRouter } from "untypeable";
import { type TypeOf, z } from "zod";

// eslint-disable-next-line id-length
const u = initUntypeable();

const indexFolderSchema = z.object({
  file: z.string(),
  flags: z.number(),
  folder: z.literal(true),
  size: z.number().min(0),
});

const indexFileSchema = z.object({
  file: z.string(),
  flags: z.number(),
  folder: z.literal(false),
  path: z.string(),
  sha1: z.string(),
  size: z.number().min(0),
});

const router = u.router({
  "/index": u
    .input(
      z.object({
        architecture: z.enum(["x64", "x84"]).default("x64").optional(),
        locale: z.string().min(2).default("en").optional(),
      }),
    )
    .output(
      z.object({
        entries: z.array(z.union([indexFolderSchema, indexFileSchema])),
      }),
    ),
  "/download": u.input(z.object({ path: z.string() })).output(z.instanceof(ArrayBuffer)),
});

type Router = typeof router;
type Routes = RoutesFromRouter<Router>;
type Path = keyof Routes;

export type FolderData = TypeOf<typeof indexFolderSchema>;
export type FileData = TypeOf<typeof indexFileSchema>;

export const gameforgeClient = createSafeClient<Router, [Path], Routes>(router, async (path, input) => {
  switch (path) {
    case "/download": {
      // noinspection HttpUrlsUsage
      const url = new URL(input.path, "http://patches.gameforge.com/");
      const requestInit: RequestInit = {
        method: "GET",
        cache: "no-cache",
      };

      return (await fetch(url, requestInit)).arrayBuffer();
    }

    case "/index": {
      const url = new URL(
        `/api/v1/patching/download/latest/nostale/default?${new URLSearchParams({
          ...input,
          branchToken: "",
        })}`,
        "https://spark.gameforge.com/",
      );

      return (await fetch(url, { method: "GET" })).json();
    }

    default:
      throw new TypeError("Unknown path");
  }
});
