<!--suppress HtmlDeprecatedAttribute, CheckImageSize -->

<h1 align="center">
    <a href="https://github.com/zakuciael/nosapi">
        <img alt="NosAPI Blackbox" src="/assets/logo.png" width="200" />
    </a>
    <br />
    NosAPI Blackbox
</h1>

<h4 align="center">
  A library for working with Gameforge's blackbox string
</h4>

## Installation

To use this crate in your project, run the following command to add it to your dependencies.

```bash
cargo add nosapi_blackbox
```

## Usage

Use this crate when you need to inspect or modify Gameforge `blackbox` values.

- Use `Blackbox::decode` and `Blackbox::encode` for transport strings that look like `tra:...`.
- Use `Blackbox::decrypt` and `Blackbox::encrypt` for encrypted strings tied to a `gsid` and account id.
- Read and edit fingerprint fields directly on `Blackbox`; it dereferences to `Fingerprint`.
- Use `AsRef<Fingerprint>`, `AsMut<Fingerprint>`, or `From<Fingerprint>` when an API needs explicit conversions.
- Use `VectorString::update` after changing fingerprint fields so the vector modification timestamp is refreshed.

### Decode and inspect

```rust
use nosapi_blackbox::Blackbox;

fn inspect(encoded_blackbox: &str) -> Result<(), Box<dyn std::error::Error>> {
    let blackbox = Blackbox::decode(encoded_blackbox.to_owned())?;

    println!("browser: {}", blackbox.browser_name);
    println!("user agent: {}", blackbox.user_agent);

    Ok(())
}
```

### Modify and encode

```rust
use nosapi_blackbox::Blackbox;

fn change_locale(encoded_blackbox: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut blackbox = Blackbox::decode(encoded_blackbox.to_owned())?;

    blackbox.languages = "en-US,en".to_owned();
    blackbox.vector.update();

    Ok(blackbox.encode()?)
}
```

### Decrypt and encrypt

```rust
use nosapi_blackbox::Blackbox;

fn round_trip(
    encrypted_blackbox: &str,
    gsid: &str,
    account_id: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let blackbox = Blackbox::decrypt(
        encrypted_blackbox.to_owned(),
        gsid.to_owned(),
        account_id.to_owned(),
    )?;

    Ok(blackbox.encrypt(gsid.to_owned(), account_id.to_owned())?)
}
```

See the [docs.rs page](https://docs.rs/nosapi_blackbox/) for the full `Fingerprint` schema and error types.

## License

Distributed under the Apache License 2.0. See [LICENSE](LICENSE) for more information.

## Notice of Non-Affiliation and Disclaimer

This project is not affiliated, associated, authorized, endorsed by, or in any way officially connected with Entwell
Co., Gameforge 4D GmbH, or any of their subsidiaries or affiliates.
