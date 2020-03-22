[![Build Status](https://github.com/jewish-interactive/ji-kids-bytes/workflows/Test%2C%20Build%2C%20and%20Deploy/badge.svg)](https://github.com/jewish-interactive/ji-kids-bytes/actions)

## [LIVE SITE](https://bytes.jikids.org)


# Development

1. Install prerequisites like [cargo-make](https://github.com/sagiegurari/cargo-make), and `npm install`
2. `npm start` (will open a browser and rebuild/reload on source change - either typescript, rust, or static files)

More commands are available via `cargo make`:

* `cargo make test` (runs all the tests)
* `cargo make build --profile production` (used in ci/cd - but can check out release builds in `dist/` this way)
* `cargo make build --profile development` (same but used for seeing how non-optmized builds look)

Boilerplate via [dominator-lit-boilerplate](https://github.com/dakom/dominator-lit-boilerplate)
