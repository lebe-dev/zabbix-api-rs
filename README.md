# Zabbix API

This is a partial implementation of the Zabbix API client, created specifically for my pet project, [wszl](https://github.com/tinyops-ru/zabbix-lld-ws). 
Due to the extensive nature of the Zabbix API, I have been unable to allocate sufficient time to complete the implementation.

## Getting started

Add `reqwest` dependency:

```shell
reqwest = { version = "0.11.23", features = ["blocking", "json"] }
```

Then use:

```rust
let http_client = Client::new();

let client = ZabbixApiV6Client::new(http_client, "http://localhost:3080/api_jsonrpc.php");

match client.get_auth_session("Admin", "zabbix") {
    Ok(session) => println!("session: {session}"),
    Err(e) => {
        error!("error: {}", e);
        panic!("unexpected error")
    }
}
```