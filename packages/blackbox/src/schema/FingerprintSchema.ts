/* eslint-disable id-length */
import { z } from "zod";

// Custom validator for date string in ISO format
export type DateISOString = `${number}-${number}-${number}T${number}:${number}:${number}.${number}Z`;

const zodDateISOString = z.custom<DateISOString>((val) => {
  try {
    // eslint-disable-next-line sonarjs/no-ignored-return
    new Date(val as string).toISOString();
    return true;
  } catch {
    return false;
  }
});

export const FingerprintRequestSchema = z.object({
  features: z.array(z.number()),
  installation: z.string(),
  session: z.string(),
});

// v7
/*
export const FingerprintSchema = z.object({
  v: z.number(),
  tz: z.string(),
  dnt: z.boolean(),
  product: z.string(),
  osType: z.string(),
  app: z.string(),
  vendor: z.string(),
  cookies: z.boolean(),
  mem: z.number(),
  con: z.number(),
  lang: z.string(),
  plugins: z.string(),
  gpu: z.string(),
  fonts: z.string(),
  audioC: z.string(),
  analyser: z.string(),
  width: z.number(),
  height: z.number(),
  depth: z.number(),
  lStore: z.boolean(),
  sStore: z.boolean(),
  video: z.string(),
  audio: z.string(),
  media: z.string(),
  permissions: z.string(),
  audioFP: z.number(),
  webglFP: z.string(),
  canvasFP: z.number(),
  dP: z.number(),
  dF: z.number(),
  dW: z.number(),
  dC: z.number(),
  creation: zodDateISOString,
  uuid: z.string(),
  d: z.number(),
  osVersion: z.string(),
  vector: z.string(),
  userAgent: z.string(),
  serverTimeInMS: zodDateISOString,
  request: FingerprintRequestSchema.nullable().optional(),
});
*/

// v8
export const FingerprintSchema = z.object({
  v: z.number(),
  tz: z.string(),
  dnt: z.boolean(),
  product: z.string(),
  osType: z.string(),
  app: z.string(),
  vendor: z.string(),
  mem: z.number(),
  con: z.number(),
  lang: z.string(),
  plugins: z.string(),
  gpu: z.string(),
  fonts: z.string(),
  audioC: z.string(),
  width: z.number(),
  height: z.number(),
  depth: z.number(),
  lStore: z.boolean(),
  sStore: z.boolean(),
  video: z.string(),
  audio: z.string(),
  media: z.string(),
  permissions: z.string(),
  audioFP: z.number(),
  webglFP: z.string(),
  canvasFP: z.number(),
  creation: zodDateISOString,
  uuid: z.string(),
  d: z.number(),
  osVersion: z.string(),
  vector: z.string(),
  userAgent: z.string(),
  serverTimeInMS: zodDateISOString,
  request: FingerprintRequestSchema.nullable().optional(),
});

export const FingerprintFields = Object.keys(FingerprintSchema.shape);

export type IFingerprint = z.infer<typeof FingerprintSchema>;
export type IFingerprintRequest = z.infer<typeof FingerprintRequestSchema>;
