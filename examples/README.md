# Running Examples

To run the examples in this directory, you need to set up a few environment variables. These variables provide the necessary credentials and endpoint for connecting to your Zabbix API.

## Environment Variables

Before running any example, ensure the following environment variables are set in your shell:

- `ZABBIX_API_URL`: The full URL to your Zabbix API endpoint.
  - Example: `http://localhost:3080/api_jsonrpc.php`
- `ZABBIX_API_USER`: The username for Zabbix API authentication.
  - Example: `Admin`
- `ZABBIX_API_PASSWORD`: The password for the Zabbix API user.
  - Example: `zabbix`

Some examples might require additional specific environment variables. These will be noted in their respective run commands.

## How to Run

You can run each example using the `cargo run --example <example_name> --features v7,full` command.

### 1. API Basics

This example demonstrates basic API connectivity, fetching API version, and obtaining an authentication token.

```bash
export ZABBIX_API_URL=http://localhost:3080/api_jsonrpc.php
export ZABBIX_API_USER=Admin
export ZABBIX_API_PASSWORD=zabbix
cargo run --example api_basics --features v7,full
```

### 2. Get Hosts

This example shows how to fetch host information from Zabbix.

```bash
export ZABBIX_API_URL=http://localhost:3080/api_jsonrpc.php
export ZABBIX_API_USER=Admin
export ZABBIX_API_PASSWORD=zabbix
cargo run --example get_hosts_example --features v7,full
```

### 3. Get Host Groups

This example demonstrates fetching host groups, optionally filtering by name.

```bash
export ZABBIX_API_URL=http://localhost:3080/api_jsonrpc.php
export ZABBIX_API_USER=Admin
export ZABBIX_API_PASSWORD=zabbix
cargo run --example get_host_groups_example --features v7,full
```

### 4. Create Host Group

This example demonstrates creating a new host group.

```bash
export ZABBIX_API_URL=http://localhost:3080/api_jsonrpc.php
export ZABBIX_API_USER=Admin
export ZABBIX_API_PASSWORD=zabbix
cargo run --example create_host_group_example --features v7,full
```

### 5. Create Item

This example shows how to create a new item on a specified host.
It requires an additional environment variable:
- `ZABBIX_HOST_ID_FOR_ITEM_EXAMPLE`: The ID of an existing host in your Zabbix instance where the item will be created.
  - Example: `10010`

```bash
export ZABBIX_API_URL=http://localhost:3080/api_jsonrpc.php
export ZABBIX_API_USER=Admin
export ZABBIX_API_PASSWORD=zabbix
export ZABBIX_HOST_ID_FOR_ITEM_EXAMPLE="10001" # Replace "10001" with an actual host ID
cargo run --example create_item_example --features v7,full
```
Replace `"10001"` with an actual host ID from your Zabbix setup. The ID "10001" is a placeholder.

### 6. Get Items

This example demonstrates fetching items, for instance, by searching for a specific item key.

```bash
export ZABBIX_API_URL=http://localhost:3080/api_jsonrpc.php
export ZABBIX_API_USER=Admin
export ZABBIX_API_PASSWORD=zabbix
cargo run --example get_items_example --features v7,full
```

### 7. Raw API Call

This example demonstrates how to use the `raw_api_call` method for direct interaction with the Zabbix API, useful for methods not yet specifically implemented in the client or for custom parameter structures.

```bash
export ZABBIX_API_URL=http://localhost:3080/api_jsonrpc.php
export ZABBIX_API_USER=Admin
export ZABBIX_API_PASSWORD=zabbix
cargo run --example raw_api_call_example --features v7,full
```

### 8. Get Triggers

This example demonstrates fetching trigger information from Zabbix, including their tags.

```bash
export ZABBIX_API_URL=http://localhost:3080/api_jsonrpc.php
export ZABBIX_API_USER=Admin
export ZABBIX_API_PASSWORD=zabbix
cargo run --example get_triggers_example --features v7,full
```

**Note:** Ensure your Zabbix server is accessible and the API user has the necessary permissions for the operations performed by each example. The example commands use the features `v7` and `full` (which enables `item`, `host`, `trigger`, etc.). The commands also demonstrate setting the required environment variables (`ZABBIX_API_URL`, `ZABBIX_API_USER`, `ZABBIX_API_PASSWORD`).
