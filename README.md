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

## API Methods RoadMap

- [x] Authentication
- [ ] RAW API Call
- [ ] Search
  - [ ] Host
  - [ ] Hosts
  - [ ] Items
  - [ ] Triggers
  - [ ] Web-scenarios
- [ ] Create
    - [ ] Host
    - [ ] Item
    - [ ] Trigger
    - [ ] Web-scenario

## Limitations

- [API v6](https://www.zabbix.com/documentation/6.0/en/manual/api)