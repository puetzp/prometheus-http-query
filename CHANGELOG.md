# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Changed
- `Client::query` and `Client::query_range` now return builder types to add parameters to a request (e.g. `timeout`) before sending the request via `InstantQueryBuilder::get` or `InstantQueryBuilder::post` and their pendants in `RangeQueryBuilder`. The purpose is to reduce the number of parameters in e.g. `Client::query` and configure a request via a builder instead as the number of parameters will certainly increase in the future (think `stats` etc.)
- `Client::base_url` now returns `Url`

### Added
- `InstantQueryBuilder` and `RangeQueryBuilder`. Each allows to set additional optional parameters to a request (e.g. a `timeout`). Request can then be sent using HTTP GET (see `InstantQueryBuilder::get`) or POST (see `InstantQueryBuilder::post`).
