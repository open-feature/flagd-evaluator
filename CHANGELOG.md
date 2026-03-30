# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0](https://github.com/open-feature/flagd-evaluator/compare/v0.1.0...v0.2.0) (2026-03-30)


### Features

* Add permissions for pull request title validation ([d9a22eb](https://github.com/open-feature/flagd-evaluator/commit/d9a22eb111efe3e05a5b72fbd8ec42fc8a8e0146))
* add PR title validation for conventional commits ([#30](https://github.com/open-feature/flagd-evaluator/issues/30)) ([e496c9c](https://github.com/open-feature/flagd-evaluator/commit/e496c9c04c9dbbb97d83a6032e4d58728b95b9d2))
* add YAML flag configuration loading with schema validation ([#7](https://github.com/open-feature/flagd-evaluator/issues/7)) ([79b52cd](https://github.com/open-feature/flagd-evaluator/commit/79b52cd364ec89a8d7bc5eb281bf87309cb7e676))
* **docs:** add Mermaid diagrams for state update, evaluation, and memory flows ([#41](https://github.com/open-feature/flagd-evaluator/issues/41)) ([f2bb69d](https://github.com/open-feature/flagd-evaluator/commit/f2bb69d929ed5a91b23fa0e18fbe2d33870e99df))
* **dotnet:** add .NET WASM evaluator package ([#107](https://github.com/open-feature/flagd-evaluator/issues/107)) ([cb73a19](https://github.com/open-feature/flagd-evaluator/commit/cb73a19cf4c603cd47abaf8fed43ca5e99ef3a33))
* **evaluation:** pre-evaluate static flags for host-side caching ([#68](https://github.com/open-feature/flagd-evaluator/issues/68)) ([0867574](https://github.com/open-feature/flagd-evaluator/commit/0867574ca632d2b043d1542fdff5d6628b3de569))
* expose flag-set metadata in UpdateStateResponse across all providers ([#16](https://github.com/open-feature/flagd-evaluator/issues/16)) ([4eab93d](https://github.com/open-feature/flagd-evaluator/commit/4eab93d1dad1ec2f51516d2b26145c96bce700be))
* **fractional:** replace float bucketing with high-resolution integer arithmetic ([#18](https://github.com/open-feature/flagd-evaluator/issues/18)) ([67fff15](https://github.com/open-feature/flagd-evaluator/commit/67fff159c309cbd8532f8f58283475cbf87c5c32)), closes [#17](https://github.com/open-feature/flagd-evaluator/issues/17)
* **fractional:** support nested JSON Logic expressions as bucket variant names ([#31](https://github.com/open-feature/flagd-evaluator/issues/31)) ([3263743](https://github.com/open-feature/flagd-evaluator/commit/32637437e33ff5aee63222b55749e192cadda52c))
* **go:** add Go package with wazero WASM runtime, instance pool, and optimized parsing ([#71](https://github.com/open-feature/flagd-evaluator/issues/71)) ([75a94f2](https://github.com/open-feature/flagd-evaluator/commit/75a94f2f4abe8e1a41c0dcd8222ec2d5622cfbb6))
* **java:** implement flagd-api Evaluator interface in FlagEvaluator ([#5](https://github.com/open-feature/flagd-evaluator/issues/5)) ([63b7879](https://github.com/open-feature/flagd-evaluator/commit/63b7879ad8288093ad4c29c5d88b1428204b2c02))
* **js:** add JavaScript/TypeScript WASM evaluator package ([#110](https://github.com/open-feature/flagd-evaluator/issues/110)) ([42bce60](https://github.com/open-feature/flagd-evaluator/commit/42bce60ea1945e33e054f987ba6459a25b69d5e9))
* python native bindings ([#49](https://github.com/open-feature/flagd-evaluator/issues/49)) ([8d2baa3](https://github.com/open-feature/flagd-evaluator/commit/8d2baa38aba7a3487f5d36518d314b58b7cb4cb0))
* **python:** add WASM evaluator and 3-way comparison benchmarks ([#73](https://github.com/open-feature/flagd-evaluator/issues/73)) ([3ebed59](https://github.com/open-feature/flagd-evaluator/commit/3ebed597b09afd6fb8175e78c45f1b830ed3f5e6))
* rust based improvements ([#52](https://github.com/open-feature/flagd-evaluator/issues/52)) ([948adcd](https://github.com/open-feature/flagd-evaluator/commit/948adcd6300dceb971dc62643ea670ccc69eacfd))
* **storage:** detect and report changed flags in update_state ([#38](https://github.com/open-feature/flagd-evaluator/issues/38)) ([8d87e01](https://github.com/open-feature/flagd-evaluator/commit/8d87e018100f0a83b21bbf9a69d540a4f1d88b65))
* support metadata merging in flag evaluation responses ([#40](https://github.com/open-feature/flagd-evaluator/issues/40)) ([decafdb](https://github.com/open-feature/flagd-evaluator/commit/decafdb81a6a5ae88d22d059acaa43e9828045b3))


### Bug Fixes

* **core:** add resource limits and harden production WASM safety ([#26](https://github.com/open-feature/flagd-evaluator/issues/26)) ([b141bd1](https://github.com/open-feature/flagd-evaluator/commit/b141bd1c838ea6b4ce050ac91705adc00b67958f))
* Fix WASM module to load in Chicory by removing wasm-bindgen imports ([#10](https://github.com/open-feature/flagd-evaluator/issues/10)) ([87f8e2c](https://github.com/open-feature/flagd-evaluator/commit/87f8e2cda0e41de86b10590458c1b3b613f9622a))
* **go:** make TestP99LatencyStabilityConcurrent reliable in CI ([#12](https://github.com/open-feature/flagd-evaluator/issues/12)) ([44fdd89](https://github.com/open-feature/flagd-evaluator/commit/44fdd89aadff0e30efcd1ffedabe8ad01c506e60))
* improve python ([#51](https://github.com/open-feature/flagd-evaluator/issues/51)) ([004db1e](https://github.com/open-feature/flagd-evaluator/commit/004db1e0d672583da719ffaeed9d556e21ad4605))
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
* **python:** add O2 and O4 operator benchmarks ([#98](https://github.com/open-feature/flagd-evaluator/issues/98)) ([a78af74](https://github.com/open-feature/flagd-evaluator/commit/a78af7478fc36fda712760dd100ad411d149695f))
* **rust:** add E4 and E6 evaluation benchmarks ([#97](https://github.com/open-feature/flagd-evaluator/issues/97)) ([7638db9](https://github.com/open-feature/flagd-evaluator/commit/7638db9acac4480b1c494ea90a52ebac37f67f39))
* **rust:** add scale benchmarks S6-S8, S10-S11 for large flag stores ([#93](https://github.com/open-feature/flagd-evaluator/issues/93)) ([044f4c3](https://github.com/open-feature/flagd-evaluator/commit/044f4c3f5be981e0e9342d90b345d11fde7c15a6))
* **rust:** eliminate context cloning and merge duplicated evaluation paths ([#112](https://github.com/open-feature/flagd-evaluator/issues/112)) ([9770d4f](https://github.com/open-feature/flagd-evaluator/commit/9770d4f165a18b4504d6e5ae87baf54427cd432e))


### Documentation

* add standardized benchmark matrix for cross-language comparison ([#67](https://github.com/open-feature/flagd-evaluator/issues/67)) ([aa3308c](https://github.com/open-feature/flagd-evaluator/commit/aa3308c68b68416885d6e94b09222de31b059133))
* align copilot-instructions with CLAUDE.md ([#4](https://github.com/open-feature/flagd-evaluator/issues/4)) ([47209a7](https://github.com/open-feature/flagd-evaluator/commit/47209a78d3f99707235bca21f34dfdbf9a862e22))
* clarify starts_with/ends_with are datalogic-rs built-ins, not custom operators ([#11](https://github.com/open-feature/flagd-evaluator/issues/11)) ([e6e6624](https://github.com/open-feature/flagd-evaluator/commit/e6e662479f4d7c02579a8a7ec6b213817c28976a))
* document WASM context serialization optimizations and updated benchmarks ([#69](https://github.com/open-feature/flagd-evaluator/issues/69)) ([53f2cb7](https://github.com/open-feature/flagd-evaluator/commit/53f2cb724bbc531eccf33245b691625f79e11334))
* restructure CLAUDE.md, add ARCHITECTURE.md, extend BENCHMARKS.md ([09b5bd4](https://github.com/open-feature/flagd-evaluator/commit/09b5bd489881f95c710f97d64dbdd85478bfd154))
* rewrite README.md as clean entry point ([#109](https://github.com/open-feature/flagd-evaluator/issues/109)) ([46e25ef](https://github.com/open-feature/flagd-evaluator/commit/46e25efdcfe1aeb30654093f24d6b372bc3220eb))
* update README to reflect instance-based API refactoring ([#53](https://github.com/open-feature/flagd-evaluator/issues/53)) ([f0fe238](https://github.com/open-feature/flagd-evaluator/commit/f0fe238f5c28bf9191c55aa318bae4a4f5f1aa96))


### Miscellaneous

* **deps:** update dependency @types/node to v20.19.37 ([#32](https://github.com/open-feature/flagd-evaluator/issues/32)) ([27e22dd](https://github.com/open-feature/flagd-evaluator/commit/27e22ddb5dcb2cfc5c72039d8b741d3b2cccc71f))
* **deps:** update dependency benchmarkdotnet to 0.15.8 ([#38](https://github.com/open-feature/flagd-evaluator/issues/38)) ([4b22135](https://github.com/open-feature/flagd-evaluator/commit/4b221356f604e2795018fd65cb64f4b33a28bf03))
* **deps:** update dependency json-logic-engine to v4.0.6 ([#33](https://github.com/open-feature/flagd-evaluator/issues/33)) ([c0531b2](https://github.com/open-feature/flagd-evaluator/commit/c0531b26e1c0792bf6aab1a1ec2da0f75efff3ed))
* **deps:** update dependency jsonlogic to 5.5.0 ([#40](https://github.com/open-feature/flagd-evaluator/issues/40)) ([41c5227](https://github.com/open-feature/flagd-evaluator/commit/41c5227646c36418097f6bfba6cb9ea4897ab085))
* **deps:** update dependency maven to v3.9.14 ([#35](https://github.com/open-feature/flagd-evaluator/issues/35)) ([3ab86ee](https://github.com/open-feature/flagd-evaluator/commit/3ab86eebaa22bee23cdf9d45a4eefa7a1c4dc004))
* **deps:** update dependency microsoft.net.test.sdk to 17.14.1 ([#41](https://github.com/open-feature/flagd-evaluator/issues/41)) ([17f30eb](https://github.com/open-feature/flagd-evaluator/commit/17f30eb64f86bc30a8540eed41a513b79c0a1b11))
* **deps:** update dependency org.apache.maven.plugins:maven-compiler-plugin to v3.15.0 ([#42](https://github.com/open-feature/flagd-evaluator/issues/42)) ([92f536a](https://github.com/open-feature/flagd-evaluator/commit/92f536af48e84a733aa4e90131587ffe4ca4c109))
* **deps:** update dependency org.apache.maven.plugins:maven-resources-plugin to v3.5.0 ([#43](https://github.com/open-feature/flagd-evaluator/issues/43)) ([aa9b7be](https://github.com/open-feature/flagd-evaluator/commit/aa9b7be989a5b749fade259dd25b38850a4ca3bc))
* **deps:** update dependency org.apache.maven.plugins:maven-shade-plugin to v3.6.2 ([#47](https://github.com/open-feature/flagd-evaluator/issues/47)) ([f10a84f](https://github.com/open-feature/flagd-evaluator/commit/f10a84ffff5c6615e11b5f5ee3d7a7673667ff28))
* **deps:** update dependency org.apache.maven.plugins:maven-surefire-plugin to v3.5.5 ([#36](https://github.com/open-feature/flagd-evaluator/issues/36)) ([0c6e8f7](https://github.com/open-feature/flagd-evaluator/commit/0c6e8f7b51e34cedf400976eb736479e8c997350))
* **deps:** update dependency org.assertj:assertj-core to v3.27.7 ([#37](https://github.com/open-feature/flagd-evaluator/issues/37)) ([76ff945](https://github.com/open-feature/flagd-evaluator/commit/76ff94524550cd25f74cf6b6272bf326d556edbc))
* **deps:** update dependency org.codehaus.mojo:exec-maven-plugin to v3.6.3 ([#48](https://github.com/open-feature/flagd-evaluator/issues/48)) ([21c43d9](https://github.com/open-feature/flagd-evaluator/commit/21c43d9b462afcfb00552630ad9bdbc50a989cc6))
* **deps:** update dependency org.junit.jupiter:junit-jupiter to v5.14.3 ([#49](https://github.com/open-feature/flagd-evaluator/issues/49)) ([4603fb4](https://github.com/open-feature/flagd-evaluator/commit/4603fb462c1360245716df19cf1ab3df1d66bdeb))
* **deps:** update rust crate criterion to 0.8 ([#50](https://github.com/open-feature/flagd-evaluator/issues/50)) ([b31f77f](https://github.com/open-feature/flagd-evaluator/commit/b31f77fc615b37906c1b5928db60bab60295f1b5))
* **deps:** update rust crate wasm-bindgen-test to v0.3.65 ([#45](https://github.com/open-feature/flagd-evaluator/issues/45)) ([8c912f1](https://github.com/open-feature/flagd-evaluator/commit/8c912f12b0d1812819412f20a60b0ab0a9346d7d))
* improvements ([#44](https://github.com/open-feature/flagd-evaluator/issues/44)) ([cb7843a](https://github.com/open-feature/flagd-evaluator/commit/cb7843af0cc9ae5338f36eb9694f0d1a5869e3b7))
* safeguard all packages as pre-release ([#15](https://github.com/open-feature/flagd-evaluator/issues/15)) ([25c232d](https://github.com/open-feature/flagd-evaluator/commit/25c232dabed753a80a085f4f32de84a236d8e838))


### Code Refactoring

* changing evaluation lib etc ([#45](https://github.com/open-feature/flagd-evaluator/issues/45)) ([124017e](https://github.com/open-feature/flagd-evaluator/commit/124017ea45923aa4dcb8e11ae47f6aa90d9542a9))
* improve architecture and add edge case tests ([#54](https://github.com/open-feature/flagd-evaluator/issues/54)) ([c192a4f](https://github.com/open-feature/flagd-evaluator/commit/c192a4f402079463ecfee6bd60c64d310570260d))
* **java:** dynamically match wasm-bindgen host functions by prefix ([#96](https://github.com/open-feature/flagd-evaluator/issues/96)) ([6c531e5](https://github.com/open-feature/flagd-evaluator/commit/6c531e52c6fcbf9e23a240817c35275b80ee1695))


### Tests

* **go:** add p99 latency stability tests for GC pressure detection ([#103](https://github.com/open-feature/flagd-evaluator/issues/103)) ([182fdad](https://github.com/open-feature/flagd-evaluator/commit/182fdad52ea3d3d23b11d0d37d76ce70bf804562))


### Continuous Integration

* add required CI gate job for branch protection ([#6](https://github.com/open-feature/flagd-evaluator/issues/6)) ([4e2034a](https://github.com/open-feature/flagd-evaluator/commit/4e2034a090cba20edfa864991300a63a61e089ab))
* distribute WASM binary to all language packages on release ([#111](https://github.com/open-feature/flagd-evaluator/issues/111)) ([cb7d304](https://github.com/open-feature/flagd-evaluator/commit/cb7d30440c8a1eb0726c1124e94a2ed479937088))
* Setup Release Please for automated releases and changelog generation ([#7](https://github.com/open-feature/flagd-evaluator/issues/7)) ([375433c](https://github.com/open-feature/flagd-evaluator/commit/375433c9ff1259031e2893ad2dc187a3d719eb56))

## [Unreleased]

### Features

- Initial release of flagd-evaluator
- WebAssembly module for JSON Logic evaluation
- Support for fractional operator
- CLI tool for testing and development

[Unreleased]: https://github.com/open-feature-forking/flagd-evaluator/compare/v0.1.0...HEAD
