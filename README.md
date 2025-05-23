# Zabbix API

This is a partial implementation of the Zabbix API client, created specifically for my pet project, [wszl](https://github.com/tinyops-ru/zabbix-lld-ws).
Due to the extensive nature of the Zabbix API, I have been unable to allocate sufficient time to cover 100% functionality.

## Getting started

Add dependencies in your `Cargo.toml`:

```toml
[dependencies]
zabbix-api = { version = "0.5.0", features = ["v7", "full"] }
```

Then use:

```rust
use reqwest::blocking::ClientBuilder;
use zabbix_api::client::client::ZabbixApiClientImpl;
use zabbix_api::client::ZabbixApiClient;

fn main() {
  let http_client = ClientBuilder::new()
       .danger_accept_invalid_certs(false) // Set true if you're using self-signed certificates.
       .build().unwrap();

  let client = ZabbixApiClientImpl::new(http_client, "http://localhost:3080/api_jsonrpc.php");

  match client.get_auth_session("Admin", "zabbix") {
    Ok(session) => println!("session: {session}"),
    Err(e) => {
        eprintln!("error: {}", e);
        panic!("unexpected error")
    }
  }
}
```

- You can make [raw api calls](src/client/client.rs#L36).

## API Methods

- [x] Get API info
- [x] Authentication
- [x] RAW API Call
- [x] Search
  - [x] Host Group
  - [x] Hosts
  - [x] Items
  - [x] Triggers
  - [x] Web-scenarios
  - [ ] User Group
  - [x] User
- [x] Create
  - [x] Host Group
  - [x] Host
  - [x] Item
  - [x] Trigger
  - [x] Web-scenario
  - [x] User Group
  - [ ] User

## TODO

- Add examples
- Use enums for Zabbix types (trigger type, value type, etc.)

## Limitations

- Synchronous requests only
