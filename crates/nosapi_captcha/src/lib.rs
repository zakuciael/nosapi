//! HTTP client for Gameforge's image-drop captcha challenge.
//!
//! `nosapi_captcha` wraps the challenge endpoints used by Gameforge's custom
//! captcha flow. Give the client a challenge id, and it can fetch the challenge
//! state, download the image/text resources, submit an answer, and reset the
//! displayed challenge after too many failed attempts.
//!
//! # Quick start
//!
//! Use [`Client::try_solve`] when you only need a best-effort automatic attempt:
//!
//! ```no_run
//! use nosapi_captcha::Client;
//!
//! async fn solve(challenge_id: &str) -> Result<bool, nosapi_captcha::error::HttpError> {
//!     let client = Client::builder()
//!         .user_agent("USER_AGENT_USED_WHEN_THE_CAPTCHA_WAS_CREATED")
//!         .build()?;
//!
//!     client.try_solve(challenge_id, None).await
//! }
//! ```
//!
//! `try_solve` follows the same request order as the browser UI, but it does
//! not inspect the captcha images. It submits random answer indexes until the
//! captcha is solved or the configured attempt limit is reached.
//!
//! # Manual challenge flow
//!
//! For a user-facing solver, call the methods directly so you can render the
//! instructions and images, collect the selected icon, and submit that answer:
//!
//! ```no_run
//! use nosapi_captcha::{Captcha, Client};
//!
//! async fn submit_answer(
//!     client: &Client,
//!     challenge_id: &str,
//!     selected_icon_index: u8,
//! ) -> Result<bool, nosapi_captcha::error::HttpError> {
//!     let captcha = client.captcha(challenge_id).await?;
//!
//!     if matches!(captcha, Captcha::Solved(_)) {
//!         return Ok(true);
//!     }
//!
//!     let resources = client.resources(&captcha).await?;
//!     // Render resources.text, resources.drag_icons, and resources.drop_target.
//!     // Then submit the zero-based index of the chosen drag icon.
//!     let _ = resources;
//!
//!     let result = client
//!         .send_answer(challenge_id, selected_icon_index)
//!         .await?;
//!
//!     Ok(matches!(result, Captcha::Solved(_)))
//! }
//! ```
//!
//! The accepted request sequence is:
//!
//! 1. [`Client::captcha`] fetches the challenge state.
//! 2. [`Client::resources`] fetches the localized instructions and images.
//! 3. [`Client::send_answer`] submits the selected drag icon index.
//! 4. [`Client::reset`] starts a new displayed challenge after failed attempts.
//!
//! # Client configuration
//!
//! [`Client::builder`] method lets you set the user agent, origin, locale, base URL,
//! and default headers. The default values target the public Gameforge captcha
//! endpoint and `en-US` locale.
//!
pub mod error;
pub mod types;

use crate::{error::HttpError, header::HeaderMap, types::CaptchaAnswer};
use bon::bon;
use futures::TryFutureExt;
use reqwest::{Method, RequestBuilder, Response};

use crate::types::Resources;
pub use crate::types::{Captcha, CaptchaData, Locale};

pub mod header {
    //! Re-export of [`reqwest::header`](https://docs.rs/reqwest/latest/reqwest/header/).
    pub use reqwest::header::*;
}

const MAX_PER_CHALLENGE_ATTEMPTS: u8 = 3;
const MAX_SOLVE_ATTEMPTS: u8 = 5;

