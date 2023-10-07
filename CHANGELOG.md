# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.7.0] - unreleased
### Added
- `RuleGroup::limit()`
- `RuleGroup::last_evaluation()`
- `RuleGroup::evaluation_time()`
- `AlertingRule::last_evaluation()`
- `AlertingRule::evaluation_time()`
- `RecordingRule::last_evaluation()`
- `RecordingRule::evaluation_time()`
- `Rule::as_recording()`
- `Rule::as_alerting()`
- `impl Eq for RuleHealth`
- `RuleHealth::is_good()`
- `RuleHealth::is_bad()`
- `RuleHealth::is_unknown()`
- `impl Eq for AlertState`
- `AlertState::is_inactive()`
- `AlertState::is_pending()`
- `AlertState::is_firing()`
- `impl Eq for TargetHealth`
- `TargetHealth::is_up()`
- `TargetHealth::is_down()`
- `TargetHealth::is_unknown()`
- `impl Eq for MetricType`
- `MetricType::is_counter()`
- `MetricType::is_gauge()`
- `MetricType::is_histogram()`
- `MetricType::is_gauge_histogram()`
- `MetricType::is_summary()`
- `MetricType::is_info()`
- `MetricType::is_stateset()`
- `MetricType::is_unknown()`
- `AlertingRule::keep_firing_for()`

### Changed
- `Alert::value()` now returns f64.
- `Client::series()` and `direct::series()` now accept any iterable container of `Selector`s as the first argument.
- `Client::label_names()` and `direct::label_names()` now accept any iterable container of `Selector`s as the first argument. Also the first argument is not longer optional. Just pass an empty slice instead of `None` if you do not want to filter by series.
- Refactored deserialization of Prometheus server responses so the explicit dependency on `serde_json` could be removed.
- Refactored the `error` module and some custom errors related to deserialization using `serde::de::Error`. The `Error` enum inside the `error` module now contains one variant less and existing error variants were improved by _properly_ implementing `std::error::Error::source()`. Libraries like `anyhow` are now able to display more detailed error messages. This change is not breaking if you did not match on specific error enum variants before.
- `Client` methods now return a more concise error if the server response (from Prometheus or an intermediate proxy) does not contain a `Content-Type` header identifying the payload as JSON, that is the media type from the response is not as expected.

## [0.6.7] - 2023-09-30
### Fixed
- Fixes an issue where a Prometheus server sends an HTTP response with `Content-Type: application/json; charset=utf-8` (see [issue](https://github.com/puetzp/prometheus-http-query/issues/7))

## [0.6.6] - 2023-04-17
### Added
- `InstantQueryBuilder::header` and `RangeQueryBuilder::header` (see also [PR #6](https://github.com/puetzp/prometheus-http-query/pull/6#issue-1667934427), thank you @lasantosr)

## [0.6.5] - 2023-01-27
### Added
- Added several feature flags pertaining to the client TLS configuration that match the feature flags of `reqwest` by the same name.

## [0.6.3] - 2022-11-20
### Added
- `Client::is_server_healthy`
- `Client::is_server_ready`

### Changed
- Update crate `url` to v2.3

## [0.6.2] - 2022-08-23
### Added
- `prometheus_http_query::error::ApiErrorType` that corresponds to the error variants that are used within the Prometheus API code.

### Changed
- `prometheus_http_query::error::ApiError` is now part of the public crate API and provides methods to inspect the cause of the error, e.g. `ApiError::error_type`, `ApiError::is_timeout` and `ApiError::message`.

### Fixed
- `Client` methods now return the proper error variant when e.g. a reverse proxy acts as intermediary. This considers the following three cases:
	- Prometheus returns 2xx with JSON body -> JSON is parsed and function returns the result in a `Result::Ok`.
	- Prometheus returns 4xx or 5xx -> JSON is parsed and function returns the `ApiError` containing the error details within `Result::Err`.
	- A proxy cannot handle the request and responds with a 4xx or 5xx itself -> error is raised and function returns the wrapped `reqwest::Error` within `Result::Err`.

## [0.6.1] - 2022-08-22
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

