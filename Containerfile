FROM rust:trixie AS build
RUN apt-get update && apt-get install -y libpq-dev libssl-dev protobuf-compiler libprotobuf-dev
COPY . .
RUN cargo build --release --bin=migration --bin=crud-server --bin=game-server

FROM rust:trixie AS build-leptos
RUN rustup target add wasm32-unknown-unknown && cargo install trunk --locked
COPY . .
WORKDIR /leptos-client
RUN trunk build --release --minify

FROM debian:trixie-slim AS runtime-common-libpq-libssl
RUN apt-get update && apt-get install -y libpq-dev libssl-dev

FROM runtime-common-libpq-libssl AS runtime-migration
COPY --from=build /target/release/migration /migration
CMD [ "/migration" ]

FROM runtime-common-libpq-libssl AS runtime-crud-server
COPY --from=build /target/release/crud-server /crud-server
EXPOSE 3000
CMD [ "/crud-server" ]

FROM nginx:1.29-alpine-slim AS runtime-leptos-client
COPY ./leptos-client/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build-leptos /leptos-client/dist /usr/share/nginx/html
