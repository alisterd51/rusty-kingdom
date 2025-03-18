# Rusty Kingdom

Rusty Kingdom est un jeu de gestion avec quelques subtilités :

- il a été conçu pour être un [jeu incrémental](https://en.wikipedia.org/wiki/Incremental_game) mais pourrait évoluer vers d'autres genres de jeux.
- Tous les composants de base sont écrits en Rust afin de pouvoir supporter une très forte charge.
- Les joueurs sont incités à écrire leur propre client et leurs propres bots pour interagir avec le serveur de jeu.
- Le client par défaut est une CLI, cependant le jeu doit pouvoir être compatible avec n'importe quel type de client, notamment :
  - Script ([bash](https://www.gnu.org/software/bash/) + [curl](https://curl.se/))
  - Binaire ([Rust](https://www.rust-lang.org/) + [Reqwest](https://github.com/seanmonstar/reqwest))
  - Webapp ([HTML](https://developer.mozilla.org/fr/docs/Web/HTML) + [CSS](https://developer.mozilla.org/fr/docs/Web/CSS) + [JS](https://developer.mozilla.org/fr/docs/Web/JavaScript))

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
