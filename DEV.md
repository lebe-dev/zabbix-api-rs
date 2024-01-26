# Development

## Integration tests

Install docker then start fresh Zabbix Server:

```shell
rm -rf data
docker-compose up -d
```

Login to Zabbix with Admin http://localhost:3080 with creds: `Admin` / `zabbix`.

Then run tests:

```shell
chmod +x run-integration-tests.sh
./run-integration-tests.sh
```
Run docker command to see concise output.

```shell
sudo docker ps --format 'table {{.Names}}\t{{.Image}}\t{{.Status}}' | grep zabbix
NAMES                                    IMAGE                                             STATUS
zabbix-api-rs-zabbix-web-nginx-mysql-1   zabbix/zabbix-web-nginx-mysql:ubuntu-6.2-latest   Up About an hour
zabbix-api-rs-zabbix-server-1            zabbix/zabbix-server-mysql:ubuntu-6.2-latest      Up About an hour
zabbix-api-rs-mysql-server-1             mysql:8.0-oracle                                  Up About an hour
Wed Jan 24 10:07:41 AM CST 2024
```
