# Rusty Kingdom

Rusty Kingdom un jeu de gestion avec quelques subtiliter:

- tout les composants de base sont ecrit en rust
- le client par defaut est une cli cepandant, le jeu doit pouvoir etre compatible avec n'importe quelle type de clients, notament:
  - script (bash + curl)
  - binaire (rust + reqwest)
  - webapp (html + css + js)

## Start

### Docker compose

```bash
# Create `.env` file
cp sample.env .env

# Run app
docker compose pull
docker compose build
docker compose up -d
```

### Kubernetes

```bash
# TODO
```

## Test Game API

```bash
cargo build --release --bin rusty-game-client
./target/release/rusty-game-client --help
```
