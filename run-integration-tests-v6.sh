#!/bin/bash

export ZABBIX_API_URL=http://localhost:3080/api_jsonrpc.php
export ZABBIX_API_USER=Admin
export ZABBIX_API_PASSWORD=zabbix

cargo test --features v6,full
cargo test --doc --features v6,full
