# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Changed
- `Client::query` and `Client::query_range` now return builder types to add parameters to a request (e.g. `timeout`) before sending the request via `InstantQueryBuilder::get` or `InstantQueryBuilder::post` and their pendants in `RangeQueryBuilder`. The purpose is to reduce the number of parameters in e.g. `Client::query` and configure a request via a builder instead as the number of parameters will certainly increase in the future.
- `InstantQueryBuilder::get`, `InstantQueryBuilder::post` and their `RangeQueryBuilder` pendants still return a `Result<PromqlResult, _>`, but the Promqlresult now contains `data` and `stats`. The former being the familiar collection of vectors or matrices, the latter the execution stats as returned by the HTTP API.
- `Client::base_url` now returns `Url`

### Added
- `InstantQueryBuilder` and `RangeQueryBuilder`. Each allows to set additional optional parameters to a request (e.g. a `timeout`). Request can then be sent using HTTP GET (see `InstantQueryBuilder::get`) or POST (see `InstantQueryBuilder::post`).
- `InstantQueryBuilder::stats` and `RangeQueryBuilder::stats` as the first _new_ query parameter/builder method. Use it to request query execution stats from Prometheus as part of the `PromqlResult` when `get` or `post` are executed. See [this pull request](https://github.com/prometheus/prometheus/pull/10369) for details and background information.
