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
COPY --from=build-stage /app/target/release/actix-rest-api-server .
COPY --from=build-stage /app/.env .

EXPOSE 8080
CMD [ "/actix-rest-api-server" ]