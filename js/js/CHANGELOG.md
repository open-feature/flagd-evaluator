# Changelog

## [0.2.0](https://github.com/open-feature/flagd-evaluator/compare/flagd-wasm-evaluator-v0.1.0...flagd-wasm-evaluator-v0.2.0) (2026-03-30)


### Features

* expose flag-set metadata in UpdateStateResponse across all providers ([#16](https://github.com/open-feature/flagd-evaluator/issues/16)) ([4eab93d](https://github.com/open-feature/flagd-evaluator/commit/4eab93d1dad1ec2f51516d2b26145c96bce700be))
* **js:** add JavaScript/TypeScript WASM evaluator package ([#110](https://github.com/open-feature/flagd-evaluator/issues/110)) ([42bce60](https://github.com/open-feature/flagd-evaluator/commit/42bce60ea1945e33e054f987ba6459a25b69d5e9))


### Bug Fixes

* **wrappers:** add config size guard before alloc() in all language wrappers ([824d1ee](https://github.com/open-feature/flagd-evaluator/commit/824d1eead94e1bf15cdc0bcb514262348b971fbc))


### Performance Improvements

* **evaluation:** add context key filtering and index-based WASM evaluation ([#70](https://github.com/open-feature/flagd-evaluator/issues/70)) ([50d7cc0](https://github.com/open-feature/flagd-evaluator/commit/50d7cc059e1d8d0149c9fc7439e8d4158dd100e4))
* **java:** replace synchronized single instance with WASM instance pool ([#104](https://github.com/open-feature/flagd-evaluator/issues/104)) ([84fff31](https://github.com/open-feature/flagd-evaluator/commit/84fff31ca3f6a570f6fdf05e1bd8a88f07d00c68))


### Miscellaneous

* **deps:** update dependency @types/node to v20.19.37 ([#32](https://github.com/open-feature/flagd-evaluator/issues/32)) ([27e22dd](https://github.com/open-feature/flagd-evaluator/commit/27e22ddb5dcb2cfc5c72039d8b741d3b2cccc71f))
* **deps:** update dependency json-logic-engine to v4.0.6 ([#33](https://github.com/open-feature/flagd-evaluator/issues/33)) ([c0531b2](https://github.com/open-feature/flagd-evaluator/commit/c0531b26e1c0792bf6aab1a1ec2da0f75efff3ed))
* safeguard all packages as pre-release ([#15](https://github.com/open-feature/flagd-evaluator/issues/15)) ([25c232d](https://github.com/open-feature/flagd-evaluator/commit/25c232dabed753a80a085f4f32de84a236d8e838))
