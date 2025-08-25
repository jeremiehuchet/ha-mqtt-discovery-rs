use super::common::Qos;
use super::common::{Availability, Device, EntityCategory, Origin};
use crate::Entity;
use serde_derive::Serialize;

/// ---
/// title: "MQTT Vacuum"
/// description: "Instructions on how to integrate your MQTT enabled Vacuum within Home Assistant."
/// ha_category:
///   - Vacuum
/// ha_release: 0.54
/// ha_domain: mqtt
/// ---
///
/// The `mqtt` vacuum `integration` allows you to control your MQTT-enabled vacuum.
/// The initial state of the MQTT vacuum `entity` will set to `unknown` and can be reset by a device by sending a `null` payload as state.
///
/// ## Configuration
///
/// To use an MQTT vacuum in your installation, add the following to your `configuration.yaml` file.
/// {% include integrations/restart_ha_after_config_inclusion.md %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - vacuum:
///       state_topic: state-topic
///       command_topic: command-topic
/// ```
///
/// Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
///
///
/// ## Configuration example
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - vacuum:
///       name: "MQTT Vacuum"
///       supported_features:
///         - start
///         - pause
///         - stop
///         - return_home
///         - status
///         - locate
///         - clean_spot
///         - fan_speed
///         - send_command
///       command_topic: "vacuum/command"
///       set_fan_speed_topic: "vacuum/set_fan_speed"
///       fan_speed_list:
///         - min
///         - medium
///         - high
///         - max
///       send_command_topic: "vacuum/send_command"
/// ```
///
/// ## MQTT Protocol
///
/// The  configuration for this integration expects an MQTT protocol like the following.
///
/// ### Basic Commands
///
/// MQTT topic: `vacuum/command`
///
/// Possible MQTT payloads:
///
/// - `start` - Start cleaning
/// - `pause` - Pause cleaning
/// - `return_to_base` - Return to base/dock
/// - `stop` - Stop the vacuum.
/// - `clean_spot` - Initialize a spot cleaning cycle
/// - `locate` - Locate the vacuum (typically by playing a song)
///
/// ### Send custom command
///
/// Vacuum send_command allows three parameters:
///
/// - entity_id
/// - command
/// - params - optional
///
/// If params are not provided it sends command as payload to MQTT send_command topic.
/// If params are provided service sends JSON as payload with such structure:
///
/// ```json
/// {
///   'command': 'command',
///   'param1-key': 'param1-value'
/// }
/// ```
///
/// Action trigger example:
///
/// ```yaml
/// - alias: "Push command based on sensor"
///     triggers:
///       - trigger: state
///         entity_id: sensor.sensor
///     actions:
///       - action: vacuum.send_command
///         target:
///           entity_id: vacuum.vacuum_entity
///         data:
///           command: "custom_command"
///           params:
///             - key: value
/// ```
///
/// MQTT topic: `vacuum/send_command`
///
/// ### Status/Sensor Updates
///
/// MQTT topic: `vacuum/state`
///
/// MQTT payload:
///
/// ```json
/// {
///     "state": "docked",
///     "fan_speed": "off"
/// }
/// ```
///
/// State has to be one of vacuum states supported by Home Assistant:
///
/// - cleaning,
/// - docked,
/// - paused,
/// - idle,
/// - returning,
/// - error.
///
/// ### Set Fan Speed
///
/// MQTT topic: `vacuum/set_fan_speed`
///
/// Possible MQTT payloads:
///
/// - `min` - Minimum fan speed
/// - `medium` - Medium fan speed
/// - `high` - High fan speed
/// - `max` - Max fan speed
///
/// ## Usage examples
///
/// ### Usage with cloudless Xiaomi vacuums
///
/// This integration is supported by the cloud-free Xiaomi Vacuum Webinterface [Valetudo](https://github.com/Hypfer/Valetudo).
///
/// ### Retrofitting non-wifi vacuums
///
/// - Retrofitting your old Roomba with an ESP8266. [This repository](https://github.com/johnboiles/esp-roomba-mqtt) provides MQTT client firmware.
/// - If you own a non-wifi Neato, you can refer to [this repository](https://github.com/jeroenterheerdt/neato-serial) that uses a Raspberry Pi to retrofit an old Neato.
///
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Vacuum {
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

    /// The MQTT topic to publish commands to control the vacuum.
    #[serde(rename = "cmd_t", skip_serializing_if = "Option::is_none")]
    pub command_topic: Option<String>,

    /// The encoding of the payloads received and published messages. Set to `""` to disable decoding of incoming payload.
    #[serde(rename = "e", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,

    /// List of possible fan speeds for the vacuum.
    #[serde(rename = "fanspd_lst", skip_serializing_if = "Option::is_none")]
    pub fan_speed_list: Option<Vec<String>>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation.
    #[serde(rename = "json_attr_tpl", skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation.
    #[serde(rename = "json_attr_t", skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,

    /// The name of the vacuum. Can be set to `null` if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// The payload to send to the `command_topic` to begin a spot cleaning cycle.
    #[serde(rename = "pl_cln_sp", skip_serializing_if = "Option::is_none")]
    pub payload_clean_spot: Option<String>,

    /// The payload to send to the `command_topic` to locate the vacuum (typically plays a song).
    #[serde(rename = "pl_loc", skip_serializing_if = "Option::is_none")]
    pub payload_locate: Option<String>,

    /// The payload to send to the `command_topic` to pause the vacuum.
    #[serde(rename = "pl_paus", skip_serializing_if = "Option::is_none")]
    pub payload_pause: Option<String>,

    /// The payload to send to the `command_topic` to tell the vacuum to return to base.
    #[serde(rename = "pl_ret", skip_serializing_if = "Option::is_none")]
    pub payload_return_to_base: Option<String>,

    /// The payload to send to the `command_topic` to begin the cleaning cycle.
    #[serde(rename = "pl_strt", skip_serializing_if = "Option::is_none")]
    pub payload_start: Option<String>,

    /// The payload to send to the `command_topic` to stop cleaning.
    #[serde(rename = "pl_stop", skip_serializing_if = "Option::is_none")]
    pub payload_stop: Option<String>,

    /// Must be `vacuum`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    #[serde(rename = "p")]
    pub platform: String,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// If the published message should have the retain flag on or not.
    #[serde(rename = "ret", skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,

    /// The MQTT topic to publish custom commands to the vacuum.
    #[serde(rename = "send_cmd_t", skip_serializing_if = "Option::is_none")]
    pub send_command_topic: Option<String>,

    /// The MQTT topic to publish commands to control the vacuum's fan speed.
    #[serde(rename = "set_fan_spd_t", skip_serializing_if = "Option::is_none")]
    pub set_fan_speed_topic: Option<String>,

    /// The MQTT topic subscribed to receive state messages from the vacuum. Messages received on the `state_topic` must be a valid JSON dictionary, with a mandatory `state` key and optionally `fan_speed` keys as shown in the [example](#configuration-example).
    #[serde(rename = "stat_t", skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,

    /// List of features that the vacuum supports (possible values are `start`, `stop`, `pause`, `return_home`, `status`, `locate`, `clean_spot`, `fan_speed`, `send_command`).
    #[serde(rename = "sup_feat", skip_serializing_if = "Option::is_none")]
    pub supported_features: Option<Vec<String>>,

    /// An ID that uniquely identifies this vacuum. If two vacuums have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
}

