FROM rust:1.82-bookworm AS build-common

RUN apt-get update && apt-get install -y libpq-dev

COPY . .

FROM build-common AS build-migration

RUN  cargo build --release --bin=migration

FROM build-common AS build-crud_server

RUN  cargo build --release --bin=crud_server

FROM debian:bookworm-slim AS runtime-common

RUN apt-get update && apt-get install -y libpq-dev

FROM runtime-common AS runtime-migration

COPY --from=build-migration /target/release/migration /migration

CMD [ "/migration" ]

FROM runtime-common AS runtime-crud_server

COPY --from=build-crud_server /target/release/crud_server /crud_server

CMD [ "/crud_server" ]
