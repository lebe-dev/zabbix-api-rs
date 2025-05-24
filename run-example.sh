#!/bin/bash

export ZABBIX_API_URL=http://localhost:3080/api_jsonrpc.php
export ZABBIX_API_USER=Admin
export ZABBIX_API_PASSWORD=zabbix

cargo run --example api_basics --features v7,full
cargo run --example create_host_group_example --features v7,full
export ZABBIX_HOST_ID_FOR_ITEM_EXAMPLE="10001" && cargo run --example create_item_example --features v7,full
cargo run --example get_host_groups_example --features v7,full
cargo run --example get_hosts_example --features v7,full
cargo run --example get_items_example --features v7,full
cargo run --example get_triggers_example --features v7,full
cargo run --example raw_api_call_example --features v7,full
