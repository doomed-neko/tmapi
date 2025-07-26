![docs status](https://docs.rs/tmapi/badge.svg) ![tests status](https://github.com/doomed-neko/tmapi/workflows/Rust/badge.svg)

# TempMailAPI for rust

This is a library crate for interacting with [vwh](https://vwh.sh)'s TempMail service located at <https://web.barid.site/>

## Example:

```rust
use tmapi::Client;
let client = Client::new("y@iusearch.lol").unwrap();
//                              limit, offset
let emails = client.get_emails(  10  ,   0   ).await.unwrap();
let first_email = emails.iter().next().unwrap();
let id = &first_email.id;
client.delete_inbox(id).await.unwrap();
```