/// HTTP client for the Gameforge image-drop captcha flow.
///
/// Use [`Client::default`] for the standard Gameforge endpoint and locale, or
/// [`Client::builder`] when you need to match the user agent, origin, locale, or
/// headers from the session that produced the challenge id.
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
    /// Attempts to solve a captcha challenge by trying random answers.
    ///
    /// The method first fetches the challenge state with [`Client::captcha`],
    /// downloads the resources with [`Client::resources`] to preserve the
    /// expected request order, and then submits random zero-based icon indexes
    /// with [`Client::send_answer`].
    ///
    /// `max_attempts` controls how many displayed challenges may be tried. Each
    /// displayed challenge can receive up to three answers before the client
    /// calls [`Client::reset`] and fetches a new one. Pass `None` to use the
    /// crate default.
    ///
    /// Returns `Ok(true)` when the challenge is solved, `Ok(false)` when the
    /// attempt budget is exhausted, and [`HttpError`] if any request or response
    /// decoding step fails.
    pub async fn try_solve(
        &self,
        challenge_id: impl AsRef<str>,
        max_attempts: Option<u8>,
    ) -> Result<bool, HttpError> {
        let challenge_id = challenge_id.as_ref();
        let max_attempts = max_attempts.unwrap_or(MAX_SOLVE_ATTEMPTS);

        log::info!(
            challenge_id,
            max_attempts;
            "Attempting to solve captcha..."
        );
        let captcha = self.captcha(&challenge_id).await?;

        if matches!(&captcha, Captcha::Solved(_)) {
            log::info!(challenge_id; "Captcha is already solved.");
            return Ok(true);
        }

        let _ = self.resources(&captcha).await?;

        let mut attempt_counter = 1;
        for _ in 0..max_attempts {
            for _ in 0..MAX_PER_CHALLENGE_ATTEMPTS {
                let answer = rand::random_range(0..3);
                let result = self.send_answer(&challenge_id, answer).await?;

                if matches!(result, Captcha::Solved(_)) {
                    log::info!(
                        challenge_id;
                        "Captcha solved in {} attempt/s",
                        attempt_counter
                    );
                    return Ok(true);
                }

                attempt_counter += 1;
            }

            self.reset(challenge_id).await?;
        }

        log::warn!(challenge_id; "Failed to solve captcha");
        Ok(false)
    }

    /// Fetches the current state of a captcha challenge.
    ///
    /// This must be the first request in the flow for a challenge id. The
    /// response tells you whether the challenge is already solved and provides
    /// the [`CaptchaData::last_updated`] timestamp needed for resource URLs.
    ///
    /// `challenge_id` is the id segment from the Gameforge challenge URL.
    pub async fn captcha(&self, challenge_id: impl AsRef<str>) -> Result<Captcha, HttpError> {
        let Self {
            base_url, locale, ..
        } = self;
        let challenge_id = challenge_id.as_ref();

        log::debug!(challenge_id; "Fetching captcha config...");
        Ok(self
            .get(format!("{base_url}/{challenge_id}/{locale}/"), true)
            .await?
            .json()
            .await?)
    }

    /// Fetches the text and image resources for a captcha challenge.
    ///
    /// Call this after [`Client::captcha`] and before [`Client::send_answer`].
    /// The provided [`Captcha`] value supplies the challenge id and cache-busting
    /// timestamp used by Gameforge's resource URLs.
    ///
    /// The returned [`Resources`] contains:
    ///
    /// - `text`: localized instructions shown above the captcha.
    /// - `drag_icons`: a sprite sheet of selectable drag icons.
    /// - `drop_target`: the target image the selected icon should be dropped on.
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

        log::debug!(challenge_id = challenge_id.as_str(); "Fetching captcha resources...");
        let (text, drag_icons, drop_target) = futures::try_join!(
            self.get(format_url("text"), false)
                .and_then(|v| v.bytes().map_err(|err| err.into())),
            self.get(format_url("drag-icons"), false)
                .and_then(|v| v.bytes().map_err(|err| err.into())),
            self.get(format_url("drop-target"), false)
                .and_then(|v| v.bytes().map_err(|err| err.into()))
        )?;

        Ok(Resources {
            text,
            drag_icons,
            drop_target,
        })
    }

    /// Submits an answer for a captcha challenge.
    ///
    /// Call this after [`Client::captcha`] and [`Client::resources`].
    ///
    /// `answer` is the zero-based index of the drag icon, counted from left to
    /// right in the `drag_icons` sprite sheet. Current challenges usually expose
    /// three choices, so the expected values are `0`, `1`, and `2`.
    ///
    /// Each displayed challenge can receive a maximum of three incorrect
    /// answers before it must be reset. If the limit is exhausted, a later
    /// correct answer for the same displayed challenge can still be rejected.
    pub async fn send_answer(
        &self,
        challenge_id: impl AsRef<str>,
        answer: u8,
    ) -> Result<Captcha, HttpError> {
        let Self {
            base_url, locale, ..
        } = self;
        let challenge_id = challenge_id.as_ref();

        log::debug!(challenge_id, answer; "Sending answer...");
        Ok(self
            .post(
                format!("{base_url}/{challenge_id}/{locale}"),
                false,
                |req| req.json(&CaptchaAnswer { answer }),
            )
            .await?
            .json()
            .await?)
    }

    /// Resets the displayed captcha challenge.
    ///
    /// After a reset, restart the flow with [`Client::captcha`] so the next
    /// [`Client::resources`] call uses the new [`CaptchaData::last_updated`]
    /// timestamp.
    pub async fn reset(&self, challenge_id: impl AsRef<str>) -> Result<(), HttpError> {
        let Self {
            base_url, locale, ..
        } = self;
        let challenge_id = challenge_id.as_ref();

        log::debug!(challenge_id; "Resetting captcha...");
        let _ = self
            .delete(format!("{base_url}/{challenge_id}/{locale}"), false)
            .await?;

        Ok(())
    }

    async fn get(&self, url: impl AsRef<str>, include_origin: bool) -> Result<Response, HttpError> {
        self.request(Method::GET, url, include_origin, |req| req)
            .await
    }

    async fn delete(
        &self,
        url: impl AsRef<str>,
        include_origin: bool,
    ) -> Result<Response, HttpError> {
        self.request(Method::DELETE, url, include_origin, |req| req)
            .await
    }

    async fn post(
        &self,
        url: impl AsRef<str>,
        include_origin: bool,
        add_data: impl FnOnce(RequestBuilder) -> RequestBuilder,
    ) -> Result<Response, HttpError> {
        self.request(Method::POST, url, include_origin, add_data)
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
        log::debug!(method:? = request.method(), url:% = request.url(), headers:? = request.headers(); "Sending an HTTP request...");
        let response = self.client.execute(request).await?;

        if response.status().is_success() {
            Ok(response)
        } else {
            Err(response.into())
        }
    }
}
