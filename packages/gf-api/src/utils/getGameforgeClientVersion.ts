/* eslint-disable promise/prefer-await-to-then */
// eslint-disable-next-line import/no-extraneous-dependencies
import { getFileProperties } from "cfv";
import fetch from "node-fetch";
import { InvalidResponseError } from "../errors";

/**
 * Represents client version information
 *
 * @public
 */
export type GameforgeClientVersion = {
  /**
   * Branch or Tag on which client was built
   */
  branch: string;
  /**
   * Git commit id that points to this client build
   */
  commitId: string;
  /**
   * Client version following semantic versioning
   */
  version: string;
};

export enum GameforgeClientReleaseVersions {
  /**
   * Beta release, containing features not released to the public
   */
  Beta = "beta-ms3",
  /**
   * Public release, used by regular users
   */
  Final = "final-ms3",
  /**
   * Testing release, restricted to testers, contains potentially breaking/buggy changes.
   */
  QualityAssurance1 = "qa1-ms3",
  /**
   * Testing release, restricted to testers, contains potentially breaking/buggy changes.
   */
  QualityAssurance2 = "qa2-ms3",
}

const cachedGetGameforgeClientVersion: Partial<
  Record<GameforgeClientReleaseVersions, GameforgeClientVersion>
> = {};

export const getGameforgeClientVersion = async (
  releaseVersion = GameforgeClientReleaseVersions.Final,
  withCache = false,
): Promise<GameforgeClientVersion> => {
  if (withCache && cachedGetGameforgeClientVersion[releaseVersion] !== undefined) {
    return cachedGetGameforgeClientVersion[releaseVersion] as GameforgeClientVersion;
  }

  // noinspection HttpUrlsUsage
  return fetch(`http://dl.tnt.gameforge.com/tnt/${releaseVersion}/gsl.exe`, {
    method: "GET",
  })
    .then(async (res) => {
      if (res.ok) return res.buffer();
      else throw new InvalidResponseError(res.status, res.statusText);
    })
    .then(async (data) => getFileProperties(data))
    .then((props) => {
      const rawVersion = props.FileVersion as string;
      const match = /((?:\d{1,2}\.){3}\d{1,5}) \(((?:\w+|-|_)+)@(\w+)\)/.exec(rawVersion);

      // eslint-disable-next-line eqeqeq
      if (match == undefined) throw new Error(`Invalid version string: ${rawVersion}`);
      cachedGetGameforgeClientVersion[releaseVersion] = {
        version: match[1],
        branch: match[2],
        commitId: match[3],
      } as GameforgeClientVersion;
      return cachedGetGameforgeClientVersion[releaseVersion] as GameforgeClientVersion;
    });
};
