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
pub struct Siren {
    /// A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`.
    #[serde(rename = "availability", skip_serializing_if = "Option::is_none")]
    pub availability: Option<Vec<crate::models::AlarmControlPanelAvailabilityInner>>,
    /// When `availability` is configured, this controls the conditions needed to set the entity to `available`. Valid entries are `all`, `any`, and `latest`. If set to `all`, `payload_available` must be received on all configured availability topics before the entity is marked as online. If set to `any`, `payload_available` must be received on at least one configured availability topic before the entity is marked as online. If set to `latest`, the last `payload_available` or `payload_not_available` received on any configured availability topic controls the availability. (Default: `latest)`
    #[serde(rename = "availability_mode", skip_serializing_if = "Option::is_none")]
    pub availability_mode: Option<String>,
    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract device's availability from the `availability_topic`. To determine the devices's availability result of this template will be compared to `payload_available` and `payload_not_available`.
    #[serde(rename = "availability_template", skip_serializing_if = "Option::is_none")]
    pub availability_template: Option<String>,
    /// The MQTT topic subscribed to receive availability (online/offline) updates. Must not be used together with `availability`.
    #[serde(rename = "availability_topic", skip_serializing_if = "Option::is_none")]
    pub availability_topic: Option<String>,
    /// A list of available tones the siren supports. When configured, this enables the support for setting a `tone` and enables the `tone` state attribute.
    #[serde(rename = "available_tones", skip_serializing_if = "Option::is_none")]
    pub available_tones: Option<Vec<String>>,
    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate a custom payload to send to `command_topic`. The variable `value` will be assigned with the configured `payload_on` or `payload_off` setting. The siren turn on service parameters `tone`, `volume_level` or `duration` can be used as variables in the template. When operation in optimistic mode the corresponding state attributes will be set. Turn on parameters will be filtered if a device misses the support.
    #[serde(rename = "command_template", skip_serializing_if = "Option::is_none")]
    pub command_template: Option<String>,
    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate a custom payload to send to `command_topic` when the siren turn off service is called. By default `command_template` will be used as template for service turn off. The variable `value` will be assigned with the configured `payload_off` setting.
    #[serde(rename = "command_off_template", skip_serializing_if = "Option::is_none")]
    pub command_off_template: Option<String>,
    /// The MQTT topic to publish commands to change the siren state. Without command templates, a default JSON payload like `{\"state\":\"ON\", \"tone\": \"bell\", \"duration\": 10, \"volume_level\": 0.5 }` is published. When the siren turn on service is called, the startup parameters will be added to the JSON payload. The `state` value of the JSON payload will be set to the the `payload_on` or `payload_off` configured payload. 
    #[serde(rename = "command_topic", skip_serializing_if = "Option::is_none")]
    pub command_topic: Option<String>,
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: Option<Box<crate::models::ButtonDevice>>,
    /// Flag which defines if the entity should be enabled when first added. (Default: `true)`
    #[serde(rename = "enabled_by_default", skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,
    /// The encoding of the payloads received and published messages. Set to `\"\"` to disable decoding of incoming payload. (Default: `utf-8)`
    #[serde(rename = "encoding", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    /// The [category](https://developers.home-assistant.io/docs/core/entity#generic-properties) of the entity. (Default: `None)`
    #[serde(rename = "entity_category", skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<String>,
    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation.
    #[serde(rename = "json_attributes_template", skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,
    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation.
    #[serde(rename = "json_attributes_topic", skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,
    /// The name to use when displaying this siren. Can be set to `null` if only the device name is relevant. (Default: `MQTT Siren)`
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Used instead of `name` for automatic generation of `entity_id`
    #[serde(rename = "object_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    /// Flag that defines if siren works in optimistic mode. (Default: ``true` if no `state_topic` defined, else `false`.)`
    #[serde(rename = "optimistic", skip_serializing_if = "Option::is_none")]
    pub optimistic: Option<bool>,
    /// The payload that represents the available state. (Default: `online)`
    #[serde(rename = "payload_available", skip_serializing_if = "Option::is_none")]
    pub payload_available: Option<String>,
    /// The payload that represents the unavailable state. (Default: `offline)`
    #[serde(rename = "payload_not_available", skip_serializing_if = "Option::is_none")]
    pub payload_not_available: Option<String>,
    /// The payload that represents `off` state. If specified, will be used for both comparing to the value in the `state_topic` (see `value_template` and `state_off` for details) and sending as `off` command to the `command_topic`. (Default: `OFF)`
    #[serde(rename = "payload_off", skip_serializing_if = "Option::is_none")]
    pub payload_off: Option<String>,
    /// The payload that represents `on` state. If specified, will be used for both comparing to the value in the `state_topic` (see `value_template` and `state_on`  for details) and sending as `on` command to the `command_topic`. (Default: `ON)`
    #[serde(rename = "payload_on", skip_serializing_if = "Option::is_none")]
    pub payload_on: Option<String>,
    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<i32>,
    /// If the published message should have the retain flag on or not.
    #[serde(rename = "retain", skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,
    /// The payload that represents the `off` state. Used when value that represents `off` state in the `state_topic` is different from value that should be sent to the `command_topic` to turn the device `off`. (Default: ``payload_off` if defined, else `'OFF'`)`
    #[serde(rename = "state_off", skip_serializing_if = "Option::is_none")]
    pub state_off: Option<String>,
    /// The payload that represents the `on` state. Used when value that represents `on` state in the `state_topic` is different from value that should be sent to the `command_topic` to turn the device `on`. (Default: ``payload_on` if defined, else `'ON'`)`
    #[serde(rename = "state_on", skip_serializing_if = "Option::is_none")]
    pub state_on: Option<String>,
    /// The MQTT topic subscribed to receive state updates. The state update may be either JSON or a simple string. When a JSON payload is detected, the `state` value of the JSON payload should supply the `payload_on` or `payload_off` defined payload to turn the siren on or off. Additionally, the state attributes `duration`, `tone` and `volume_level` can be updated. Use `value_template` to transform the received state udpate to a compliant JSON payload. Attributes will only be set if the function is supported by the device and a valid value is supplied. When a non JSON payload is detected, it should be either of the `payload_on` or `payload_off` defined payloads or `None` to reset the siren's state to `unknown`. The initial state will be `unknown`. The state will be reset to `unknown` if a `None` payload or `null` JSON value is received as a state update.
    #[serde(rename = "state_topic", skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,
    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract device's state from the `state_topic`. To determine the siren's state result of this template will be compared to `state_on` and `state_off`. Alternatively `value_template` can be used to render to a valid JSON payload.
    #[serde(rename = "state_value_template", skip_serializing_if = "Option::is_none")]
    pub state_value_template: Option<String>,
    /// Set to `true` if the MQTT siren supports the `duration` service turn on parameter and enables the `duration` state attribute. (Default: `true)`
    #[serde(rename = "support_duration", skip_serializing_if = "Option::is_none")]
    pub support_duration: Option<bool>,
    /// Set to `true` if the MQTT siren supports the `volume_set` service turn on parameter and enables the `volume_level` state attribute. (Default: `true)`
    #[serde(rename = "support_volume_set", skip_serializing_if = "Option::is_none")]
    pub support_volume_set: Option<bool>,
    /// An ID that uniquely identifies this siren device. If two sirens have the same unique ID, Home Assistant will raise an exception.
    #[serde(rename = "unique_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
}

impl Siren {
    pub fn new() -> Siren {
        Siren {
            availability: None,
            availability_mode: None,
            availability_template: None,
            availability_topic: None,
            available_tones: None,
            command_template: None,
            command_off_template: None,
            command_topic: None,
            device: None,
            enabled_by_default: None,
            encoding: None,
            entity_category: None,
            json_attributes_template: None,
            json_attributes_topic: None,
            name: None,
            object_id: None,
            optimistic: None,
            payload_available: None,
            payload_not_available: None,
            payload_off: None,
            payload_on: None,
            qos: None,
            retain: None,
            state_off: None,
            state_on: None,
            state_topic: None,
            state_value_template: None,
            support_duration: None,
            support_volume_set: None,
            unique_id: None,
        }
    }
}

