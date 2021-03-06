# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog],
and this project adheres to [Semantic Versioning].

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html

## [Unreleased]

### 🚀 Added

- Add `cargo-sort` to CI [#51]
- Add `dprint` to check formatting [#42]
- Add GitHub action to check links + CI optimization [#38]

### ⚙️ Changed

### 🛠 Fixed

[#51]: https://github.com/mgrachev/update-informer/pull/51
[#42]: https://github.com/mgrachev/update-informer/pull/42
[#38]: https://github.com/mgrachev/update-informer/pull/38

## [v0.5.0] - 2022-03-24

### 🚀 Added

- Add ability to implement your own registry to check updates [#37]
- Add `stale` action [#33]
- Add dependabot [#28]

[#37]: https://github.com/mgrachev/update-informer/pull/37
[#33]: https://github.com/mgrachev/update-informer/pull/33
[#28]: https://github.com/mgrachev/update-informer/pull/28

## [v0.4.0] - 2022-02-21

### 🚀 Added

- Add ability to not use cache files [#27]

[#27]: https://github.com/mgrachev/update-informer/pull/27

## [v0.3.0] - 2022-02-19

### 🚀 Added

- Add cargo features [#26]
- Add configurable request timeout and interval [#24]
- Add PyPI support [#16] ([@itamarst](https://github.com/itamarst))
- Add logo [#19]
- Set up code coverage [#15]

### ⚙️ Changed

- Add more examples [#23]
- Use better cache directory naming scheme [#21] ([@itamarst](https://github.com/itamarst))

[#26]: https://github.com/mgrachev/update-informer/pull/26
[#24]: https://github.com/mgrachev/update-informer/pull/24
[#23]: https://github.com/mgrachev/update-informer/pull/23
[#21]: https://github.com/mgrachev/update-informer/pull/21
[#19]: https://github.com/mgrachev/update-informer/pull/19
[#16]: https://github.com/mgrachev/update-informer/pull/16
[#15]: https://github.com/mgrachev/update-informer/pull/15

## [v0.2.0] - 2022-01-05

### 🚀 Added

- Add `UpdateInformer` and `FakeUpdateInformer` structs for convenient use [#14]

[#14]: https://github.com/mgrachev/update-informer/pull/14

## [v0.1.0] - 2021-12-30

### 🚀 Added

- Add `stub_check_version` function and update docs [#13]
- Add documentation and update examples [#12]
- Save latest version to file and add interval check [#11]
- Set up CI/CD [#10]
- Add tests for registries: Crates.io and GitHub [#9]
- Check updates on GitHub [#8]
- Check updates on Crates.io [#1]

[#13]: https://github.com/mgrachev/update-informer/pull/13
[#12]: https://github.com/mgrachev/update-informer/pull/12
[#11]: https://github.com/mgrachev/update-informer/pull/11
[#10]: https://github.com/mgrachev/update-informer/pull/10
[#9]: https://github.com/mgrachev/update-informer/pull/9
[#8]: https://github.com/mgrachev/update-informer/pull/8
[#1]: https://github.com/mgrachev/update-informer/pull/1
[v0.5.0]: https://github.com/mgrachev/update-informer/releases/tag/v0.5.0
[v0.4.0]: https://github.com/mgrachev/update-informer/releases/tag/v0.4.0
[v0.3.0]: https://github.com/mgrachev/update-informer/releases/tag/v0.3.0
[v0.2.0]: https://github.com/mgrachev/update-informer/releases/tag/v0.2.0
[v0.1.0]: https://github.com/mgrachev/update-informer/releases/tag/v0.1.0
