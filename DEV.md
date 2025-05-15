# Development

## Integration tests

### V7

Start fresh Zabbix Server:

```shell
docker compose -f docker-compose-v7.yml up -d
```

Login to Zabbix with Admin http://localhost:3080 with creds: `Admin` / `zabbix` and
check if service available.

Then run tests:

```shell
chmod +x *.sh
./run-integration-tests-v7.sh
```

### V6

Start fresh Zabbix Server:

```shell
docker compose -f docker-compose-v6.yml up -d
```

Login to Zabbix with Admin http://localhost:3080 with creds: `Admin` / `zabbix` and
check if service available.

Then run tests:

```shell
chmod +x *.sh
./run-integration-tests-v6.sh
```
