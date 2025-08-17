FROM rust:trixie AS build
RUN apt-get update && apt-get install -y libpq-dev libssl-dev
COPY . .
RUN cargo build --release

FROM debian:trixie-slim AS runtime-common-libpq-libssl
RUN apt-get update && apt-get install -y libpq-dev libssl-dev

FROM debian:trixie-slim AS runtime-common-libssl
RUN apt-get update && apt-get install -y libssl-dev

FROM runtime-common-libpq-libssl AS runtime-migration
COPY --from=build /target/release/migration /migration
CMD [ "/migration" ]

FROM runtime-common-libpq-libssl AS runtime-crud-server
COPY --from=build /target/release/crud-server /crud-server
EXPOSE 3000
CMD [ "/crud-server" ]

FROM runtime-common-libssl AS runtime-game-server
COPY --from=build /target/release/game-server /game-server
EXPOSE 3000
CMD [ "/game-server" ]
