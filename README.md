# Zabbix API

Partial implementation of the Zabbix API client, created specifically for my pet project, [wszl](https://github.com/tinyops-ru/zabbix-lld-ws).

## Getting started

Add dependencies in your `Cargo.toml`:

```toml
[dependencies]
zabbix-api = { version = "0.5.0", features = ["v7", "full"] }
```

Check [examples](examples) directory.

## API Methods

- [x] Get API info
- [x] Authentication
- [x] RAW API Call
- [x] Get
  - [x] Host Group
  - [x] Hosts
  - [x] Items
  - [x] Triggers
  - [x] Web-scenarios
  - [x] User Group
  - [x] User
- [x] Create
  - [x] Host Group
  - [x] Host
  - [x] Item
  - [x] Trigger
  - [x] Web-scenario
  - [x] User Group
  - [x] User
- [ ] Delete
  - [ ] Host Group
  - [ ] Host
  - [ ] Item
  - [ ] Trigger
  - [ ] Web-scenario
  - [ ] User Group
  - [ ] User

## TODO

- Add examples
- Use enums for Zabbix types (trigger type, value type, etc.)

## Limitations

- Synchronous requests only
