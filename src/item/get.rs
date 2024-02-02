use serde::Serialize;

/// API: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/item/get
#[derive(Serialize,Debug)]
pub struct GetItemsRequestById<R> {
    pub output: String,
    pub with_triggers: bool,
    #[serde(rename = "hostids")]
    pub host_ids: String,
    pub search: R,
    #[serde(rename = "sortfield")]
    pub sort_field: String
}

/// API: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/item/get
#[derive(Serialize,Debug)]
pub struct GetItemsRequestByKey<R> {
    pub output: String,
    pub with_triggers: bool,
    pub search: R,
    #[serde(rename = "sortfield")]
    pub sort_field: String
}

#[derive(Serialize,Debug)]
pub struct SearchByKey {
    pub key_: String,
}

impl GetItemsRequestByKey<SearchByKey> {
    pub fn new(key: &str) -> GetItemsRequestByKey<SearchByKey> {
        GetItemsRequestByKey {
            output: "extend".to_string(),
            with_triggers: false,
            search: SearchByKey {
                key_: key.to_string(),
            },
            sort_field: "name".to_string(),
        }
    }
}