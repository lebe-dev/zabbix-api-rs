# Zabbix API

This is a partial implementation of the Zabbix API client, created specifically for my pet project, [wszl](https://github.com/tinyops-ru/zabbix-lld-ws). 
Due to the extensive nature of the Zabbix API, I have been unable to allocate sufficient time to complete the implementation.

## Getting started

Add dependencies:

```toml
[dependencies]
reqwest = { version = "0.11.23", features = ["blocking", "json"] }
zabbix-api = "0.1.0"
```

Then use:

```rust
use reqwest::blocking::Client;
use crate::client::v6::ZabbixApiV6Client;
use crate::client::ZabbixApiClient;

fn main() {
  let http_client = Client::new();

  let client = ZabbixApiV6Client::new(http_client, "http://localhost:3080/api_jsonrpc.php");
    
  match client.get_auth_session("Admin", "zabbix") {
    Ok(session) => println!("session: {session}"),
    Err(e) => {
        error!("error: {}", e);
        panic!("unexpected error")
    }
  }
}
```

- You can make [raw api calls](src/client/v6/mod.rs#L113).

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
- [x] Create
  - [x] Host Group
  - [x] Host
  - [x] Item
  - [x] Trigger
  - [x] Web-scenario

## TODO

- Add examples
- Use enums for Zabbix types (trigger type, value type, etc.)

## Limitations

- API support: [v6](https://www.zabbix.com/documentation/6.0/en/manual/api)