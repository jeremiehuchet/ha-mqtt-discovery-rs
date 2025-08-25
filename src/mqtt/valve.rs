use super::common::Qos;
use super::common::{Availability, Device, EntityCategory, Origin};
use crate::Entity;
use serde_derive::Serialize;

/// ---
/// title: "MQTT Valve"
/// description: "Instructions on how to integrate MQTT valves into Home Assistant."
/// ha_category:
///   - Valve
/// ha_iot_class: Configurable
/// ha_release: 2024.1
/// ha_domain: mqtt
/// ---
///
/// The `mqtt` valve platform allows you to control an MQTT valve (such a gas or water valve).
///
/// ## Configuration
///
/// A valve entity can be have the following states: `open`, `opening`, `closed` or `closing`.
///
/// ### Valve controlled by states
///
/// If a `state_topic` is configured, the entity's state will be updated only after an MQTT message is received on `state_topic` matching `state_open`, `state_opening`, `state_closed` or `state_closing`. Commands configured through `payload_open`, `payload_closed`, and `payload_stop` will be published to `command_topic` to control the valve.
///
/// To use an MQTT valve in your installation, add the following to your `configuration.yaml` file.
/// {% include integrations/restart_ha_after_config_inclusion.md %}
///
/// ```yaml
/// # Example configuration.yaml entry for a value that is set by open or close command
/// mqtt:
///   - valve:
///       command_topic: "heater/valve/set"
///       state_topic: "heater/valve/state"
/// ```
///
/// Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
///
/// ### Valve controlled by position
///
/// If the valve supports reporting its position (the `reports_position` config option is set to `true`), a numeric state is expected on `state_topic`, but state updates are still allowed for `state_opening` and `state_closing`. Also, a JSON format is supported. It allows both `state` and `position` to be reported together.
///
/// Example of a JSON state update:
///
/// ```json
/// {"state": "opening", "position": 10}
/// ```
///
/// The wanted position value or `payload_stop` will be published to `command_topic` to control the valve when the actions `valve.open`, `value.close`, or `value.set_position` are called.
///
/// To use your MQTT valve in your installation, add the following to your `configuration.yaml` file:
///
/// ```yaml
/// # Example configuration.yaml entry for a valve that reports position
/// mqtt:
///   - valve:
///       command_topic: "heater/valve/set"
///       state_topic: "heater/valve/state"
///       reports_position: true
/// ```
///
/// ### Optimistic operation
///
/// If a `state_topic` is not defined, the valve will work in optimistic mode. In this mode, the valve will immediately change state (`open` or `closed`) after every command sent by Home Assistant. It won't wait for an update from the device. Optimistic mode can be forced by setting `optimistic` to `true`, even if a `state_topic` is defined. Try to enable it if you are experiencing an incorrect valve operation.
///
///
///
/// 🛈 Note\
/// MQTT valve expects position values to be in the range of 0 to 100, where 0 indicates a closed position and 100 indicates a fully open position.
/// If `position_open` or `position_closed` are set to a different range (for example, 40 to 140), when sending a command to the device, the range will be adjusted to the device range. For example, position 0 will send a value of 40 to device. When the device receives a position payload, it will be adjusted back to the 0 to 100 range. In our example, the device value of 40 will report valve position 0.
/// `position_open` and `position_closed` can also be used to reverse the direction of the device: If `position_closed` is set to 100 and `position_open` is set to `0`, the device operation will be inverted. For example, when setting the position to 40, a value of 60 will be sent to the device.
///
/// ## Examples
///
/// This section provides some examples showing how you can use this platform.
///
/// ### Full configuration for a value that does not report position
///
/// The example below shows a full configuration for a valve that does not report position.
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - valve:
///       name: "MQTT valve"
///       command_template: '{"x": {{ value }} }'
///       command_topic: "heater/valve/set"
///       state_topic: "heater/valve/state"
///       availability:
///         - topic: "heater/valve/availability"
///       qos: 0
///       reports_position: false
///       retain: true
///       payload_open: "OPEN"
///       payload_close: "CLOSE"
///       payload_stop: "STOP"
///       state_open: "open"
///       state_opening: "opening"
///       state_closed: "closed"
///       state_closing: "closing"
///       payload_available: "online"
///       payload_not_available: "offline"
///       optimistic: false
///       value_template: "{{ value_json.x }}"
/// ```
///
///
/// ### Sample configuration of a valve that reports the position
///
/// The example below shows a sample configuration for a valve that reports the position using JSON messages.
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - valve:
///       name: "MQTT valve"
///       command_template: '{"x": {{ value }} }'
///       command_topic: "heater/valve/set"
///       state_topic: "heater/valve/state"
///       availability:
///         - topic: "heater/valve/availability"
///       reports_position: true
///       value_template: "{{ value_json.x }}"
/// ```
///
///
/// ### Configuration for disabling valve commands
///
/// The example below shows a configuration for a valve that does not have a close command.
/// Setting the `payload_close` to empty or to `null` disables the close command and will not show the close button.
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - valve:
///       payload_open: "on"
///       payload_close:
///       payload_stop: "on"
/// ```
///
///
/// An MQTT valve will support `open` and `close` commands if a `command_topic` is set. The MQTT valve supports `stop` if `payload_stop` is set.
///
/// ### Testing your configuration
///
/// To test, you can use the command line tool `mosquitto_pub` shipped with `mosquitto` or the `mosquitto-clients` package to send MQTT messages. This allows you to operate your valve manually:
///
/// ```bash
/// mosquitto_pub -h 127.0.0.1 -t home-assistant/valve/set -m "CLOSE"
/// ```
///
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Valve {
    /// Replaces `~` with this value in any MQTT topic attribute.
    /// [See Home Assistant documentation](https://www.home-assistant.io/integrations/mqtt/#using-abbreviations-and-base-topic)
    #[serde(rename = "~", skip_serializing_if = "Option::is_none")]
    pub topic_prefix: Option<String>,

    /// It is encouraged to add additional information about the origin that supplies MQTT entities via MQTT discovery by adding the origin option (can be abbreviated to o) to the discovery payload. Note that these options also support abbreviations. Information of the origin will be logged to the core event log when an item is discovered or updated.
    #[serde(rename = "o")]
    pub origin: Origin,

    /// Information about the device this button is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/en/device_registry_index.html). Only works when [`unique_id`](#unique_id) is set. At least one of identifiers or connections must be present to identify the device.
    #[serde(rename = "dev")]
    pub device: Device,

    /// A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`.
    #[serde(flatten)]
    pub availability: Availability,

    /// The category of the entity. (optional, default: None)
    #[serde(rename = "ent_cat", skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<EntityCategory>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `command_topic`.
    #[serde(rename = "cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub command_template: Option<String>,

    /// The MQTT topic to publish commands to control the valve. The value sent can be a value defined by `payload_open`, `payload_close` or `payload_stop`. If `reports_position` is set to `true`, a numeric value will be published instead.
    #[serde(rename = "cmd_t", skip_serializing_if = "Option::is_none")]
    pub command_topic: Option<String>,

    /// Sets the [class of the device](/integrations/valve/#device_class), changing the device state and icon that is displayed on the frontend. The `device_class` can be `null`.
    #[serde(rename = "dev_cla", skip_serializing_if = "Option::is_none")]
    pub device_class: Option<String>,

    /// Flag which defines if the entity should be enabled when first added.
    #[serde(rename = "en", skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,

    /// The encoding of the payloads received and published messages. Set to `""` to disable decoding of incoming payload.
    #[serde(rename = "e", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,

    /// Picture URL for the entity.
    #[serde(rename = "ent_pic", skip_serializing_if = "Option::is_none")]
    pub entity_picture: Option<String>,

    /// [Icon](/docs/configuration/customizing-devices/#icon) for the entity.
    #[serde(rename = "ic", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the JSON dictionary from messages received on the `json_attributes_topic`. A usage example can be found in the [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation.
    #[serde(rename = "json_attr_tpl", skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. A usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation.
    #[serde(rename = "json_attr_t", skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,

    /// The name of the valve. Can be set to `null` if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used instead of `name` to have the `entity_id` generated automatically.
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// Flag that defines if a switch works in optimistic mode.
    #[serde(rename = "opt", skip_serializing_if = "Option::is_none")]
    pub optimistic: Option<bool>,

    /// The command payload that closes the valve. Is only used when `reports_position` is set to `false` (default). The `payload_close` is not allowed if `reports_position` is set to `true`. Can be set to `null` to disable the valve's close option.
    #[serde(rename = "pl_cls", skip_serializing_if = "Option::is_none")]
    pub payload_close: Option<String>,

    /// The command payload that opens the valve. Is only used when `reports_position` is set to `false` (default). The `payload_open` is not allowed if `reports_position` is set to `true`. Can be set to `null` to disable the valve's open option.
    #[serde(rename = "pl_open", skip_serializing_if = "Option::is_none")]
    pub payload_open: Option<String>,

    /// The command payload that stops the valve. When not configured, the valve will not support the `valve.stop` action.
    #[serde(rename = "pl_stop", skip_serializing_if = "Option::is_none")]
    pub payload_stop: Option<String>,

    /// Must be `valve`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    #[serde(rename = "p")]
    pub platform: String,

    /// Number which represents closed position. The valve's position will be scaled to the(`position_closed`...`position_open`) range when an action is performed and scaled back when a value is received.
    #[serde(rename = "pos_clsd", skip_serializing_if = "Option::is_none")]
    pub position_closed: Option<i32>,

    /// Number which represents open position. The valve's position will be scaled to (`position_closed`...`position_open`) range when an is performed and scaled back when a value is received.
    #[serde(rename = "pos_open", skip_serializing_if = "Option::is_none")]
    pub position_open: Option<i32>,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// Set to `true` if the value reports the position or supports setting the position. Enabling the `reports_position` option will cause the position to be published instead of a payload defined by `payload_open`, `payload_close` or `payload_stop`. When receiving messages, `state_topic` will accept numeric payloads or one of the following state messages: `open`, `opening`, `closed`, or `closing`.
    #[serde(rename = "pos", skip_serializing_if = "Option::is_none")]
    pub reports_position: Option<bool>,

    /// Defines if published messages should have the retain flag set.
    #[serde(rename = "ret", skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,

    /// The payload that represents the closed state. Is only allowed when `reports_position` is set to `False` (default).
    #[serde(rename = "stat_clsd", skip_serializing_if = "Option::is_none")]
    pub state_closed: Option<String>,

    /// The payload that represents the closing state.
    #[serde(rename = "stat_closing", skip_serializing_if = "Option::is_none")]
    pub state_closing: Option<String>,

    /// The payload that represents the open state. Is only allowed when `reports_position` is set to `False` (default).
    #[serde(rename = "stat_open", skip_serializing_if = "Option::is_none")]
    pub state_open: Option<String>,

    /// The payload that represents the opening state.
    #[serde(rename = "stat_opening", skip_serializing_if = "Option::is_none")]
    pub state_opening: Option<String>,

    /// The MQTT topic subscribed to receive valve state messages. State topic accepts a state payload (`open`, `opening`, `closed`, or `closing`) or, if `reports_position` is supported, a numeric value representing the position. In a JSON format with variables `state` and `position` both values can received together. A "None" state value resets to an `unknown` state. An empty string is ignored.
    #[serde(rename = "stat_t", skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,

    /// An ID that uniquely identifies this valve. If two valves have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) that can be used to extract the payload for the `state_topic` topic. The rendered value should be a defined state payload or, if reporting a `position` is supported and `reports_position` is set to `true`, a numeric value is expected representing the position. See also `state_topic`.
    #[serde(rename = "val_tpl", skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

impl Valve {
    /// Replaces `~` with this value in any MQTT topic attribute.
    /// [See Home Assistant documentation](https://www.home-assistant.io/integrations/mqtt/#using-abbreviations-and-base-topic)
    pub fn topic_prefix<S: Into<String>>(mut self, topic_prefix: S) -> Self {
        self.topic_prefix = Some(topic_prefix.into());
        self
    }

    /// It is encouraged to add additional information about the origin that supplies MQTT entities via MQTT discovery by adding the origin option (can be abbreviated to o) to the discovery payload. Note that these options also support abbreviations. Information of the origin will be logged to the core event log when an item is discovered or updated.
    pub fn origin(mut self, origin: Origin) -> Self {
        self.origin = origin;
        self
    }

    /// Information about the device this sensor is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/device_registry_index/). Only works when `unique_id` is set. At least one of identifiers or connections must be present to identify the device.
    pub fn device(mut self, device: Device) -> Self {
        self.device = device;
        self
    }

    /// The category of the entity. (optional, default: None)
    pub fn entity_category(mut self, entity_category: EntityCategory) -> Self {
        self.entity_category = Some(entity_category);
        self
    }

    /// Defines how HA will check for entity availability.
    pub fn availability(mut self, availability: Availability) -> Self {
        self.availability = availability;
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `command_topic`.
    pub fn command_template<T: Into<String>>(mut self, command_template: T) -> Self {
        self.command_template = Some(command_template.into());
        self
    }

    /// The MQTT topic to publish commands to control the valve. The value sent can be a value defined by `payload_open`, `payload_close` or `payload_stop`. If `reports_position` is set to `true`, a numeric value will be published instead.
    pub fn command_topic<T: Into<String>>(mut self, command_topic: T) -> Self {
        self.command_topic = Some(command_topic.into());
        self
    }

    /// Sets the [class of the device](/integrations/valve/#device_class), changing the device state and icon that is displayed on the frontend. The `device_class` can be `null`.
    pub fn device_class<T: Into<String>>(mut self, device_class: T) -> Self {
        self.device_class = Some(device_class.into());
        self
    }

    /// Flag which defines if the entity should be enabled when first added.
    pub fn enabled_by_default(mut self, enabled_by_default: bool) -> Self {
        self.enabled_by_default = Some(enabled_by_default);
        self
    }

    /// The encoding of the payloads received and published messages. Set to `""` to disable decoding of incoming payload.
    pub fn encoding<T: Into<String>>(mut self, encoding: T) -> Self {
        self.encoding = Some(encoding.into());
        self
    }

    /// Picture URL for the entity.
    pub fn entity_picture<T: Into<String>>(mut self, entity_picture: T) -> Self {
        self.entity_picture = Some(entity_picture.into());
        self
    }

    /// [Icon](/docs/configuration/customizing-devices/#icon) for the entity.
    pub fn icon<T: Into<String>>(mut self, icon: T) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the JSON dictionary from messages received on the `json_attributes_topic`. A usage example can be found in the [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation.
    pub fn json_attributes_template<T: Into<String>>(
        mut self,
        json_attributes_template: T,
    ) -> Self {
        self.json_attributes_template = Some(json_attributes_template.into());
        self
    }

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. A usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation.
    pub fn json_attributes_topic<T: Into<String>>(mut self, json_attributes_topic: T) -> Self {
        self.json_attributes_topic = Some(json_attributes_topic.into());
        self
    }

    /// The name of the valve. Can be set to `null` if only the device name is relevant.
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Used instead of `name` to have the `entity_id` generated automatically.
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    /// Flag that defines if a switch works in optimistic mode.
    pub fn optimistic(mut self, optimistic: bool) -> Self {
        self.optimistic = Some(optimistic);
        self
    }

    /// The command payload that closes the valve. Is only used when `reports_position` is set to `false` (default). The `payload_close` is not allowed if `reports_position` is set to `true`. Can be set to `null` to disable the valve's close option.
    pub fn payload_close<T: Into<String>>(mut self, payload_close: T) -> Self {
        self.payload_close = Some(payload_close.into());
        self
    }

    /// The command payload that opens the valve. Is only used when `reports_position` is set to `false` (default). The `payload_open` is not allowed if `reports_position` is set to `true`. Can be set to `null` to disable the valve's open option.
    pub fn payload_open<T: Into<String>>(mut self, payload_open: T) -> Self {
        self.payload_open = Some(payload_open.into());
        self
    }

    /// The command payload that stops the valve. When not configured, the valve will not support the `valve.stop` action.
    pub fn payload_stop<T: Into<String>>(mut self, payload_stop: T) -> Self {
        self.payload_stop = Some(payload_stop.into());
        self
    }

    /// Must be `valve`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    pub fn platform<T: Into<String>>(mut self, platform: T) -> Self {
        self.platform = platform.into();
        self
    }

    /// Number which represents closed position. The valve's position will be scaled to the(`position_closed`...`position_open`) range when an action is performed and scaled back when a value is received.
    pub fn position_closed(mut self, position_closed: i32) -> Self {
        self.position_closed = Some(position_closed);
        self
    }

    /// Number which represents open position. The valve's position will be scaled to (`position_closed`...`position_open`) range when an is performed and scaled back when a value is received.
    pub fn position_open(mut self, position_open: i32) -> Self {
        self.position_open = Some(position_open);
        self
    }

    /// The maximum QoS level to be used when receiving and publishing messages.
    pub fn qos(mut self, qos: Qos) -> Self {
        self.qos = Some(qos);
        self
    }

    /// Set to `true` if the value reports the position or supports setting the position. Enabling the `reports_position` option will cause the position to be published instead of a payload defined by `payload_open`, `payload_close` or `payload_stop`. When receiving messages, `state_topic` will accept numeric payloads or one of the following state messages: `open`, `opening`, `closed`, or `closing`.
    pub fn reports_position(mut self, reports_position: bool) -> Self {
        self.reports_position = Some(reports_position);
        self
    }

    /// Defines if published messages should have the retain flag set.
    pub fn retain(mut self, retain: bool) -> Self {
        self.retain = Some(retain);
        self
    }

    /// The payload that represents the closed state. Is only allowed when `reports_position` is set to `False` (default).
    pub fn state_closed<T: Into<String>>(mut self, state_closed: T) -> Self {
        self.state_closed = Some(state_closed.into());
        self
    }

    /// The payload that represents the closing state.
    pub fn state_closing<T: Into<String>>(mut self, state_closing: T) -> Self {
        self.state_closing = Some(state_closing.into());
        self
    }

    /// The payload that represents the open state. Is only allowed when `reports_position` is set to `False` (default).
    pub fn state_open<T: Into<String>>(mut self, state_open: T) -> Self {
        self.state_open = Some(state_open.into());
        self
    }

    /// The payload that represents the opening state.
    pub fn state_opening<T: Into<String>>(mut self, state_opening: T) -> Self {
        self.state_opening = Some(state_opening.into());
        self
    }

    /// The MQTT topic subscribed to receive valve state messages. State topic accepts a state payload (`open`, `opening`, `closed`, or `closing`) or, if `reports_position` is supported, a numeric value representing the position. In a JSON format with variables `state` and `position` both values can received together. A "None" state value resets to an `unknown` state. An empty string is ignored.
    pub fn state_topic<T: Into<String>>(mut self, state_topic: T) -> Self {
        self.state_topic = Some(state_topic.into());
        self
    }

    /// An ID that uniquely identifies this valve. If two valves have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    pub fn unique_id<T: Into<String>>(mut self, unique_id: T) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) that can be used to extract the payload for the `state_topic` topic. The rendered value should be a defined state payload or, if reporting a `position` is supported and `reports_position` is set to `true`, a numeric value is expected representing the position. See also `state_topic`.
    pub fn value_template<T: Into<String>>(mut self, value_template: T) -> Self {
        self.value_template = Some(value_template.into());
        self
    }
}

impl Default for Valve {
    fn default() -> Self {
        Self {
            topic_prefix: Default::default(),
            origin: Default::default(),
            device: Default::default(),
            entity_category: Default::default(),
            availability: Default::default(),
            command_template: Default::default(),
            command_topic: Default::default(),
            device_class: Default::default(),
            enabled_by_default: Default::default(),
            encoding: Default::default(),
            entity_picture: Default::default(),
            icon: Default::default(),
            json_attributes_template: Default::default(),
            json_attributes_topic: Default::default(),
            name: Default::default(),
            object_id: Default::default(),
            optimistic: Default::default(),
            payload_close: Default::default(),
            payload_open: Default::default(),
            payload_stop: Default::default(),
            platform: "valve".to_string(),
            position_closed: Default::default(),
            position_open: Default::default(),
            qos: Default::default(),
            reports_position: Default::default(),
            retain: Default::default(),
            state_closed: Default::default(),
            state_closing: Default::default(),
            state_open: Default::default(),
            state_opening: Default::default(),
            state_topic: Default::default(),
            unique_id: Default::default(),
            value_template: Default::default(),
        }
    }
}

impl From<Valve> for Entity {
    fn from(value: Valve) -> Self {
        Entity::Valve(value)
    }
}
