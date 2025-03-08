use super::common::{Availability, Device, EntityCategory, Origin};
use crate::Entity;
use serde_derive::Serialize;

/// ---
/// title: "MQTT tag scanner"
/// description: "Instructions on how to integrate MQTT scanner within Home Assistant."
/// ha_category:
///   - Tag scanner
/// ha_release: 0.116
/// ha_iot_class: Configurable
/// ha_domain: mqtt
/// ---
///
/// The `mqtt` tag scanner platform uses an MQTT message payload to generate tag scanned events.
///
/// ## Configuration
///
/// MQTT scanners are only supported through [MQTT discovery](/integrations/mqtt/#mqtt-discovery), manual setup through `configuration.yaml` is not supported.
/// The discovery topic needs to be: `<discovery_prefix>/tag/[<node_id>/]<object_id>/config`.
///
///
/// ## Examples
///
/// In this section, you will find some real-life examples of how to use this sensor.
///
/// ### Full configuration with tag ID extracted from JSON data
///
/// This is an example of a configuration where the tag ID is extracted from a JSON formatted MQTT message.
/// To test, you can use the command line tool `mosquitto_pub` shipped with `mosquitto` or the `mosquitto-clients` package to send MQTT messages.
///
/// Discover the tag scanner:
///
///
/// ```bash
/// mosquitto_pub -h 127.0.0.1 -t homeassistant/tag/0AFFD2/config -m '{"topic": "0AFFD2/tag_scanned", "value_template": "{{ value_json.PN532.UID }}"}'
/// ```
///
///
/// Generate tag scanned event:
///
///
/// ```bash
/// mosquitto_pub -h 127.0.0.1 -t 0AFFD2/tag_scanned -m '{"Time":"2020-09-28T17:02:10","PN532":{"UID":"E9F35959", "DATA":"ILOVETASMOTA"}}'
/// ```
///
///
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Tag {
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

    /// The MQTT topic subscribed to receive tag scanned events.
    #[serde(rename = "t")]
    pub topic: String,

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) that returns a tag ID.
    #[serde(rename = "val_tpl", skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

impl Tag {
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

    /// The MQTT topic subscribed to receive tag scanned events.
    pub fn topic<T: Into<String>>(mut self, topic: T) -> Self {
        self.topic = topic.into();
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) that returns a tag ID.
    pub fn value_template<T: Into<String>>(mut self, value_template: T) -> Self {
        self.value_template = Some(value_template.into());
        self
    }
}

impl Default for Tag {
    fn default() -> Self {
        Self {
            topic_prefix: Default::default(),
            origin: Default::default(),
            device: Default::default(),
            entity_category: Default::default(),
            availability: Default::default(),
            topic: Default::default(),
            value_template: Default::default(),
        }
    }
}

impl From<Tag> for Entity {
    fn from(value: Tag) -> Self {
        Entity::Tag(value)
    }
}
