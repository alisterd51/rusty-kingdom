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

## WEB client

<https://ruty.anclarma.fr>

## CLI client

### Install

```bash
wget https://github.com/alisterd51/rusty-kingdom/releases/download/v0.7.0/game-client
chmod +x ./game-client
source <(./game-client completions bash)
./game-client --version
```

### Start

```bash
# Create your fortress
./game-client fortress new | jq

# See other commands
./game-client --help
```

## Serveur privé

Vous êtes encouragé à créer votre propre serveur privé.
Voici quelques possibilités pour y parvenir :

### Exemple avec Docker Compose et Traefik

Cet exemple montre comment est déployé le serveur officiel (accessible via <https://rusty.anclarma.fr>
).

(Le fichier `.env` est dérivé de `sample.env`.)

```yaml
services:
  postgres:
    image: postgres:18-alpine
    restart: always
    environment:
      POSTGRES_USER: ${POSTGRES_USER}
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
      POSTGRES_DB: ${POSTGRES_DB}
    networks:
      - rusty-network
    healthcheck:
      test: ["CMD-SHELL", "sh -c 'pg_isready -U ${POSTGRES_USER} -d ${POSTGRES_DB}'"]
      interval: 5s
      timeout: 3s
      retries: 10
  migration:
    image: ghcr.io/alisterd51/rusty-migration:latest
    environment:
      DATABASE_URL: "postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@postgres/${POSTGRES_DB}"
    networks:
      - rusty-network
    depends_on:
      postgres:
        condition: service_healthy
  crud_server:
    image: ghcr.io/alisterd51/rusty-crud-server:latest
    restart: always
    environment:
      DATABASE_URL: "postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@postgres/${POSTGRES_DB}"
    networks:
      - rusty-network
    depends_on:
      postgres:
        condition: service_healthy
      migration:
        condition: service_completed_successfully
  game_server:
    image: ghcr.io/alisterd51/rusty-game-server:latest
    restart: always
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.game_server.rule=Host(`rusty.anclarma.fr`) && PathPrefix(`/game.`)"
      - "traefik.http.routers.game_server.entrypoints=websecure"
      - "traefik.http.routers.game_server.tls=true"
      - "traefik.http.routers.game_server.tls.certresolver=myresolver"
      - "traefik.http.services.game_server.loadbalancer.server.port=3000"
      - "traefik.http.services.game_server.loadbalancer.server.scheme=h2c"
      - "traefik.docker.network=traefik-network"
    environment:
      CRUD_SERVER_URL: "http://crud_server:3000"
    networks:
      - rusty-network
      - traefik-network
  game_frontend:
    image: ghcr.io/alisterd51/rusty-game-frontend:latest
    restart: always
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.game_frontend.rule=Host(`rusty.anclarma.fr`)"
      - "traefik.http.routers.game_frontend.entrypoints=websecure"
      - "traefik.http.routers.game_frontend.tls=true"
      - "traefik.http.routers.game_frontend.tls.certresolver=myresolver"
      - "traefik.http.services.game_frontend.loadbalancer.server.port=80"
      - "traefik.docker.network=traefik-network"
    networks:
      - traefik-network
  traefik:
    image: traefik:v3.6
    restart: always
    command:
      - "--providers.docker=true"
      - "--providers.docker.exposedbydefault=false"
      - "--entrypoints.web.address=:80"
      - "--entrypoints.web.http.redirections.entryPoint.to=websecure"
      - "--entrypoints.web.http.redirections.entryPoint.scheme=https"
      - "--entrypoints.web.http.redirections.entryPoint.permanent=true"
      - "--entrypoints.websecure.address=:443"
      - "--entrypoints.websecure.http3"
      - "--certificatesresolvers.myresolver.acme.email=antoinereims28@gmail.com"
      - "--certificatesresolvers.myresolver.acme.storage=/acme/acme.json"
      - "--certificatesresolvers.myresolver.acme.httpchallenge.entrypoint=web"
    ports:
      - "80:80"
      - "443:443/tcp"
      - "443:443/udp"
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock:ro
      - acme:/acme:rw
    networks:
      - traefik-network

volumes:
  acme:

networks:
  rusty-network:
    external: false
  traefik-network:
    external: false
```

### Exemple avec Kubernetes

TODO
