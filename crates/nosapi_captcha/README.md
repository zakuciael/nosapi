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

You can either use the all-in-one `try_solve` method that will automatically do all the necessary steps needed to solve the captcha like this:

```rust
use nosapi_captcha::Client;

async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let client = Client::builder()
    .user_agent("USER_AGENT_USED_WHEN_ENCOUNTERED_THE_CAPTCHA")
    .build()?;
  println!("{:?}", client.try_solve("CAPTCHA_CHALLENGE_ID", None).await?);
}
```

Or manually using the `captcha`, `resources`, `send_answer` and `reset` methods.  
For more information on how each of these methods works, refer to the [**docs.rs**](https://docs.rs/nosapi_captcha/) page.

## License

Distributed under the Apache License 2.0. See [LICENSE](LICENSE) for more information.

## Notice of Non-Affiliation and Disclaimer

This project is not affiliated, associated, authorized, endorsed by, or in any way officially connected with Entwell
Co., Gameforge 4D GmbH, or any of their subsidiaries or affiliates.
