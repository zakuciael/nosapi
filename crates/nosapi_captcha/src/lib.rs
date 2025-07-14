pub mod error;
pub mod types;

use crate::{error::HttpError, header::HeaderMap, types::CaptchaAnswer};
use bon::bon;
use futures::TryFutureExt;
use pluralizer::pluralize;
use reqwest::{Method, RequestBuilder, Response};

use crate::types::Resources;
pub use crate::types::{Captcha, CaptchaData, Locale};

pub mod header {
  //! Re-export of the [`reqwest::header`](http://docs.rs/reqwest/*/reqwest/header) module
  pub use reqwest::header::*;
}

const MAX_PER_CHALLENGE_ATTEMPTS: u8 = 3;
const MAX_SOLVE_ATTEMPTS: u8 = 5;

/// The HTTP client for solving the Gameforge's custom captcha implementation.
pub struct Client {
  client: reqwest::Client,
  base_url: String,
  origin: String,
  locale: Locale,
}

impl Default for Client {
  fn default() -> Self {
    // SAFETY: The default builder values should never produce an error.
    Self::builder().build().unwrap()
  }
}

#[bon]
impl Client {
  /// Returns a [`ClientBuilder`] used to modify the default [`Client`] options.
  #[builder(
    builder_type(doc {
      /// A struct for generating the [`Client`] struct with custom options.
    })
  )]
  pub fn builder(
    /// The `User-Agent` header used in all requests.
    #[builder(
      default = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36",
      into,
      setters(option_fn(vis = ""))
    )]
    user_agent: String,
    /// The `Origin` header used in all requests.
    #[builder(
      default = "spark://www.gameforge.com",
      into,
      setters(option_fn(vis = ""))
    )]
    origin: String,
    /// The captcha locale.
    #[builder(default = Locale::EnglishUS, setters(option_fn(vis = "")))]
    locale: Locale,
    /// The base url from which the captcha is served.
    #[builder(
      default = "https://image-drop-challenge.gameforge.com/challenge",
      into,
      setters(option_fn(vis = ""))
    )]
    base_url: String,
    /// The extra headers to include in all requests.
    #[builder(default = HeaderMap::new(), setters(option_fn(vis = "")))]
    extra_headers: HeaderMap,
  ) -> Result<Self, reqwest::Error> {
    let mut default_headers = HeaderMap::new();
    default_headers.extend(extra_headers);

    let client = reqwest::Client::builder()
      .user_agent(user_agent)
      .default_headers(default_headers)
      .build()?;

    Ok(Self {
      client,
      origin,
      base_url,
      locale,
    })
  }
}

impl Client {
  /// This method tries to automatically solve the captcha challenge for you.
  ///
  /// Returns a [`bool`] value indicating whether the captcha has been solved or not.
  pub async fn try_solve(
    &self,
    challenge_id: impl AsRef<str>,
    max_attempts: Option<u8>,
  ) -> Result<bool, HttpError> {
    let challenge_id = challenge_id.as_ref();
    let max_attempts = max_attempts.unwrap_or(MAX_SOLVE_ATTEMPTS);

    tracing::debug!(challenge_id, max_attempts, "Attempting to solve captcha...");
    let captcha = self.captcha(&challenge_id).await?;

    if matches!(&captcha, Captcha::Solved(_)) {
      tracing::debug!(challenge_id, "Captcha is already solved.");
      return Ok(true);
    }

    let _ = self.resources(&captcha).await?;

    let mut attempt_counter = 1;
    for _ in 0..max_attempts {
      for _ in 0..MAX_PER_CHALLENGE_ATTEMPTS {
        let answer = rand::random_range(0..3);
        let result = self.send_answer(&challenge_id, answer).await?;

        if matches!(result, Captcha::Solved(_)) {
          tracing::debug!(
            challenge_id,
            "Captcha solved in {}",
            pluralize("attempt", attempt_counter, true)
          );
          return Ok(true);
        }

        attempt_counter += 1;
      }

      self.reset(challenge_id).await?;
    }

    tracing::debug!(challenge_id, "Failed to solve captcha");
    Ok(false)
  }

  /// Fetches the information about the captcha challenge.
  ///
  /// ## Additional information
  /// This request is the first in a sequence of requests needed for the captcha answer to be accepted.
  ///
  /// Internally, the UI fetches this information to determine if the captcha challenge is already solved
  /// and uses the [`CaptchaData.last_updated`](CaptchaData#structfield.last_updated) when fetching the resources.
  pub async fn captcha(&self, challenge_id: impl AsRef<str>) -> Result<Captcha, HttpError> {
    let Self {
      base_url, locale, ..
    } = self;
    let challenge_id = challenge_id.as_ref();

    tracing::trace!(challenge_id, "Fetching captcha config...");
    Ok(
      self
        .get(format!("{base_url}/{challenge_id}/{locale}/"), true)
        .await?
        .json()
        .await?,
    )
  }