impl Vacuum {
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

    /// The MQTT topic to publish commands to control the vacuum.
    pub fn command_topic<T: Into<String>>(mut self, command_topic: T) -> Self {
        self.command_topic = Some(command_topic.into());
        self
    }

    /// The encoding of the payloads received and published messages. Set to `""` to disable decoding of incoming payload.
    pub fn encoding<T: Into<String>>(mut self, encoding: T) -> Self {
        self.encoding = Some(encoding.into());
        self
    }

    /// List of possible fan speeds for the vacuum.
    pub fn fan_speed_list<T: Into<String>>(mut self, fan_speed_list: Vec<T>) -> Self {
        self.fan_speed_list = Some(fan_speed_list.into_iter().map(|v| v.into()).collect());
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

    /// The name of the vacuum. Can be set to `null` if only the device name is relevant.
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    /// The payload to send to the `command_topic` to begin a spot cleaning cycle.
    pub fn payload_clean_spot<T: Into<String>>(mut self, payload_clean_spot: T) -> Self {
        self.payload_clean_spot = Some(payload_clean_spot.into());
        self
    }

    /// The payload to send to the `command_topic` to locate the vacuum (typically plays a song).
    pub fn payload_locate<T: Into<String>>(mut self, payload_locate: T) -> Self {
        self.payload_locate = Some(payload_locate.into());
        self
    }

    /// The payload to send to the `command_topic` to pause the vacuum.
    pub fn payload_pause<T: Into<String>>(mut self, payload_pause: T) -> Self {
        self.payload_pause = Some(payload_pause.into());
        self
    }

    /// The payload to send to the `command_topic` to tell the vacuum to return to base.
    pub fn payload_return_to_base<T: Into<String>>(mut self, payload_return_to_base: T) -> Self {
        self.payload_return_to_base = Some(payload_return_to_base.into());
        self
    }

    /// The payload to send to the `command_topic` to begin the cleaning cycle.
    pub fn payload_start<T: Into<String>>(mut self, payload_start: T) -> Self {
        self.payload_start = Some(payload_start.into());
        self
    }

    /// The payload to send to the `command_topic` to stop cleaning.
    pub fn payload_stop<T: Into<String>>(mut self, payload_stop: T) -> Self {
        self.payload_stop = Some(payload_stop.into());
        self
    }

    /// Must be `vacuum`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
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

    /// The MQTT topic to publish custom commands to the vacuum.
    pub fn send_command_topic<T: Into<String>>(mut self, send_command_topic: T) -> Self {
        self.send_command_topic = Some(send_command_topic.into());
        self
    }

    /// The MQTT topic to publish commands to control the vacuum's fan speed.
    pub fn set_fan_speed_topic<T: Into<String>>(mut self, set_fan_speed_topic: T) -> Self {
        self.set_fan_speed_topic = Some(set_fan_speed_topic.into());
        self
    }

    /// The MQTT topic subscribed to receive state messages from the vacuum. Messages received on the `state_topic` must be a valid JSON dictionary, with a mandatory `state` key and optionally `fan_speed` keys as shown in the [example](#configuration-example).
    pub fn state_topic<T: Into<String>>(mut self, state_topic: T) -> Self {
        self.state_topic = Some(state_topic.into());
        self
    }

    /// List of features that the vacuum supports (possible values are `start`, `stop`, `pause`, `return_home`, `status`, `locate`, `clean_spot`, `fan_speed`, `send_command`).
    pub fn supported_features<T: Into<String>>(mut self, supported_features: Vec<T>) -> Self {
        self.supported_features = Some(supported_features.into_iter().map(|v| v.into()).collect());
        self
    }

    /// An ID that uniquely identifies this vacuum. If two vacuums have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    pub fn unique_id<T: Into<String>>(mut self, unique_id: T) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }
}

impl Default for Vacuum {
    fn default() -> Self {
        Self {
            topic_prefix: Default::default(),
            origin: Default::default(),
            device: Default::default(),
            entity_category: Default::default(),
            availability: Default::default(),
            command_topic: Default::default(),
            encoding: Default::default(),
            fan_speed_list: Default::default(),
            json_attributes_template: Default::default(),
            json_attributes_topic: Default::default(),
            name: Default::default(),
            object_id: Default::default(),
            payload_clean_spot: Default::default(),
            payload_locate: Default::default(),
            payload_pause: Default::default(),
            payload_return_to_base: Default::default(),
            payload_start: Default::default(),
            payload_stop: Default::default(),
            platform: "vacuum".to_string(),
            qos: Default::default(),
            retain: Default::default(),
            send_command_topic: Default::default(),
            set_fan_speed_topic: Default::default(),
            state_topic: Default::default(),
            supported_features: Default::default(),
            unique_id: Default::default(),
        }
    }
}

impl From<Vacuum> for Entity {
    fn from(value: Vacuum) -> Self {
        Entity::Vacuum(value)
    }
}
