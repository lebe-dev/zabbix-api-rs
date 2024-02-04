# Development

## Integration tests

Install docker then start fresh Zabbix Server:

```shell
docker-compose up -d
```

Login to Zabbix with Admin http://localhost:3080 with creds: `Admin` / `zabbix`.

Then run tests:

```shell
chmod +x run-integration-tests.sh
./run-integration-tests.sh
```
