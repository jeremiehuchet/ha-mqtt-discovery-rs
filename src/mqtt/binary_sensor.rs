use super::common::Qos;
use super::common::{Availability, Device, EntityCategory, Origin};
use super::device_classes::BinarySensorDeviceClass;
use crate::Entity;
use serde_derive::Serialize;

/// ---
/// title: "MQTT binary sensor"
/// description: "Instructions on how to integrate MQTT binary sensors within Home Assistant."
/// ha_category:
///   - Binary sensor
/// ha_release: 0.9
/// ha_iot_class: Configurable
/// ha_domain: mqtt
/// ---
///
/// The `mqtt` binary sensor platform uses an MQTT message received to set the binary sensor's state to `on`, `off` or `unknown`.
///
/// The state will be updated only after a new message is published on `state_topic` matching `payload_on`, `payload_off` or `None`. If these messages are published with the `retain` flag set,
/// the binary sensor will receive an instant state update after subscription and Home Assistant will display the correct state on startup.
/// Otherwise, the initial state displayed in Home Assistant will be `unknown`.
///
/// Stateless devices such as buttons, remote controls etc are better represented by [MQTT device triggers](/integrations/device_trigger.mqtt/) than by binary sensors.
///
/// ## Configuration
///
/// The `mqtt` binary sensor platform optionally supports a list of  `availability` topics to receive online and offline messages (birth and LWT messages) from the MQTT device. During normal operation, if the MQTT sensor device goes offline (i.e., publishes `payload_not_available` to an `availability` topic), Home Assistant will display the binary sensor as `unavailable`. If these messages are published with the `retain` flag set, the binary sensor will receive an instant update after subscription and Home Assistant will display the correct availability state of the binary sensor when Home Assistant starts up. If the `retain` flag is not set, Home Assistant will display the binary sensor as `unavailable` when Home Assistant starts up. If no `availability` topic is defined, Home Assistant will consider the MQTT device to be `available` and will display its state.
///
/// To use an MQTT binary sensor in your installation,
/// add the following to your {% term "`configuration.yaml`" %} file:
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - binary_sensor:
///       state_topic: "home-assistant/window/contact"
/// ```
///
///
/// ## Examples
///
/// In this section, you will find some real-life examples of how to use this sensor.
///
/// ### Full configuration with JSON data
///
/// This is an example of a configuration where the state is extracted from a JSON formatted MQTT message.
/// To test, you can use the command line tool `mosquitto_pub` shipped with `mosquitto` or the `mosquitto-clients` package to send MQTT messages.
///
/// To set the state of the binary sensor manually:
///
/// ```bash
/// mosquitto_pub -h 127.0.0.1 -t home-assistant/window/availability -m "online"
/// mosquitto_pub -h 127.0.0.1 -t home-assistant/window/contact -m '{"state":"ON"}'
/// mosquitto_pub -h 127.0.0.1 -t home-assistant/window/contact -m '{"state":"OFF"}'
/// ```
///
/// The example below shows a full configuration for a binary sensor:
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - binary_sensor:
///       name: "Window Contact Sensor"
///       state_topic: "home-assistant/window/contact"
///       payload_on: "ON"
///       availability:
///         - topic: "home-assistant/window/availability"
///           payload_available: "online"
///           payload_not_available: "offline"
///       qos: 0
///       device_class: opening
///       value_template: "{{ value_json.state }}"
/// ```
///
///
/// ### Toggle the binary sensor each time a message is received on state_topic
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - binary_sensor:
///       state_topic: "lab_button/cmnd/POWER"
///       value_template: "{%if is_state(entity_id,\"on\")-%}OFF{%-else-%}ON{%-endif%}"
/// ```
///
///
/// ### Get the state of a device with ESPEasy
///
/// Assuming that you have flashed your ESP8266 unit with [ESPEasy](https://github.com/letscontrolit/ESPEasy). Under "Config" is a name ("Unit Name:") set for your device (here it's "bathroom"). A configuration for a "Controller" for MQTT with the protocol "OpenHAB MQTT" is present and the entries ("Controller Subscribe:" and "Controller Publish:") are adjusted to match your needs. In this example, the topics are prefixed with "home". Also, add a "Switch Input" in the "Devices" tap with the name "switch" and "button" as value.
///
/// As soon as the unit is online, you will get the state of the attached button.
///
/// ```txt
/// home/bathroom/status Connected
/// ...
/// home/bathroom/switch/button 1
/// ```
///
/// The configuration will look like the example below:
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - binary_sensor:
///       name: Bathroom
///       state_topic: "home/bathroom/switch/button"
///       payload_on: "1"
///       payload_off: "0"
/// ```
///
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BinarySensor {
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

    /// Sets the [class of the device](/integrations/binary_sensor/#device-class), changing the device state and icon that is displayed on the frontend. The `device_class` can be `null`.
    #[serde(rename = "dev_cla", skip_serializing_if = "Option::is_none")]
    pub device_class: Option<BinarySensorDeviceClass>,

    /// Flag which defines if the entity should be enabled when first added.
    #[serde(rename = "en", skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,

    /// The encoding of the payloads received. Set to `""` to disable decoding of incoming payload.
    #[serde(rename = "e", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,

    /// Picture URL for the entity.
    #[serde(rename = "ent_pic", skip_serializing_if = "Option::is_none")]
    pub entity_picture: Option<String>,

    /// Sends update events (which results in update of [state object](/docs/configuration/state_object/)'s `last_changed`) even if the sensor's state hasn't changed. Useful if you want to have meaningful value graphs in history or want to create an automation that triggers on *every* incoming state message (not only when the sensor's new state is different to the current one).
    #[serde(rename = "frc_upd", skip_serializing_if = "Option::is_none")]
    pub force_update: Option<bool>,

    /// [Icon](/docs/configuration/customizing-devices/#icon) for the entity.
    #[serde(rename = "ic", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation.
    #[serde(rename = "json_attr_tpl", skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation.
    #[serde(rename = "json_attr_t", skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,

    /// The name of the binary sensor. Can be set to `null` if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used instead of `name` for automatic generation of `entity_id`
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// For sensors that only send `on` state updates (like PIRs), this variable sets a delay in seconds after which the sensor's state will be updated back to `off`.
    #[serde(rename = "off_dly", skip_serializing_if = "Option::is_none")]
    pub off_delay: Option<i32>,

    /// The string that represents the `off` state. It will be compared to the message in the `state_topic` (see `value_template` for details)
    #[serde(rename = "pl_off", skip_serializing_if = "Option::is_none")]
    pub payload_off: Option<String>,

    /// The string that represents the `on` state. It will be compared to the message in the `state_topic` (see `value_template` for details)
    #[serde(rename = "pl_on", skip_serializing_if = "Option::is_none")]
    pub payload_on: Option<String>,

    /// Must be `binary_sensor`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    #[serde(rename = "platform")]
    pub platform: String,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// The MQTT topic subscribed to receive sensor's state. Valid states are `OFF` and `ON`. Custom `OFF` and `ON` values can be set with the `payload_off` and `payload_on` config options.
    #[serde(rename = "stat_t")]
    pub state_topic: String,

    /// An ID that uniquely identifies this sensor. If two sensors have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) that returns a string to be compared to `payload_on`/`payload_off` or an empty string, in which case the MQTT message will be removed. Remove this option when `payload_on` and `payload_off` are sufficient to match your payloads (i.e no preprocessing of original message is required).
    #[serde(rename = "val_tpl", skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

impl BinarySensor {
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

    /// Sets the [class of the device](/integrations/binary_sensor/#device-class), changing the device state and icon that is displayed on the frontend. The `device_class` can be `null`.
    pub fn device_class<T: Into<BinarySensorDeviceClass>>(mut self, device_class: T) -> Self {
        self.device_class = Some(device_class.into());
        self
    }

    /// Flag which defines if the entity should be enabled when first added.
    pub fn enabled_by_default(mut self, enabled_by_default: bool) -> Self {
        self.enabled_by_default = Some(enabled_by_default);
        self
    }

    /// The encoding of the payloads received. Set to `""` to disable decoding of incoming payload.
    pub fn encoding<T: Into<String>>(mut self, encoding: T) -> Self {
        self.encoding = Some(encoding.into());
        self
    }

    /// Picture URL for the entity.
    pub fn entity_picture<T: Into<String>>(mut self, entity_picture: T) -> Self {
        self.entity_picture = Some(entity_picture.into());
        self
    }

    /// Sends update events (which results in update of [state object](/docs/configuration/state_object/)'s `last_changed`) even if the sensor's state hasn't changed. Useful if you want to have meaningful value graphs in history or want to create an automation that triggers on *every* incoming state message (not only when the sensor's new state is different to the current one).
    pub fn force_update(mut self, force_update: bool) -> Self {
        self.force_update = Some(force_update);
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

    /// The name of the binary sensor. Can be set to `null` if only the device name is relevant.
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Used instead of `name` for automatic generation of `entity_id`
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    /// For sensors that only send `on` state updates (like PIRs), this variable sets a delay in seconds after which the sensor's state will be updated back to `off`.
    pub fn off_delay(mut self, off_delay: i32) -> Self {
        self.off_delay = Some(off_delay);
        self
    }

    /// The string that represents the `off` state. It will be compared to the message in the `state_topic` (see `value_template` for details)
    pub fn payload_off<T: Into<String>>(mut self, payload_off: T) -> Self {
        self.payload_off = Some(payload_off.into());
        self
    }

    /// The string that represents the `on` state. It will be compared to the message in the `state_topic` (see `value_template` for details)
    pub fn payload_on<T: Into<String>>(mut self, payload_on: T) -> Self {
        self.payload_on = Some(payload_on.into());
        self
    }

    /// Must be `binary_sensor`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    pub fn platform<T: Into<String>>(mut self, platform: T) -> Self {
        self.platform = platform.into();
        self
    }

    /// The maximum QoS level to be used when receiving and publishing messages.
    pub fn qos(mut self, qos: Qos) -> Self {
        self.qos = Some(qos);
        self
    }

    /// The MQTT topic subscribed to receive sensor's state. Valid states are `OFF` and `ON`. Custom `OFF` and `ON` values can be set with the `payload_off` and `payload_on` config options.
    pub fn state_topic<T: Into<String>>(mut self, state_topic: T) -> Self {
        self.state_topic = state_topic.into();
        self
    }

    /// An ID that uniquely identifies this sensor. If two sensors have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    pub fn unique_id<T: Into<String>>(mut self, unique_id: T) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) that returns a string to be compared to `payload_on`/`payload_off` or an empty string, in which case the MQTT message will be removed. Remove this option when `payload_on` and `payload_off` are sufficient to match your payloads (i.e no preprocessing of original message is required).
    pub fn value_template<T: Into<String>>(mut self, value_template: T) -> Self {
        self.value_template = Some(value_template.into());
        self
    }
}

impl Default for BinarySensor {
    fn default() -> Self {
        Self {
            topic_prefix: Default::default(),
            origin: Default::default(),
            device: Default::default(),
            entity_category: Default::default(),
            availability: Default::default(),
            device_class: Default::default(),
            enabled_by_default: Default::default(),
            encoding: Default::default(),
            entity_picture: Default::default(),
            force_update: Default::default(),
            icon: Default::default(),
            json_attributes_template: Default::default(),
            json_attributes_topic: Default::default(),
            name: Default::default(),
            object_id: Default::default(),
            off_delay: Default::default(),
            payload_off: Default::default(),
            payload_on: Default::default(),
            platform: "binary_sensor".to_string(),
            qos: Default::default(),
            state_topic: Default::default(),
            unique_id: Default::default(),
            value_template: Default::default(),
        }
    }
}

impl From<BinarySensor> for Entity {
    fn from(value: BinarySensor) -> Self {
        Entity::BinarySensor(value)
    }
}
