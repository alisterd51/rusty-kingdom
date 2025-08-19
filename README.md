# Rusty Kingdom

Rusty Kingdom est un jeu de gestion avec quelques subtilités :

- il a été conçu pour être un [jeu incrémental](https://en.wikipedia.org/wiki/Incremental_game) mais pourrait évoluer vers d'autres genres de jeux.
- Tous les composants de base sont écrits en Rust afin de pouvoir supporter une très forte charge.
- Les joueurs sont incités à écrire leur propre client et leurs propres bots pour interagir avec le serveur de jeu.
- Le client par défaut est une CLI, cependant le jeu doit pouvoir être compatible avec n'importe quel type de client, notamment :
  - Script ([bash](https://www.gnu.org/software/bash/) + [curl](https://curl.se/))
  - Binaire ([Rust](https://www.rust-lang.org/) + [Reqwest](https://github.com/seanmonstar/reqwest))
  - Webapp ([HTML](https://developer.mozilla.org/fr/docs/Web/HTML) + [CSS](https://developer.mozilla.org/fr/docs/Web/CSS) + [JS](https://developer.mozilla.org/fr/docs/Web/JavaScript))
  - [gRPC](https://grpc.io/), pourquoi?
    la promesse de gRPC est de pouvoir ecrire une api 5 a 10 fois plus rapide qu'une api REST classique
    si c'est vrai, cela voudrat dire que le seul point limitant restant sera postgrsql
    il reste envisageable d'ajouter un proxy pour permettre a un client d'utiliser une api rest classique.

## Install

```bash
wget https://github.com/alisterd51/rusty-kingdom/releases/download/v0.6.3/game-client
chmod +x ./game-client
source <(./game-client completions bash)
./game-client --version
```

## Start

```bash
# Create your fortress
./game-client fortress new | jq

# See other commands
./game-client --help
```
