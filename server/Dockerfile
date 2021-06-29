# Development
FROM rust:1.53.0 as develop-stage
WORKDIR /app
RUN cargo install cargo-watch diesel_cli
COPY . .

# Build
FROM develop-stage as build-stage
RUN cargo build --release

# Production
FROM rust:1.53.0 as production
WORKDIR /app
RUN cargo install diesel_cli
COPY . .

EXPOSE 8080