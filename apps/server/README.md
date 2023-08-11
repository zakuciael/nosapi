<!--suppress HtmlDeprecatedAttribute, CheckImageSize -->
<h1 align="center">
    <a href="https://github.com/zakuciael/nosapi">
        <img alt="NosAPI" src="/.github/logo.png" width="200" />
    </a>
    <br />
    NosAPI
</h1>

<h4 align="center">
An REST API providing access to parsed <a href="https://gameforge.com/en-GB/play/nostale">NosTale's</a> data files.
</h4>

## Self-hosting
#### Requirements
- [Rust](https://www.rust-lang.org/tools/install) v1.70 or higher

#### Installation
```
git clone https://github.com/zakuciael/nosapi.git
cd nosapi/
cargo build --release --package nosapi_server --bin nosapi_server
./target/release/nosapi_server
```

## License

Distributed under the Apache License 2.0. See [LICENSE](LICENSE) for
more information.

## Notice of Non-Affiliation and Disclaimer

This project is not affiliated, associated, authorized, endorsed by, or in any way officially connected with Entwell
Co., Gameforge 4D GmbH or any of their subsidiaries or affiliates.
