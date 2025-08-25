use super::common::Qos;
use super::common::{Availability, Device, EntityCategory, Origin};
use super::device_classes::EventDeviceClass;
use crate::Entity;
use serde_derive::Serialize;

/// ---
/// title: "MQTT Event"
/// description: "Instructions on how to integrate MQTT events into Home Assistant."
/// ha_category:
///   - Event
/// ha_release: 2023.8
/// ha_iot_class: Configurable
/// ha_domain: mqtt
/// ---
///
/// The `mqtt` event platform allows you to process event info from an MQTT message. Events are signals that are emitted when something happens, for example, when a user presses a physical button like a doorbell or when a button on a remote control is pressed. With the event some event attributes can be sent to become available as an attribute on the entity. MQTT events are stateless. For example, a doorbell does not have a state like being "on" or "off" but instead is momentarily pressed.
///
/// ## Configuration
///
/// To use an MQTT event entity in your installation, add the following to your `configuration.yaml` file.
/// {% include integrations/restart_ha_after_config_inclusion.md %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - event:
///       state_topic: "home/doorbell/state"
///       event_types:
///         - press
/// ```
///
/// Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
///
///
/// ⚠ Important\
/// Make sure that your topic matches exactly. `some-topic/` and `some-topic` are different topics.
///
/// ### Full configuration with JSON data
///
/// The example below shows a full configuration for an event MQTT entity.
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - event:
///       state_topic: "home/doorbell/state"
///       event_types:
///         - "press"
///         - "hold"
///       availability:
///         - topic: "home/doorbell/available"
///       qos: 0
///       device_class: "doorbell"
/// ```
///
/// The event information is extracted from a JSON formatted MQTT message.
/// To test, you can use the command line tool `mosquitto_pub` shipped with `mosquitto` or the `mosquitto-clients` package to send MQTT messages.
///
/// To set trigger the `mqtt` event entity manually:
///
/// ```bash
/// mosquitto_pub -h 127.0.0.1 -t home/doorbell/available -m "online"
/// mosquitto_pub -h 127.0.0.1 -t home/doorbell/state -m '{"event_type": "hold"}'
/// ```
///
/// Besides the required `event_type` element, the payload can contain additional event attributes.
/// These additional attribute updates will be exposed as attributes on the `mqtt` event entity.
///
/// The example below demonstrates how event attributes can be added to the event data.
///
/// ```bash
/// mosquitto_pub -h 127.0.0.1 -t home/doorbell/state -m '{"event_type": "press", "duration": 0.1}'
/// ```
///
/// ### Example: processing event data using a value template
///
/// In many cases, translation of an existing published payload is needed.
/// The example config below translates the payload `{"Button1": {"Action": "SINGLE"}}` of
/// the device `Button1` with event type `single` to the required format.
/// An extra attribute `button` will be set to `Button1` and be added to the entity,
/// but only if the `Action` property is set. Empty dictionaries will be ignored.
///
///
/// ```yaml
/// mqtt:
///   - event:
///       name: "Desk button"
///       state_topic: "desk/button/state"
///       event_types:
///         - single
///         - double
///       device_class: "button"
///       value_template: |
///         { {% for key in value_json %}
///         {% if value_json[key].get("Action") %}
///         "button": "{{ key }}",
///         "event_type": "{{ value_json[key].Action | lower }}"
///         {% endif %}
///         {% endfor %} }
/// ```
///
///
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Event {
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

    /// The [type/class](/integrations/event/#device-class) of the event to set the icon in the frontend. The `device_class` can be `null`.
    #[serde(rename = "dev_cla", skip_serializing_if = "Option::is_none")]
    pub device_class: Option<EventDeviceClass>,

    /// Flag which defines if the entity should be enabled when first added.
    #[serde(rename = "en", skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,

    /// The encoding of the published messages.
    #[serde(rename = "e", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,

    /// Picture URL for the entity.
    #[serde(rename = "ent_pic", skip_serializing_if = "Option::is_none")]
    pub entity_picture: Option<String>,

    /// A list of valid `event_type` strings.
    #[serde(rename = "evt_typ")]
    pub event_types: Vec<String>,

    /// [Icon](/docs/configuration/customizing-devices/#icon) for the entity.
    #[serde(rename = "ic", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation.
    #[serde(rename = "json_attr_tpl", skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation.
    #[serde(rename = "json_attr_t", skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,

    /// The name to use when displaying this event.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// Must be `event`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    #[serde(rename = "p")]
    pub platform: String,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// The MQTT topic subscribed to receive JSON event payloads. The JSON payload should contain the `event_type` element. The event type should be one of the configured `event_types`. Note that replayed retained messages will be discarded.
    #[serde(rename = "stat_t")]
    pub state_topic: String,

    /// An ID that uniquely identifies this event entity. If two events have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the value and render it to a valid JSON event payload. If the template throws an error, the current state will be used instead.
    #[serde(rename = "val_tpl", skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

impl Event {
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

    /// The [type/class](/integrations/event/#device-class) of the event to set the icon in the frontend. The `device_class` can be `null`.
    pub fn device_class(mut self, device_class: EventDeviceClass) -> Self {
        self.device_class = Some(device_class);
        self
    }

    /// Flag which defines if the entity should be enabled when first added.
    pub fn enabled_by_default(mut self, enabled_by_default: bool) -> Self {
        self.enabled_by_default = Some(enabled_by_default);
        self
    }

    /// The encoding of the published messages.
    pub fn encoding<T: Into<String>>(mut self, encoding: T) -> Self {
        self.encoding = Some(encoding.into());
        self
    }

    /// Picture URL for the entity.
    pub fn entity_picture<T: Into<String>>(mut self, entity_picture: T) -> Self {
        self.entity_picture = Some(entity_picture.into());
        self
    }

    /// A list of valid `event_type` strings.
    pub fn event_types<T: Into<String>>(mut self, event_types: Vec<T>) -> Self {
        self.event_types = event_types.into_iter().map(|v| v.into()).collect();
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

    /// The name to use when displaying this event.
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    /// Must be `event`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    pub fn platform<T: Into<String>>(mut self, platform: T) -> Self {
        self.platform = platform.into();
        self
    }

    /// The maximum QoS level to be used when receiving and publishing messages.
    pub fn qos(mut self, qos: Qos) -> Self {
        self.qos = Some(qos);
        self
    }

    /// The MQTT topic subscribed to receive JSON event payloads. The JSON payload should contain the `event_type` element. The event type should be one of the configured `event_types`. Note that replayed retained messages will be discarded.
    pub fn state_topic<T: Into<String>>(mut self, state_topic: T) -> Self {
        self.state_topic = state_topic.into();
        self
    }

    /// An ID that uniquely identifies this event entity. If two events have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    pub fn unique_id<T: Into<String>>(mut self, unique_id: T) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the value and render it to a valid JSON event payload. If the template throws an error, the current state will be used instead.
    pub fn value_template<T: Into<String>>(mut self, value_template: T) -> Self {
        self.value_template = Some(value_template.into());
        self
    }
}

impl Default for Event {
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
            event_types: Default::default(),
            icon: Default::default(),
            json_attributes_template: Default::default(),
            json_attributes_topic: Default::default(),
            name: Default::default(),
            object_id: Default::default(),
            platform: "event".to_string(),
            qos: Default::default(),
            state_topic: Default::default(),
            unique_id: Default::default(),
            value_template: Default::default(),
        }
    }
}

impl From<Event> for Entity {
    fn from(value: Event) -> Self {
        Entity::Event(value)
    }
}
