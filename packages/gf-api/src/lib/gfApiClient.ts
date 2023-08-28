/* eslint-disable import/no-extraneous-dependencies */
/* eslint-disable promise/prefer-await-to-then */
import { type IFingerprint, validateFingerprint, createFingerprint } from "@nosapi/blackbox";
import type { GameforgeClientVersion } from "../utils/getGameforgeClientVersion";
import { getAccountToken } from "./getAccountToken";

export type GfApiClientSettings = {
  certStore: any;

  clientVersion: GameforgeClientVersion;

  fingerprint: IFingerprint;

  installationId: string;
};

export default class GfApiClient {
  public authToken?: string;

  public readonly clientVersion: GameforgeClientVersion;

  public readonly fingerprint: IFingerprint;

  public readonly installationId: string;

  public constructor(conf: GfApiClientSettings) {
    this.clientVersion = conf.clientVersion;
    this.installationId = conf.installationId;
    const fp = validateFingerprint(conf.fingerprint);
    this.fingerprint = createFingerprint(fp);
  }

  public async login(email: string, password: string) {
    const fp = createFingerprint(this.fingerprint);
    this.authToken = await getAccountToken(email, password, this.installationId, fp);
    return this.authToken;
  }

  public getGameAccounts() {
    const authToken = this.requireAuthToken();

    throw new Error("Not implemented");
  }

  public getGameToken(gameAccount: any) {
    throw new Error("Not implemented");
  }

  private requireAuthToken() {
    if (!this.authToken) throw new Error("Not logged in");
    return this.authToken;
  }
}
