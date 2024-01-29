#!/bin/bash

ZABBIX_API_URL=http://localhost:3080/api_jsonrpc.php \
ZABBIX_API_USER=Admin \
ZABBIX_API_PASSWORD=zabbix \
cargo test