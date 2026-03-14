# Changelog

## [0.2.0](https://github.com/open-feature/flagd-evaluator/compare/flagd-evaluator-v0.1.0...flagd-evaluator-v0.2.0) (2026-03-14)


### Features

* add YAML flag configuration loading with schema validation ([#7](https://github.com/open-feature/flagd-evaluator/issues/7)) ([79b52cd](https://github.com/open-feature/flagd-evaluator/commit/79b52cd364ec89a8d7bc5eb281bf87309cb7e676))
* expose flag-set metadata in UpdateStateResponse across all providers ([#16](https://github.com/open-feature/flagd-evaluator/issues/16)) ([4eab93d](https://github.com/open-feature/flagd-evaluator/commit/4eab93d1dad1ec2f51516d2b26145c96bce700be))
* python native bindings ([#49](https://github.com/open-feature/flagd-evaluator/issues/49)) ([8d2baa3](https://github.com/open-feature/flagd-evaluator/commit/8d2baa38aba7a3487f5d36518d314b58b7cb4cb0))
* **python:** add WASM evaluator and 3-way comparison benchmarks ([#73](https://github.com/open-feature/flagd-evaluator/issues/73)) ([3ebed59](https://github.com/open-feature/flagd-evaluator/commit/3ebed597b09afd6fb8175e78c45f1b830ed3f5e6))
* rust based improvements ([#52](https://github.com/open-feature/flagd-evaluator/issues/52)) ([948adcd](https://github.com/open-feature/flagd-evaluator/commit/948adcd6300dceb971dc62643ea670ccc69eacfd))


### Bug Fixes

* improve python ([#51](https://github.com/open-feature/flagd-evaluator/issues/51)) ([004db1e](https://github.com/open-feature/flagd-evaluator/commit/004db1e0d672583da719ffaeed9d556e21ad4605))


### Performance Improvements

* cross-language concurrent comparison benchmarks ([#102](https://github.com/open-feature/flagd-evaluator/issues/102)) ([a4ea118](https://github.com/open-feature/flagd-evaluator/commit/a4ea118c1b2d5422951b975f7348df96af5ef595))
* **evaluation:** add context key filtering and index-based WASM evaluation ([#70](https://github.com/open-feature/flagd-evaluator/issues/70)) ([50d7cc0](https://github.com/open-feature/flagd-evaluator/commit/50d7cc059e1d8d0149c9fc7439e8d4158dd100e4))
* **java:** replace synchronized single instance with WASM instance pool ([#104](https://github.com/open-feature/flagd-evaluator/issues/104)) ([84fff31](https://github.com/open-feature/flagd-evaluator/commit/84fff31ca3f6a570f6fdf05e1bd8a88f07d00c68))
* **python:** add host-side optimizations for pre-evaluation, context filtering, and index-based eval ([#72](https://github.com/open-feature/flagd-evaluator/issues/72)) ([2b7a0b9](https://github.com/open-feature/flagd-evaluator/commit/2b7a0b9b30b297aefc8cdbd0fc4f96a2b133f56c))
* **python:** add O2 and O4 operator benchmarks ([#98](https://github.com/open-feature/flagd-evaluator/issues/98)) ([a78af74](https://github.com/open-feature/flagd-evaluator/commit/a78af7478fc36fda712760dd100ad411d149695f))
* **rust:** eliminate context cloning and merge duplicated evaluation paths ([#112](https://github.com/open-feature/flagd-evaluator/issues/112)) ([9770d4f](https://github.com/open-feature/flagd-evaluator/commit/9770d4f165a18b4504d6e5ae87baf54427cd432e))


### Miscellaneous

* safeguard all packages as pre-release ([#15](https://github.com/open-feature/flagd-evaluator/issues/15)) ([25c232d](https://github.com/open-feature/flagd-evaluator/commit/25c232dabed753a80a085f4f32de84a236d8e838))
