`docker-compose up -d`

`cargo build --release`

`wash claims sign target/wasm32-unknown-unknown/release/client_test.wasm -c wasmcloud:httpserver -c wasmcloud:httpclient --name "client_test" --ver 0.1.0 --rev 0`

`wash reg push localhost:5000/foo:0.1.0 target/wasm32-unknown-unknown/release/client_test_s.wasm --insecure`

`wasmcloud --allowed-insecure localhost:5000 -m manifest.yaml`

```
❯ curl localhost:8080
Malformed response from actor
```
