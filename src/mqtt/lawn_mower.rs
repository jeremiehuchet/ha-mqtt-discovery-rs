use super::common::Qos;
use super::common::{Availability, Device, EntityCategory, Origin};
use crate::Entity;
use serde_derive::Serialize;

/// ---
/// title: "MQTT lawn mower"
/// description: "Instructions on how to integrate MQTT lawn mowers into Home Assistant."
/// ha_category:
///   - Lawn mower
/// ha_release: 2023.9
/// ha_iot_class: Configurable
/// ha_domain: mqtt
/// ---
///
/// The `mqtt` `lawn_mower` platform allows controlling a lawn mower over MQTT.
///
/// ## Configuration
///
/// To enable MQTT lawn mower in your installation, add the following to your {% term "`configuration.yaml`" %} file:
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - lawn_mower:
///       command_topic: topic
///       name: "Test Lawn Mower"
/// ```
///
///
/// ⚠ Important\
/// Make sure that your topic matches exactly. `some-topic/` and `some-topic` are different topics.
///
/// ## Example
///
/// The example below shows how to use a single command topic with a command template.
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - lawn_mower:
///       name: "Lawn Mower Plus"
///       activity_state_topic: "lawn_mower_plus/state"
///       activity_value_template: "{{ value_json.activity }}"
///       pause_command_topic: "lawn_mower_plus/set"
///       pause_command_template: '{"activity": "{{ value }}"}'
///       dock_command_topic: "lawn_mower_plus/set"
///       dock_command_template: '{"activity": "{{ value }}"}'
///       start_mowing_command_topic: "lawn_mower_plus/set"
///       start_mowing_command_template: '{"activity": "{{ value }}"}'
/// ```
///
///
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LawnMower {
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

    /// The MQTT topic subscribed to receive an update of the activity. Valid activities are `mowing`, `paused`, `docked`, and `error`. Use `value_template` to extract the activity state from a custom payload. When payload `none` is received, the activity state will be reset to `unknown`.
    #[serde(
        rename = "activity_state_topic",
        skip_serializing_if = "Option::is_none"
    )]
    pub activity_state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the value.
    #[serde(
        rename = "activity_value_template",
        skip_serializing_if = "Option::is_none"
    )]
    pub activity_value_template: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `dock_command_topic`. The `value` parameter in the template will be set to `dock`.
    #[serde(
        rename = "dock_command_template",
        skip_serializing_if = "Option::is_none"
    )]
    pub dock_command_template: Option<String>,

    /// The MQTT topic that publishes commands when the `lawn_mower.dock` action is performed. The value `dock` is published when the action is used. Use a `dock_command_template` to publish a custom format.
    #[serde(rename = "dock_command_topic", skip_serializing_if = "Option::is_none")]
    pub dock_command_topic: Option<String>,

    /// Flag which defines if the entity should be enabled when first added.
    #[serde(rename = "en", skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,

    /// The encoding of the payloads received and published messages. Set to `""` to disable decoding of the incoming payload.
    #[serde(rename = "e", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,

    /// Picture URL for the entity.
    #[serde(rename = "ent_pic", skip_serializing_if = "Option::is_none")]
    pub entity_picture: Option<String>,

    /// [Icon](/docs/configuration/customizing-devices/#icon) for the entity.
    #[serde(rename = "ic", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the JSON dictionary from messages received on the `json_attributes_topic`.
    #[serde(rename = "json_attr_tpl", skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as entity attributes. Implies `force_update` of the current activity state when a message is received on this topic.
    #[serde(rename = "json_attr_t", skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,

    /// The name of the lawn mower. Can be set to `null` if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used instead of `name` for automatic generation of `entity_id`
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// Flag that defines if the lawn mower works in optimistic mode.
    #[serde(rename = "opt", skip_serializing_if = "Option::is_none")]
    pub optimistic: Option<bool>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `pause_command_topic`. The `value` parameter in the template will be set to `pause`.
    #[serde(
        rename = "pause_command_template",
        skip_serializing_if = "Option::is_none"
    )]
    pub pause_command_template: Option<String>,

    /// The MQTT topic that publishes commands when the `lawn_mower.pause` action is performed. The value `pause` is published when the action is used. Use a `pause_command_template` to publish a custom format.
    #[serde(
        rename = "pause_command_topic",
        skip_serializing_if = "Option::is_none"
    )]
    pub pause_command_topic: Option<String>,

    /// Must be `lawn_mower`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    #[serde(rename = "platform")]
    pub platform: String,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// If the published message should have the retain flag on or not.
    #[serde(rename = "ret", skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,

    /// The MQTT topic that publishes commands when the `lawn_mower.start_mowing` action is performed. The value `start_mowing` is published when the action used. Use a `start_mowing_command_template` to publish a custom format.
    #[serde(
        rename = "start_mowing_command_topic",
        skip_serializing_if = "Option::is_none"
    )]
    pub start_mowing_command_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `start_mowing_command_topic`. The `value` parameter in the template will be set to `start_mowing`.
    #[serde(
        rename = "start_mowing_template",
        skip_serializing_if = "Option::is_none"
    )]
    pub start_mowing_template: Option<String>,

    /// An ID that uniquely identifies this lawn mower. If two lawn mowers have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
}

