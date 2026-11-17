FROM lukemathwalker/cargo-chef:latest-rust-1-bookworm AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS build
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --locked --recipe-path recipe.json
COPY . .
RUN cargo build --release --locked --bin actix-web-starter

# Match the builder's Debian release for compatibility with its linked libraries.
# A shell is needed to map Vercel's PORT to the application's configuration key.
FROM debian:bookworm-slim AS runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libgcc-s1 libstdc++6 \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=build /app/target/release/actix-web-starter /usr/local/bin/server
COPY configuration/ ./configuration/
ENV APP_ENVIRONMENT=production \
    APP_APPLICATION__HOST=0.0.0.0 \
    APP_APPLICATION__BASE_URL=http://localhost \
    PORT=80
USER 65532:65532
# exec lets Actix receive termination signals directly, rather than through sh.
ENTRYPOINT ["/bin/sh", "-c", "export APP_APPLICATION__PORT=\"${PORT:-80}\"; exec /usr/local/bin/server"]
