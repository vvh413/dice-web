# dice web

## Run

```sh
yarn build
cargo watch -x run
```

## Usage

```sh
curl -X GET http://localhost:8000/api/1d100
# {"values":[11],"total":11,"min":11,"max":11}

curl -X GET http://localhost:8000/api/3d20
# {"values":[4,18,5],"total":27,"min":4,"max":18}
```
