# Rusty Kingdom

## Start

```bash
docker compose up --build
```

### client crud (curl)

fortress

```bash
$ curl -X POST http://localhost:3000/api/fortress
    -H 'Content-Type: application/json'
    -d '{"gold":0,"food":0,"wood":0,"energy":0}'
{"id":1,"gold":0,"food":0,"wood":0,"energy":0}

$ curl -X GET http://localhost:3000/api/fortress
[{"id":1,"gold":0,"food":0,"wood":0,"energy":0}]

$ curl -X GET http://localhost:3000/api/fortress/1
{"id":1,"gold":0,"food":0,"wood":0,"energy":0}

$ curl -X PATCH http://localhost:3000/api/fortress/1
    -H 'Content-Type: application/json'
    -d '{"energy":42}'
{"id":1,"gold":0,"food":0,"wood":0,"energy":42}

$ curl -X DELETE http://localhost:3000/api/fortress/1
1
```

building

```bash
$ curl -X POST http://localhost:3000/api/building
   -H 'Content-Type: application/json'
   -d '{"name":"new building","level":1,"fortress_id":1}'
{"id":1,"name":"new building","level":1,"fortress_id":1}

$ curl -X GET http://localhost:3000/api/building
[{"id":1,"name":"new building","level":1,"fortress_id":1}]

$ curl -X GET http://localhost:3000/api/building/1
{"id":1,"name":"new building","level":1,"fortress_id":1}

$ curl -X PATCH http://localhost:3000/api/building/1
    -H 'Content-Type: application/json'
    -d '{"level":42}'
{"id":1,"name":"new building","level":42,"fortress_id":1}

$ curl -X DELETE http://localhost:3000/api/building/1
1
```
