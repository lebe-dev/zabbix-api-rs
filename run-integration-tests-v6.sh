#!/bin/bash

export ZABBIX_API_URL=http://localhost:3080/api_jsonrpc.php
export ZABBIX_API_USER=Admin
export ZABBIX_API_PASSWORD=zabbix

cargo test --no-default-features --features v6,full
cargo test --doc --no-default-features --features v6,full
