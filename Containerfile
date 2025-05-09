FROM rust:bookworm AS build
RUN apt-get update && apt-get install -y libpq-dev libssl-dev
COPY . .
RUN cargo build --release --bin=rusty-migration --bin=rusty-crud-server --bin=rusty-game-server

FROM debian:bookworm-slim AS runtime-common-libpq-libssl
RUN apt-get update && apt-get install -y libpq-dev libssl-dev curl

FROM debian:bookworm-slim AS runtime-common-libssl
RUN apt-get update && apt-get install -y libssl-dev curl

FROM runtime-common-libpq-libssl AS runtime-rusty-migration
COPY --from=build /target/release/rusty-migration /rusty-migration
CMD [ "/rusty-migration" ]

FROM runtime-common-libpq-libssl AS runtime-rusty-crud-server
COPY --from=build /target/release/rusty-crud-server /rusty-crud-server
EXPOSE 3000
HEALTHCHECK --interval=5s --timeout=3s \
    CMD curl -f  http://localhost:3000/health || exit 1
CMD [ "/rusty-crud-server" ]

FROM runtime-common-libssl AS runtime-rusty-game-server
COPY --from=build /target/release/rusty-game-server /rusty-game-server
EXPOSE 3000
HEALTHCHECK --interval=5s --timeout=3s \
    CMD curl -f  http://localhost:3000/health || exit 1
CMD [ "/rusty-game-server" ]
