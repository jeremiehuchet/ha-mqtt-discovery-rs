use super::common::Qos;
use super::common::TemperatureUnit;
use super::common::{Availability, Device, EntityCategory, Origin};
use crate::Entity;
pub use rust_decimal::Decimal;
use serde_derive::Serialize;

/// ---
/// title: "MQTT water heater"
/// description: "Instructions on how to integrate MQTT water heater into Home Assistant."
/// ha_category:
///   - Water heater
/// ha_release: 2023.7
/// ha_iot_class: Local Polling
/// ha_domain: mqtt
/// ---
///
/// The `mqtt` water heater platform lets you control your MQTT enabled water heater devices.
///
/// ## Configuration
///
/// To use an MQTT water heater in your installation, add the following to your `configuration.yaml` file.
/// {% include integrations/restart_ha_after_config_inclusion.md %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - water_heater:
///       name: Boiler
///       mode_command_topic: "basement/boiler/mode/set"
/// ```
///
/// Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
///
///
/// ## Optimistic mode
///
/// If a property works in *optimistic mode* (when the corresponding state topic is not set), Home Assistant will assume that any state changes published to the command topics did work and change the internal state of the `entity %} immediately after publishing to the command topic. If it does not work in optimistic mode, the internal state of the {% term entity` is only updated when the requested update is confirmed by the device through the state topic. You can enforce optimistic mode by setting the `optimistic` option to `true`. When set, the internal state will always be updated, even when a state topic is defined.
///
/// ## Using templates
///
/// For all `*_state_topic`s, a template can be specified that will be used to render the incoming payloads on these topics. Also, a default template that applies to all state topics can be specified as `value_template`. This can be useful if you received payloads are e.g., in JSON format. Since in JSON, a quoted string (e.g., `"foo"`) is just a string, this can also be used for unquoting.
///
/// Say you receive the operation mode `"off"` via your `mode_state_topic`, but the mode is actually called just `off`, here's what you could do:
///
///
/// ```yaml
/// mqtt:
///   - water_heater:
///       name: Boiler
///       modes:
///         - "off"
///         - "eco"
///         - "performance"
///       mode_command_topic: "basement/boiler/mode/set"
///       mode_state_topic: "basement/boiler/mode/state"
///       mode_state_template: "{{ value_json }}"
/// ```
///
///
/// This will parse the incoming `"off"` as JSON, resulting in `off`. Obviously, in this case you could also just set `value_template: {% raw %}"{{ value_json }}"{% endraw %}`.
///
/// Similarly for `*_command_topic`s, a template can be specified to render the outgoing payloads on these topics.
///
/// ## Example
///
/// A full configuration example looks like the one below.
///
///
/// ```yaml
/// # Full example configuration.yaml entry
/// mqtt:
///   - water_heater:
///       name: Boiler
///       modes:
///         - "off"
///         - "eco"
///         - "performance"
///       mode_state_topic: "basement/boiler/mode"
///       mode_command_topic: "basement/boiler/mode/set"
///       mode_command_template: "{{ value if value=="off" else "on" }}"
///       temperature_state_topic: "basement/boiler/temperature"
///       temperature_command_topic: "basement/boiler/temperature/set"
///       current_temperature_topic: "basement/boiler/current_temperature"
///       precision: 1.0
/// ```
///
///
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct WaterHeater {
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

    /// A template with which the value received on `current_temperature_topic` will be rendered.
    #[serde(rename = "curr_temp_tpl", skip_serializing_if = "Option::is_none")]
    pub current_temperature_template: Option<String>,

    /// The MQTT topic on which to listen for the current temperature. A `"None"` value received will reset the current temperature. Empty values (`'''`) will be ignored.
    #[serde(rename = "curr_temp_t", skip_serializing_if = "Option::is_none")]
    pub current_temperature_topic: Option<String>,

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

    /// Set the initial target temperature. The default value depends on the temperature unit, and will be 43.3°C or 110°F.
    #[serde(rename = "init", skip_serializing_if = "Option::is_none")]
    pub initial: Option<i32>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation.
    #[serde(rename = "json_attr_tpl", skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation.
    #[serde(rename = "json_attr_t", skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,

    /// Maximum set point available. The default value depends on the temperature unit, and will be 60°C or 140°F.
    #[serde(rename = "max_temp", skip_serializing_if = "Option::is_none")]
    pub max_temp: Option<Decimal>,

    /// Minimum set point available. The default value depends on the temperature unit, and will be 43.3°C or 110°F.
    #[serde(rename = "min_temp", skip_serializing_if = "Option::is_none")]
    pub min_temp: Option<Decimal>,

    /// A template to render the value sent to the `mode_command_topic` with.
    #[serde(rename = "mode_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub mode_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the water heater operation mode.
    #[serde(rename = "mode_cmd_t", skip_serializing_if = "Option::is_none")]
    pub mode_command_topic: Option<String>,

    /// A template to render the value received on the `mode_state_topic` with.
    #[serde(rename = "mode_stat_tpl", skip_serializing_if = "Option::is_none")]
    pub mode_state_template: Option<String>,

    /// The MQTT topic to subscribe for changes of the water heater operation mode. If this is not set, the operation mode works in optimistic mode (see below). A "None" payload resets to an `unknown` state. An empty payload is ignored.
    #[serde(rename = "mode_stat_t", skip_serializing_if = "Option::is_none")]
    pub mode_state_topic: Option<String>,

    /// A list of supported modes. Needs to be a subset of the default values.
    #[serde(rename = "modes", skip_serializing_if = "Option::is_none")]
    pub modes: Option<Vec<String>>,

    /// The name of the water heater. Can be set to `null` if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// Flag that defines if the water heater works in optimistic mode
    #[serde(rename = "opt", skip_serializing_if = "Option::is_none")]
    pub optimistic: Option<bool>,

    /// The payload that represents disabled state.
    #[serde(rename = "pl_off", skip_serializing_if = "Option::is_none")]
    pub payload_off: Option<String>,

    /// The payload that represents enabled state.
    #[serde(rename = "pl_on", skip_serializing_if = "Option::is_none")]
    pub payload_on: Option<String>,

    /// Must be `water_heater`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    #[serde(rename = "p")]
    pub platform: String,

    /// A template to render the value sent to the `power_command_topic` with. The `value` parameter is the payload set for `payload_on` or `payload_off`.
    #[serde(rename = "pow_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub power_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the water heater power state. Sends the payload configured with `payload_on` if the water heater is turned on via the `water_heater.turn_on`, or the payload configured with `payload_off` if the water heater is turned off via the `water_heater.turn_off` action. Note that `optimistic` mode is not supported through `water_heater.turn_on` and `water_heater.turn_off` actions. When called, these actions will send a power command to the device but will not optimistically update the state of the water heater. The water heater device should report its state back via `mode_state_topic`.
    #[serde(rename = "pow_cmd_t", skip_serializing_if = "Option::is_none")]
    pub power_command_topic: Option<String>,

    /// The desired precision for this device. Can be used to match your actual water heater's precision. Supported values are `0.1`, `0.5` and `1.0`.
    #[serde(rename = "precision", skip_serializing_if = "Option::is_none")]
    pub precision: Option<Decimal>,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// Defines if published messages should have the retain flag set.
    #[serde(rename = "ret", skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,

    /// A template to render the value sent to the `temperature_command_topic` with.
    #[serde(rename = "temp_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub temperature_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the target temperature.
    #[serde(rename = "temp_cmd_t", skip_serializing_if = "Option::is_none")]
    pub temperature_command_topic: Option<String>,

    /// A template to render the value received on the `temperature_state_topic` with.
    #[serde(rename = "temp_stat_tpl", skip_serializing_if = "Option::is_none")]
    pub temperature_state_template: Option<String>,

    /// The MQTT topic to subscribe for changes in the target temperature. If this is not set, the target temperature works in optimistic mode (see below). A `"None"` value received will reset the temperature set point. Empty values (`'''`) will be ignored.
    #[serde(rename = "temp_stat_t", skip_serializing_if = "Option::is_none")]
    pub temperature_state_topic: Option<String>,

    /// Defines the temperature unit of the device, `C` or `F`. If this is not set, the temperature unit is set to the system temperature unit.
    #[serde(rename = "temp_unit", skip_serializing_if = "Option::is_none")]
    pub temperature_unit: Option<TemperatureUnit>,

    /// An ID that uniquely identifies this water heater device. If two water heater devices have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,

    /// Default template to render the payloads on *all* `*_state_topic`s with.
    #[serde(rename = "val_tpl", skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

impl WaterHeater {
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

    /// A template with which the value received on `current_temperature_topic` will be rendered.
    pub fn current_temperature_template<T: Into<String>>(
        mut self,
        current_temperature_template: T,
    ) -> Self {
        self.current_temperature_template = Some(current_temperature_template.into());
        self
    }

    /// The MQTT topic on which to listen for the current temperature. A `"None"` value received will reset the current temperature. Empty values (`'''`) will be ignored.
    pub fn current_temperature_topic<T: Into<String>>(
        mut self,
        current_temperature_topic: T,
    ) -> Self {
        self.current_temperature_topic = Some(current_temperature_topic.into());
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

    /// Set the initial target temperature. The default value depends on the temperature unit, and will be 43.3°C or 110°F.
    pub fn initial(mut self, initial: i32) -> Self {
        self.initial = Some(initial);
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation.
    pub fn json_attributes_template<T: Into<String>>(
        mut self,
        json_attributes_template: T,
    ) -> Self {
        self.json_attributes_template = Some(json_attributes_template.into());
        self
    }

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation.
    pub fn json_attributes_topic<T: Into<String>>(mut self, json_attributes_topic: T) -> Self {
        self.json_attributes_topic = Some(json_attributes_topic.into());
        self
    }

    /// Maximum set point available. The default value depends on the temperature unit, and will be 60°C or 140°F.
    pub fn max_temp(mut self, max_temp: Decimal) -> Self {
        self.max_temp = Some(max_temp);
        self
    }

    /// Minimum set point available. The default value depends on the temperature unit, and will be 43.3°C or 110°F.
    pub fn min_temp(mut self, min_temp: Decimal) -> Self {
        self.min_temp = Some(min_temp);
        self
    }

    /// A template to render the value sent to the `mode_command_topic` with.
    pub fn mode_command_template<T: Into<String>>(mut self, mode_command_template: T) -> Self {
        self.mode_command_template = Some(mode_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the water heater operation mode.
    pub fn mode_command_topic<T: Into<String>>(mut self, mode_command_topic: T) -> Self {
        self.mode_command_topic = Some(mode_command_topic.into());
        self
    }

    /// A template to render the value received on the `mode_state_topic` with.
    pub fn mode_state_template<T: Into<String>>(mut self, mode_state_template: T) -> Self {
        self.mode_state_template = Some(mode_state_template.into());
        self
    }

    /// The MQTT topic to subscribe for changes of the water heater operation mode. If this is not set, the operation mode works in optimistic mode (see below). A "None" payload resets to an `unknown` state. An empty payload is ignored.
    pub fn mode_state_topic<T: Into<String>>(mut self, mode_state_topic: T) -> Self {
        self.mode_state_topic = Some(mode_state_topic.into());
        self
    }

    /// A list of supported modes. Needs to be a subset of the default values.
    pub fn modes<T: Into<String>>(mut self, modes: Vec<T>) -> Self {
        self.modes = Some(modes.into_iter().map(|v| v.into()).collect());
        self
    }

    /// The name of the water heater. Can be set to `null` if only the device name is relevant.
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    /// Flag that defines if the water heater works in optimistic mode
    pub fn optimistic(mut self, optimistic: bool) -> Self {
        self.optimistic = Some(optimistic);
        self
    }

    /// The payload that represents disabled state.
    pub fn payload_off<T: Into<String>>(mut self, payload_off: T) -> Self {
        self.payload_off = Some(payload_off.into());
        self
    }

    /// The payload that represents enabled state.
    pub fn payload_on<T: Into<String>>(mut self, payload_on: T) -> Self {
        self.payload_on = Some(payload_on.into());
        self
    }

    /// Must be `water_heater`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    pub fn platform<T: Into<String>>(mut self, platform: T) -> Self {
        self.platform = platform.into();
        self
    }

    /// A template to render the value sent to the `power_command_topic` with. The `value` parameter is the payload set for `payload_on` or `payload_off`.
    pub fn power_command_template<T: Into<String>>(mut self, power_command_template: T) -> Self {
        self.power_command_template = Some(power_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the water heater power state. Sends the payload configured with `payload_on` if the water heater is turned on via the `water_heater.turn_on`, or the payload configured with `payload_off` if the water heater is turned off via the `water_heater.turn_off` action. Note that `optimistic` mode is not supported through `water_heater.turn_on` and `water_heater.turn_off` actions. When called, these actions will send a power command to the device but will not optimistically update the state of the water heater. The water heater device should report its state back via `mode_state_topic`.
    pub fn power_command_topic<T: Into<String>>(mut self, power_command_topic: T) -> Self {
        self.power_command_topic = Some(power_command_topic.into());
        self
    }

    /// The desired precision for this device. Can be used to match your actual water heater's precision. Supported values are `0.1`, `0.5` and `1.0`.
    pub fn precision(mut self, precision: Decimal) -> Self {
        self.precision = Some(precision);
        self
    }

    /// The maximum QoS level to be used when receiving and publishing messages.
    pub fn qos(mut self, qos: Qos) -> Self {
        self.qos = Some(qos);
        self
    }

    /// Defines if published messages should have the retain flag set.
    pub fn retain(mut self, retain: bool) -> Self {
        self.retain = Some(retain);
        self
    }

    /// A template to render the value sent to the `temperature_command_topic` with.
    pub fn temperature_command_template<T: Into<String>>(
        mut self,
        temperature_command_template: T,
    ) -> Self {
        self.temperature_command_template = Some(temperature_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the target temperature.
    pub fn temperature_command_topic<T: Into<String>>(
        mut self,
        temperature_command_topic: T,
    ) -> Self {
        self.temperature_command_topic = Some(temperature_command_topic.into());
        self
    }

    /// A template to render the value received on the `temperature_state_topic` with.
    pub fn temperature_state_template<T: Into<String>>(
        mut self,
        temperature_state_template: T,
    ) -> Self {
        self.temperature_state_template = Some(temperature_state_template.into());
        self
    }

    /// The MQTT topic to subscribe for changes in the target temperature. If this is not set, the target temperature works in optimistic mode (see below). A `"None"` value received will reset the temperature set point. Empty values (`'''`) will be ignored.
    pub fn temperature_state_topic<T: Into<String>>(mut self, temperature_state_topic: T) -> Self {
        self.temperature_state_topic = Some(temperature_state_topic.into());
        self
    }

    /// Defines the temperature unit of the device, `C` or `F`. If this is not set, the temperature unit is set to the system temperature unit.
    pub fn temperature_unit<T: Into<TemperatureUnit>>(mut self, temperature_unit: T) -> Self {
        self.temperature_unit = Some(temperature_unit.into());
        self
    }

    /// An ID that uniquely identifies this water heater device. If two water heater devices have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    pub fn unique_id<T: Into<String>>(mut self, unique_id: T) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }

    /// Default template to render the payloads on *all* `*_state_topic`s with.
    pub fn value_template<T: Into<String>>(mut self, value_template: T) -> Self {
        self.value_template = Some(value_template.into());
        self
    }
}

impl Default for WaterHeater {
    fn default() -> Self {
        Self {
            topic_prefix: Default::default(),
            origin: Default::default(),
            device: Default::default(),
            entity_category: Default::default(),
            availability: Default::default(),
            current_temperature_template: Default::default(),
            current_temperature_topic: Default::default(),
            enabled_by_default: Default::default(),
            encoding: Default::default(),
            entity_picture: Default::default(),
            icon: Default::default(),
            initial: Default::default(),
            json_attributes_template: Default::default(),
            json_attributes_topic: Default::default(),
            max_temp: Default::default(),
            min_temp: Default::default(),
            mode_command_template: Default::default(),
            mode_command_topic: Default::default(),
            mode_state_template: Default::default(),
            mode_state_topic: Default::default(),
            modes: Default::default(),
            name: Default::default(),
            object_id: Default::default(),
            optimistic: Default::default(),
            payload_off: Default::default(),
            payload_on: Default::default(),
            platform: "water_heater".to_string(),
            power_command_template: Default::default(),
            power_command_topic: Default::default(),
            precision: Default::default(),
            qos: Default::default(),
            retain: Default::default(),
            temperature_command_template: Default::default(),
            temperature_command_topic: Default::default(),
            temperature_state_template: Default::default(),
            temperature_state_topic: Default::default(),
            temperature_unit: Default::default(),
            unique_id: Default::default(),
            value_template: Default::default(),
        }
    }
}

impl From<WaterHeater> for Entity {
    fn from(value: WaterHeater) -> Self {
        Entity::WaterHeater(value)
    }
}
