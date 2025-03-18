FROM rust:bookworm AS build
RUN apt-get update && apt-get install -y libpq-dev libssl-dev
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim AS runtime-common-libpq-libssl
RUN apt-get update && apt-get install -y libpq-dev libssl-dev

FROM debian:bookworm-slim AS runtime-common-libssl
RUN apt-get update && apt-get install -y libssl-dev

FROM runtime-common-libpq-libssl AS runtime-rusty-migration
COPY --from=build /target/release/rusty-migration /rusty-migration
CMD [ "/rusty-migration" ]

FROM runtime-common-libpq-libssl AS runtime-rusty-crud-server
COPY --from=build /target/release/rusty-crud-server /rusty-crud-server
EXPOSE 3000
CMD [ "/rusty-crud-server" ]

FROM runtime-common-libssl AS runtime-rusty-game-server
COPY --from=build /target/release/rusty-game-server /rusty-game-server
EXPOSE 3000
CMD [ "/rusty-game-server" ]
