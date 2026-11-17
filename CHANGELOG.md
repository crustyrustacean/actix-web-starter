# Changelog

All notable changes to the Actix Web Starter project will be documented in this file.

## [0.3.2] - 2026-11-17

### Remove Unused Base URL Configuration

- Remove the unused application `base_url` setting and its registration as application data.
- Remove `APP_APPLICATION__BASE_URL` from the Dockerfile and README examples.
- Local runs, production runs, and tests no longer require a base URL environment override.

## [0.3.1] - 2026-11-17

### Vercel Docker Proof of Concept

- Add a multi-stage Dockerfile with Cargo Chef dependency caching and locked release builds.
- Use matching Debian Bookworm build/runtime images and run the application as a non-root user.
- Include runtime YAML configuration, select production settings, and map `PORT` to `APP_APPLICATION__PORT`.
- Supply a container-only `APP_APPLICATION__BASE_URL` default of `http://localhost`; override it with the deployment URL.
- Add `.dockerignore` to exclude build output and tests from the build context.
- Add `GET /api/v1/hello`, returning a JSON greeting for the deployment demo.
- Enable release binary stripping and thin link-time optimization.
- Remove the local `base_url` default; local runs and tests now require `APP_APPLICATION__BASE_URL`.
- Store the application base URL directly as application data instead of using the `ApplicationBaseUrl` wrapper.

## [0.3.0] - 2026-11-12

### General Cleanup and Dependencies Update

- remove incorrect `response.rs` and `error.rs`
- clean up `Cargo.toml`
- add proper `README.md`

## [0.2.0] - 2026-02-22

### Finish Zero2Prod style template

- **Split into bin/lib Format**
  - create modules for configuration, telemetry, startup
  - add integration tests

## [0.1.0] - 2026-02-22

### Initial Commit

- **Scaffold Based on Zero2Prod**
  - use intro example from Actix Web docs as starter
  - add `/health_check` route
