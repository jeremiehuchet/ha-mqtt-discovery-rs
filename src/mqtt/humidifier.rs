use super::common::Qos;
use super::common::{Availability, Device, EntityCategory, Origin};
use super::device_classes::HumidifierDeviceClass;
use crate::Entity;
pub use rust_decimal::Decimal;
use serde_derive::Serialize;

/// ---
/// title: "MQTT Humidifier"
/// description: "Instructions on how to integrate MQTT humidifiers into Home Assistant."
/// ha_category:
///   - Humidifier
/// ha_release: 2021.8
/// ha_iot_class: Configurable
/// ha_domain: mqtt
/// ---
///
/// The `mqtt` humidifier platform lets you control your MQTT enabled humidifiers.
///
/// ## Configuration
///
/// In an ideal scenario, the MQTT device will have a `state_topic` to publish state changes. If these messages are published with a `RETAIN` flag, the MQTT humidifier will receive an instant state update after subscription and will start with the correct state. Otherwise, the initial state of the humidifier will be `unknown`. A MQTT device can reset the current state to `unknown` using a `None` payload.
///
/// When a `state_topic` is not available, the humidifier will work in optimistic mode. In this mode, the humidifier will immediately change state after every command. Otherwise, the humidifier will wait for state confirmation from the device (message from `state_topic`). The initial state is set to `False` / `off` in optimistic mode.
///
/// Optimistic mode can be forced even if a `state_topic` is available. Try to enable it if you are experiencing incorrect humidifier operation.
///
/// To use an MQTT humidifier in your installation, add the following to your `configuration.yaml` file.
/// {% include integrations/restart_ha_after_config_inclusion.md %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - humidifier:
///       command_topic: "bedroom_humidifier/on/set"
///       target_humidity_command_topic: "bedroom_humidifier/humidity/set"
/// ```
///
/// Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
///
///
/// ⚠ Important\
/// Make sure that your topics match exactly. `some-topic/` and `some-topic` are different topics.
///
/// ## Examples
///
/// In this section you find some real-life examples of how to use this humidifier.
///
/// ### Full configuration
///
/// The example below shows a full configuration for a MQTT humidifier including modes.
///
///
/// ```yaml
/// # Example configuration.yaml
/// mqtt:
///   - humidifier:
///       name: "Bedroom humidifier"
///       device_class: "humidifier"
///       state_topic: "bedroom_humidifier/on/state"
///       action_topic: "bedroom_humidifier/action"
///       command_topic: "bedroom_humidifier/on/set"
///       current_humidity_topic: "bedroom_humidifier/humidity/current"
///       target_humidity_command_topic: "bedroom_humidifier/humidity/set"
///       target_humidity_state_topic: "bedroom_humidifier/humidity/state"
///       mode_state_topic: "bedroom_humidifier/mode/state"
///       mode_command_topic: "bedroom_humidifier/preset/preset_mode"
///       modes:
///         - "normal"
///         - "eco"
///         - "away"
///         - "boost"
///         - "comfort"
///         - "home"
///         - "sleep"
///         - "auto"
///         - "baby"
///       qos: 0
///       payload_on: "true"
///       payload_off: "false"
///       min_humidity: 30
///       max_humidity: 80
/// ```
///
///
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Humidifier {
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

    /// A template to render the value received on the `action_topic` with.
    #[serde(rename = "act_tpl", skip_serializing_if = "Option::is_none")]
    pub action_template: Option<String>,

    /// The MQTT topic to subscribe for changes of the current action. Valid values: `off`, `humidifying`, `drying`, `idle`
    #[serde(rename = "act_t", skip_serializing_if = "Option::is_none")]
    pub action_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `command_topic`.
    #[serde(rename = "cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub command_template: Option<String>,

    /// The MQTT topic to publish commands to change the humidifier state.
    #[serde(rename = "cmd_t")]
    pub command_topic: String,

    /// A template with which the value received on `current_humidity_topic` will be rendered.
    #[serde(
        rename = "current_humidity_template",
        skip_serializing_if = "Option::is_none"
    )]
    pub current_humidity_template: Option<String>,

    /// The MQTT topic on which to listen for the current humidity. A `"None"` value received will reset the current humidity. Empty values (`'''`) will be ignored.
    #[serde(
        rename = "current_humidity_topic",
        skip_serializing_if = "Option::is_none"
    )]
    pub current_humidity_topic: Option<String>,

    /// The [device class](/integrations/humidifier/#device-class) of the MQTT device. Must be either `humidifier`, `dehumidifier` or `null`.
    #[serde(rename = "dev_cla", skip_serializing_if = "Option::is_none")]
    pub device_class: Option<HumidifierDeviceClass>,

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

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation.
    #[serde(rename = "json_attr_tpl", skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation.
    #[serde(rename = "json_attr_t", skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,

    /// The minimum target humidity percentage that can be set.
    #[serde(rename = "max_hum", skip_serializing_if = "Option::is_none")]
    pub max_humidity: Option<Decimal>,

    /// The maximum target humidity percentage that can be set.
    #[serde(rename = "min_hum", skip_serializing_if = "Option::is_none")]
    pub min_humidity: Option<Decimal>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `mode_command_topic`.
    #[serde(rename = "mode_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub mode_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the `mode` on the humidifier. This attribute ust be configured together with the `modes` attribute.
    #[serde(rename = "mode_cmd_t", skip_serializing_if = "Option::is_none")]
    pub mode_command_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract a value for the humidifier `mode` state.
    #[serde(rename = "mode_stat_tpl", skip_serializing_if = "Option::is_none")]
    pub mode_state_template: Option<String>,

    /// The MQTT topic subscribed to receive the humidifier `mode`.
    #[serde(rename = "mode_stat_t", skip_serializing_if = "Option::is_none")]
    pub mode_state_topic: Option<String>,

    /// List of available modes this humidifier is capable of running at. Common examples include `normal`, `eco`, `away`, `boost`, `comfort`, `home`, `sleep`, `auto` and `baby`. These examples offer built-in translations but other custom modes are allowed as well.  This attribute ust be configured together with the `mode_command_topic` attribute.
    #[serde(rename = "modes", skip_serializing_if = "Option::is_none")]
    pub modes: Option<Vec<String>>,

    /// The name of the humidifier. Can be set to `null` if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// Flag that defines if humidifier works in optimistic mode
    #[serde(rename = "opt", skip_serializing_if = "Option::is_none")]
    pub optimistic: Option<bool>,

    /// The payload that represents the stop state.
    #[serde(rename = "pl_off", skip_serializing_if = "Option::is_none")]
    pub payload_off: Option<String>,

    /// The payload that represents the running state.
    #[serde(rename = "pl_on", skip_serializing_if = "Option::is_none")]
    pub payload_on: Option<String>,

    /// A special payload that resets the `target_humidity` state attribute to an `unknown` state when received at the `target_humidity_state_topic`. When received at `current_humidity_topic` it will reset the current humidity state.
    #[serde(rename = "pl_rst_hum", skip_serializing_if = "Option::is_none")]
    pub payload_reset_humidity: Option<String>,

    /// A special payload that resets the `mode` state attribute to an `unknown` state when received at the `mode_state_topic`.
    #[serde(rename = "pl_rst_mode", skip_serializing_if = "Option::is_none")]
    pub payload_reset_mode: Option<String>,

    /// Must be `humidifier`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    #[serde(rename = "platform")]
    pub platform: String,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// If the published message should have the retain flag on or not.
    #[serde(rename = "ret", skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,

    /// The MQTT topic subscribed to receive state updates. A "None" payload resets to an `unknown` state. An empty payload is ignored. Valid state payloads are `OFF` and `ON`. Custom `OFF` and `ON` values can be set with the `payload_off` and `payload_on` config options.
    #[serde(rename = "stat_t", skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract a value from the state.
    #[serde(rename = "stat_val_tpl", skip_serializing_if = "Option::is_none")]
    pub state_value_template: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `target_humidity_command_topic`.
    #[serde(rename = "hum_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub target_humidity_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the humidifier target humidity state based on a percentage.
    #[serde(rename = "hum_cmd_t")]
    pub target_humidity_command_topic: String,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract a value for the humidifier `target_humidity` state.
    #[serde(rename = "hum_state_tpl", skip_serializing_if = "Option::is_none")]
    pub target_humidity_state_template: Option<String>,

    /// The MQTT topic subscribed to receive humidifier target humidity.
    #[serde(rename = "hum_stat_t", skip_serializing_if = "Option::is_none")]
    pub target_humidity_state_topic: Option<String>,

    /// An ID that uniquely identifies this humidifier. If two humidifiers have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
}

