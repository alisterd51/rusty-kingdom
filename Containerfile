FROM rust:trixie AS build-common
RUN apt-get update && apt-get install -y libpq-dev libssl-dev protobuf-compiler libprotobuf-dev wget
COPY . .
RUN cargo fetch --locked

FROM build-common AS build-migration
RUN cargo build --frozen --release --bin=migration

FROM build-common AS build-crud-server
RUN cargo build --frozen --release --bin=crud-server

FROM build-common AS build-game-server
RUN cargo build --frozen --release --bin=game-server

FROM build-common AS build-game-frontend
RUN rustup target add wasm32-unknown-unknown
RUN wget -qO- https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-$(uname -m)-unknown-linux-gnu.tar.gz | tar -xzf- -C /usr/local/bin
WORKDIR /game-frontend
ARG GAME_API_URL="https://rusty.anclarma.fr"
RUN trunk build --frozen --release --minify

FROM debian:trixie-slim AS runtime-common-libpq-libssl
RUN apt-get update && apt-get install -y --no-install-recommends \
    libpq5 \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

FROM runtime-common-libpq-libssl AS runtime-migration
COPY --from=build-migration /target/release/migration /migration
CMD [ "/migration" ]

FROM runtime-common-libpq-libssl AS runtime-crud-server
COPY --from=build-crud-server /target/release/crud-server /crud-server
EXPOSE 3000
CMD [ "/crud-server" ]

FROM debian:trixie-slim AS runtime-game-server
COPY --from=build-game-server /target/release/game-server /game-server
EXPOSE 3000
CMD [ "/game-server" ]

FROM nginx:1.29-alpine-slim AS runtime-game-frontend
COPY ./game-frontend/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build-game-frontend /game-frontend/dist /usr/share/nginx/html
