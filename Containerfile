FROM rust:1.84-bookworm AS build

RUN apt-get update && apt-get install -y libpq-dev libssl-dev
COPY . .
RUN  cargo build --release

FROM debian:bookworm-slim AS runtime-common-libpq

RUN apt-get update && apt-get install -y libpq-dev libssl-dev

FROM debian:bookworm-slim AS runtime-common-libssl

RUN apt-get update && apt-get install -y libssl-dev

FROM runtime-common-libpq AS runtime-migration

COPY --from=build /target/release/migration /migration

CMD [ "/migration" ]

FROM runtime-common-libpq AS runtime-crud_server

COPY --from=build /target/release/crud_server /crud_server

CMD [ "/crud_server" ]

FROM runtime-common-libssl AS runtime-game_server

COPY --from=build /target/release/game_server /game_server

CMD [ "/game_server" ]
