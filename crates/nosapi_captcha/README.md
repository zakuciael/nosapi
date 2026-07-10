<!--suppress HtmlDeprecatedAttribute, CheckImageSize -->

<h1 align="center">
    <a href="https://github.com/zakuciael/nosapi">
        <img alt="NosAPI Captcha" src="/assets/logo.png" width="200" />
    </a>
    <br />
    NosAPI Captcha
</h1>

<h4 align="center">
  An HTTP client for solving Gameforge's custom captcha implementation.
</h4>

## Installation

To use this crate in your project, run the following command to add it to your dependencies.

```bash
cargo add nosapi_captcha
```

## Usage

Start with `Client::builder()` when you need the requests to use the same user agent, origin, locale, or headers as the session that received the captcha.

### Best-effort automatic solving

`try_solve` performs the required request sequence and then tries random answer indexes until the challenge is solved or the attempt budget is exhausted.

```rust
use nosapi_captcha::Client;

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .user_agent("USER_AGENT_USED_WHEN_THE_CAPTCHA_WAS_CREATED")
        .build()?;

    let solved = client.try_solve("CAPTCHA_CHALLENGE_ID", None).await?;
    println!("solved: {solved}");

    Ok(())
}
```

This helper does not perform image recognition. For a user-facing or deterministic solver, use the manual flow.

### Manual challenge flow

The Gameforge endpoint expects requests in this order:

1. `captcha` fetches the challenge state.
2. `resources` fetches the localized instructions, drag icon sprite sheet, and drop target image.
3. `send_answer` submits the zero-based index of the selected drag icon.
4. `reset` starts a new displayed challenge after too many failed answers.

```rust
use nosapi_captcha::{Captcha, Client};

async fn submit_answer(
    client: &Client,
    challenge_id: &str,
    selected_icon_index: u8,
) -> Result<bool, nosapi_captcha::error::HttpError> {
    let captcha = client.captcha(challenge_id).await?;

    if matches!(captcha, Captcha::Solved(_)) {
        return Ok(true);
    }

    let resources = client.resources(&captcha).await?;
    // Render resources.text, resources.drag_icons, and resources.drop_target.
    // Let the user pick an icon, then submit its zero-based index.

    let result = client
        .send_answer(challenge_id, selected_icon_index)
        .await?;

    Ok(matches!(result, Captcha::Solved(_)))
}
```

See the [docs.rs page](https://docs.rs/nosapi_captcha/) for builder options, locale values, response types, and error details.

## License

Distributed under the Apache License 2.0. See [LICENSE](LICENSE) for more information.

## Notice of Non-Affiliation and Disclaimer

This project is not affiliated, associated, authorized, endorsed by, or in any way officially connected with Entwell
Co., Gameforge 4D GmbH, or any of their subsidiaries or affiliates.
