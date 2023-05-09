import sha512 from "./utils/sha512";
import { xorEncrypt } from "./utils/xor";

function fingerprintToEncodedBlackbox(dictObj: any) {}

function randomIntFromRange(min: number, max: number) {
  return Math.floor(Math.random() * (max - min + 1) + min);
}

export class BlackBox {
  private readonly gsid: string;

  private readonly installationId: string;

  private readonly key: string;

  /**
   * BlackBox constructor
   * @param accountId - Account id (got from Gameforge API)
   * @param gameSessionId - ???
   * @param installationId - Nostale installation id in UUID format
   */
  public constructor(accountId: string, gameSessionId: string, installationId: string) {
    this.gsid = gameSessionId;
    this.installationId = installationId;

    // generate key
    this.key = sha512(`${this.gsid}-${accountId}`);
  }

  public encrypt(fingerprint: Fingerprint): string {
    fingerprint.setRequest(this.createRequest());
    const blackbox = fingerprintToEncodedBlackbox(fingerprint.fingerprint);

    const encryptedBlackbox = xorEncrypt(blackbox, this.key);
    return Buffer.from(encryptedBlackbox).toString("base64");
  }

  /**
   * Creates request object used by fingerprint
   */
  private createRequest() {
    const gsidSplited = this.gsid.split("-");

    return {
      features: [randomIntFromRange(1, 2 ** 32)],
      installation: this.installationId,
      session: gsidSplited[gsidSplited.length - 1] as string,
    };
  }
}
