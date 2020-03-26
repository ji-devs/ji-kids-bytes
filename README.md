[![Build Status](https://github.com/jewish-interactive/ji-kids-bytes/workflows/Test%2C%20Build%2C%20and%20Deploy/badge.svg)](https://github.com/jewish-interactive/ji-kids-bytes/actions)

## [LIVE SITE](https://bytes.jikids.org)

# Configuration

## Manifest

The starting point is the "manifest list" at this Google sheet: https://docs.google.com/spreadsheets/d/1kugXziYFFDwiJmIxQ-T_8cckOc4qSfHoFw9KoM56Hvs/edit#gid=0

This sheet lists the topic manifest id's as well as top-level metadata (currently only `lock` status)

The listed topic sheet ids become the URL for each manifest on Google Drive. For example if the sheet id is `1B5qEuf4pXq0bhfRN7mDrR_opzUqArI7JYFq1NqF7hTg` it becomes https://docs.google.com/spreadsheets/d/1B5qEuf4pXq0bhfRN7mDrR_opzUqArI7JYFq1NqF7hTg/edit#gid=0

In each sheet there is a vertical divider. Everything to the *left* is very precise and becomes the actual data for the website. Everything to the *right* is just commentary and a free-for-all

**Note: The sheets must be Published to work! In Google sheets it's File -> Publish to Web. Default settings are fine**


## Media

Media is stored in dropbox. The manifest sometimes lists filenames that point to filenames in the respective dropbox folder. For example, `Passover_Mashup.jpg` in the `discover` topic of the `moses` manifest will translate to `live-media/topics/moses/discover/Passover_Mashup` in the Dropbox folder

## Syncing

Note that re-generating manifests as well as syncing all media changes to the live site both require running a manual command. It's a simple one-liner but does not happen automatically (currently only David is setup to do this - so ping him if it needs an update).

----

# Development

1. Install prerequisites like [cargo-make](https://github.com/sagiegurari/cargo-make), and `npm install`
2. `npm start` (will open a browser and rebuild/reload on source change - either typescript, rust, or static files)

More commands are available via `cargo make`:

* `cargo make test` (runs all the tests)
* `cargo make build --profile production` (used in ci/cd - but can check out release builds in `dist/` this way)
* `cargo make build --profile development` (same but used for seeing how non-optmized builds look)

Boilerplate via [dominator-lit-boilerplate](https://github.com/dakom/dominator-lit-boilerplate)
