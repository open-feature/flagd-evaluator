# Changelog

## [0.2.0](https://github.com/open-feature/flagd-evaluator/compare/FlagdEvaluator-v0.1.0...FlagdEvaluator-v0.2.0) (2026-03-30)


### Features

* add YAML flag configuration loading with schema validation ([#7](https://github.com/open-feature/flagd-evaluator/issues/7)) ([79b52cd](https://github.com/open-feature/flagd-evaluator/commit/79b52cd364ec89a8d7bc5eb281bf87309cb7e676))
* **dotnet:** add .NET WASM evaluator package ([#107](https://github.com/open-feature/flagd-evaluator/issues/107)) ([cb73a19](https://github.com/open-feature/flagd-evaluator/commit/cb73a19cf4c603cd47abaf8fed43ca5e99ef3a33))
* expose flag-set metadata in UpdateStateResponse across all providers ([#16](https://github.com/open-feature/flagd-evaluator/issues/16)) ([4eab93d](https://github.com/open-feature/flagd-evaluator/commit/4eab93d1dad1ec2f51516d2b26145c96bce700be))


### Bug Fixes

* **wrappers:** add config size guard before alloc() in all language wrappers ([824d1ee](https://github.com/open-feature/flagd-evaluator/commit/824d1eead94e1bf15cdc0bcb514262348b971fbc))


### Performance Improvements

* **evaluation:** add context key filtering and index-based WASM evaluation ([#70](https://github.com/open-feature/flagd-evaluator/issues/70)) ([50d7cc0](https://github.com/open-feature/flagd-evaluator/commit/50d7cc059e1d8d0149c9fc7439e8d4158dd100e4))
* **java:** replace synchronized single instance with WASM instance pool ([#104](https://github.com/open-feature/flagd-evaluator/issues/104)) ([84fff31](https://github.com/open-feature/flagd-evaluator/commit/84fff31ca3f6a570f6fdf05e1bd8a88f07d00c68))


### Miscellaneous

* **deps:** update dependency benchmarkdotnet to 0.15.8 ([#38](https://github.com/open-feature/flagd-evaluator/issues/38)) ([4b22135](https://github.com/open-feature/flagd-evaluator/commit/4b221356f604e2795018fd65cb64f4b33a28bf03))
* **deps:** update dependency jsonlogic to 5.5.0 ([#40](https://github.com/open-feature/flagd-evaluator/issues/40)) ([41c5227](https://github.com/open-feature/flagd-evaluator/commit/41c5227646c36418097f6bfba6cb9ea4897ab085))
* **deps:** update dependency microsoft.net.test.sdk to 17.14.1 ([#41](https://github.com/open-feature/flagd-evaluator/issues/41)) ([17f30eb](https://github.com/open-feature/flagd-evaluator/commit/17f30eb64f86bc30a8540eed41a513b79c0a1b11))
* safeguard all packages as pre-release ([#15](https://github.com/open-feature/flagd-evaluator/issues/15)) ([25c232d](https://github.com/open-feature/flagd-evaluator/commit/25c232dabed753a80a085f4f32de84a236d8e838))
