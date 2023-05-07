import { defineConfig, type Options } from "tsup";

export const createTsupConfig = ({
  entry = ["src/index.ts"],
  external = [],
  noExternal = [],
  platform = "node",
  format = ["esm"],
  target = "es2022",
  metafile = true,
  clean = true,
  minify = true,
  dts = true,
  sourcemap = true,
}: Options = {}) =>
  defineConfig({
    entry,
    external,
    noExternal,
    platform,
    format,
    target,
    skipNodeModulesBundle: true,
    metafile,
    clean,
    minify,
    keepNames: true,
    dts,
    sourcemap,
  });
