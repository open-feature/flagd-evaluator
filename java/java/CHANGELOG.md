# Changelog

## [0.2.0](https://github.com/open-feature/flagd-evaluator/compare/flagd-evaluator-java-v0.1.0-SNAPSHOT...flagd-evaluator-java-v0.2.0) (2026-03-14)


### Features

* add YAML flag configuration loading with schema validation ([#7](https://github.com/open-feature/flagd-evaluator/issues/7)) ([79b52cd](https://github.com/open-feature/flagd-evaluator/commit/79b52cd364ec89a8d7bc5eb281bf87309cb7e676))
* **evaluation:** pre-evaluate static flags for host-side caching ([#68](https://github.com/open-feature/flagd-evaluator/issues/68)) ([0867574](https://github.com/open-feature/flagd-evaluator/commit/0867574ca632d2b043d1542fdff5d6628b3de569))
* expose flag-set metadata in UpdateStateResponse across all providers ([#16](https://github.com/open-feature/flagd-evaluator/issues/16)) ([4eab93d](https://github.com/open-feature/flagd-evaluator/commit/4eab93d1dad1ec2f51516d2b26145c96bce700be))
* **java:** implement flagd-api Evaluator interface in FlagEvaluator ([#5](https://github.com/open-feature/flagd-evaluator/issues/5)) ([63b7879](https://github.com/open-feature/flagd-evaluator/commit/63b7879ad8288093ad4c29c5d88b1428204b2c02))


### Bug Fixes

* **wrappers:** add config size guard before alloc() in all language wrappers ([824d1ee](https://github.com/open-feature/flagd-evaluator/commit/824d1eead94e1bf15cdc0bcb514262348b971fbc))


### Performance Improvements

* add C7-C10 high-concurrency benchmarks (16 threads) ([#99](https://github.com/open-feature/flagd-evaluator/issues/99)) ([9e85110](https://github.com/open-feature/flagd-evaluator/commit/9e85110496d1c45df26c653c09e95ab416e29099))
* cross-language concurrent comparison benchmarks ([#102](https://github.com/open-feature/flagd-evaluator/issues/102)) ([a4ea118](https://github.com/open-feature/flagd-evaluator/commit/a4ea118c1b2d5422951b975f7348df96af5ef595))
* **evaluation:** add context key filtering and index-based WASM evaluation ([#70](https://github.com/open-feature/flagd-evaluator/issues/70)) ([50d7cc0](https://github.com/open-feature/flagd-evaluator/commit/50d7cc059e1d8d0149c9fc7439e8d4158dd100e4))
* **java:** add concurrency benchmarks C1-C6 ([#94](https://github.com/open-feature/flagd-evaluator/issues/94)) ([23b47f4](https://github.com/open-feature/flagd-evaluator/commit/23b47f430f51342731dec145d0194b4c2c32e6fa))
* **java:** add custom operator benchmarks O1-O6 ([#91](https://github.com/open-feature/flagd-evaluator/issues/91)) ([eb820f3](https://github.com/open-feature/flagd-evaluator/commit/eb820f32313f399fca3bff3bcccf8f1bc6e6ea60))
* **java:** add evaluation benchmarks E3, E6, E7, E10, E11 ([#92](https://github.com/open-feature/flagd-evaluator/issues/92)) ([033f287](https://github.com/open-feature/flagd-evaluator/commit/033f2878060422a026a4575277108e9993998fc3))
* **java:** replace synchronized single instance with WASM instance pool ([#104](https://github.com/open-feature/flagd-evaluator/issues/104)) ([84fff31](https://github.com/open-feature/flagd-evaluator/commit/84fff31ca3f6a570f6fdf05e1bd8a88f07d00c68))
* **python:** add host-side optimizations for pre-evaluation, context filtering, and index-based eval ([#72](https://github.com/open-feature/flagd-evaluator/issues/72)) ([2b7a0b9](https://github.com/open-feature/flagd-evaluator/commit/2b7a0b9b30b297aefc8cdbd0fc4f96a2b133f56c))


### Documentation

* document WASM context serialization optimizations and updated benchmarks ([#69](https://github.com/open-feature/flagd-evaluator/issues/69)) ([53f2cb7](https://github.com/open-feature/flagd-evaluator/commit/53f2cb724bbc531eccf33245b691625f79e11334))


### Code Refactoring

* **java:** dynamically match wasm-bindgen host functions by prefix ([#96](https://github.com/open-feature/flagd-evaluator/issues/96)) ([6c531e5](https://github.com/open-feature/flagd-evaluator/commit/6c531e52c6fcbf9e23a240817c35275b80ee1695))
