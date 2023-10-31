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
pub struct Humidifier {
    /// A template to render the value received on the `action_topic` with.
    #[serde(rename = "action_template", skip_serializing_if = "Option::is_none")]
    pub action_template: Option<String>,
    /// The MQTT topic to subscribe for changes of the current action. Valid values: `off`, `humidifying`, `drying`, `idle`
    #[serde(rename = "action_topic", skip_serializing_if = "Option::is_none")]
    pub action_topic: Option<String>,
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
    /// A template with which the value received on `current_humidity_topic` will be rendered.
    #[serde(rename = "current_humidity_template", skip_serializing_if = "Option::is_none")]
    pub current_humidity_template: Option<String>,
    /// The MQTT topic on which to listen for the current humidity. A `\"None\"` value received will reset the current humidity. Empty values (`'''`) will be ignored.
    #[serde(rename = "current_humidity_topic", skip_serializing_if = "Option::is_none")]
    pub current_humidity_topic: Option<String>,
    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `command_topic`.
    #[serde(rename = "command_template", skip_serializing_if = "Option::is_none")]
    pub command_template: Option<String>,
    /// The MQTT topic to publish commands to change the humidifier state.
    #[serde(rename = "command_topic")]
    pub command_topic: String,
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: Option<Box<crate::models::FanDevice>>,
    /// The device class of the MQTT device. Must be either `humidifier`, `dehumidifier` or `null`. (Default: `humidifier)`
    #[serde(rename = "device_class", skip_serializing_if = "Option::is_none")]
    pub device_class: Option<String>,
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
    /// The minimum target humidity percentage that can be set. (Default: `100)`
    #[serde(rename = "max_humidity", skip_serializing_if = "Option::is_none")]
    pub max_humidity: Option<i32>,
    /// The maximum target humidity percentage that can be set.
    #[serde(rename = "min_humidity", skip_serializing_if = "Option::is_none")]
    pub min_humidity: Option<i32>,
    /// The name of the humidifier. Can be set to `null` if only the device name is relevant. (Default: `MQTT humidifier)`
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Used instead of `name` for automatic generation of `entity_id`
    #[serde(rename = "object_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    /// Flag that defines if humidifier works in optimistic mode (Default: ``true` if no state topic defined, else `false`.)`
    #[serde(rename = "optimistic", skip_serializing_if = "Option::is_none")]
    pub optimistic: Option<bool>,
    /// The payload that represents the available state. (Default: `online)`
    #[serde(rename = "payload_available", skip_serializing_if = "Option::is_none")]
    pub payload_available: Option<String>,
    /// The payload that represents the unavailable state. (Default: `offline)`
    #[serde(rename = "payload_not_available", skip_serializing_if = "Option::is_none")]
    pub payload_not_available: Option<String>,
    /// The payload that represents the stop state. (Default: `OFF)`
    #[serde(rename = "payload_off", skip_serializing_if = "Option::is_none")]
    pub payload_off: Option<String>,
    /// The payload that represents the running state. (Default: `ON)`
    #[serde(rename = "payload_on", skip_serializing_if = "Option::is_none")]
    pub payload_on: Option<String>,
    /// A special payload that resets the `target_humidity` state attribute to an `unknown` state when received at the `target_humidity_state_topic`. When received at `current_humidity_topic` it will reset the current humidity state. (Default: `None)`
    #[serde(rename = "payload_reset_humidity", skip_serializing_if = "Option::is_none")]
    pub payload_reset_humidity: Option<String>,
    /// A special payload that resets the `mode` state attribute to an `unknown` state when received at the `mode_state_topic`. (Default: `None)`
    #[serde(rename = "payload_reset_mode", skip_serializing_if = "Option::is_none")]
    pub payload_reset_mode: Option<String>,
    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `target_humidity_command_topic`.
    #[serde(rename = "target_humidity_command_template", skip_serializing_if = "Option::is_none")]
    pub target_humidity_command_template: Option<String>,
    /// The MQTT topic to publish commands to change the humidifier target humidity state based on a percentage.
    #[serde(rename = "target_humidity_command_topic")]
    pub target_humidity_command_topic: String,
    /// The MQTT topic subscribed to receive humidifier target humidity.
    #[serde(rename = "target_humidity_state_topic", skip_serializing_if = "Option::is_none")]
    pub target_humidity_state_topic: Option<String>,
    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract a value for the humidifier `target_humidity` state.
    #[serde(rename = "target_humidity_state_template", skip_serializing_if = "Option::is_none")]
    pub target_humidity_state_template: Option<String>,
    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `mode_command_topic`.
    #[serde(rename = "mode_command_template", skip_serializing_if = "Option::is_none")]
    pub mode_command_template: Option<String>,
    /// The MQTT topic to publish commands to change the `mode` on the humidifier. This attribute ust be configured together with the `modes` attribute.
    #[serde(rename = "mode_command_topic", skip_serializing_if = "Option::is_none")]
    pub mode_command_topic: Option<String>,
    /// The MQTT topic subscribed to receive the humidifier `mode`.
    #[serde(rename = "mode_state_topic", skip_serializing_if = "Option::is_none")]
    pub mode_state_topic: Option<String>,
    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract a value for the humidifier `mode` state.
    #[serde(rename = "mode_state_template", skip_serializing_if = "Option::is_none")]
    pub mode_state_template: Option<String>,
    #[serde(rename = "modes", skip_serializing_if = "Option::is_none")]
    pub modes: Option<Box<crate::models::FanPresetModes>>,
    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<i32>,
    /// If the published message should have the retain flag on or not. (Default: `true)`
    #[serde(rename = "retain", skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,
    /// The MQTT topic subscribed to receive state updates.
    #[serde(rename = "state_topic", skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,
    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract a value from the state.
    #[serde(rename = "state_value_template", skip_serializing_if = "Option::is_none")]
    pub state_value_template: Option<String>,
    /// An ID that uniquely identifies this humidifier. If two humidifiers have the same unique ID, Home Assistant will raise an exception.
    #[serde(rename = "unique_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
}

impl Humidifier {
    pub fn new(command_topic: String, target_humidity_command_topic: String) -> Humidifier {
        Humidifier {
            action_template: None,
            action_topic: None,
            availability: None,
            availability_mode: None,
            availability_template: None,
            availability_topic: None,
            current_humidity_template: None,
            current_humidity_topic: None,
            command_template: None,
            command_topic,
            device: None,
            device_class: None,
            enabled_by_default: None,
            encoding: None,
            entity_category: None,
            json_attributes_template: None,
            json_attributes_topic: None,
            max_humidity: None,
            min_humidity: None,
            name: None,
            object_id: None,
            optimistic: None,
            payload_available: None,
            payload_not_available: None,
            payload_off: None,
            payload_on: None,
            payload_reset_humidity: None,
            payload_reset_mode: None,
            target_humidity_command_template: None,
            target_humidity_command_topic,
            target_humidity_state_topic: None,
            target_humidity_state_template: None,
            mode_command_template: None,
            mode_command_topic: None,
            mode_state_topic: None,
            mode_state_template: None,
            modes: None,
            qos: None,
            retain: None,
            state_topic: None,
            state_value_template: None,
            unique_id: None,
        }
    }
}

