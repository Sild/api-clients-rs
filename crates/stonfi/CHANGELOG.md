# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.8.0](https://github.com/Sild/api_clients_rs/compare/stonfi_api_client-v0.7.0...stonfi_api_client-v0.8.0) - 2025-11-25

### Added

- add support to /transactions/query

## [0.6.3](https://github.com/Sild/api_clients_rs/compare/stonfi_api_client-v0.6.2...stonfi_api_client-v0.6.3) - 2025-11-23

### Added

- rename v1_dex -> v1
- add support to /v1/transactions/{hash}/action_tree

## [0.6.2](https://github.com/Sild/api_clients_rs/compare/stonfi_api_client-v0.6.1...stonfi_api_client-v0.6.2) - 2025-11-05

### Other

- support timeout

## [0.6.1](https://github.com/Sild/api_clients_rs/compare/stonfi_api_client-v0.6.0...stonfi_api_client-v0.6.1) - 2025-10-30

### Fixed

- fix stonfi.assets/query unconditional_assets

## [0.6.0](https://github.com/Sild/api_clients_rs/compare/stonfi_api_client-v0.5.1...stonfi_api_client-v0.6.0) - 2025-10-30

### Other

- hide builder

## [0.5.1](https://github.com/Sild/api_clients_rs/compare/stonfi_api_client-v0.5.0...stonfi_api_client-v0.5.1) - 2025-10-29

### Other

- restruct clients, update errors

## [0.5.0](https://github.com/Sild/api_clients_rs/compare/stonfi_api_client-v0.4.0...stonfi_api_client-v0.5.0) - 2025-10-15

### Other

- restruct clients
- meaningful default for Builders

## [0.4.0](https://github.com/Sild/api_clients_rs/compare/stonfi_api_client-v0.3.4...stonfi_api_client-v0.4.0) - 2025-10-12

### Other

- Merge pull request #21 from Sild/tonco
- rewrite clients using builder, add tonco client
- add tonco_api_client

## [0.3.4](https://github.com/Sild/api_clients_rs/compare/stonfi_api_client-v0.3.3...stonfi_api_client-v0.3.4) - 2025-09-05

### Fixed

- fix exports
- fix exports

## [0.3.3](https://github.com/Sild/api_clients_rs/compare/stonfi_api_client-v0.3.2...stonfi_api_client-v0.3.3) - 2025-09-05

### Fixed

- fix types

### Other

- ApiClientError without imports

## [0.3.2](https://github.com/Sild/api_clients_rs/compare/stonfi_api_client-v0.3.1...stonfi_api_client-v0.3.2) - 2025-09-05

### Other

- re-export core

## [0.3.1](https://github.com/Sild/api_clients_rs/compare/stonfi_api_client-v0.3.0...stonfi_api_client-v0.3.1) - 2025-08-19

### Other

- all types support ser/de now

## [0.3.0](https://github.com/Sild/api_clients_rs/compare/stonfi_api_client-v0.2.5...stonfi_api_client-v0.3.0) - 2025-08-19

### Other

- up Asset fields

## [0.2.5](https://github.com/Sild/api_clients_rs/compare/stonfi_api_client-v0.2.4...stonfi_api_client-v0.2.5) - 2025-08-19

### Other

- pub fields

## [0.2.4](https://github.com/Sild/api_clients_rs/compare/stonfi_api_client-v0.2.3...stonfi_api_client-v0.2.4) - 2025-08-11

### Other

- support assets_query

## [0.2.3](https://github.com/Sild/api_clients_rs/compare/stonfi_api_client-v0.2.2...stonfi_api_client-v0.2.3) - 2025-08-11

### Other

- rustfmt
- moveout core part

## [0.2.2](https://github.com/Sild/api_clients_rs/compare/stonfi_api_client-v0.2.1...stonfi_api_client-v0.2.2) - 2025-02-02

### Other

- support stonfi::pools_by_market

## [0.2.0](https://github.com/Sild/api_clients_rs/compare/stonfi_api_client-v0.1.1...stonfi_api_client-v0.2.0) - 2024-11-10

### Other

- support farms and farms/addr
