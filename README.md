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

<https://rusty.anclarma.fr>

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

(les `hardened-images` nécessitent un `docker login dhi.io`)

compose.yaml

```yaml
services:
  postgres:
    image: dhi.io/postgres:18-alpine3.22
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
    environment:
      CRUD_SERVER_URL: "http://crud_server:3000"
    networks:
      - rusty-network
      - traefik-network
  game_frontend:
    image: ghcr.io/alisterd51/rusty-game-frontend:latest
    restart: always
    networks:
      - traefik-network
  init-acme:
    image: busybox
    user: root
    volumes:
      - acme:/acme:rw
    command: chown -R 65532:65532 /acme
  traefik:
    depends_on:
      init-acme:
        condition: service_completed_successfully
    image: dhi.io/traefik:3.6
    restart: always
    ports:
      - "80:80"
      - "443:443/tcp"
      - "443:443/udp"
    volumes:
      - ./traefik/traefik.yml:/etc/traefik/traefik.yml:ro
      - ./traefik/config/dynamic:/config/dynamic:ro
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

traefik/traefik.yml

```yaml
entryPoints:
  web:
    address: ":80"
    http:
      redirections:
        entryPoint:
          to: websecure
          scheme: https
          permanent: true
  websecure:
    address: ":443"
    http3: {}

certificatesResolvers:
  myresolver:
    acme:
      email: antoinereims28@gmail.com
      storage: /acme/acme.json
      httpChallenge:
        entryPoint: web

providers:
  file:
    directory: "/config/dynamic"
    watch: true
```

traefik/config/dynamic/routes.yml

```yaml
http:
  routers:
    game-frontend:
      rule: "Host(`rusty.anclarma.fr`)"
      service: game-frontend-service
      entryPoints:
        - websecure
      tls:
        certresolver: myresolver
    game-server:
      rule: "Host(`rusty.anclarma.fr`) && PathPrefix(`/game.`)"
      service: game-server-service
      entryPoints:
        - websecure
      tls:
        certresolver: myresolver

  services:
    game-frontend-service:
      loadBalancer:
        servers:
          - url: "http://game_frontend:80"
    game-server-service:
      loadBalancer:
        servers:
          - url: "h2c://game_server:3000"
```

### Exemple avec Kubernetes

TODO
