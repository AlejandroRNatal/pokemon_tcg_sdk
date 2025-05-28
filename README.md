# A Rust Implementation for the Pokemon TCG v2 API 

## API

### Usage

```rust
use pokemon_tcg_sdk_rs::{ Client, Query };
use pokemon_tcg_sdk_rs::models::models::{ Card, Set, Pokemon, Type };

let api_key = "".into();
let api = Client::new(api_key);

let card: Option<Card> = api.find::<Card>("xy1-1".into()).await;
let set: Option<Set> = api.find::<Set>("xy1-1".into()).await;
let _types: Vec<Type> = api.all::<Type>().await;
```

## Dependencies
- serde
- reqwest
- tokio

## Dev

Testing command
```bash
POKEMON_TCG_API_KEY=KEY_HERE RUST_BACKTRACE=full cargo test -- --show-output
```
