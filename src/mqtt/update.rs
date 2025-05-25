use super::common::Qos;
use super::common::{Availability, Device, EntityCategory, Origin};
use super::device_classes::UpdateDeviceClass;
use crate::Entity;
use serde_derive::Serialize;

/// ---
/// title: "MQTT Update"
/// description: "Instructions on how to interact with a device exposing an Update entity through MQTT from within Home Assistant."
/// ha_category:
///   - Update
/// ha_release: "2021.11"
/// ha_iot_class: Configurable
/// ha_domain: mqtt
/// ---
///
/// The `mqtt` Update platform allows you to integrate devices that might expose firmware/software installed and the latest versions through MQTT into Home Assistant as an Update entity. Every time a message under the `topic` in the configuration is received, the entity will be updated in Home Assistant.
///
/// ## Configuration
///
/// To enable MQTT Update in your installation, add the following to your {% term "`configuration.yaml`" %} file:
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - update:
///       state_topic: topic-installed
///       latest_version_topic: topic-latest
/// ```
///
///
/// ⚠ Important\
/// Make sure that your topic matches exactly. `some-topic/` and `some-topic` are different topics.
///
/// ## Examples
///
/// This is an example of Update entity configuration for Shelly Gen1 device.
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - update:
///       name: "Shelly Plug S Firmware Update"
///       title: "Shelly Plug S Firmware"
///       release_url: "https://shelly-api-docs.shelly.cloud/gen1/#changelog"
///       entity_picture: "https://brands.home-assistant.io/_/shelly/icon.png"
///       state_topic: "shellies/shellyplug-s-112233/info"
///       value_template: "{{ value_json['update'].old_version }}"
///       latest_version_topic: "shellies/shellyplug-s-112233/info"
///       latest_version_template: "{% if value_json['update'].new_version %}{{ value_json['update'].new_version }}{% else %}{{ value_json['update'].old_version }}{% endif %}"
///       device_class: "firmware"
///       command_topic: "shellies/shellyplug-s-112233/command"
///       payload_install: "update_fw"
/// ```
///
///
/// JSON can also be used as `state_topic` payload. Note that this feature also allows to process and show live progress information.
///
///
/// ```json
/// {
///   "installed_version": "1.21.0",
///   "latest_version": "1.22.0",
///   "title": "Device Firmware",
///   "release_url": "https://example.com/release",
///   "release_summary": "A new version of our amazing firmware",
///   "entity_picture": "https://example.com/icon.png"
/// }
/// ```
///
///
/// Simple progress state update example:
///
///
/// ```json
/// {
///   "installed_version": "1.21.0",
///   "latest_version": "1.22.0",
///   "title": "Device Firmware",
///   "release_url": "https://example.com/release",
///   "release_summary": "A new version of our amazing firmware",
///   "entity_picture": "https://example.com/icon.png",
///   "in_progress": true
/// }
/// ```
///
///
/// Update percentage state update example:
///
///
/// ```json
/// {
///   "installed_version": "1.21.0",
///   "latest_version": "1.22.0",
///   "title": "Device Firmware",
///   "release_url": "https://example.com/release",
///   "release_summary": "A new version of our amazing firmware",
///   "entity_picture": "https://example.com/icon.png",
///   "update_percentage": 78
/// }
/// ```
///
///
/// Publish `null` to reset the update percentage state update's:
///
///
/// ```json
/// {
///   "installed_version": "1.22.0",
///   "latest_version": "1.22.0",
///   "title": "Device Firmware",
///   "release_url": "https://example.com/release",
///   "release_summary": "A new version of our amazing firmware",
///   "entity_picture": "https://example.com/icon.png",
///   "update_percentage": null
/// }
/// ```
///
///
/// The values in the JSON are optional but must be valid within the following schema:
///
///
/// For the above JSON payload examples, the `update` entity configuration should look like this:
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - update:
///       name: "Amazing Device Update"
///       title: "Device Firmware"
///       state_topic: "amazing-device/state-topic"
///       device_class: "firmware"
///       command_topic: "amazing-device/command"
///       payload_install: "install"
/// ```
///
///
/// If the device/service sends data as JSON but the schema differs, `value_template` can be use to reformat the JSON.
///
///
/// ```json
/// {
///   "installed_ver": "2022.11",
///   "new_ver": "2022.12"
/// }
/// ```
///
///
/// For the above JSON payload, the `update` entity configuration should look like this:
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///    update:
///       name: "Amazing Device Update"
///       title: "Device Firmware"
///       state_topic: "amazing-device/state-topic"
///       value_template: "{{ {'installed_version': value_json.installed_ver, 'latest_version': value_json.new_ver } | to_json }}"
///       device_class: "firmware"
///       command_topic: "amazing-device/command"
///       payload_install: "install"
/// ```
///
///
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Update {
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

    /// The MQTT topic to publish `payload_install` to start installing process.
    #[serde(rename = "cmd_t", skip_serializing_if = "Option::is_none")]
    pub command_topic: Option<String>,

    /// The [type/class](/integrations/update/#device-classes) of the update to set the icon in the frontend. The `device_class` can be `null`.
    #[serde(rename = "dev_cla", skip_serializing_if = "Option::is_none")]
    pub device_class: Option<UpdateDeviceClass>,

    /// Number of decimal digits for display of update progress.
    #[serde(rename = "display_precision", skip_serializing_if = "Option::is_none")]
    pub display_precision: Option<i32>,

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

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the JSON dictionary from messages received on the `json_attributes_topic`.
    #[serde(rename = "json_attr_tpl", skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as entity attributes. Implies `force_update` of the current select state when a message is received on this topic.
    #[serde(rename = "json_attr_t", skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the latest version value. Use `state_topic` with a `value_template` if all update state values can be extracted from a single JSON payload.
    #[serde(rename = "l_ver_tpl", skip_serializing_if = "Option::is_none")]
    pub latest_version_template: Option<String>,

    /// The MQTT topic subscribed to receive an update of the latest version. Use `state_topic` with a `value_template` if all update state values can be extracted from a single JSON payload.
    #[serde(rename = "l_ver_t", skip_serializing_if = "Option::is_none")]
    pub latest_version_topic: Option<String>,

    /// The name of the Update. Can be set to `null` if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used instead of `name` for automatic generation of `entity_id`
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// The MQTT payload to start installing process.
    #[serde(rename = "pl_inst", skip_serializing_if = "Option::is_none")]
    pub payload_install: Option<String>,

    /// Must be `update`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    #[serde(rename = "platform")]
    pub platform: String,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// Summary of the release notes or changelog. This is suitable a brief update description of max 255 characters.
    #[serde(rename = "rel_s", skip_serializing_if = "Option::is_none")]
    pub release_summary: Option<String>,

    /// URL to the full release notes of the latest version available.
    #[serde(rename = "rel_u", skip_serializing_if = "Option::is_none")]
    pub release_url: Option<String>,

    /// If the published message should have the retain flag on or not.
    #[serde(rename = "ret", skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,

    /// The MQTT topic subscribed to receive state updates. The state update may be either JSON or a simple string with `installed_version` value. When a JSON payload is detected, the state value of the JSON payload should supply the `installed_version` and can optionally supply: `latest_version`, `title`, `release_summary`, `release_url`, and an `entity_picture` URL. To allow progress monitoring `in_progress` (a boolean to indicate an update is in progress), or `update_percentage` (a float value to indicate the progress percentage) may be part of the JSON message.
    #[serde(rename = "stat_t", skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,

    /// Title of the software, or firmware update. This helps to differentiate between the device or entity name versus the title of the software installed.
    #[serde(rename = "tit", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// An ID that uniquely identifies this Update. If two Updates have the same unique ID Home Assistant will raise an exception.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the `installed_version` state value or to render to a valid JSON payload on from the payload received on `state_topic`.
    #[serde(rename = "val_tpl", skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

impl Update {
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

    /// The MQTT topic to publish `payload_install` to start installing process.
    pub fn command_topic<T: Into<String>>(mut self, command_topic: T) -> Self {
        self.command_topic = Some(command_topic.into());
        self
    }

    /// The [type/class](/integrations/update/#device-classes) of the update to set the icon in the frontend. The `device_class` can be `null`.
    pub fn device_class(mut self, device_class: UpdateDeviceClass) -> Self {
        self.device_class = Some(device_class);
        self
    }

    /// Number of decimal digits for display of update progress.
    pub fn display_precision(mut self, display_precision: i32) -> Self {
        self.display_precision = Some(display_precision);
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

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the JSON dictionary from messages received on the `json_attributes_topic`.
    pub fn json_attributes_template<T: Into<String>>(
        mut self,
        json_attributes_template: T,
    ) -> Self {
        self.json_attributes_template = Some(json_attributes_template.into());
        self
    }

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as entity attributes. Implies `force_update` of the current select state when a message is received on this topic.
    pub fn json_attributes_topic<T: Into<String>>(mut self, json_attributes_topic: T) -> Self {
        self.json_attributes_topic = Some(json_attributes_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the latest version value. Use `state_topic` with a `value_template` if all update state values can be extracted from a single JSON payload.
    pub fn latest_version_template<T: Into<String>>(mut self, latest_version_template: T) -> Self {
        self.latest_version_template = Some(latest_version_template.into());
        self
    }

    /// The MQTT topic subscribed to receive an update of the latest version. Use `state_topic` with a `value_template` if all update state values can be extracted from a single JSON payload.
    pub fn latest_version_topic<T: Into<String>>(mut self, latest_version_topic: T) -> Self {
        self.latest_version_topic = Some(latest_version_topic.into());
        self
    }

    /// The name of the Update. Can be set to `null` if only the device name is relevant.
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Used instead of `name` for automatic generation of `entity_id`
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    /// The MQTT payload to start installing process.
    pub fn payload_install<T: Into<String>>(mut self, payload_install: T) -> Self {
        self.payload_install = Some(payload_install.into());
        self
    }

    /// Must be `update`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    pub fn platform<T: Into<String>>(mut self, platform: T) -> Self {
        self.platform = platform.into();
        self
    }

    /// The maximum QoS level to be used when receiving and publishing messages.
    pub fn qos(mut self, qos: Qos) -> Self {
        self.qos = Some(qos);
        self
    }

    /// Summary of the release notes or changelog. This is suitable a brief update description of max 255 characters.
    pub fn release_summary<T: Into<String>>(mut self, release_summary: T) -> Self {
        self.release_summary = Some(release_summary.into());
        self
    }

    /// URL to the full release notes of the latest version available.
    pub fn release_url<T: Into<String>>(mut self, release_url: T) -> Self {
        self.release_url = Some(release_url.into());
        self
    }

    /// If the published message should have the retain flag on or not.
    pub fn retain(mut self, retain: bool) -> Self {
        self.retain = Some(retain);
        self
    }

    /// The MQTT topic subscribed to receive state updates. The state update may be either JSON or a simple string with `installed_version` value. When a JSON payload is detected, the state value of the JSON payload should supply the `installed_version` and can optionally supply: `latest_version`, `title`, `release_summary`, `release_url`, and an `entity_picture` URL. To allow progress monitoring `in_progress` (a boolean to indicate an update is in progress), or `update_percentage` (a float value to indicate the progress percentage) may be part of the JSON message.
    pub fn state_topic<T: Into<String>>(mut self, state_topic: T) -> Self {
        self.state_topic = Some(state_topic.into());
        self
    }

    /// Title of the software, or firmware update. This helps to differentiate between the device or entity name versus the title of the software installed.
    pub fn title<T: Into<String>>(mut self, title: T) -> Self {
        self.title = Some(title.into());
        self
    }

    /// An ID that uniquely identifies this Update. If two Updates have the same unique ID Home Assistant will raise an exception.
    pub fn unique_id<T: Into<String>>(mut self, unique_id: T) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the `installed_version` state value or to render to a valid JSON payload on from the payload received on `state_topic`.
    pub fn value_template<T: Into<String>>(mut self, value_template: T) -> Self {
        self.value_template = Some(value_template.into());
        self
    }
}

impl Default for Update {
    fn default() -> Self {
        Self {
            topic_prefix: Default::default(),
            origin: Default::default(),
            device: Default::default(),
            entity_category: Default::default(),
            availability: Default::default(),
            command_topic: Default::default(),
            device_class: Default::default(),
            display_precision: Default::default(),
            enabled_by_default: Default::default(),
            encoding: Default::default(),
            entity_picture: Default::default(),
            icon: Default::default(),
            json_attributes_template: Default::default(),
            json_attributes_topic: Default::default(),
            latest_version_template: Default::default(),
            latest_version_topic: Default::default(),
            name: Default::default(),
            object_id: Default::default(),
            payload_install: Default::default(),
            platform: "update".to_string(),
            qos: Default::default(),
            release_summary: Default::default(),
            release_url: Default::default(),
            retain: Default::default(),
            state_topic: Default::default(),
            title: Default::default(),
            unique_id: Default::default(),
            value_template: Default::default(),
        }
    }
}

impl From<Update> for Entity {
    fn from(value: Update) -> Self {
        Entity::Update(value)
    }
}
