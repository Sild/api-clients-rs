# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- add smooth rate limiting to the core executor builder with a default of 10 RPS;
  `max_rps = 0` waits without sending requests

## [0.2.1](https://github.com/Sild/api_clients_rs/compare/api_clients_core-v0.2.0...api_clients_core-v0.2.1) - 2025-11-05

### Other

- support timeout

## [0.2.0](https://github.com/Sild/api_clients_rs/compare/api_clients_core-v0.1.1...api_clients_core-v0.2.0) - 2025-10-29

### Other

- restruct clients, update errors

## [0.1.1](https://github.com/Sild/api_clients_rs/compare/api_clients_core-v0.1.0...api_clients_core-v0.1.1) - 2025-10-15

### Other

- allow custom http_client for executor

## [0.0.5](https://github.com/Sild/api_clients_rs/compare/api_clients_core-v0.0.4...api_clients_core-v0.0.5) - 2025-10-12

### Other

- rewrite clients using builder, add tonco client
- add tonco_api_client

## [0.0.4](https://github.com/Sild/api_clients_rs/compare/api_clients_core-v0.0.3...api_clients_core-v0.0.4) - 2025-09-05

### Fixed

- fix types

## [0.0.3](https://github.com/Sild/api_clients_rs/compare/api_clients_core-v0.0.2...api_clients_core-v0.0.3) - 2025-08-19

### Other

- update Cargo.toml dependencies

## [0.0.2](https://github.com/Sild/api_clients_rs/compare/api_clients_core-v0.0.1...api_clients_core-v0.0.2) - 2025-08-11

### Other

- support assets_query

## [0.0.1](https://github.com/Sild/api_clients_rs/releases/tag/api_clients_core-v0.0.1) - 2025-08-11

### Other

- rustfmt
- moveout core part