  /// Fetches the resources used inside the captcha challenge
  /// (text, drag icons, drop target)
  ///
  /// ## Additional information
  /// This request is the second in a sequence of requests needed for the captcha answer to be accepted.
  ///
  /// This fetches the following resources:
  /// - `text` - The text displayed on top of the captcha containing the instructions on how to solve the captcha challenge.
  /// - `drag-icons` - A sprite-sheet of the drag icons, which the user needs to drag-and-drop onto the target, according to the instructions.
  /// - `drop-target` - A drop target icon, on which the user needs to drag-and-drop the drag icon according to the instructions.
  pub async fn resources(&self, captcha: &Captcha) -> Result<Resources, HttpError> {
    let Self {
      base_url, locale, ..
    } = self;
    let CaptchaData {
      challenge_id,
      last_updated,
      ..
    } = match captcha {
      Captcha::Unsolved(v) => v,
      Captcha::Solved(v) => v,
    };

    let format_url = |resource: &str| {
      format!(
        "{base_url}/{challenge_id}/{locale}/{resource}?{}",
        last_updated.timestamp_millis()
      )
    };

    tracing::trace!(challenge_id, "Fetching captcha resources...");
    let (text, drag_icons, drop_target) = futures::try_join!(
      self
        .get(format_url("text"), false)
        .and_then(|v| v.bytes().map_err(|err| err.into())),
      self
        .get(format_url("drag-icons"), false)
        .and_then(|v| v.bytes().map_err(|err| err.into())),
      self
        .get(format_url("drop-target"), false)
        .and_then(|v| v.bytes().map_err(|err| err.into()))
    )?;

    Ok(Resources {
      text,
      drag_icons,
      drop_target,
    })
  }

  /// Sends the answer to the captcha challenge.
  ///
  /// ## Additional information
  /// This request is the third and the last one in a sequence of requests needed for the captcha answer to be accepted.
  ///
  /// Internally, the drag icons are assigned an ID from 0-3 (left to right),
  /// and the answer is the ID of the drag icon which you would drop onto the drop target
  /// according to the captcha instructions.
  ///
  /// Each "displayed" captcha challenge can receive a maximum of 3 incorrect answers before needing to be reset.
  /// If you attempt to send a correct answer after exhausting the limit, the answer will be rejected regardless.
  pub async fn send_answer(
    &self,
    challenge_id: impl AsRef<str>,
    answer: u8,
  ) -> Result<Captcha, HttpError> {
    let Self {
      base_url, locale, ..
    } = self;
    let challenge_id = challenge_id.as_ref();

    tracing::trace!(challenge_id, answer, "Sending answer...");
    Ok(
      self
        .post(
          format!("{base_url}/{challenge_id}/{locale}"),
          false,
          |req| req.json(&CaptchaAnswer { answer }),
        )
        .await?
        .json()
        .await?,
    )
  }

  /// Resets the captcha challenge.
  ///
  /// ## Additional information
  /// After sending the reset request, the captcha challenge needs to be re-fetched according to the sequence of requests.
  pub async fn reset(&self, challenge_id: impl AsRef<str>) -> Result<(), HttpError> {
    let Self {
      base_url, locale, ..
    } = self;
    let challenge_id = challenge_id.as_ref();

    tracing::trace!(challenge_id, "Resetting captcha...");
    let _ = self
      .delete(format!("{base_url}/{challenge_id}/{locale}"), false)
      .await?;

    Ok(())
  }

  async fn get(&self, url: impl AsRef<str>, include_origin: bool) -> Result<Response, HttpError> {
    self
      .request(Method::GET, url, include_origin, |req| req)
      .await
  }

  async fn delete(
    &self,
    url: impl AsRef<str>,
    include_origin: bool,
  ) -> Result<Response, HttpError> {
    self
      .request(Method::DELETE, url, include_origin, |req| req)
      .await
  }

  async fn post(
    &self,
    url: impl AsRef<str>,
    include_origin: bool,
    add_data: impl FnOnce(RequestBuilder) -> RequestBuilder,
  ) -> Result<Response, HttpError> {
    self
      .request(Method::POST, url, include_origin, add_data)
      .await
  }

  async fn request(
    &self,
    method: Method,
    url: impl AsRef<str>,
    include_origin: bool,
    add_data: impl FnOnce(RequestBuilder) -> RequestBuilder,
  ) -> Result<Response, HttpError> {
    let mut request = self.client.request(method, url.as_ref());
    if include_origin {
      request = request.header(header::ORIGIN, &self.origin);
    }

    let request = add_data(request).build()?;
    tracing::trace!(method =? request.method(), url =% request.url(), headers =? request.headers(), "Sending an HTTP request...");
    let response = self.client.execute(request).await?;

    if response.status().is_success() {
      Ok(response)
    } else {
      Err(response.into())
    }
  }
}
