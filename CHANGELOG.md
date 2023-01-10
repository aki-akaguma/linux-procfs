# Changelog: linux-procfs

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]


## [0.3.13] (2023-01-10)
### Added
* badges into `README.tpl`

### Changed
* reformat `CHANGELOG.md`

## [0.3.12] (2023-01-05)
### Changed
* update crates: naive_opt(0.1.23), memx(0.1.23)

## [0.3.11] (2022-06-13)
### Changed
* changes to edition 2021

## [0.3.10] (2021-11-14)
### Added
* add more documents

### Changed
* update crates: cfg-iif(0.2.2), memx(0.1.18), naive_opt(0.1.16)

## [0.3.9] (2021-09-11)
### Changed
* update crates: memx(0.1.17)

## [0.3.8] (2021-09-10)
### Changed
* update crates: memx(0.1.16), naive_opt(0.1.14), semver(1.0.4)

## [0.3.7] (2021-07-07)
### Changed
* update depends: cfg-iif(0.2.1), memx(0.1.14), naive_opt(0.1.13)
* modify .gitignore

## [0.3.6] (2021-07-01)
### Changed
* to github

## 0.3.5 (2021-07-01)
### Added
* add document

### Changed
* change a param of System::new()

## 0.3.4 (2021-06-26)
### Changed
* update depends: cfg-iif(0.2)

## 0.3.3 (2021-06-23)
### Added
* add util::skip_to_pos()

## 0.3.2 (2021-06-06)
### Changed
* update depends: naive_opt(0.1.11)

## 0.3.1 (2021-04-20)
### Changed
* change from twoway to naive_opt

## 0.3.0 (2021-04-02)
### Fixed
* some fix for clippy of rustc v1.51.0, a renaming struct.

## 0.2.1 (2020-11-18)
### Added
* add README.md, COPYING, LICENSE-APACHE, LICENSE-MIT

## 0.2.0 (2020-10-08)
### Added
* add pub fn get_pidentry_stat()

## 0.1.5 (2020-10-04)
### Added
* add has_meminfo_slab_reclaimable

## 0.1.4 (2020-06-08)
### Fixed
* bug: support linux 5.4.0 procfs

## 0.1.3 (2018-06-08)
### Added
* add support linux 5.4.0 procfs

## 0.1.2 (2020-05-10)
### Changed
* change edition 2015 to 2018
* update crates

## 0.1.1 (2019-04-14)
### Changed
* i revised that based on 'cargo clippy'

## 0.1.0 (2018-06-14)
* first commit

[Unreleased]: https://github.com/aki-akaguma/linux-procfs/compare/v0.3.13..HEAD
[0.3.13]: https://github.com/aki-akaguma/linux-procfs/compare/v0.3.12..v0.3.13
[0.3.12]: https://github.com/aki-akaguma/linux-procfs/compare/v0.3.11..v0.3.12
[0.3.11]: https://github.com/aki-akaguma/linux-procfs/compare/v0.3.10..v0.3.11
[0.3.10]: https://github.com/aki-akaguma/linux-procfs/compare/v0.3.9..v0.3.10
[0.3.9]: https://github.com/aki-akaguma/linux-procfs/compare/v0.3.8..v0.3.9
[0.3.8]: https://github.com/aki-akaguma/linux-procfs/compare/v0.3.7..v0.3.8
[0.3.7]: https://github.com/aki-akaguma/linux-procfs/compare/v0.3.6..v0.3.7
[0.3.6]: https://github.com/aki-akaguma/linux-procfs/releases/tag/v0.3.6
