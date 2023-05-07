import "./src/env.mjs";

/** @type {import("next").NextConfig} */
const config = {
  reactStrictMode: true,
  eslint: {
    ignoreDuringBuilds: true,
  },
  typescript: {
    ignoreBuildErrors: true,
  },
  experimental: {
    typedRoutes: true,
    runtime: "experimental-edge",
  },
  i18n: {
    locales: ["en"],
    defaultLocale: "en",
  },
};

export default config;
