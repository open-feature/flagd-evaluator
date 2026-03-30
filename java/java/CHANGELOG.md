# Changelog

## [0.2.0](https://github.com/open-feature/flagd-evaluator/compare/flagd-evaluator-java-v0.1.0-SNAPSHOT...flagd-evaluator-java-v0.2.0) (2026-03-30)


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


### Miscellaneous

* **deps:** update dependency maven to v3.9.14 ([#35](https://github.com/open-feature/flagd-evaluator/issues/35)) ([3ab86ee](https://github.com/open-feature/flagd-evaluator/commit/3ab86eebaa22bee23cdf9d45a4eefa7a1c4dc004))
* **deps:** update dependency org.apache.maven.plugins:maven-compiler-plugin to v3.15.0 ([#42](https://github.com/open-feature/flagd-evaluator/issues/42)) ([92f536a](https://github.com/open-feature/flagd-evaluator/commit/92f536af48e84a733aa4e90131587ffe4ca4c109))
* **deps:** update dependency org.apache.maven.plugins:maven-resources-plugin to v3.5.0 ([#43](https://github.com/open-feature/flagd-evaluator/issues/43)) ([aa9b7be](https://github.com/open-feature/flagd-evaluator/commit/aa9b7be989a5b749fade259dd25b38850a4ca3bc))
* **deps:** update dependency org.apache.maven.plugins:maven-shade-plugin to v3.6.2 ([#47](https://github.com/open-feature/flagd-evaluator/issues/47)) ([f10a84f](https://github.com/open-feature/flagd-evaluator/commit/f10a84ffff5c6615e11b5f5ee3d7a7673667ff28))
* **deps:** update dependency org.apache.maven.plugins:maven-surefire-plugin to v3.5.5 ([#36](https://github.com/open-feature/flagd-evaluator/issues/36)) ([0c6e8f7](https://github.com/open-feature/flagd-evaluator/commit/0c6e8f7b51e34cedf400976eb736479e8c997350))
* **deps:** update dependency org.assertj:assertj-core to v3.27.7 ([#37](https://github.com/open-feature/flagd-evaluator/issues/37)) ([76ff945](https://github.com/open-feature/flagd-evaluator/commit/76ff94524550cd25f74cf6b6272bf326d556edbc))
* **deps:** update dependency org.codehaus.mojo:exec-maven-plugin to v3.6.3 ([#48](https://github.com/open-feature/flagd-evaluator/issues/48)) ([21c43d9](https://github.com/open-feature/flagd-evaluator/commit/21c43d9b462afcfb00552630ad9bdbc50a989cc6))
* **deps:** update dependency org.junit.jupiter:junit-jupiter to v5.14.3 ([#49](https://github.com/open-feature/flagd-evaluator/issues/49)) ([4603fb4](https://github.com/open-feature/flagd-evaluator/commit/4603fb462c1360245716df19cf1ab3df1d66bdeb))


### Code Refactoring

* **java:** dynamically match wasm-bindgen host functions by prefix ([#96](https://github.com/open-feature/flagd-evaluator/issues/96)) ([6c531e5](https://github.com/open-feature/flagd-evaluator/commit/6c531e52c6fcbf9e23a240817c35275b80ee1695))
