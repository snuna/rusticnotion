# rusticnotion

[![Build](https://github.com/snuna/rusticnotion/actions/workflows/build.yml/badge.svg)](https://github.com/snuna/rusticnotion/actions/workflows/build.yml)
[![Crates.io](https://img.shields.io/crates/v/rusticnotion?style=for-the-badge)](https://crates.io/crates/rusticnotion)

Notion Offical API client library for rust.

Maintained fork, based on the awesome [jakeswenson/notion](https://github.com/jakeswenson/notion).

Under active development.

## How to use

Right now we are working on Version 0.6.0 with the goal to fix all current issues in [jakeswenson/notion](https://github.com/jakeswenson/notion) and make the library compatible with the latest version of the Notion API (2022-06-28).

During this phase we recommend using git for linking the library to your project.

```toml
rusticnotion = { git = "https://github.com/snuna/rusticnotion.git" }
```

Once Version 0.6.0 is released, we will publish the library to crates.io.

```toml
rusticnotion = "0.6.0"
```

## Docs

The generated documentation site is available here: https://docs.rs/rusticnotion/

## Building

```bash
cargo build
```

## Testing

To run integration tests, a env variable of `NOTION_API_TOKEN` must be set.
You can use the provided token in `example.env` by duplicating it to `.env`.

You can also create your own token [here](https://www.notion.so/my-integrations) with minimal permissions. And duplicating this [test page](https://snuna.notion.site/snuna/rusticnotion-test-b8b944b5cc3d444ea25ca7ddacd528cb) into your notion workspace. And then giving your integration only access to this page.

NOTE: While making the integration tests more useful, the template will probably change a lot.

```bash
# Run all tests
cargo test

# Run tests with tracing enabled
RUST_LOG=debug cargo test

# Run tests without integration tests (offline)
cargo test --lib

```

## Contributing

Contributions are always welcome!
If you have an idea, it's best to float it by us before working on it to ensure no effort is wasted.
If there's already an open issue for it, knock yourself out.

If you have any questions, feel free to use [Discussions](https://github.com/snuna/rusticnotion/discussions).
Please don't hesitate to ask questions!
