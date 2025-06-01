# Zabbix API

Partial implementation of the Zabbix API client, created specifically for my pet project, [wszl](https://github.com/tinyops-ru/zabbix-lld-ws).

## Getting started

Add dependencies in your `Cargo.toml`:

```toml
[dependencies]
zabbix-api = { version = "0.6.0", features = ["v7", "full"] }
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
  - [x] Host
  - [ ] Item
  - [ ] Trigger
  - [ ] Web-scenario
  - [ ] User Group
  - [ ] User

## Disclaimer

As of May 2025, the integration tests failed on a just initialized zabbix-server/mysql.
The integration tests work by starting a mysql server, a zabbix server and a zabbix web
interface using docker-compose.
On every first run post-start of zabbix-server, the following error is observed:

```
{"jsonrpc":"2.0","error":{"code":-32500,"message":"Application error.","data":"DBEXECUTE_ERROR"},"id":1}
```

This error was also described in [issue ZBX-9916](https://support.zabbix.com/browse/ZBX-9916).
It seems to only occur the first time the tests are run.

In the container, zabbix-web logs the following error:

```
WARNING: [pool zabbix] child 45 said into stderr:
"PHP Warning:  Error in query [INSERT INTO ids (table_name,field_name,nextid) VALUES ('hstgrp','groupid',22)] [Deadlock found when trying to get lock; try restarting transaction]
    in /usr/share/zabbix/include/db.inc.php on line 249"
```

This error seems to come from zabbix itself and is never triggered when re-running
integration tests.

## RoadMap

- Add missing fields for models
- Delete entities (items, triggers, etc.)
- Async requests support

## Limitations

- Synchronous requests only
