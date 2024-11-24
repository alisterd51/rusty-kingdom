FROM rust:1.83-bookworm AS build-common

RUN apt-get update && apt-get install -y libpq-dev libssl-dev

COPY . .

FROM build-common AS build-migration

RUN  cargo build --release --bin=migration

FROM build-common AS build-crud_server

RUN  cargo build --release --bin=crud_server

FROM build-common AS build-game_server

RUN  cargo build --release --bin=game_server

FROM debian:bookworm-slim AS runtime-common-libpq

RUN apt-get update && apt-get install -y libpq-dev

FROM debian:bookworm-slim AS runtime-common-libssl

RUN apt-get update && apt-get install -y libssl-dev

FROM runtime-common-libpq AS runtime-migration

COPY --from=build-migration /target/release/migration /migration

CMD [ "/migration" ]

FROM runtime-common-libpq AS runtime-crud_server

COPY --from=build-crud_server /target/release/crud_server /crud_server

CMD [ "/crud_server" ]

FROM runtime-common-libssl AS runtime-game_server

COPY --from=build-game_server /target/release/game_server /game_server

CMD [ "/game_server" ]
