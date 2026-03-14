# Changelog

## [0.2.0](https://github.com/open-feature/flagd-evaluator/compare/flagd-evaluator-go-v0.1.0...flagd-evaluator-go-v0.2.0) (2026-03-14)


### Features

* expose flag-set metadata in UpdateStateResponse across all providers ([#16](https://github.com/open-feature/flagd-evaluator/issues/16)) ([4eab93d](https://github.com/open-feature/flagd-evaluator/commit/4eab93d1dad1ec2f51516d2b26145c96bce700be))
* **go:** add Go package with wazero WASM runtime, instance pool, and optimized parsing ([#71](https://github.com/open-feature/flagd-evaluator/issues/71)) ([75a94f2](https://github.com/open-feature/flagd-evaluator/commit/75a94f2f4abe8e1a41c0dcd8222ec2d5622cfbb6))


### Bug Fixes

* **go:** make TestP99LatencyStabilityConcurrent reliable in CI ([#12](https://github.com/open-feature/flagd-evaluator/issues/12)) ([44fdd89](https://github.com/open-feature/flagd-evaluator/commit/44fdd89aadff0e30efcd1ffedabe8ad01c506e60))
* **wrappers:** add config size guard before alloc() in all language wrappers ([824d1ee](https://github.com/open-feature/flagd-evaluator/commit/824d1eead94e1bf15cdc0bcb514262348b971fbc))


### Performance Improvements

* **evaluation:** add context key filtering and index-based WASM evaluation ([#70](https://github.com/open-feature/flagd-evaluator/issues/70)) ([50d7cc0](https://github.com/open-feature/flagd-evaluator/commit/50d7cc059e1d8d0149c9fc7439e8d4158dd100e4))
* **java:** replace synchronized single instance with WASM instance pool ([#104](https://github.com/open-feature/flagd-evaluator/issues/104)) ([84fff31](https://github.com/open-feature/flagd-evaluator/commit/84fff31ca3f6a570f6fdf05e1bd8a88f07d00c68))
* **rust:** add E4 and E6 evaluation benchmarks ([#97](https://github.com/open-feature/flagd-evaluator/issues/97)) ([7638db9](https://github.com/open-feature/flagd-evaluator/commit/7638db9acac4480b1c494ea90a52ebac37f67f39))


### Tests

* **go:** add p99 latency stability tests for GC pressure detection ([#103](https://github.com/open-feature/flagd-evaluator/issues/103)) ([182fdad](https://github.com/open-feature/flagd-evaluator/commit/182fdad52ea3d3d23b11d0d37d76ce70bf804562))
