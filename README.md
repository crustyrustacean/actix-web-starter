# Actix Web Starter

A small, opinionated starting point for a versioned HTTP API built with [Actix Web](https://actix.rs/). It includes YAML and environment-based configuration, structured request logging, a health endpoint, and integration tests that exercise a running server. There is no database or external service to set up.

## Get started

You'll need [Rust and Cargo](https://rustup.rs/) and a suitable linker for your platform. The project uses Rust edition 2024, which requires Rust 1.85 or later; its dependencies may require a newer compiler.

Clone the repository or create a project from the template, then run the following from the project root:

```sh
cargo run --locked
```

In another terminal, try the health endpoint:

```sh
curl -i http://127.0.0.1:8000/api/v1/health_check
```

In Windows PowerShell, use `curl.exe` instead of `curl` if `curl` is an alias for `Invoke-WebRequest`.

You should see `HTTP/1.1 200 OK` with an empty body. This is a simple liveness response: it does not check a database or other dependencies. Press Ctrl+C in the server terminal to stop it.

## Configuration

Configuration is loaded in this order, with later sources overriding earlier ones:

1. `configuration/base.yaml`
2. `configuration/<environment>.yaml`
3. Environment variables prefixed with `APP_`, using `__` to separate nested keys

Set `APP_ENVIRONMENT` to `local` (the default) or `production`. These values are case-insensitive.

| Setting | Environment variable | Local default | Production default |
| --- | --- | --- | --- |
| Listen host | `APP_APPLICATION__HOST` | `127.0.0.1` | `0.0.0.0` |
| Listen port | `APP_APPLICATION__PORT` | `8000` | `8000` |

For example, to run locally on a different port:

```sh
APP_APPLICATION__PORT=8080 cargo run --locked
```

In PowerShell, set the variable before running Cargo instead:

```powershell
$env:APP_APPLICATION__PORT = "8080"
cargo run --locked
```

> **Production:** Production listens on all IPv4 interfaces by default.

For example, in a POSIX shell:

```sh
APP_ENVIRONMENT=production cargo run --locked --release
```

Or in PowerShell:

```powershell
$env:APP_ENVIRONMENT = "production"
cargo run --locked --release
```

The server currently serves plain HTTP, so terminate TLS at a reverse proxy or add TLS support before exposing it over HTTPS. Set environment variables in your shell or deployment environment; the starter does not load `.env` files.

Configuration files are resolved relative to the **current working directory**, not the executable. Run from the project root, or deploy the `configuration/` directory beneath the server's working directory.

## API routes

| Method | Path | Response |
| --- | --- | --- |
| `GET` | `/api/v1/health_check` | `200 OK`, empty body |

Routes are registered in the `/api/v1` scope in `src/startup.rs`. Unversioned paths and versions that are not registered return `404 Not Found`.

### Add an endpoint

1. Add a handler under `src/routes/`, declare and re-export its module in `src/routes.rs`.
2. Import the handler in `src/startup.rs` and register it in the `/api/v1` scope. For example, for a handler named `hello`:

   ```rust
   .service(
       web::scope("/api/v1")
           .route("/health_check", web::get().to(health_check))
           .route("/hello", web::get().to(hello)),
   )
   ```

3. Add an integration test under `tests/api/` and declare its module in `tests/api/main.rs`. Use `spawn_app()` from `tests/api/helpers.rs` to start a test server on an available port.

When turning the template into your own service, rename the package and binary in `Cargo.toml` and update references to the `actix_web_starter` library in the source and tests.

## Development

```sh
cargo test --locked                       # run all tests
cargo test --locked --test api            # run API integration tests
cargo fmt --all -- --check                # check formatting
cargo clippy --locked --all-targets       # run Clippy
cargo build --locked --release            # build a release binary
```

The integration tests start real HTTP servers on OS-assigned ports and send requests using Reqwest. They read the normal configuration before overriding the port, so your environment must still provide valid settings.

The application writes structured Bunyan-style JSON logs to stdout. Set `RUST_LOG` to change the log filter (the default is `info`). Test logs are suppressed by default; set `TEST_LOG` to any value and disable output capture to see them:

```sh
TEST_LOG=1 cargo test --locked -- --nocapture
```

In PowerShell:

```powershell
$env:TEST_LOG = "1"
cargo test --locked -- --nocapture
```

## Project layout

```text
configuration/          Base, local, and production YAML settings
src/bin/main.rs         Application entry point
src/configuration.rs    Configuration loading and settings
src/startup.rs          Server setup, middleware, and versioned routes
src/routes/             HTTP handlers
src/telemetry.rs        Structured logging and tracing setup
src/utils.rs            Error-chain formatting helper
tests/api/              Integration tests and test-server helper
```

## License

This project is licensed under the [MIT License](License.txt).