impl LawnMower {
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

    /// The MQTT topic subscribed to receive an update of the activity. Valid activities are `mowing`, `paused`, `docked`, and `error`. Use `value_template` to extract the activity state from a custom payload. When payload `none` is received, the activity state will be reset to `unknown`.
    pub fn activity_state_topic<T: Into<String>>(mut self, activity_state_topic: T) -> Self {
        self.activity_state_topic = Some(activity_state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the value.
    pub fn activity_value_template<T: Into<String>>(mut self, activity_value_template: T) -> Self {
        self.activity_value_template = Some(activity_value_template.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `dock_command_topic`. The `value` parameter in the template will be set to `dock`.
    pub fn dock_command_template<T: Into<String>>(mut self, dock_command_template: T) -> Self {
        self.dock_command_template = Some(dock_command_template.into());
        self
    }

    /// The MQTT topic that publishes commands when the `lawn_mower.dock` action is performed. The value `dock` is published when the action is used. Use a `dock_command_template` to publish a custom format.
    pub fn dock_command_topic<T: Into<String>>(mut self, dock_command_topic: T) -> Self {
        self.dock_command_topic = Some(dock_command_topic.into());
        self
    }

    /// Flag which defines if the entity should be enabled when first added.
    pub fn enabled_by_default(mut self, enabled_by_default: bool) -> Self {
        self.enabled_by_default = Some(enabled_by_default);
        self
    }

    /// The encoding of the payloads received and published messages. Set to `""` to disable decoding of the incoming payload.
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

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the JSON dictionary from messages received on the `json_attributes_topic`.
    pub fn json_attributes_template<T: Into<String>>(
        mut self,
        json_attributes_template: T,
    ) -> Self {
        self.json_attributes_template = Some(json_attributes_template.into());
        self
    }

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as entity attributes. Implies `force_update` of the current activity state when a message is received on this topic.
    pub fn json_attributes_topic<T: Into<String>>(mut self, json_attributes_topic: T) -> Self {
        self.json_attributes_topic = Some(json_attributes_topic.into());
        self
    }

    /// The name of the lawn mower. Can be set to `null` if only the device name is relevant.
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Used instead of `name` for automatic generation of `entity_id`
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    /// Flag that defines if the lawn mower works in optimistic mode.
    pub fn optimistic(mut self, optimistic: bool) -> Self {
        self.optimistic = Some(optimistic);
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `pause_command_topic`. The `value` parameter in the template will be set to `pause`.
    pub fn pause_command_template<T: Into<String>>(mut self, pause_command_template: T) -> Self {
        self.pause_command_template = Some(pause_command_template.into());
        self
    }

    /// The MQTT topic that publishes commands when the `lawn_mower.pause` action is performed. The value `pause` is published when the action is used. Use a `pause_command_template` to publish a custom format.
    pub fn pause_command_topic<T: Into<String>>(mut self, pause_command_topic: T) -> Self {
        self.pause_command_topic = Some(pause_command_topic.into());
        self
    }

    /// Must be `lawn_mower`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
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

    /// The MQTT topic that publishes commands when the `lawn_mower.start_mowing` action is performed. The value `start_mowing` is published when the action used. Use a `start_mowing_command_template` to publish a custom format.
    pub fn start_mowing_command_topic<T: Into<String>>(
        mut self,
        start_mowing_command_topic: T,
    ) -> Self {
        self.start_mowing_command_topic = Some(start_mowing_command_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `start_mowing_command_topic`. The `value` parameter in the template will be set to `start_mowing`.
    pub fn start_mowing_template<T: Into<String>>(mut self, start_mowing_template: T) -> Self {
        self.start_mowing_template = Some(start_mowing_template.into());
        self
    }

    /// An ID that uniquely identifies this lawn mower. If two lawn mowers have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    pub fn unique_id<T: Into<String>>(mut self, unique_id: T) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }
}

impl Default for LawnMower {
    fn default() -> Self {
        Self {
            topic_prefix: Default::default(),
            origin: Default::default(),
            device: Default::default(),
            entity_category: Default::default(),
            availability: Default::default(),
            activity_state_topic: Default::default(),
            activity_value_template: Default::default(),
            dock_command_template: Default::default(),
            dock_command_topic: Default::default(),
            enabled_by_default: Default::default(),
            encoding: Default::default(),
            entity_picture: Default::default(),
            icon: Default::default(),
            json_attributes_template: Default::default(),
            json_attributes_topic: Default::default(),
            name: Default::default(),
            object_id: Default::default(),
            optimistic: Default::default(),
            pause_command_template: Default::default(),
            pause_command_topic: Default::default(),
            platform: "lawn_mower".to_string(),
            qos: Default::default(),
            retain: Default::default(),
            start_mowing_command_topic: Default::default(),
            start_mowing_template: Default::default(),
            unique_id: Default::default(),
        }
    }
}

impl From<LawnMower> for Entity {
    fn from(value: LawnMower) -> Self {
        Entity::LawnMower(value)
    }
}
