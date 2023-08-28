/* eslint-disable promise/prefer-await-to-then */
/* eslint-disable import/no-extraneous-dependencies */
import { createBlackbox, type IFingerprint } from "@nosapi/blackbox";
import fetch from "node-fetch";
import { ForbiddenError, InvalidResponseError, CaptchaRequiredError } from "../errors";

/**
 * Authenticates to the API using Gameforge account
 *
 * @public
 * @param email - The account's email address
 * @param password - The account's password
 * @param installationID - The installation id
 * @return The account's auth token
 */
export const getAccountToken = async (
  email: string,
  password: string,
  installationID: string,
  fingerprint: IFingerprint,
  // opts?: GetAccountTokenOptions
): Promise<string> => {
  const BROWSER_USERAGENT =
    "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/72.0.3626.121 Safari/537.36";
  const blackbox = createBlackbox(fingerprint);
  console.log("WIEEEEEEEE");
  console.log(blackbox);
  return fetch(`https://spark.gameforge.com/api/v1/auth/sessions`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "TNT-Installation-Id": installationID,
      Origin: "spark://www.gameforge.com",
      "User-Agent": BROWSER_USERAGENT,
      // ...(options.challengeId ? { "gf-challenge-id": options.challengeId } : undefined),
    },

    body: JSON.stringify({
      email,
      password,
      locale: "en_GB",
      blackbox,
      //   blackbox: "tra:...."
    }),
  })
    .then(async (res) => {
      const challengeIdHeader = res.headers.get("gf-challenge-id");

      console.log("rozjebało się xdDDdDd");
      console.log(await res.text());

      if (res.ok) {
        return res.json() as Promise<{ token: string }>;
      } else if (res.status === 403) {
        throw new ForbiddenError();
      } else if (res.status === 409 && challengeIdHeader) {
        const challengeId = challengeIdHeader.split(";")[0];
        throw new CaptchaRequiredError(challengeId ?? "");
        // TODO: Solve captcha
      }

      throw new InvalidResponseError(res.status, res.statusText);
    })
    .then((data) => (typeof data === "string" ? data : data.token));
};
