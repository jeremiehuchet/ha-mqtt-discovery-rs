/*
 * Data structures for Home Assistant MQTT discovery
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceTrigger {
    /// The type of automation, must be 'trigger'.
    #[serde(rename = "automation_type")]
    pub automation_type: String,
    /// Optional payload to match the payload being sent over the topic.
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<i32>,
    /// The MQTT topic subscribed to receive trigger events.
    #[serde(rename = "topic")]
    pub topic: String,
    /// The type of the trigger, e.g. `button_short_press`. Entries supported by the frontend: `button_short_press`, `button_short_release`, `button_long_press`, `button_long_release`, `button_double_press`, `button_triple_press`, `button_quadruple_press`, `button_quintuple_press`. If set to an unsupported value, will render as `subtype type`, e.g. `button_1 spammed` with `type` set to `spammed` and `subtype` set to `button_1`
    #[serde(rename = "type")]
    pub r#type: String,
    /// The subtype of the trigger, e.g. `button_1`. Entries supported by the frontend: `turn_on`, `turn_off`, `button_1`, `button_2`, `button_3`, `button_4`, `button_5`, `button_6`. If set to an unsupported value, will render as `subtype type`, e.g. `left_button pressed` with `type` set to `button_short_press` and `subtype` set to `left_button`
    #[serde(rename = "subtype")]
    pub subtype: String,
    #[serde(rename = "device")]
    pub device: Box<crate::models::DeviceTriggerDevice>,
    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the value.
    #[serde(rename = "value_template", skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

impl DeviceTrigger {
    pub fn new(automation_type: String, topic: String, r#type: String, subtype: String, device: crate::models::DeviceTriggerDevice) -> DeviceTrigger {
        DeviceTrigger {
            automation_type,
            payload: None,
            qos: None,
            topic,
            r#type,
            subtype,
            device: Box::new(device),
            value_template: None,
        }
    }
}

