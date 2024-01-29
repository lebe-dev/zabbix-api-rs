# Zabbix API

This is a partial implementation of the Zabbix API client, created specifically for my pet project, [wszl](https://github.com/tinyops-ru/zabbix-lld-ws). 
Due to the extensive nature of the Zabbix API, I have been unable to allocate sufficient time to cover 100% functionality.

## Getting started

Add dependencies in your Cargo.toml :

```toml
[dependencies]
reqwest = { version = "0.11.23", features = ["blocking", "json"] }
zabbix-api = "0.2.0"
```

Then use:

```rust
use reqwest::blocking::ClientBuilder;
use zabbix_api::client::v6::ZabbixApiV6Client;
use zabbix_api::client::ZabbixApiClient;

fn main() {
  let http_client = ClientBuilder::new()
       .danger_accept_invalid_certs(false) // Set true if you're using self-signed certificates.
       .build().unwrap();

  let client = ZabbixApiV6Client::new(http_client, "http://localhost:3080/api_jsonrpc.php");
    
  match client.get_auth_session("Admin", "zabbix") {
    Ok(session) => println!("session: {session}"),
    Err(e) => {
        eprintln!("error: {}", e);
        panic!("unexpected error")
    }
  }
}
```

- You can make [raw api calls](src/client/v6/mod.rs#L113).

## Versions

- Stable: `0.2.0`
- Dev: `0.2.1`

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
  - [ ] User
- [x] Create
  - [x] Host Group
  - [x] Host
  - [x] Item
  - [x] Trigger
  - [x] Web-scenario
  - [ ] User Group
  - [ ] User

## TODO

- Add examples
- Use enums for Zabbix types (trigger type, value type, etc.)

## Limitations

- API support: [v6](https://www.zabbix.com/documentation/6.0/en/manual/api)
- Synchronous requests only