impl Humidifier {
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

    /// A template to render the value received on the `action_topic` with.
    pub fn action_template<T: Into<String>>(mut self, action_template: T) -> Self {
        self.action_template = Some(action_template.into());
        self
    }

    /// The MQTT topic to subscribe for changes of the current action. Valid values: `off`, `humidifying`, `drying`, `idle`
    pub fn action_topic<T: Into<String>>(mut self, action_topic: T) -> Self {
        self.action_topic = Some(action_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `command_topic`.
    pub fn command_template<T: Into<String>>(mut self, command_template: T) -> Self {
        self.command_template = Some(command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the humidifier state.
    pub fn command_topic<T: Into<String>>(mut self, command_topic: T) -> Self {
        self.command_topic = command_topic.into();
        self
    }

    /// A template with which the value received on `current_humidity_topic` will be rendered.
    pub fn current_humidity_template<T: Into<String>>(
        mut self,
        current_humidity_template: T,
    ) -> Self {
        self.current_humidity_template = Some(current_humidity_template.into());
        self
    }

    /// The MQTT topic on which to listen for the current humidity. A `"None"` value received will reset the current humidity. Empty values (`'''`) will be ignored.
    pub fn current_humidity_topic<T: Into<String>>(mut self, current_humidity_topic: T) -> Self {
        self.current_humidity_topic = Some(current_humidity_topic.into());
        self
    }

    /// The [device class](/integrations/humidifier/#device-class) of the MQTT device. Must be either `humidifier`, `dehumidifier` or `null`.
    pub fn device_class<T: Into<HumidifierDeviceClass>>(mut self, device_class: T) -> Self {
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

    /// The minimum target humidity percentage that can be set.
    pub fn max_humidity(mut self, max_humidity: Decimal) -> Self {
        self.max_humidity = Some(max_humidity);
        self
    }

    /// The maximum target humidity percentage that can be set.
    pub fn min_humidity(mut self, min_humidity: Decimal) -> Self {
        self.min_humidity = Some(min_humidity);
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `mode_command_topic`.
    pub fn mode_command_template<T: Into<String>>(mut self, mode_command_template: T) -> Self {
        self.mode_command_template = Some(mode_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the `mode` on the humidifier. This attribute ust be configured together with the `modes` attribute.
    pub fn mode_command_topic<T: Into<String>>(mut self, mode_command_topic: T) -> Self {
        self.mode_command_topic = Some(mode_command_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract a value for the humidifier `mode` state.
    pub fn mode_state_template<T: Into<String>>(mut self, mode_state_template: T) -> Self {
        self.mode_state_template = Some(mode_state_template.into());
        self
    }

    /// The MQTT topic subscribed to receive the humidifier `mode`.
    pub fn mode_state_topic<T: Into<String>>(mut self, mode_state_topic: T) -> Self {
        self.mode_state_topic = Some(mode_state_topic.into());
        self
    }

    /// List of available modes this humidifier is capable of running at. Common examples include `normal`, `eco`, `away`, `boost`, `comfort`, `home`, `sleep`, `auto` and `baby`. These examples offer built-in translations but other custom modes are allowed as well.  This attribute ust be configured together with the `mode_command_topic` attribute.
    pub fn modes<T: Into<String>>(mut self, modes: Vec<T>) -> Self {
        self.modes = Some(modes.into_iter().map(|v| v.into()).collect());
        self
    }

    /// The name of the humidifier. Can be set to `null` if only the device name is relevant.
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    /// Flag that defines if humidifier works in optimistic mode
    pub fn optimistic(mut self, optimistic: bool) -> Self {
        self.optimistic = Some(optimistic);
        self
    }

    /// The payload that represents the stop state.
    pub fn payload_off<T: Into<String>>(mut self, payload_off: T) -> Self {
        self.payload_off = Some(payload_off.into());
        self
    }

    /// The payload that represents the running state.
    pub fn payload_on<T: Into<String>>(mut self, payload_on: T) -> Self {
        self.payload_on = Some(payload_on.into());
        self
    }

    /// A special payload that resets the `target_humidity` state attribute to an `unknown` state when received at the `target_humidity_state_topic`. When received at `current_humidity_topic` it will reset the current humidity state.
    pub fn payload_reset_humidity<T: Into<String>>(mut self, payload_reset_humidity: T) -> Self {
        self.payload_reset_humidity = Some(payload_reset_humidity.into());
        self
    }

    /// A special payload that resets the `mode` state attribute to an `unknown` state when received at the `mode_state_topic`.
    pub fn payload_reset_mode<T: Into<String>>(mut self, payload_reset_mode: T) -> Self {
        self.payload_reset_mode = Some(payload_reset_mode.into());
        self
    }

    /// Must be `humidifier`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    pub fn platform<T: Into<String>>(mut self, platform: T) -> Self {
        self.platform = platform.into();
        self
    }

    /// The maximum QoS level to be used when receiving and publishing messages.
    pub fn qos(mut self, qos: Qos) -> Self {
        self.qos = Some(qos);
        self
    }

    /// If the published message should have the retain flag on or not.
    pub fn retain(mut self, retain: bool) -> Self {
        self.retain = Some(retain);
        self
    }

    /// The MQTT topic subscribed to receive state updates. A "None" payload resets to an `unknown` state. An empty payload is ignored. Valid state payloads are `OFF` and `ON`. Custom `OFF` and `ON` values can be set with the `payload_off` and `payload_on` config options.
    pub fn state_topic<T: Into<String>>(mut self, state_topic: T) -> Self {
        self.state_topic = Some(state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract a value from the state.
    pub fn state_value_template<T: Into<String>>(mut self, state_value_template: T) -> Self {
        self.state_value_template = Some(state_value_template.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `target_humidity_command_topic`.
    pub fn target_humidity_command_template<T: Into<String>>(
        mut self,
        target_humidity_command_template: T,
    ) -> Self {
        self.target_humidity_command_template = Some(target_humidity_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the humidifier target humidity state based on a percentage.
    pub fn target_humidity_command_topic<T: Into<String>>(
        mut self,
        target_humidity_command_topic: T,
    ) -> Self {
        self.target_humidity_command_topic = target_humidity_command_topic.into();
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract a value for the humidifier `target_humidity` state.
    pub fn target_humidity_state_template<T: Into<String>>(
        mut self,
        target_humidity_state_template: T,
    ) -> Self {
        self.target_humidity_state_template = Some(target_humidity_state_template.into());
        self
    }

    /// The MQTT topic subscribed to receive humidifier target humidity.
    pub fn target_humidity_state_topic<T: Into<String>>(
        mut self,
        target_humidity_state_topic: T,
    ) -> Self {
        self.target_humidity_state_topic = Some(target_humidity_state_topic.into());
        self
    }

    /// An ID that uniquely identifies this humidifier. If two humidifiers have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    pub fn unique_id<T: Into<String>>(mut self, unique_id: T) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }
}

impl Default for Humidifier {
    fn default() -> Self {
        Self {
            topic_prefix: Default::default(),
            origin: Default::default(),
            device: Default::default(),
            entity_category: Default::default(),
            availability: Default::default(),
            action_template: Default::default(),
            action_topic: Default::default(),
            command_template: Default::default(),
            command_topic: Default::default(),
            current_humidity_template: Default::default(),
            current_humidity_topic: Default::default(),
            device_class: Default::default(),
            enabled_by_default: Default::default(),
            encoding: Default::default(),
            entity_picture: Default::default(),
            icon: Default::default(),
            json_attributes_template: Default::default(),
            json_attributes_topic: Default::default(),
            max_humidity: Default::default(),
            min_humidity: Default::default(),
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
            payload_reset_humidity: Default::default(),
            payload_reset_mode: Default::default(),
            platform: "humidifier".to_string(),
            qos: Default::default(),
            retain: Default::default(),
            state_topic: Default::default(),
            state_value_template: Default::default(),
            target_humidity_command_template: Default::default(),
            target_humidity_command_topic: Default::default(),
            target_humidity_state_template: Default::default(),
            target_humidity_state_topic: Default::default(),
            unique_id: Default::default(),
        }
    }
}

impl From<Humidifier> for Entity {
    fn from(value: Humidifier) -> Self {
        Entity::Humidifier(value)
    }
}
