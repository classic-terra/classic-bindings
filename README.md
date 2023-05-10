# terra-bindings
Terra bindings smart contracts for CosmWasm/wasmd v1.1.0+

This new package of terra bindings will bring CosmWasm/wasmd to parity with the rest of the cosmos ecosystem

Run this will produce wasm file

```bash
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.12.8
```