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
cargo run --example api_basics
```

### 2. Get Hosts

This example shows how to fetch host information from Zabbix.

```bash
cargo run --example get_hosts_example
```

### 3. Create Host Group

This example demonstrates creating a new host group.

```bash
cargo run --example create_host_group_example
```

### 4. Create Item

This example shows how to create a new item on a specified host.
It requires an additional environment variable:
- `ZABBIX_HOST_ID_FOR_ITEM_EXAMPLE`: The ID of an existing host in your Zabbix instance where the item will be created.
  - Example: `10010`

```bash
ZABBIX_HOST_ID_FOR_ITEM_EXAMPLE="your_host_id" \
cargo run --example create_item_example
```
Replace `"your_host_id"` with an actual host ID from your Zabbix setup.

### 5. Get Items

This example demonstrates fetching items, for instance, by searching for a specific item key.

```bash
cargo run --example get_items_example
```

**Note:** Ensure your Zabbix server is accessible and the API user has the necessary permissions for the operations performed by each example. The default feature `v7` and `full` (which includes `item`, `host`, etc.) should be enabled in `Cargo.toml` for these examples to compile and run correctly. Remember to set the common environment variables (`ZABBIX_API_URL`, `ZABBIX_API_USER`, `ZABBIX_API_PASSWORD`) in your shell session before running these commands.
