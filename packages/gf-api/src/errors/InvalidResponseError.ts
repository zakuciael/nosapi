/**
 * An error that indicates an invalid response.
 *
 * @public
 */
export class InvalidResponseError extends Error {
  /**
   * The text received in the response
   */
  public readonly statusText: string;

  /**
   * The HTTP status code
   */
  public readonly status: number;

  /**
   * @param status - HTTP status code
   * @param statusText - Text received in the response
   */
  public constructor(status: number, statusText: string) {
    super("An invalid response was received from the server (status: " + status + " " + statusText + ")");
    this.statusText = statusText;
    this.status = status;
  }
}
