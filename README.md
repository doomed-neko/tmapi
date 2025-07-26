# TempMailAPI for rust

This is a library crate for interacting with [vwh](https://vwh.sh)'s TempMail service located at <https://web.barid.site/>

## Example:

```rust
use tmapi::Client;

let client = Client::new("y@iusearch.lol").unwrap();
client.delete_inbox("usm2sw0qfv9a5ku9z4xmh8og").await.unwrap();
```
