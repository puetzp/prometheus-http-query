# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.1] - unreleased
### Fixed
- All `Client` methods now properly return `prometheus_http_query::error::ApiError` when Prometheus responds with an HTTP 4xx and the error type and message are captured.

### Changed
- Simplified JSON body parsing with serde (no API changes).

## [0.6.0] - 2022-08-01
### Changed
- `Client::query` and `Client::query_range` now return builder types to add parameters to a request (e.g. `timeout`) before sending the request via `InstantQueryBuilder::get` or `InstantQueryBuilder::post` and their pendants in `RangeQueryBuilder`. The purpose is to reduce the number of parameters in e.g. `Client::query` and configure a request via a builder instead as the number of parameters will certainly increase in the future.
- `InstantQueryBuilder::get`, `InstantQueryBuilder::post` and their `RangeQueryBuilder` pendants still return a `Result<PromqlResult, _>`, but the Promqlresult now contains `data` and `stats`. The former being the familiar collection of vectors or matrices, the latter the execution stats as returned by the HTTP API.
- `Client::base_url` now returns `Url`
- Move and rename `PromqlResult::as_instant` -> `Data::as_vector`
- Move and rename `PromqlResult::as_range` -> `Data::as_matrix`
- Move `PromqlResult::as_scalar` -> `Data::as_scalar`

### Added
- `InstantQueryBuilder` and `RangeQueryBuilder`. Each allows to set additional optional parameters to a request (e.g. a `timeout`). Request can then be sent using HTTP GET (see `InstantQueryBuilder::get`) or POST (see `InstantQueryBuilder::post`).
- `InstantQueryBuilder::stats` and `RangeQueryBuilder::stats` as the first _new_ query parameter/builder method. Use it to request query execution stats from Prometheus as part of the `PromqlResult` when `get` or `post` are executed. See [this pull request](https://github.com/prometheus/prometheus/pull/10369) for details and background information.
- `PromqlResult::data`
- `PromqlResult::stats`

