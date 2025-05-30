use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::DisplayFromStr;

/// API Object: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/trigger/object
#[serde_as]
#[derive(Deserialize, Clone, Debug)]
pub struct ZabbixTrigger {
    /// ID of the trigger.
    #[serde(alias = "triggerid")]
    pub trigger_id: String,

    /// Event name generated by the trigger.
    pub event_name: String,

    /// URL associated with the trigger.
    pub url: String,

    /// Name of the trigger (required).
    pub description: String,

    /// Reduced trigger expression (required).
    pub expression: String,

    /// Severity of the trigger.
    ///
    /// Possible values are:
    ///
    /// 0 - (default) not classified;
    ///
    /// 1 - information;
    ///
    /// 2 - warning;
    ///
    /// 3 - average;
    ///
    /// 4 - high;
    ///
    /// 5 - disaster.
    #[serde_as(as = "DisplayFromStr")]
    pub priority: u8,

    /// OK event generation mode.
    ///
    /// Possible values are:
    ///
    /// 0 - (default) Expression;
    ///
    /// 1 - Recovery expression;
    ///
    /// 2 - None.
    #[serde_as(as = "DisplayFromStr")]
    pub recovery_mode: u8,

    /// Reduced trigger recovery expression.
    pub recovery_expression: String,
}

/// API Object: https://www.zabbix.com/documentation/6.0/en/manual/api/reference/trigger/object#trigger-tag
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ZabbixTriggerTag {
    pub tag: String,
    pub value: String,
}

#[cfg(test)]
mod trigger_tests {
    use super::ZabbixTrigger;

    #[test]
    fn deserialize_test() {
        let input = r#"
            {"triggerid":"24099","expression":"{34519}>=1","description":"Site 'example.com' is unavailable","url":"https://example.com","status":"0","value":"0","priority":"4","lastchange":"0","comments":"","error":"","templateid":"0","type":"0","state":"0","flags":"0","recovery_mode":"1","recovery_expression":"{34520}=0","correlation_mode":"0","correlation_tag":"","manual_close":"0","opdata":"","event_name":"example.com is down","uuid":"","url_name":"","functions":[{"functionid":"34519","itemid":"48175","triggerid":"24099","parameter":"$,#3","function":"avg"},{"functionid":"34520","itemid":"48175","triggerid":"24099","parameter":"$","function":"last"}]}
            "#;

        let result: ZabbixTrigger = serde_json::from_str(&input).unwrap();

        assert_eq!(result.priority, 4);
        assert_eq!(result.recovery_mode, 1);
    }
}
