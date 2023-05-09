import { v4 as uuid } from "uuid";
import { GameforgeClientVersion, GfLang, GfLocale } from "./types";
import { getGameforgeClientVersion } from "./lib/getGameforgeClientVersion";
import { sendStartTimeEvent } from "./GfAccount/sendStartTimeEvent";
import { getAccountToken } from "./GfAccount/getAccountToken";

export default class GameforgeApi {
  public readonly locale: GfLocale;

  public readonly gfLang: GfLang;

  public readonly installationId: string;

  public readonly gameSessionId = uuid();

  private clientVersion: GameforgeClientVersion | undefined = undefined;

  // Token used for authentication
  private authToken: string | undefined = undefined;

  public constructor({
    locale,
    gfLang,
    installationId,
  }: {
    gfLang?: GfLang;
    installationId: string;
    locale?: GfLocale;
  }) {
    this.gfLang = gfLang ?? GfLang.en;
    this.locale = locale ?? GfLocale.en_UK;

    this.installationId = installationId;
  }

  public async auth(email: string, password: string): Promise<boolean> {
    const clientVersion = await this.getClientVersion();

    // sendStartTimeEvent(
    //     this.installationId,
    //     this.clientVersion,
    //     this.certStore,
    //     this.gameSessionId,
    // )

    return getAccountToken(email, password, this.installationId)
      .then((token) => {
        console.log(token);
        this.authToken = token;
        return true;
      })
      .catch((err) => {
        console.log(err);
        this.authToken = undefined;
        return false;
      });
  }

  private async getClientVersion(noCache = false): Promise<GameforgeClientVersion | undefined> {
    if (noCache) {
      this.clientVersion = await getGameforgeClientVersion();
      return this.clientVersion;
    }

    if (!this.clientVersion) {
      this.clientVersion = await getGameforgeClientVersion();
    }
    return this.clientVersion;
  }
}
