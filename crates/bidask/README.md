# bidask_api_client

`bidask_api_client` is not supported. It is intentionally not published to
crates.io and is excluded from the release-plz publish cycle.

Do not add this crate as a dependency in final applications. No Bidask endpoint
is currently documented as supported by this workspace.

Public request and response types are marked `#[non_exhaustive]` for semver
headroom. Public POD structs support `Default::default().with_<field>(...)` for
tests and migration code. Include a wildcard arm when matching response enums.

The source still contains legacy request/response wrappers for migration and
reference work, but they are not part of the supported public-library guidance.
Future work should not add examples or recommend runtime integration unless
Bidask support is explicitly revived.
