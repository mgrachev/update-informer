# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog],
and this project adheres to [Semantic Versioning].

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html

## [Unreleased]

### 🚀 Added

- Add cargo-sort ([#51](https://github.com/mgrachev/update-informer/pull/51))
- Add list of users
- Add `dprint` to check formatting ([#42](https://github.com/mgrachev/update-informer/pull/42))
- Add action to check links + ci optimization ([#38](https://github.com/mgrachev/update-informer/pull/38))

### ⚙️ Changed

- Generate a changelog and update release process
- Bump lycheeverse/lychee-action from 1.5.0 to 1.5.1 ([#55](https://github.com/mgrachev/update-informer/pull/55))
- Bump Swatinem/rust-cache from 1.4.0 to 2.0.0 ([#53](https://github.com/mgrachev/update-informer/pull/53))
- Bump dprint/check from 2.0 to 2.1 ([#54](https://github.com/mgrachev/update-informer/pull/54))
- Bump wagoid/commitlint-github-action from 5.0.1 to 5.0.2 ([#52](https://github.com/mgrachev/update-informer/pull/52))
- Bump wagoid/commitlint-github-action from 4.1.11 to 5.0.1 ([#50](https://github.com/mgrachev/update-informer/pull/50))
- Bump lycheeverse/lychee-action from 1.4.1 to 1.5.0 ([#48](https://github.com/mgrachev/update-informer/pull/48))
- Bump actions/stale from 4 to 5 ([#45](https://github.com/mgrachev/update-informer/pull/45))
- Bump Swatinem/rust-cache from 1.3.0 to 1.4.0 ([#44](https://github.com/mgrachev/update-informer/pull/44))
- Bump wagoid/commitlint-github-action from 4.1.10 to 4.1.11 ([#43](https://github.com/mgrachev/update-informer/pull/43))
- Bump codecov/codecov-action from 2.1.0 to 3 ([#41](https://github.com/mgrachev/update-informer/pull/41))
- Bump actions-rs/toolchain from 1.0.6 to 1.0.7 ([#39](https://github.com/mgrachev/update-informer/pull/39))
- Bump wagoid/commitlint-github-action from 4.1.9 to 4.1.10 ([#40](https://github.com/mgrachev/update-informer/pull/40))

## [v0.5.0](https://github.com/mgrachev/update-informer/releases/tag/v0.5.0) - 2022-03-24

### 🚀 Added

- Add ability to implement your own registry ([#37](https://github.com/mgrachev/update-informer/pull/37))
- Add `stale` action ([#33](https://github.com/mgrachev/update-informer/pull/33))
- Add dependabot ([#28](https://github.com/mgrachev/update-informer/pull/28))

### ⚙️ Changed

- Bump actions/cache from 2 to 3 ([#36](https://github.com/mgrachev/update-informer/pull/36))
- Update `stale` action schedule
- Update mockito requirement from 0.30.0 to 0.31.0 ([#32](https://github.com/mgrachev/update-informer/pull/32))
- Bump actions/checkout from 1 to 3 ([#31](https://github.com/mgrachev/update-informer/pull/31))
- Bump wagoid/commitlint-github-action from 2 to 4.1.9 ([#29](https://github.com/mgrachev/update-informer/pull/29))
- Bump codecov/codecov-action from 1 to 2.1.0 ([#30](https://github.com/mgrachev/update-informer/pull/30))
- Update dependabot config
- Update `stale` schedule
- Update `stale` schedule ([#35](https://github.com/mgrachev/update-informer/pull/35))
- Update `stale` action schedule ([#34](https://github.com/mgrachev/update-informer/pull/34))

## [v0.4.0](https://github.com/mgrachev/update-informer/releases/tag/v0.4.0) - 2022-02-21

### 🚀 Added

- Add ability to not use cache files ([#27](https://github.com/mgrachev/update-informer/pull/27))

## [v0.3.0](https://github.com/mgrachev/update-informer/releases/tag/v0.3.0) - 2022-02-19

### 🚀 Added

- Add cargo features ([#26](https://github.com/mgrachev/update-informer/pull/26))
- Add configurable request timeout and interval ([#24](https://github.com/mgrachev/update-informer/pull/24))
- Add open collective
- Add more examples ([#23](https://github.com/mgrachev/update-informer/pull/23))
- Add logo ([#19](https://github.com/mgrachev/update-informer/pull/19))

### ⚙️ Changed

- Better cache directory naming scheme ([#21](https://github.com/mgrachev/update-informer/pull/21))
- PyPI support ([#16](https://github.com/mgrachev/update-informer/pull/16))
- Update readme
- Set up code coverage ([#15](https://github.com/mgrachev/update-informer/pull/15))

## [v0.2.0](https://github.com/mgrachev/update-informer/releases/tag/v0.2.0) - 2022-01-05

### 🚀 Added

- Add `UpdateInformer` and `FakeUpdateInformer` structs for convenient use ([#14](https://github.com/mgrachev/update-informer/pull/14))

## [v0.1.0](https://github.com/mgrachev/update-informer/releases/tag/v0.1.0) - 2021-12-30

### 🚀 Added

- Add `stub_check_version` function and update docs ([#13](https://github.com/mgrachev/update-informer/pull/13))
- Add documentation and update examples ([#12](https://github.com/mgrachev/update-informer/pull/12))
- Add tests for registries: crates.io and github ([#9](https://github.com/mgrachev/update-informer/pull/9))
- Add main files

### ⚙️ Changed

- Save latest version to file and add interval check ([#11](https://github.com/mgrachev/update-informer/pull/11))
- Set up ci/cd ([#10](https://github.com/mgrachev/update-informer/pull/10))
- Check updates on github ([#8](https://github.com/mgrachev/update-informer/pull/8))
- Check version on crates.io ([#1](https://github.com/mgrachev/update-informer/pull/1))
- Initial commit
