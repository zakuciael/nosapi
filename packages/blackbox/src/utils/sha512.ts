import crypto from "node:crypto";

export function sha512(data: string): string {
  return crypto.createHash("sha512").update(data).digest("hex");
}
