use crate::common::Qos;
use crate::common::SensorStateClass;
use crate::common::TemperatureUnit;
use crate::generated::BinarySensorDeviceClass;
use crate::generated::ButtonDeviceClass;
use crate::generated::EventDeviceClass;
use crate::generated::HumidifierDeviceClass;
use crate::generated::NumberDeviceClass;
use crate::generated::SensorDeviceClass;
use crate::generated::SwitchDeviceClass;
use crate::generated::Unit;
use crate::generated::UpdateDeviceClass;
use crate::{
    Entity,
    common::{Availability, DeviceInformation, EntityCategory, Origin},
};
pub use rust_decimal::Decimal;
use serde_derive::Serialize;

/// ---
/// title: "MQTT Camera"
/// description: "Instructions on how to use an MQTT image message as a Camera within Home Assistant."
/// ha_category:
///   - Camera
/// ha_release: 0.43
/// ha_iot_class: Configurable
/// ha_domain: mqtt
/// ---
///
/// The `mqtt` camera platform allows you to integrate the content of an image file sent through MQTT into Home Assistant as a camera. Every time a message under the `topic` in the configuration is received, the image displayed in Home Assistant will also be updated. Messages received on `topic` should contain the full contents of an image file, for example, a JPEG image, without any additional encoding or metadata.
///
/// This can be used with an application or a service capable of sending images through MQTT.
///
/// ## Configuration
///
/// To use an MQTT camera in your installation, add the following to your `configuration.yaml` file.
/// {% include integrations/restart_ha_after_config_inclusion.md %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - camera:
///       topic: zanzito/shared_locations/my-device
/// ```
///
/// Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
///
/// The sample configuration above can be tested by publishing an image to the topic from the console:
///
/// ```shell
/// mosquitto_pub -h <mqtt_broker> -t zanzito/shared_locations/my-device -f <camera_imaga.jpg>
/// ```
///
///
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Camera {
    /// Replaces `~` with this value in any MQTT topic attribute.
    /// [See Home Assistant documentation](https://www.home-assistant.io/integrations/mqtt/#using-abbreviations-and-base-topic)
    #[serde(rename = "~", skip_serializing_if = "Option::is_none")]
    pub topic_prefix: Option<String>,

    /// It is encouraged to add additional information about the origin that supplies MQTT entities via MQTT discovery by adding the origin option (can be abbreviated to o) to the discovery payload. Note that these options also support abbreviations. Information of the origin will be logged to the core event log when an item is discovered or updated.
    #[serde(rename = "o")]
    pub origin: Origin,

    /// Information about the device this button is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/en/device_registry_index.html). Only works when [`unique_id`](#unique_id) is set. At least one of identifiers or connections must be present to identify the device.
    #[serde(rename = "dev")]
    pub device: DeviceInformation,

    /// A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`.
    #[serde(flatten)]
    pub availability: Availability,

    /// The category of the entity. (optional, default: None)
    #[serde(rename = "ent_cat", skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<EntityCategory>,

    /// Flag which defines if the entity should be enabled when first added.
    #[serde(rename = "en", skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,

    /// The encoding of the payloads received. Set to `""` to disable decoding of incoming payload. Use `image_encoding` to enable `Base64` decoding on `topic`.
    #[serde(rename = "e", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,

    /// Picture URL for the entity.
    #[serde(rename = "ent_pic", skip_serializing_if = "Option::is_none")]
    pub entity_picture: Option<String>,

    /// [Icon](/docs/configuration/customizing-devices/#icon) for the entity.
    #[serde(rename = "ic", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// The encoding of the image payloads received. Set to `"b64"` to enable base64 decoding of image payload. If not set, the image payload must be raw binary data.
    #[serde(rename = "img_e", skip_serializing_if = "Option::is_none")]
    pub image_encoding: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the JSON dictionary from messages received on the `json_attributes_topic`.
    #[serde(rename = "json_attr_tpl", skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Implies `force_update` of the current sensor state when a message is received on this topic.
    #[serde(rename = "json_attr_t", skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,

    /// The name of the camera. Can be set to `null` if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// The MQTT topic to subscribe to.
    #[serde(rename = "t")]
    pub topic: String,

    /// An ID that uniquely identifies this camera. If two cameras have the same unique ID Home Assistant will raise an exception. Required when used with device-based discovery.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
}

impl Camera {
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
    pub fn device(mut self, device: DeviceInformation) -> Self {
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

    /// Flag which defines if the entity should be enabled when first added.
    pub fn enabled_by_default(mut self, enabled_by_default: bool) -> Self {
        self.enabled_by_default = Some(enabled_by_default);
        self
    }

    /// The encoding of the payloads received. Set to `""` to disable decoding of incoming payload. Use `image_encoding` to enable `Base64` decoding on `topic`.
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

    /// The encoding of the image payloads received. Set to `"b64"` to enable base64 decoding of image payload. If not set, the image payload must be raw binary data.
    pub fn image_encoding<T: Into<String>>(mut self, image_encoding: T) -> Self {
        self.image_encoding = Some(image_encoding.into());
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

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Implies `force_update` of the current sensor state when a message is received on this topic.
    pub fn json_attributes_topic<T: Into<String>>(mut self, json_attributes_topic: T) -> Self {
        self.json_attributes_topic = Some(json_attributes_topic.into());
        self
    }

    /// The name of the camera. Can be set to `null` if only the device name is relevant.
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    /// The MQTT topic to subscribe to.
    pub fn topic<T: Into<String>>(mut self, topic: T) -> Self {
        self.topic = topic.into();
        self
    }

    /// An ID that uniquely identifies this camera. If two cameras have the same unique ID Home Assistant will raise an exception. Required when used with device-based discovery.
    pub fn unique_id<T: Into<String>>(mut self, unique_id: T) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            topic_prefix: Default::default(),
            origin: Default::default(),
            device: Default::default(),
            entity_category: Default::default(),
            availability: Default::default(),
            enabled_by_default: Default::default(),
            encoding: Default::default(),
            entity_picture: Default::default(),
            icon: Default::default(),
            image_encoding: Default::default(),
            json_attributes_template: Default::default(),
            json_attributes_topic: Default::default(),
            name: Default::default(),
            object_id: Default::default(),
            topic: Default::default(),
            unique_id: Default::default(),
        }
    }
}

impl From<Camera> for Entity {
    fn from(value: Camera) -> Self {
        Entity::Camera(value)
    }
}
/// ---
/// title: "MQTT button"
/// description: "Instructions on how to integrate MQTT buttons into Home Assistant."
/// ha_category:
///   - Button
/// ha_release: 2021.12
/// ha_iot_class: Configurable
/// ha_domain: mqtt
/// ---
///
/// The `mqtt` button platform lets you send an MQTT message when the button is pressed in the frontend or the button press action is called. This can be used to expose some service of a remote device, for example reboot.
///
/// To use an MQTT button in your installation, [add a MQTT device as a subentry](/integrations/mqtt/#configuration), or add the following to your `configuration.yaml` file.
/// {% include integrations/restart_ha_after_config_inclusion.md %}
///
/// ## Configuration
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - button:
///       command_topic: "home/bedroom/switch1/reboot"
/// ```
///
/// Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
///
///
/// ⚠ Important\
/// Make sure that your topic matches exactly. `some-topic/` and `some-topic` are different topics.
///
/// ## Examples
///
/// In this section, you will find some real-life examples of how to use this feature.
///
/// ### Full configuration
///
/// The example below shows a full configuration for a button.
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - button:
///       unique_id: bedroom_switch_reboot_btn
///       name: "Restart Bedroom Switch"
///       command_topic: "home/bedroom/switch1/commands"
///       payload_press: "restart"
///       availability:
///         - topic: "home/bedroom/switch1/available"
///       qos: 0
///       retain: false
///       entity_category: "config"
///       device_class: "restart"
/// ```
///
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Button {
    /// Replaces `~` with this value in any MQTT topic attribute.
    /// [See Home Assistant documentation](https://www.home-assistant.io/integrations/mqtt/#using-abbreviations-and-base-topic)
    #[serde(rename = "~", skip_serializing_if = "Option::is_none")]
    pub topic_prefix: Option<String>,

    /// It is encouraged to add additional information about the origin that supplies MQTT entities via MQTT discovery by adding the origin option (can be abbreviated to o) to the discovery payload. Note that these options also support abbreviations. Information of the origin will be logged to the core event log when an item is discovered or updated.
    #[serde(rename = "o")]
    pub origin: Origin,

    /// Information about the device this button is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/en/device_registry_index.html). Only works when [`unique_id`](#unique_id) is set. At least one of identifiers or connections must be present to identify the device.
    #[serde(rename = "dev")]
    pub device: DeviceInformation,

    /// A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`.
    #[serde(flatten)]
    pub availability: Availability,

    /// The category of the entity. (optional, default: None)
    #[serde(rename = "ent_cat", skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<EntityCategory>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `command_topic`.
    #[serde(rename = "cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub command_template: Option<String>,

    /// The MQTT topic to publish commands to trigger the button.
    #[serde(rename = "cmd_t")]
    pub command_topic: String,

    /// The [type/class](/integrations/button/#device-class) of the button to set the icon in the frontend. The `device_class` can be `null`.
    #[serde(rename = "dev_cla", skip_serializing_if = "Option::is_none")]
    pub device_class: Option<ButtonDeviceClass>,

    /// Flag which defines if the entity should be enabled when first added.
    #[serde(rename = "en", skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,

    /// The encoding of the published messages.
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

    /// The name to use when displaying this button. Can be set to `null` if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// The payload To send to trigger the button.
    #[serde(rename = "pl_prs", skip_serializing_if = "Option::is_none")]
    pub payload_press: Option<String>,

    /// Must be `button`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    #[serde(rename = "p")]
    pub platform: String,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// If the published message should have the retain flag on or not.
    #[serde(rename = "ret", skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,

    /// An ID that uniquely identifies this button entity. If two buttons have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
}

impl Button {
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
    pub fn device(mut self, device: DeviceInformation) -> Self {
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

    /// The MQTT topic to publish commands to trigger the button.
    pub fn command_topic<T: Into<String>>(mut self, command_topic: T) -> Self {
        self.command_topic = command_topic.into();
        self
    }

    /// The [type/class](/integrations/button/#device-class) of the button to set the icon in the frontend. The `device_class` can be `null`.
    pub fn device_class(mut self, device_class: ButtonDeviceClass) -> Self {
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

    /// The name to use when displaying this button. Can be set to `null` if only the device name is relevant.
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    /// The payload To send to trigger the button.
    pub fn payload_press<T: Into<String>>(mut self, payload_press: T) -> Self {
        self.payload_press = Some(payload_press.into());
        self
    }

    /// Must be `button`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
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

    /// An ID that uniquely identifies this button entity. If two buttons have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    pub fn unique_id<T: Into<String>>(mut self, unique_id: T) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }
}

impl Default for Button {
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
            payload_press: Default::default(),
            platform: "button".to_string(),
            qos: Default::default(),
            retain: Default::default(),
            unique_id: Default::default(),
        }
    }
}

impl From<Button> for Entity {
    fn from(value: Button) -> Self {
        Entity::Button(value)
    }
}
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
    pub device: DeviceInformation,

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
    #[serde(rename = "curr_hum_tpl", skip_serializing_if = "Option::is_none")]
    pub current_humidity_template: Option<String>,

    /// The MQTT topic on which to listen for the current humidity. A `"None"` value received will reset the current humidity. Empty values (`'''`) will be ignored.
    #[serde(rename = "curr_hum_t", skip_serializing_if = "Option::is_none")]
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
    #[serde(rename = "p")]
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
    pub fn device(mut self, device: DeviceInformation) -> Self {
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
/// ---
/// title: "MQTT climate (HVAC)"
/// description: "Instructions on how to integrate MQTT climate into Home Assistant."
/// ha_category:
///   - Climate
/// ha_release: 0.55
/// ha_iot_class: Local Polling
/// ha_domain: mqtt
/// ---
///
/// The `mqtt` climate platform lets you control your MQTT enabled HVAC devices.
///
/// ## Configuration
///
/// To use an MQTT climate in your installation, [add an MQTT device as a subentry](/integrations/mqtt/#configuration), or add the following to your `configuration.yaml` file.
/// {% include integrations/restart_ha_after_config_inclusion.md %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - climate:
///       name: Study
///       mode_command_topic: "study/ac/mode/set"
/// ```
///
/// Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
///
///
/// ## Optimistic mode
///
/// If a property works in *optimistic mode* (when the corresponding state topic is not set), Home Assistant will assume that any state changes published to the command topics did work and change the internal state of the entity immediately after publishing to the command topic. If it does not work in optimistic mode, the internal state of the entity is only updated when the requested update is confirmed by the device through the state topic. You can enforce optimistic mode by setting the `optimistic` option to `true`. When set, the internal state will always be updated, even when a state topic is defined.
///
/// ## Using templates
///
/// For all `*_state_topic`s, a template can be specified that will be used to render the incoming payloads on these topics. Also, a default template that applies to all state topics can be specified as `value_template`. This can be useful if you received payloads are e.g., in JSON format. Since in JSON, a quoted string (e.g., `"foo"`) is just a string, this can also be used for unquoting.
///
/// Say you receive the operation mode `"auto"` via your `mode_state_topic`, but the mode is actually called just `auto`, here's what you could do:
///
///
/// ```yaml
/// mqtt:
///   - climate:
///       name: Study
///       modes:
///         - "off"
///         - "heat"
///         - "auto"
///       mode_command_topic: "study/ac/mode/set"
///       mode_state_topic: "study/ac/mode/state"
///       mode_state_template: "{{ value_json }}"
/// ```
///
///
/// This will parse the incoming `"auto"` as JSON, resulting in `auto`. Obviously, in this case you could also just set `value_template: {% raw %}"{{ value_json }}"{% endraw %}`.
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
///   - climate:
///       name: Study
///       modes:
///         - "off"
///         - "cool"
///         - "fan_only"
///       swing_horizontal_modes:
///         - "on"
///         - "off"
///       swing_modes:
///         - "on"
///         - "off"
///       fan_modes:
///         - "high"
///         - "medium"
///         - "low"
///       preset_modes:
///         - "eco"
///         - "sleep"
///         - "activity"
///       power_command_topic: "study/ac/power/set"
///       preset_mode_command_topic: "study/ac/preset_mode/set"
///       mode_command_topic: "study/ac/mode/set"
///       mode_command_template: "{{ value if value=="off" else "on" }}"
///       temperature_command_topic: "study/ac/temperature/set"
///       fan_mode_command_topic: "study/ac/fan/set"
///       swing_horizontal_mode_command_topic: "study/ac/swingH/set"
///       swing_mode_command_topic: "study/ac/swing/set"
///       precision: 1.0
/// ```
///
///
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Climate {
    /// Replaces `~` with this value in any MQTT topic attribute.
    /// [See Home Assistant documentation](https://www.home-assistant.io/integrations/mqtt/#using-abbreviations-and-base-topic)
    #[serde(rename = "~", skip_serializing_if = "Option::is_none")]
    pub topic_prefix: Option<String>,

    /// It is encouraged to add additional information about the origin that supplies MQTT entities via MQTT discovery by adding the origin option (can be abbreviated to o) to the discovery payload. Note that these options also support abbreviations. Information of the origin will be logged to the core event log when an item is discovered or updated.
    #[serde(rename = "o")]
    pub origin: Origin,

    /// Information about the device this button is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/en/device_registry_index.html). Only works when [`unique_id`](#unique_id) is set. At least one of identifiers or connections must be present to identify the device.
    #[serde(rename = "dev")]
    pub device: DeviceInformation,

    /// A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`.
    #[serde(flatten)]
    pub availability: Availability,

    /// The category of the entity. (optional, default: None)
    #[serde(rename = "ent_cat", skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<EntityCategory>,

    /// A template to render the value received on the `action_topic` with.
    #[serde(rename = "act_tpl", skip_serializing_if = "Option::is_none")]
    pub action_template: Option<String>,

    /// The MQTT topic to subscribe for changes of the current action. If this is set, the climate graph uses the value received as data source. A "None" payload resets the current action state. An empty payload is ignored. Valid action values: `off`, `heating`, `cooling`, `drying`, `idle`, `fan`.
    #[serde(rename = "act_t", skip_serializing_if = "Option::is_none")]
    pub action_topic: Option<String>,

    /// A template with which the value received on `current_humidity_topic` will be rendered.
    #[serde(rename = "curr_hum_tpl", skip_serializing_if = "Option::is_none")]
    pub current_humidity_template: Option<String>,

    /// The MQTT topic on which to listen for the current humidity. A `"None"` value received will reset the current humidity. Empty values (`'''`) will be ignored.
    #[serde(rename = "curr_hum_t", skip_serializing_if = "Option::is_none")]
    pub current_humidity_topic: Option<String>,

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

    /// A template to render the value sent to the `fan_mode_command_topic` with.
    #[serde(rename = "fan_mode_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub fan_mode_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the fan mode.
    #[serde(rename = "fan_mode_cmd_t", skip_serializing_if = "Option::is_none")]
    pub fan_mode_command_topic: Option<String>,

    /// A template to render the value received on the `fan_mode_state_topic` with.
    #[serde(rename = "fan_mode_stat_tpl", skip_serializing_if = "Option::is_none")]
    pub fan_mode_state_template: Option<String>,

    /// The MQTT topic to subscribe for changes of the HVAC fan mode. If this is not set, the fan mode works in optimistic mode (see below). A "None" payload resets the fan mode state. An empty payload is ignored.
    #[serde(rename = "fan_mode_stat_t", skip_serializing_if = "Option::is_none")]
    pub fan_mode_state_topic: Option<String>,

    /// A list of supported fan modes.
    #[serde(rename = "fan_modes", skip_serializing_if = "Option::is_none")]
    pub fan_modes: Option<Vec<String>>,

    /// [Icon](/docs/configuration/customizing-devices/#icon) for the entity.
    #[serde(rename = "ic", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// Set the initial target temperature. The default value depends on the temperature unit and will be 21° or 69.8°F.
    #[serde(rename = "init", skip_serializing_if = "Option::is_none")]
    pub initial: Option<Decimal>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation.
    #[serde(rename = "json_attr_tpl", skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation.
    #[serde(rename = "json_attr_t", skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,

    /// The minimum target humidity percentage that can be set.
    #[serde(rename = "max_hum", skip_serializing_if = "Option::is_none")]
    pub max_humidity: Option<Decimal>,

    /// Maximum set point available. The default value depends on the temperature unit, and will be 35°C or 95°F.
    #[serde(rename = "max_temp", skip_serializing_if = "Option::is_none")]
    pub max_temp: Option<Decimal>,

    /// The maximum target humidity percentage that can be set.
    #[serde(rename = "min_hum", skip_serializing_if = "Option::is_none")]
    pub min_humidity: Option<Decimal>,

    /// Minimum set point available. The default value depends on the temperature unit, and will be 7°C or 44.6°F.
    #[serde(rename = "min_temp", skip_serializing_if = "Option::is_none")]
    pub min_temp: Option<Decimal>,

    /// A template to render the value sent to the `mode_command_topic` with.
    #[serde(rename = "mode_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub mode_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the HVAC operation mode.
    #[serde(rename = "mode_cmd_t", skip_serializing_if = "Option::is_none")]
    pub mode_command_topic: Option<String>,

    /// A template to render the value received on the `mode_state_topic` with.
    #[serde(rename = "mode_stat_tpl", skip_serializing_if = "Option::is_none")]
    pub mode_state_template: Option<String>,

    /// The MQTT topic to subscribe for changes of the HVAC operation mode. If this is not set, the operation mode works in optimistic mode (see below). A "None" payload resets to an `unknown` state. An empty payload is ignored.
    #[serde(rename = "mode_stat_t", skip_serializing_if = "Option::is_none")]
    pub mode_state_topic: Option<String>,

    /// A list of supported modes. Needs to be a subset of the default values.
    #[serde(rename = "modes", skip_serializing_if = "Option::is_none")]
    pub modes: Option<Vec<String>>,

    /// The name of the HVAC. Can be set to `null` if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// Flag that defines if the climate works in optimistic mode
    #[serde(rename = "opt", skip_serializing_if = "Option::is_none")]
    pub optimistic: Option<bool>,

    /// The payload sent to turn off the device.
    #[serde(rename = "pl_off", skip_serializing_if = "Option::is_none")]
    pub payload_off: Option<String>,

    /// The payload sent to turn the device on.
    #[serde(rename = "pl_on", skip_serializing_if = "Option::is_none")]
    pub payload_on: Option<String>,

    /// A template to render the value sent to the `power_command_topic` with. The `value` parameter is the payload set for `payload_on` or `payload_off`.
    #[serde(rename = "pow_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub power_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the HVAC power state. Sends the payload configured with `payload_on` if the climate is turned on via the `climate.turn_on`, or the payload configured with `payload_off` if the climate is turned off via the `climate.turn_off` action. Note that `optimistic` mode is not supported through `climate.turn_on` and `climate.turn_off` actions. When called, these actions will send a power command to the device but will not optimistically update the state of the climate entity. The climate device should report its state back via `mode_state_topic`.
    #[serde(rename = "pow_cmd_t", skip_serializing_if = "Option::is_none")]
    pub power_command_topic: Option<String>,

    /// The desired precision for this device. Can be used to match your actual thermostat's precision. Supported values are `0.1`, `0.5` and `1.0`.
    #[serde(rename = "precision", skip_serializing_if = "Option::is_none")]
    pub precision: Option<Decimal>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `preset_mode_command_topic`.
    #[serde(rename = "pr_mode_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub preset_mode_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the preset mode.
    #[serde(rename = "pr_mode_cmd_t", skip_serializing_if = "Option::is_none")]
    pub preset_mode_command_topic: Option<String>,

    /// The MQTT topic subscribed to receive climate speed based on presets. When preset 'none' is received or `None` the `preset_mode` will be reset.
    #[serde(rename = "pr_mode_stat_t", skip_serializing_if = "Option::is_none")]
    pub preset_mode_state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the `preset_mode` value from the payload received on `preset_mode_state_topic`.
    #[serde(rename = "pr_mode_val_tpl", skip_serializing_if = "Option::is_none")]
    pub preset_mode_value_template: Option<String>,

    /// List of preset modes this climate is supporting. Common examples include `eco`, `away`, `boost`, `comfort`, `home`, `sleep` and `activity`.
    #[serde(rename = "pr_modes", skip_serializing_if = "Option::is_none")]
    pub preset_modes: Option<Vec<String>>,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// Defines if published messages should have the retain flag set.
    #[serde(rename = "ret", skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,

    /// A template to render the value sent to the `swing_horizontal_mode_command_topic` with.
    #[serde(
        rename = "swing_h_mode_cmd_tpl",
        skip_serializing_if = "Option::is_none"
    )]
    pub swing_horizontal_mode_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the swing horizontal mode.
    #[serde(rename = "swing_h_mode_cmd_t", skip_serializing_if = "Option::is_none")]
    pub swing_horizontal_mode_command_topic: Option<String>,

    /// A template to render the value received on the `swing_horizontal_mode_state_topic` with.
    #[serde(
        rename = "swing_h_mode_stat_tpl",
        skip_serializing_if = "Option::is_none"
    )]
    pub swing_horizontal_mode_state_template: Option<String>,

    /// The MQTT topic to subscribe for changes of the HVAC swing horizontal mode. If this is not set, the swing horizontal mode works in optimistic mode (see below).
    #[serde(
        rename = "swing_h_mode_stat_t",
        skip_serializing_if = "Option::is_none"
    )]
    pub swing_horizontal_mode_state_topic: Option<String>,

    /// A list of supported swing horizontal modes.
    #[serde(rename = "swing_h_modes", skip_serializing_if = "Option::is_none")]
    pub swing_horizontal_modes: Option<Vec<String>>,

    /// A template to render the value sent to the `swing_mode_command_topic` with.
    #[serde(rename = "swing_mode_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub swing_mode_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the swing mode.
    #[serde(rename = "swing_mode_cmd_t", skip_serializing_if = "Option::is_none")]
    pub swing_mode_command_topic: Option<String>,

    /// A template to render the value received on the `swing_mode_state_topic` with.
    #[serde(
        rename = "swing_mode_stat_tpl",
        skip_serializing_if = "Option::is_none"
    )]
    pub swing_mode_state_template: Option<String>,

    /// The MQTT topic to subscribe for changes of the HVAC swing mode. If this is not set, the swing mode works in optimistic mode (see below).
    #[serde(rename = "swing_mode_stat_t", skip_serializing_if = "Option::is_none")]
    pub swing_mode_state_topic: Option<String>,

    /// A list of supported swing modes.
    #[serde(rename = "swing_modes", skip_serializing_if = "Option::is_none")]
    pub swing_modes: Option<Vec<String>>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `target_humidity_command_topic`.
    #[serde(rename = "hum_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub target_humidity_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the target humidity.
    #[serde(rename = "hum_cmd_t", skip_serializing_if = "Option::is_none")]
    pub target_humidity_command_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract a value for the climate `target_humidity` state.
    #[serde(rename = "hum_state_tpl", skip_serializing_if = "Option::is_none")]
    pub target_humidity_state_template: Option<String>,

    /// The MQTT topic subscribed to receive the target humidity. If this is not set, the target humidity works in optimistic mode (see below). A `"None"` value received will reset the target humidity. Empty values (`'''`) will be ignored.
    #[serde(rename = "hum_stat_t", skip_serializing_if = "Option::is_none")]
    pub target_humidity_state_topic: Option<String>,

    /// Step size for temperature set point.
    #[serde(rename = "temp_step", skip_serializing_if = "Option::is_none")]
    pub temp_step: Option<Decimal>,

    /// A template to render the value sent to the `temperature_command_topic` with.
    #[serde(rename = "temp_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub temperature_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the target temperature.
    #[serde(rename = "temp_cmd_t", skip_serializing_if = "Option::is_none")]
    pub temperature_command_topic: Option<String>,

    /// A template to render the value sent to the `temperature_high_command_topic` with.
    #[serde(rename = "temp_hi_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub temperature_high_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the upper target temperature.
    #[serde(rename = "temp_hi_cmd_t", skip_serializing_if = "Option::is_none")]
    pub temperature_high_command_topic: Option<String>,

    /// A template to render the value received on the `temperature_high_state_topic` with. A `"None"` value received will reset the upper temperature setpoint. Empty values (`""'`) will be ignored.
    #[serde(rename = "temp_hi_stat_tpl", skip_serializing_if = "Option::is_none")]
    pub temperature_high_state_template: Option<String>,

    /// The MQTT topic to subscribe for changes in the upper target temperature. If this is not set, the upper target temperature works in optimistic mode (see below).
    #[serde(rename = "temp_hi_stat_t", skip_serializing_if = "Option::is_none")]
    pub temperature_high_state_topic: Option<String>,

    /// A template to render the value sent to the `temperature_low_command_topic` with.
    #[serde(rename = "temp_lo_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub temperature_low_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the lower target temperature.
    #[serde(rename = "temp_lo_cmd_t", skip_serializing_if = "Option::is_none")]
    pub temperature_low_command_topic: Option<String>,

    /// A template to render the value received on the `temperature_low_state_topic` with. A `"None"` value received will reset the lower temperature setpoint. Empty values (`""`) will be ignored.
    #[serde(rename = "temp_lo_stat_tpl", skip_serializing_if = "Option::is_none")]
    pub temperature_low_state_template: Option<String>,

    /// The MQTT topic to subscribe for changes in the lower target temperature. If this is not set, the lower target temperature works in optimistic mode (see below).
    #[serde(rename = "temp_lo_stat_t", skip_serializing_if = "Option::is_none")]
    pub temperature_low_state_topic: Option<String>,

    /// A template to render the value received on the `temperature_state_topic` with.
    #[serde(rename = "temp_stat_tpl", skip_serializing_if = "Option::is_none")]
    pub temperature_state_template: Option<String>,

    /// The MQTT topic to subscribe for changes in the target temperature. If this is not set, the target temperature works in optimistic mode (see below). A `"None"` value received will reset the temperature set point. Empty values (`'''`) will be ignored.
    #[serde(rename = "temp_stat_t", skip_serializing_if = "Option::is_none")]
    pub temperature_state_topic: Option<String>,

    /// Defines the temperature unit of the device, `C` or `F`. If this is not set, the temperature unit is set to the system temperature unit.
    #[serde(rename = "temp_unit", skip_serializing_if = "Option::is_none")]
    pub temperature_unit: Option<TemperatureUnit>,

    /// An ID that uniquely identifies this HVAC device. If two HVAC devices have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,

    /// Default template to render the payloads on *all* `*_state_topic`s with.
    #[serde(rename = "val_tpl", skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

impl Climate {
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
    pub fn device(mut self, device: DeviceInformation) -> Self {
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

    /// The MQTT topic to subscribe for changes of the current action. If this is set, the climate graph uses the value received as data source. A "None" payload resets the current action state. An empty payload is ignored. Valid action values: `off`, `heating`, `cooling`, `drying`, `idle`, `fan`.
    pub fn action_topic<T: Into<String>>(mut self, action_topic: T) -> Self {
        self.action_topic = Some(action_topic.into());
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

    /// A template to render the value sent to the `fan_mode_command_topic` with.
    pub fn fan_mode_command_template<T: Into<String>>(
        mut self,
        fan_mode_command_template: T,
    ) -> Self {
        self.fan_mode_command_template = Some(fan_mode_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the fan mode.
    pub fn fan_mode_command_topic<T: Into<String>>(mut self, fan_mode_command_topic: T) -> Self {
        self.fan_mode_command_topic = Some(fan_mode_command_topic.into());
        self
    }

    /// A template to render the value received on the `fan_mode_state_topic` with.
    pub fn fan_mode_state_template<T: Into<String>>(mut self, fan_mode_state_template: T) -> Self {
        self.fan_mode_state_template = Some(fan_mode_state_template.into());
        self
    }

    /// The MQTT topic to subscribe for changes of the HVAC fan mode. If this is not set, the fan mode works in optimistic mode (see below). A "None" payload resets the fan mode state. An empty payload is ignored.
    pub fn fan_mode_state_topic<T: Into<String>>(mut self, fan_mode_state_topic: T) -> Self {
        self.fan_mode_state_topic = Some(fan_mode_state_topic.into());
        self
    }

    /// A list of supported fan modes.
    pub fn fan_modes<T: Into<String>>(mut self, fan_modes: Vec<T>) -> Self {
        self.fan_modes = Some(fan_modes.into_iter().map(|v| v.into()).collect());
        self
    }

    /// [Icon](/docs/configuration/customizing-devices/#icon) for the entity.
    pub fn icon<T: Into<String>>(mut self, icon: T) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set the initial target temperature. The default value depends on the temperature unit and will be 21° or 69.8°F.
    pub fn initial(mut self, initial: Decimal) -> Self {
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

    /// The minimum target humidity percentage that can be set.
    pub fn max_humidity(mut self, max_humidity: Decimal) -> Self {
        self.max_humidity = Some(max_humidity);
        self
    }

    /// Maximum set point available. The default value depends on the temperature unit, and will be 35°C or 95°F.
    pub fn max_temp(mut self, max_temp: Decimal) -> Self {
        self.max_temp = Some(max_temp);
        self
    }

    /// The maximum target humidity percentage that can be set.
    pub fn min_humidity(mut self, min_humidity: Decimal) -> Self {
        self.min_humidity = Some(min_humidity);
        self
    }

    /// Minimum set point available. The default value depends on the temperature unit, and will be 7°C or 44.6°F.
    pub fn min_temp(mut self, min_temp: Decimal) -> Self {
        self.min_temp = Some(min_temp);
        self
    }

    /// A template to render the value sent to the `mode_command_topic` with.
    pub fn mode_command_template<T: Into<String>>(mut self, mode_command_template: T) -> Self {
        self.mode_command_template = Some(mode_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the HVAC operation mode.
    pub fn mode_command_topic<T: Into<String>>(mut self, mode_command_topic: T) -> Self {
        self.mode_command_topic = Some(mode_command_topic.into());
        self
    }

    /// A template to render the value received on the `mode_state_topic` with.
    pub fn mode_state_template<T: Into<String>>(mut self, mode_state_template: T) -> Self {
        self.mode_state_template = Some(mode_state_template.into());
        self
    }

    /// The MQTT topic to subscribe for changes of the HVAC operation mode. If this is not set, the operation mode works in optimistic mode (see below). A "None" payload resets to an `unknown` state. An empty payload is ignored.
    pub fn mode_state_topic<T: Into<String>>(mut self, mode_state_topic: T) -> Self {
        self.mode_state_topic = Some(mode_state_topic.into());
        self
    }

    /// A list of supported modes. Needs to be a subset of the default values.
    pub fn modes<T: Into<String>>(mut self, modes: Vec<T>) -> Self {
        self.modes = Some(modes.into_iter().map(|v| v.into()).collect());
        self
    }

    /// The name of the HVAC. Can be set to `null` if only the device name is relevant.
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    /// Flag that defines if the climate works in optimistic mode
    pub fn optimistic(mut self, optimistic: bool) -> Self {
        self.optimistic = Some(optimistic);
        self
    }

    /// The payload sent to turn off the device.
    pub fn payload_off<T: Into<String>>(mut self, payload_off: T) -> Self {
        self.payload_off = Some(payload_off.into());
        self
    }

    /// The payload sent to turn the device on.
    pub fn payload_on<T: Into<String>>(mut self, payload_on: T) -> Self {
        self.payload_on = Some(payload_on.into());
        self
    }

    /// A template to render the value sent to the `power_command_topic` with. The `value` parameter is the payload set for `payload_on` or `payload_off`.
    pub fn power_command_template<T: Into<String>>(mut self, power_command_template: T) -> Self {
        self.power_command_template = Some(power_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the HVAC power state. Sends the payload configured with `payload_on` if the climate is turned on via the `climate.turn_on`, or the payload configured with `payload_off` if the climate is turned off via the `climate.turn_off` action. Note that `optimistic` mode is not supported through `climate.turn_on` and `climate.turn_off` actions. When called, these actions will send a power command to the device but will not optimistically update the state of the climate entity. The climate device should report its state back via `mode_state_topic`.
    pub fn power_command_topic<T: Into<String>>(mut self, power_command_topic: T) -> Self {
        self.power_command_topic = Some(power_command_topic.into());
        self
    }

    /// The desired precision for this device. Can be used to match your actual thermostat's precision. Supported values are `0.1`, `0.5` and `1.0`.
    pub fn precision(mut self, precision: Decimal) -> Self {
        self.precision = Some(precision);
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `preset_mode_command_topic`.
    pub fn preset_mode_command_template<T: Into<String>>(
        mut self,
        preset_mode_command_template: T,
    ) -> Self {
        self.preset_mode_command_template = Some(preset_mode_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the preset mode.
    pub fn preset_mode_command_topic<T: Into<String>>(
        mut self,
        preset_mode_command_topic: T,
    ) -> Self {
        self.preset_mode_command_topic = Some(preset_mode_command_topic.into());
        self
    }

    /// The MQTT topic subscribed to receive climate speed based on presets. When preset 'none' is received or `None` the `preset_mode` will be reset.
    pub fn preset_mode_state_topic<T: Into<String>>(mut self, preset_mode_state_topic: T) -> Self {
        self.preset_mode_state_topic = Some(preset_mode_state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the `preset_mode` value from the payload received on `preset_mode_state_topic`.
    pub fn preset_mode_value_template<T: Into<String>>(
        mut self,
        preset_mode_value_template: T,
    ) -> Self {
        self.preset_mode_value_template = Some(preset_mode_value_template.into());
        self
    }

    /// List of preset modes this climate is supporting. Common examples include `eco`, `away`, `boost`, `comfort`, `home`, `sleep` and `activity`.
    pub fn preset_modes<T: Into<String>>(mut self, preset_modes: Vec<T>) -> Self {
        self.preset_modes = Some(preset_modes.into_iter().map(|v| v.into()).collect());
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

    /// A template to render the value sent to the `swing_horizontal_mode_command_topic` with.
    pub fn swing_horizontal_mode_command_template<T: Into<String>>(
        mut self,
        swing_horizontal_mode_command_template: T,
    ) -> Self {
        self.swing_horizontal_mode_command_template =
            Some(swing_horizontal_mode_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the swing horizontal mode.
    pub fn swing_horizontal_mode_command_topic<T: Into<String>>(
        mut self,
        swing_horizontal_mode_command_topic: T,
    ) -> Self {
        self.swing_horizontal_mode_command_topic = Some(swing_horizontal_mode_command_topic.into());
        self
    }

    /// A template to render the value received on the `swing_horizontal_mode_state_topic` with.
    pub fn swing_horizontal_mode_state_template<T: Into<String>>(
        mut self,
        swing_horizontal_mode_state_template: T,
    ) -> Self {
        self.swing_horizontal_mode_state_template =
            Some(swing_horizontal_mode_state_template.into());
        self
    }

    /// The MQTT topic to subscribe for changes of the HVAC swing horizontal mode. If this is not set, the swing horizontal mode works in optimistic mode (see below).
    pub fn swing_horizontal_mode_state_topic<T: Into<String>>(
        mut self,
        swing_horizontal_mode_state_topic: T,
    ) -> Self {
        self.swing_horizontal_mode_state_topic = Some(swing_horizontal_mode_state_topic.into());
        self
    }

    /// A list of supported swing horizontal modes.
    pub fn swing_horizontal_modes<T: Into<String>>(
        mut self,
        swing_horizontal_modes: Vec<T>,
    ) -> Self {
        self.swing_horizontal_modes = Some(
            swing_horizontal_modes
                .into_iter()
                .map(|v| v.into())
                .collect(),
        );
        self
    }

    /// A template to render the value sent to the `swing_mode_command_topic` with.
    pub fn swing_mode_command_template<T: Into<String>>(
        mut self,
        swing_mode_command_template: T,
    ) -> Self {
        self.swing_mode_command_template = Some(swing_mode_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the swing mode.
    pub fn swing_mode_command_topic<T: Into<String>>(
        mut self,
        swing_mode_command_topic: T,
    ) -> Self {
        self.swing_mode_command_topic = Some(swing_mode_command_topic.into());
        self
    }

    /// A template to render the value received on the `swing_mode_state_topic` with.
    pub fn swing_mode_state_template<T: Into<String>>(
        mut self,
        swing_mode_state_template: T,
    ) -> Self {
        self.swing_mode_state_template = Some(swing_mode_state_template.into());
        self
    }

    /// The MQTT topic to subscribe for changes of the HVAC swing mode. If this is not set, the swing mode works in optimistic mode (see below).
    pub fn swing_mode_state_topic<T: Into<String>>(mut self, swing_mode_state_topic: T) -> Self {
        self.swing_mode_state_topic = Some(swing_mode_state_topic.into());
        self
    }

    /// A list of supported swing modes.
    pub fn swing_modes<T: Into<String>>(mut self, swing_modes: Vec<T>) -> Self {
        self.swing_modes = Some(swing_modes.into_iter().map(|v| v.into()).collect());
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

    /// The MQTT topic to publish commands to change the target humidity.
    pub fn target_humidity_command_topic<T: Into<String>>(
        mut self,
        target_humidity_command_topic: T,
    ) -> Self {
        self.target_humidity_command_topic = Some(target_humidity_command_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract a value for the climate `target_humidity` state.
    pub fn target_humidity_state_template<T: Into<String>>(
        mut self,
        target_humidity_state_template: T,
    ) -> Self {
        self.target_humidity_state_template = Some(target_humidity_state_template.into());
        self
    }

    /// The MQTT topic subscribed to receive the target humidity. If this is not set, the target humidity works in optimistic mode (see below). A `"None"` value received will reset the target humidity. Empty values (`'''`) will be ignored.
    pub fn target_humidity_state_topic<T: Into<String>>(
        mut self,
        target_humidity_state_topic: T,
    ) -> Self {
        self.target_humidity_state_topic = Some(target_humidity_state_topic.into());
        self
    }

    /// Step size for temperature set point.
    pub fn temp_step(mut self, temp_step: Decimal) -> Self {
        self.temp_step = Some(temp_step);
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

    /// A template to render the value sent to the `temperature_high_command_topic` with.
    pub fn temperature_high_command_template<T: Into<String>>(
        mut self,
        temperature_high_command_template: T,
    ) -> Self {
        self.temperature_high_command_template = Some(temperature_high_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the upper target temperature.
    pub fn temperature_high_command_topic<T: Into<String>>(
        mut self,
        temperature_high_command_topic: T,
    ) -> Self {
        self.temperature_high_command_topic = Some(temperature_high_command_topic.into());
        self
    }

    /// A template to render the value received on the `temperature_high_state_topic` with. A `"None"` value received will reset the upper temperature setpoint. Empty values (`""'`) will be ignored.
    pub fn temperature_high_state_template<T: Into<String>>(
        mut self,
        temperature_high_state_template: T,
    ) -> Self {
        self.temperature_high_state_template = Some(temperature_high_state_template.into());
        self
    }

    /// The MQTT topic to subscribe for changes in the upper target temperature. If this is not set, the upper target temperature works in optimistic mode (see below).
    pub fn temperature_high_state_topic<T: Into<String>>(
        mut self,
        temperature_high_state_topic: T,
    ) -> Self {
        self.temperature_high_state_topic = Some(temperature_high_state_topic.into());
        self
    }

    /// A template to render the value sent to the `temperature_low_command_topic` with.
    pub fn temperature_low_command_template<T: Into<String>>(
        mut self,
        temperature_low_command_template: T,
    ) -> Self {
        self.temperature_low_command_template = Some(temperature_low_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the lower target temperature.
    pub fn temperature_low_command_topic<T: Into<String>>(
        mut self,
        temperature_low_command_topic: T,
    ) -> Self {
        self.temperature_low_command_topic = Some(temperature_low_command_topic.into());
        self
    }

    /// A template to render the value received on the `temperature_low_state_topic` with. A `"None"` value received will reset the lower temperature setpoint. Empty values (`""`) will be ignored.
    pub fn temperature_low_state_template<T: Into<String>>(
        mut self,
        temperature_low_state_template: T,
    ) -> Self {
        self.temperature_low_state_template = Some(temperature_low_state_template.into());
        self
    }

    /// The MQTT topic to subscribe for changes in the lower target temperature. If this is not set, the lower target temperature works in optimistic mode (see below).
    pub fn temperature_low_state_topic<T: Into<String>>(
        mut self,
        temperature_low_state_topic: T,
    ) -> Self {
        self.temperature_low_state_topic = Some(temperature_low_state_topic.into());
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

    /// An ID that uniquely identifies this HVAC device. If two HVAC devices have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
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

impl Default for Climate {
    fn default() -> Self {
        Self {
            topic_prefix: Default::default(),
            origin: Default::default(),
            device: Default::default(),
            entity_category: Default::default(),
            availability: Default::default(),
            action_template: Default::default(),
            action_topic: Default::default(),
            current_humidity_template: Default::default(),
            current_humidity_topic: Default::default(),
            current_temperature_template: Default::default(),
            current_temperature_topic: Default::default(),
            enabled_by_default: Default::default(),
            encoding: Default::default(),
            entity_picture: Default::default(),
            fan_mode_command_template: Default::default(),
            fan_mode_command_topic: Default::default(),
            fan_mode_state_template: Default::default(),
            fan_mode_state_topic: Default::default(),
            fan_modes: Default::default(),
            icon: Default::default(),
            initial: Default::default(),
            json_attributes_template: Default::default(),
            json_attributes_topic: Default::default(),
            max_humidity: Default::default(),
            max_temp: Default::default(),
            min_humidity: Default::default(),
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
            power_command_template: Default::default(),
            power_command_topic: Default::default(),
            precision: Default::default(),
            preset_mode_command_template: Default::default(),
            preset_mode_command_topic: Default::default(),
            preset_mode_state_topic: Default::default(),
            preset_mode_value_template: Default::default(),
            preset_modes: Default::default(),
            qos: Default::default(),
            retain: Default::default(),
            swing_horizontal_mode_command_template: Default::default(),
            swing_horizontal_mode_command_topic: Default::default(),
            swing_horizontal_mode_state_template: Default::default(),
            swing_horizontal_mode_state_topic: Default::default(),
            swing_horizontal_modes: Default::default(),
            swing_mode_command_template: Default::default(),
            swing_mode_command_topic: Default::default(),
            swing_mode_state_template: Default::default(),
            swing_mode_state_topic: Default::default(),
            swing_modes: Default::default(),
            target_humidity_command_template: Default::default(),
            target_humidity_command_topic: Default::default(),
            target_humidity_state_template: Default::default(),
            target_humidity_state_topic: Default::default(),
            temp_step: Default::default(),
            temperature_command_template: Default::default(),
            temperature_command_topic: Default::default(),
            temperature_high_command_template: Default::default(),
            temperature_high_command_topic: Default::default(),
            temperature_high_state_template: Default::default(),
            temperature_high_state_topic: Default::default(),
            temperature_low_command_template: Default::default(),
            temperature_low_command_topic: Default::default(),
            temperature_low_state_template: Default::default(),
            temperature_low_state_topic: Default::default(),
            temperature_state_template: Default::default(),
            temperature_state_topic: Default::default(),
            temperature_unit: Default::default(),
            unique_id: Default::default(),
            value_template: Default::default(),
        }
    }
}

impl From<Climate> for Entity {
    fn from(value: Climate) -> Self {
        Entity::Climate(value)
    }
}
/// ---
/// title: "MQTT Alarm control panel"
/// description: "Instructions on how to integrate MQTT capable alarm panels into Home Assistant."
/// ha_category:
///   - Alarm
/// ha_release: 0.7.4
/// ha_iot_class: Configurable
/// ha_domain: mqtt
/// related:
///   - docs: /docs/configuration/
///     title: Configuration file
/// ---
///
/// The `mqtt` alarm panel `integration` enables the possibility to control MQTT capable alarm panels. The Alarm icon will change state after receiving a new state from `state_topic`. If these messages are published with *RETAIN* flag, the MQTT alarm panel will receive an instant state update after subscription and will start with the correct state. Otherwise, the initial state will be `unknown`.
///
/// The `integration` will accept the following states from your Alarm Panel (in lower case):
///
/// - `disarmed`
/// - `armed_home`
/// - `armed_away`
/// - `armed_night`
/// - `armed_vacation`
/// - `armed_custom_bypass`
/// - `pending`
/// - `triggered`
/// - `arming`
/// - `disarming`
///
/// The `integration` can control your Alarm Panel by publishing to the `command_topic` when a user interacts with the Home Assistant frontend.
///
/// ## Configuration
///
/// To use an MQTT alarm control panel in your installation, add the following to your `configuration.yaml` file.
/// {% include integrations/restart_ha_after_config_inclusion.md %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - alarm_control_panel:
///       state_topic: "home/alarm"
///       command_topic: "home/alarm/set"
/// ```
///
/// Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
///
///
/// ## Examples
///
/// In this section you find some real-life examples of how to use this alarm control panel.
///
/// ### Configuration with partial feature support
///
/// The example below shows a full configuration with an alarm panel that only supports the `arm_home` and `arm_away` features.
///
///
/// ```yaml
/// # Example with partial feature support
/// mqtt:
///   - alarm_control_panel:
///       name: "Alarm Panel"
///       supported_features:
///         - arm_home
///         - arm_away
///       state_topic: "alarmdecoder/panel"
///       command_topic: "alarmdecoder/panel/set"
/// ```
///
///
/// ### Configuration with local code validation
///
/// The example below shows a full configuration with local code validation.
///
///
/// ```yaml
/// # Example using text based code with local validation configuration.yaml
/// mqtt:
///   - alarm_control_panel:
///       name: "Alarm Panel With Numeric Keypad"
///       state_topic: "alarmdecoder/panel"
///       value_template: "{{value_json.state}}"
///       command_topic: "alarmdecoder/panel/set"
///       code: mys3cretc0de
/// ```
///
///
/// ### Configurations with remote code validation
///
/// The example below shows a full configuration with remote code validation and `command_template`.
///
///
/// ```yaml
/// # Example using text code with remote validation configuration.yaml
/// mqtt:
///   - alarm_control_panel:
///       name: "Alarm Panel With Text Code Dialog"
///       state_topic: "alarmdecoder/panel"
///       value_template: "{{ value_json.state }}"
///       command_topic: "alarmdecoder/panel/set"
///       code: REMOTE_CODE_TEXT
///       command_template: >
///         { "action": "{{ action }}", "code": "{{ code }}" }
/// ```
///
/// ```yaml
/// # Example using numeric code with remote validation configuration.yaml
/// mqtt:
///   - alarm_control_panel:
///       name: "Alarm Panel With Numeric Keypad"
///       state_topic: "alarmdecoder/panel"
///       value_template: "{{ value_json.state }}"
///       command_topic: "alarmdecoder/panel/set"
///       code: REMOTE_CODE
///       command_template: >
///         { "action": "{{ action }}", "code": "{{ code }}" }
/// ```
///
///
/// 🚨 Caution\
/// When your MQTT connection is not secured, this will send your secret code over the network unprotected!
///  
///
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AlarmControlPanel {
    /// Replaces `~` with this value in any MQTT topic attribute.
    /// [See Home Assistant documentation](https://www.home-assistant.io/integrations/mqtt/#using-abbreviations-and-base-topic)
    #[serde(rename = "~", skip_serializing_if = "Option::is_none")]
    pub topic_prefix: Option<String>,

    /// It is encouraged to add additional information about the origin that supplies MQTT entities via MQTT discovery by adding the origin option (can be abbreviated to o) to the discovery payload. Note that these options also support abbreviations. Information of the origin will be logged to the core event log when an item is discovered or updated.
    #[serde(rename = "o")]
    pub origin: Origin,

    /// Information about the device this button is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/en/device_registry_index.html). Only works when [`unique_id`](#unique_id) is set. At least one of identifiers or connections must be present to identify the device.
    #[serde(rename = "dev")]
    pub device: DeviceInformation,

    /// A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`.
    #[serde(flatten)]
    pub availability: Availability,

    /// The category of the entity. (optional, default: None)
    #[serde(rename = "ent_cat", skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<EntityCategory>,

    /// If defined, specifies a code to enable or disable the alarm in the frontend. Note that the code is validated locally and blocks sending MQTT messages to the remote device. For remote code validation, the code can be configured to either of the special values `REMOTE_CODE` (numeric code) or `REMOTE_CODE_TEXT` (text code). In this case, local code validation is bypassed but the frontend will still show a numeric or text code dialog. Use `command_template` to send the code to the remote device. Example configurations for remote code validation [can be found here](#configurations-with-remote-code-validation).
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// If true the code is required to arm the alarm. If false the code is not validated.
    #[serde(rename = "cod_arm_req", skip_serializing_if = "Option::is_none")]
    pub code_arm_required: Option<bool>,

    /// If true the code is required to disarm the alarm. If false the code is not validated.
    #[serde(rename = "cod_dis_req", skip_serializing_if = "Option::is_none")]
    pub code_disarm_required: Option<bool>,

    /// If true the code is required to trigger the alarm. If false the code is not validated.
    #[serde(rename = "cod_trig_req", skip_serializing_if = "Option::is_none")]
    pub code_trigger_required: Option<bool>,

    /// The [template](/docs/configuration/templating/#using-command-templates-with-mqtt) used for the command payload. Available variables: `action` and `code`.
    #[serde(rename = "cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub command_template: Option<String>,

    /// The MQTT topic to publish commands to change the alarm state.
    #[serde(rename = "cmd_t")]
    pub command_topic: String,

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

    /// The name of the alarm. Can be set to `null` if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// The payload to set armed-away mode on your Alarm Panel.
    #[serde(rename = "pl_arm_away", skip_serializing_if = "Option::is_none")]
    pub payload_arm_away: Option<String>,

    /// The payload to set armed-custom-bypass mode on your Alarm Panel.
    #[serde(rename = "pl_arm_custom_b", skip_serializing_if = "Option::is_none")]
    pub payload_arm_custom_bypass: Option<String>,

    /// The payload to set armed-home mode on your Alarm Panel.
    #[serde(rename = "pl_arm_home", skip_serializing_if = "Option::is_none")]
    pub payload_arm_home: Option<String>,

    /// The payload to set armed-night mode on your Alarm Panel.
    #[serde(rename = "pl_arm_nite", skip_serializing_if = "Option::is_none")]
    pub payload_arm_night: Option<String>,

    /// The payload to set armed-vacation mode on your Alarm Panel.
    #[serde(rename = "pl_arm_vacation", skip_serializing_if = "Option::is_none")]
    pub payload_arm_vacation: Option<String>,

    /// The payload to disarm your Alarm Panel.
    #[serde(rename = "pl_disarm", skip_serializing_if = "Option::is_none")]
    pub payload_disarm: Option<String>,

    /// The payload to trigger the alarm on your Alarm Panel.
    #[serde(rename = "pl_trig", skip_serializing_if = "Option::is_none")]
    pub payload_trigger: Option<String>,

    /// Must be `alarm_control_panel`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    #[serde(rename = "p")]
    pub platform: String,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// If the published message should have the retain flag on or not.
    #[serde(rename = "ret", skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,

    /// The MQTT topic subscribed to receive state updates. A "None" payload resets to an `unknown` state. An empty payload is ignored. Valid state payloads are: `armed_away`, `armed_custom_bypass`, `armed_home`, `armed_night`, `armed_vacation`, `arming`, `disarmed`, `disarming` `pending` and `triggered`.
    #[serde(rename = "stat_t")]
    pub state_topic: String,

    /// A list of features that the alarm control panel supports. The available list options are `arm_home`, `arm_away`, `arm_night`, `arm_vacation`, `arm_custom_bypass`, and `trigger`.
    #[serde(rename = "sup_feat", skip_serializing_if = "Option::is_none")]
    pub supported_features: Option<Vec<String>>,

    /// An ID that uniquely identifies this alarm panel. If two alarm panels have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the value.
    #[serde(rename = "val_tpl", skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

impl AlarmControlPanel {
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
    pub fn device(mut self, device: DeviceInformation) -> Self {
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

    /// If defined, specifies a code to enable or disable the alarm in the frontend. Note that the code is validated locally and blocks sending MQTT messages to the remote device. For remote code validation, the code can be configured to either of the special values `REMOTE_CODE` (numeric code) or `REMOTE_CODE_TEXT` (text code). In this case, local code validation is bypassed but the frontend will still show a numeric or text code dialog. Use `command_template` to send the code to the remote device. Example configurations for remote code validation [can be found here](#configurations-with-remote-code-validation).
    pub fn code<T: Into<String>>(mut self, code: T) -> Self {
        self.code = Some(code.into());
        self
    }

    /// If true the code is required to arm the alarm. If false the code is not validated.
    pub fn code_arm_required(mut self, code_arm_required: bool) -> Self {
        self.code_arm_required = Some(code_arm_required);
        self
    }

    /// If true the code is required to disarm the alarm. If false the code is not validated.
    pub fn code_disarm_required(mut self, code_disarm_required: bool) -> Self {
        self.code_disarm_required = Some(code_disarm_required);
        self
    }

    /// If true the code is required to trigger the alarm. If false the code is not validated.
    pub fn code_trigger_required(mut self, code_trigger_required: bool) -> Self {
        self.code_trigger_required = Some(code_trigger_required);
        self
    }

    /// The [template](/docs/configuration/templating/#using-command-templates-with-mqtt) used for the command payload. Available variables: `action` and `code`.
    pub fn command_template<T: Into<String>>(mut self, command_template: T) -> Self {
        self.command_template = Some(command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the alarm state.
    pub fn command_topic<T: Into<String>>(mut self, command_topic: T) -> Self {
        self.command_topic = command_topic.into();
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

    /// The name of the alarm. Can be set to `null` if only the device name is relevant.
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    /// The payload to set armed-away mode on your Alarm Panel.
    pub fn payload_arm_away<T: Into<String>>(mut self, payload_arm_away: T) -> Self {
        self.payload_arm_away = Some(payload_arm_away.into());
        self
    }

    /// The payload to set armed-custom-bypass mode on your Alarm Panel.
    pub fn payload_arm_custom_bypass<T: Into<String>>(
        mut self,
        payload_arm_custom_bypass: T,
    ) -> Self {
        self.payload_arm_custom_bypass = Some(payload_arm_custom_bypass.into());
        self
    }

    /// The payload to set armed-home mode on your Alarm Panel.
    pub fn payload_arm_home<T: Into<String>>(mut self, payload_arm_home: T) -> Self {
        self.payload_arm_home = Some(payload_arm_home.into());
        self
    }

    /// The payload to set armed-night mode on your Alarm Panel.
    pub fn payload_arm_night<T: Into<String>>(mut self, payload_arm_night: T) -> Self {
        self.payload_arm_night = Some(payload_arm_night.into());
        self
    }

    /// The payload to set armed-vacation mode on your Alarm Panel.
    pub fn payload_arm_vacation<T: Into<String>>(mut self, payload_arm_vacation: T) -> Self {
        self.payload_arm_vacation = Some(payload_arm_vacation.into());
        self
    }

    /// The payload to disarm your Alarm Panel.
    pub fn payload_disarm<T: Into<String>>(mut self, payload_disarm: T) -> Self {
        self.payload_disarm = Some(payload_disarm.into());
        self
    }

    /// The payload to trigger the alarm on your Alarm Panel.
    pub fn payload_trigger<T: Into<String>>(mut self, payload_trigger: T) -> Self {
        self.payload_trigger = Some(payload_trigger.into());
        self
    }

    /// Must be `alarm_control_panel`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
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

    /// The MQTT topic subscribed to receive state updates. A "None" payload resets to an `unknown` state. An empty payload is ignored. Valid state payloads are: `armed_away`, `armed_custom_bypass`, `armed_home`, `armed_night`, `armed_vacation`, `arming`, `disarmed`, `disarming` `pending` and `triggered`.
    pub fn state_topic<T: Into<String>>(mut self, state_topic: T) -> Self {
        self.state_topic = state_topic.into();
        self
    }

    /// A list of features that the alarm control panel supports. The available list options are `arm_home`, `arm_away`, `arm_night`, `arm_vacation`, `arm_custom_bypass`, and `trigger`.
    pub fn supported_features<T: Into<String>>(mut self, supported_features: Vec<T>) -> Self {
        self.supported_features = Some(supported_features.into_iter().map(|v| v.into()).collect());
        self
    }

    /// An ID that uniquely identifies this alarm panel. If two alarm panels have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    pub fn unique_id<T: Into<String>>(mut self, unique_id: T) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the value.
    pub fn value_template<T: Into<String>>(mut self, value_template: T) -> Self {
        self.value_template = Some(value_template.into());
        self
    }
}

impl Default for AlarmControlPanel {
    fn default() -> Self {
        Self {
            topic_prefix: Default::default(),
            origin: Default::default(),
            device: Default::default(),
            entity_category: Default::default(),
            availability: Default::default(),
            code: Default::default(),
            code_arm_required: Default::default(),
            code_disarm_required: Default::default(),
            code_trigger_required: Default::default(),
            command_template: Default::default(),
            command_topic: Default::default(),
            enabled_by_default: Default::default(),
            encoding: Default::default(),
            entity_picture: Default::default(),
            icon: Default::default(),
            json_attributes_template: Default::default(),
            json_attributes_topic: Default::default(),
            name: Default::default(),
            object_id: Default::default(),
            payload_arm_away: Default::default(),
            payload_arm_custom_bypass: Default::default(),
            payload_arm_home: Default::default(),
            payload_arm_night: Default::default(),
            payload_arm_vacation: Default::default(),
            payload_disarm: Default::default(),
            payload_trigger: Default::default(),
            platform: "alarm_control_panel".to_string(),
            qos: Default::default(),
            retain: Default::default(),
            state_topic: Default::default(),
            supported_features: Default::default(),
            unique_id: Default::default(),
            value_template: Default::default(),
        }
    }
}

impl From<AlarmControlPanel> for Entity {
    fn from(value: AlarmControlPanel) -> Self {
        Entity::AlarmControlPanel(value)
    }
}
/// ---
/// title: "MQTT device tracker"
/// description: "Instructions on how to use MQTT to track devices in Home Assistant."
/// ha_category:
///   - Presence detection
/// ha_iot_class: Configurable
/// ha_release: 0.7.3
/// ha_domain: mqtt
/// related:
///   - docs: /docs/configuration/
///     title: Configuration file
/// ---
///
///
/// The `mqtt` device tracker `integration` allows you to define new device_trackers through [manual YAML configuration](#yaml-configuration) in `configuration.yaml` and also to automatically discover device_trackers [using the MQTT Discovery protocol](#using-the-discovery-protocol).
///
/// ## Configuration
///
/// To use an MQTT device tracker in your installation, add the following to your `configuration.yaml` file.
/// {% include integrations/restart_ha_after_config_inclusion.md %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - device_tracker:
///       name: "annetherese_n4"
///       state_topic: "location/annetherese"
///   - device_tracker:
///       name: "paulus_oneplus"
///       state_topic: "location/paulus"
/// ```
///
/// Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
///
///
/// ## Examples
///
/// ### Using the discovery protocol
///
/// The device_tracker can be created via publishing to a discovery topic that follows the following [MQTT Discovery](/integrations/mqtt/#mqtt-discovery#discovery-topic) topic name format: `<discovery_prefix>/device_tracker/[<node_id>/]<object_id>/config`.
///
/// You can use the command line tool `mosquitto_pub` shipped with `mosquitto` or the `mosquitto-clients` package to send MQTT messages.
///
/// To create the device_tracker:
///
/// ```bash
/// mosquitto_pub -h 127.0.0.1 -t homeassistant/device_tracker/a4567d663eaf/config -m '{"state_topic": "homeassistant/device_tracker/a4567d663eaf/state", "name": "My Tracker", "payload_home": "home", "payload_not_home": "not_home"}'
/// ```
///
/// To set the state of the device tracker to "home":
///
/// ```bash
/// mosquitto_pub -h 127.0.0.1 -t homeassistant/device_tracker/a4567d663eaf/state -m 'home'
/// ```
///
/// To set the state of the device tracker to a named location:
///
/// ```bash
/// mosquitto_pub -h 127.0.0.1 -t homeassistant/device_tracker/a4567d663eaf/state -m 'location_name'
/// ```
///
/// If the device supports GPS coordinates then they can be sent to Home Assistant by specifying an attributes topic (i.e. "json_attributes_topic") in the configuration payload:
///
/// - Attributes topic: `homeassistant/device_tracker/a4567d663eaf/attributes`
/// - Example attributes payload:
///
/// Example message to be received at topic `homeassistant/device_tracker/a4567d663eaf/attributes`:
///
/// ```json
/// {
///   "latitude": 32.87336,
///   "longitude": -117.22743,
///   "gps_accuracy": 1.2
///  }
/// ```
///
/// To create the device_tracker with GPS coordinates support:
///
/// ```bash
/// mosquitto_pub -h 127.0.0.1 -t homeassistant/device_tracker/a4567d663eaf/config -m '{"json_attributes_topic": "homeassistant/device_tracker/a4567d663eaf/attributes", "name": "My Tracker"}'
/// ```
///
/// 🛈 Note\
///
/// Using `state_topic` is optional when using `json_attributes_topic` to determine the state of the device tracker.
///
///
/// To set the state of the device tracker to specific coordinates:
///
/// ```bash
/// mosquitto_pub -h 127.0.0.1 -t homeassistant/device_tracker/a4567d663eaf/attributes -m '{"latitude": 32.87336, "longitude": -117.22743, "gps_accuracy": 1.2}'
/// ```
///
///
/// ### YAML configuration
///
/// The following example shows how to configure the same device tracker through configuration.yaml
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - device_tracker:
///       name: "My Tracker"
///       state_topic: "a4567d663eaf/state"
///       payload_home: "home"
///       payload_not_home: "not_home"
/// ```
///
///
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeviceTracker {
    /// Replaces `~` with this value in any MQTT topic attribute.
    /// [See Home Assistant documentation](https://www.home-assistant.io/integrations/mqtt/#using-abbreviations-and-base-topic)
    #[serde(rename = "~", skip_serializing_if = "Option::is_none")]
    pub topic_prefix: Option<String>,

    /// It is encouraged to add additional information about the origin that supplies MQTT entities via MQTT discovery by adding the origin option (can be abbreviated to o) to the discovery payload. Note that these options also support abbreviations. Information of the origin will be logged to the core event log when an item is discovered or updated.
    #[serde(rename = "o")]
    pub origin: Origin,

    /// Information about the device this button is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/en/device_registry_index.html). Only works when [`unique_id`](#unique_id) is set. At least one of identifiers or connections must be present to identify the device.
    #[serde(rename = "dev")]
    pub device: DeviceInformation,

    /// A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`.
    #[serde(flatten)]
    pub availability: Availability,

    /// The category of the entity. (optional, default: None)
    #[serde(rename = "ent_cat", skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<EntityCategory>,

    /// [Icon](/docs/configuration/customizing-devices/#icon) for the entity.
    #[serde(rename = "ic", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation.
    #[serde(rename = "json_attr_tpl", skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,

    /// The MQTT topic subscribed to receive a JSON dictionary message containing device tracker attributes. This topic can be used to set the location of the device tracker under the following conditions:
    /// - If the attributes in the JSON message include `longitude`, `latitude`, and `gps_accuracy` (optional).
    ///  - If the device tracker is within a configured [zone](/integrations/zone/).
    ///
    /// If these conditions are met, it is not required to configure `state_topic`.
    ///
    ///  Be aware that any location message received at `state_topic`  overrides the location received via `json_attributes_topic` until a message configured with `payload_reset` is received at `state_topic`. For a more generic usage example of the `json_attributes_topic`, refer to the [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation.
    #[serde(rename = "json_attr_t", skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,

    /// The name of the MQTT device_tracker.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// The payload value that represents the 'home' state for the device.
    #[serde(rename = "pl_home", skip_serializing_if = "Option::is_none")]
    pub payload_home: Option<String>,

    /// The payload value that represents the 'not_home' state for the device.
    #[serde(rename = "pl_not_home", skip_serializing_if = "Option::is_none")]
    pub payload_not_home: Option<String>,

    /// The payload value that will have the device's location automatically derived from Home Assistant's zones.
    #[serde(rename = "pl_rst", skip_serializing_if = "Option::is_none")]
    pub payload_reset: Option<String>,

    /// Must be `device_tracker`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    #[serde(rename = "p")]
    pub platform: String,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// Attribute of a device tracker that affects state when being used to track a [person](/integrations/person/). Valid options are `gps`, `router`, `bluetooth`, or `bluetooth_le`.
    #[serde(rename = "src_type", skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,

    /// The MQTT topic subscribed to receive device tracker state changes. The states defined in `state_topic` override the location states defined by the `json_attributes_topic`. This state override is turned inactive if the `state_topic` receives a message containing `payload_reset`. The `state_topic` can only be omitted if `json_attributes_topic` is used. An empty payload is ignored. Valid payloads are `not_home`, `home` or any other custom location or zone name. Payloads for `not_home`, `home` can be overridden with the `payload_not_home`and `payload_home` config options.
    #[serde(rename = "stat_t", skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,

    /// An ID that uniquely identifies this device_tracker. If two device_trackers have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) that returns a device tracker state.
    #[serde(rename = "val_tpl", skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

impl DeviceTracker {
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
    pub fn device(mut self, device: DeviceInformation) -> Self {
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

    /// The MQTT topic subscribed to receive a JSON dictionary message containing device tracker attributes. This topic can be used to set the location of the device tracker under the following conditions:
    /// - If the attributes in the JSON message include `longitude`, `latitude`, and `gps_accuracy` (optional).
    ///  - If the device tracker is within a configured [zone](/integrations/zone/).
    ///
    /// If these conditions are met, it is not required to configure `state_topic`.
    ///
    ///  Be aware that any location message received at `state_topic`  overrides the location received via `json_attributes_topic` until a message configured with `payload_reset` is received at `state_topic`. For a more generic usage example of the `json_attributes_topic`, refer to the [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation.
    pub fn json_attributes_topic<T: Into<String>>(mut self, json_attributes_topic: T) -> Self {
        self.json_attributes_topic = Some(json_attributes_topic.into());
        self
    }

    /// The name of the MQTT device_tracker.
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    /// The payload value that represents the 'home' state for the device.
    pub fn payload_home<T: Into<String>>(mut self, payload_home: T) -> Self {
        self.payload_home = Some(payload_home.into());
        self
    }

    /// The payload value that represents the 'not_home' state for the device.
    pub fn payload_not_home<T: Into<String>>(mut self, payload_not_home: T) -> Self {
        self.payload_not_home = Some(payload_not_home.into());
        self
    }

    /// The payload value that will have the device's location automatically derived from Home Assistant's zones.
    pub fn payload_reset<T: Into<String>>(mut self, payload_reset: T) -> Self {
        self.payload_reset = Some(payload_reset.into());
        self
    }

    /// Must be `device_tracker`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    pub fn platform<T: Into<String>>(mut self, platform: T) -> Self {
        self.platform = platform.into();
        self
    }

    /// The maximum QoS level to be used when receiving and publishing messages.
    pub fn qos(mut self, qos: Qos) -> Self {
        self.qos = Some(qos);
        self
    }

    /// Attribute of a device tracker that affects state when being used to track a [person](/integrations/person/). Valid options are `gps`, `router`, `bluetooth`, or `bluetooth_le`.
    pub fn source_type<T: Into<String>>(mut self, source_type: T) -> Self {
        self.source_type = Some(source_type.into());
        self
    }

    /// The MQTT topic subscribed to receive device tracker state changes. The states defined in `state_topic` override the location states defined by the `json_attributes_topic`. This state override is turned inactive if the `state_topic` receives a message containing `payload_reset`. The `state_topic` can only be omitted if `json_attributes_topic` is used. An empty payload is ignored. Valid payloads are `not_home`, `home` or any other custom location or zone name. Payloads for `not_home`, `home` can be overridden with the `payload_not_home`and `payload_home` config options.
    pub fn state_topic<T: Into<String>>(mut self, state_topic: T) -> Self {
        self.state_topic = Some(state_topic.into());
        self
    }

    /// An ID that uniquely identifies this device_tracker. If two device_trackers have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    pub fn unique_id<T: Into<String>>(mut self, unique_id: T) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) that returns a device tracker state.
    pub fn value_template<T: Into<String>>(mut self, value_template: T) -> Self {
        self.value_template = Some(value_template.into());
        self
    }
}

impl Default for DeviceTracker {
    fn default() -> Self {
        Self {
            topic_prefix: Default::default(),
            origin: Default::default(),
            device: Default::default(),
            entity_category: Default::default(),
            availability: Default::default(),
            icon: Default::default(),
            json_attributes_template: Default::default(),
            json_attributes_topic: Default::default(),
            name: Default::default(),
            object_id: Default::default(),
            payload_home: Default::default(),
            payload_not_home: Default::default(),
            payload_reset: Default::default(),
            platform: "device_tracker".to_string(),
            qos: Default::default(),
            source_type: Default::default(),
            state_topic: Default::default(),
            unique_id: Default::default(),
            value_template: Default::default(),
        }
    }
}

impl From<DeviceTracker> for Entity {
    fn from(value: DeviceTracker) -> Self {
        Entity::DeviceTracker(value)
    }
}
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
    pub device: DeviceInformation,

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
    pub fn device(mut self, device: DeviceInformation) -> Self {
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
/// ---
/// title: "MQTT Image"
/// description: "Instructions on how to use an MQTT image message as an Image within Home Assistant."
/// ha_category:
///   - Image
/// ha_release: 2023.7
/// ha_iot_class: Configurable
/// ha_domain: mqtt
/// ---
///
/// The `mqtt` image platform allows you to integrate the content of an image file sent through MQTT into Home Assistant as an image.
/// The `image` platform is a simplified version of the `camera` platform that only accepts images.
/// Every time a message under the `image_topic` in the configuration is received, the image displayed in Home Assistant will also be updated. Messages received on `image_topic` should contain the full contents of an image file, for example, a JPEG image, without any additional encoding or metadata.
///
/// This can be used with an application or a service capable of sending images through MQTT.
///
/// An alternative setup is to use the `url_topic` option to receive an image URL for a new picture to show.
///
/// ## Configuration
///
/// To use an MQTT image entity in your installation, add the following to your `configuration.yaml` file.
/// {% include integrations/restart_ha_after_config_inclusion.md %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - image:
///       url_topic: mynas/status/url
/// ```
///
/// Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
///
///
/// ### Example receiving images from a URL
///
/// Add the configuration below to your `configuration.yaml`.
///
/// To test it publish an image URL to the topic from the console:
///
/// ```shell
/// mosquitto_pub -h <mqtt_broker> -t mynas/status/url -m "https://design.home-assistant.io/images/logo.png"
/// ```
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - image:
///       url_topic: mynas/status/url
/// ```
///
///
/// ### Example receiving images from a file
///
/// Add the configuration below to your `configuration.yaml`.
///
/// To test it, publish an image URL to the topic from the console:
///
/// ```shell
/// mosquitto_pub -h <mqtt_broker> -t mynas/status/file -f <logo.png>
/// ```
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - image:
///       image_topic: mynas/status/file
///       content_type: image/png
/// ```
///
///
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Image {
    /// Replaces `~` with this value in any MQTT topic attribute.
    /// [See Home Assistant documentation](https://www.home-assistant.io/integrations/mqtt/#using-abbreviations-and-base-topic)
    #[serde(rename = "~", skip_serializing_if = "Option::is_none")]
    pub topic_prefix: Option<String>,

    /// It is encouraged to add additional information about the origin that supplies MQTT entities via MQTT discovery by adding the origin option (can be abbreviated to o) to the discovery payload. Note that these options also support abbreviations. Information of the origin will be logged to the core event log when an item is discovered or updated.
    #[serde(rename = "o")]
    pub origin: Origin,

    /// Information about the device this button is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/en/device_registry_index.html). Only works when [`unique_id`](#unique_id) is set. At least one of identifiers or connections must be present to identify the device.
    #[serde(rename = "dev")]
    pub device: DeviceInformation,

    /// A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`.
    #[serde(flatten)]
    pub availability: Availability,

    /// The category of the entity. (optional, default: None)
    #[serde(rename = "ent_cat", skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<EntityCategory>,

    /// The content type of and image data message received on `image_topic`. This option cannot be used with the `url_topic` because the content type is derived when downloading the image.
    #[serde(rename = "cont_type", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,

    /// Flag which defines if the entity should be enabled when first added.
    #[serde(rename = "en", skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,

    /// The encoding of the payloads received. Set to `""` to disable decoding of incoming payload. Use `image_encoding` to enable `Base64` decoding on `image_topic`.
    #[serde(rename = "e", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,

    /// Picture URL for the entity.
    #[serde(rename = "ent_pic", skip_serializing_if = "Option::is_none")]
    pub entity_picture: Option<String>,

    /// [Icon](/docs/configuration/customizing-devices/#icon) for the entity.
    #[serde(rename = "ic", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// The encoding of the image payloads received. Set to `"b64"` to enable base64 decoding of image payload. If not set, the image payload must be raw binary data.
    #[serde(rename = "img_e", skip_serializing_if = "Option::is_none")]
    pub image_encoding: Option<String>,

    /// The MQTT topic to subscribe to receive the image payload of the image to be downloaded. Ensure the `content_type` type option is set to the corresponding content type. This option cannot be used together with the `url_topic` option. But at least one of these option is required.
    #[serde(rename = "img_t")]
    pub image_topic: String,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the JSON dictionary from messages received on the `json_attributes_topic`.
    #[serde(rename = "json_attr_tpl", skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Implies `force_update` of the current sensor state when a message is received on this topic.
    #[serde(rename = "json_attr_t", skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,

    /// The name of the image. Can be set to `null` if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// An ID that uniquely identifies this image. If two images have the same unique ID Home Assistant will raise an exception. Required when used with device-based discovery.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the image URL from a message received at `url_topic`.
    #[serde(rename = "url_tpl", skip_serializing_if = "Option::is_none")]
    pub url_template: Option<String>,

    /// The MQTT topic to subscribe to receive an image URL. A `url_template` option can extract the URL from the message. The `content_type` will be derived from the image when downloaded. This option cannot be used together with the `image_topic` option, but at least one of these options is required.
    #[serde(rename = "url_t")]
    pub url_topic: String,
}

impl Image {
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
    pub fn device(mut self, device: DeviceInformation) -> Self {
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

    /// The content type of and image data message received on `image_topic`. This option cannot be used with the `url_topic` because the content type is derived when downloading the image.
    pub fn content_type<T: Into<String>>(mut self, content_type: T) -> Self {
        self.content_type = Some(content_type.into());
        self
    }

    /// Flag which defines if the entity should be enabled when first added.
    pub fn enabled_by_default(mut self, enabled_by_default: bool) -> Self {
        self.enabled_by_default = Some(enabled_by_default);
        self
    }

    /// The encoding of the payloads received. Set to `""` to disable decoding of incoming payload. Use `image_encoding` to enable `Base64` decoding on `image_topic`.
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

    /// The encoding of the image payloads received. Set to `"b64"` to enable base64 decoding of image payload. If not set, the image payload must be raw binary data.
    pub fn image_encoding<T: Into<String>>(mut self, image_encoding: T) -> Self {
        self.image_encoding = Some(image_encoding.into());
        self
    }

    /// The MQTT topic to subscribe to receive the image payload of the image to be downloaded. Ensure the `content_type` type option is set to the corresponding content type. This option cannot be used together with the `url_topic` option. But at least one of these option is required.
    pub fn image_topic<T: Into<String>>(mut self, image_topic: T) -> Self {
        self.image_topic = image_topic.into();
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

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Implies `force_update` of the current sensor state when a message is received on this topic.
    pub fn json_attributes_topic<T: Into<String>>(mut self, json_attributes_topic: T) -> Self {
        self.json_attributes_topic = Some(json_attributes_topic.into());
        self
    }

    /// The name of the image. Can be set to `null` if only the device name is relevant.
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    /// An ID that uniquely identifies this image. If two images have the same unique ID Home Assistant will raise an exception. Required when used with device-based discovery.
    pub fn unique_id<T: Into<String>>(mut self, unique_id: T) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the image URL from a message received at `url_topic`.
    pub fn url_template<T: Into<String>>(mut self, url_template: T) -> Self {
        self.url_template = Some(url_template.into());
        self
    }

    /// The MQTT topic to subscribe to receive an image URL. A `url_template` option can extract the URL from the message. The `content_type` will be derived from the image when downloaded. This option cannot be used together with the `image_topic` option, but at least one of these options is required.
    pub fn url_topic<T: Into<String>>(mut self, url_topic: T) -> Self {
        self.url_topic = url_topic.into();
        self
    }
}

impl Default for Image {
    fn default() -> Self {
        Self {
            topic_prefix: Default::default(),
            origin: Default::default(),
            device: Default::default(),
            entity_category: Default::default(),
            availability: Default::default(),
            content_type: Default::default(),
            enabled_by_default: Default::default(),
            encoding: Default::default(),
            entity_picture: Default::default(),
            icon: Default::default(),
            image_encoding: Default::default(),
            image_topic: Default::default(),
            json_attributes_template: Default::default(),
            json_attributes_topic: Default::default(),
            name: Default::default(),
            object_id: Default::default(),
            unique_id: Default::default(),
            url_template: Default::default(),
            url_topic: Default::default(),
        }
    }
}

impl From<Image> for Entity {
    fn from(value: Image) -> Self {
        Entity::Image(value)
    }
}
/// ---
/// title: "MQTT Device trigger"
/// description: "Instructions on how to integrate MQTT device triggers within Home Assistant."
/// ha_category:
///   - Device automation
/// ha_release: 0.106
/// ha_iot_class: Configurable
/// ha_domain: mqtt
/// ---
///
/// The `mqtt` device trigger platform uses an MQTT message payload to generate device trigger events.
///
/// An MQTT device trigger is a better option than a [binary sensor](/integrations/binary_sensor.mqtt/) for buttons, remote controls etc.
///
/// ## Configuration
///
/// MQTT device triggers are only supported through [MQTT discovery](/integrations/mqtt/#mqtt-discovery), manual setup through `configuration.yaml` is not supported.
/// The discovery topic needs to be: `<discovery_prefix>/device_automation/[<node_id>/]<object_id>/config`. Note that only one trigger may be defined per unique discovery topic. Also note that the combination of `type` and `subtype` should be unique for a device.
///
///
/// ### Example
///
/// This shows a complete example of defining a remote control type device with two triggers: "left arrow click" and "right arrow click".
///
/// Note that it is not necessary to provide the full device information in each message, but the identifying information, `identifier` in the example, must be the same.
///
/// #### Left arrow click configuration
///
/// - Discovery topic: `homeassistant/device_automation/0x90fd9ffffedf1266/action_arrow_left_click/config`
/// - Discovery payload:
///
///   ```json
///   {
///       "automation_type": "trigger",
///       "type": "action",
///       "subtype": "arrow_left_click",
///       "payload": "arrow_left_click",
///       "topic": "zigbee2mqtt/0x90fd9ffffedf1266/action",
///       "device": {
///           "identifiers": [
///               "zigbee2mqtt_0x90fd9ffffedf1266"
///           ],
///           "name": "0x90fd9ffffedf1266",
///           "sw_version": "Zigbee2MQTT 1.14.0",
///           "model": "TRADFRI remote control (E1524/E1810)",
///           "manufacturer": "IKEA"
///       }
///   }
///   ```
///
/// - Trigger topic: `zigbee2mqtt/0x90fd9ffffedf1266/action`
/// - Trigger payload: `arrow_left_click`
///
/// #### Right arrow click configuration
///
/// - Discovery topic: `homeassistant/device_automation/0x90fd9ffffedf1266/action_arrow_right_click/config`
/// - Discovery payload:
///
///   ```json
///   {
///       "automation_type": "trigger",
///       "type": "action",
///       "subtype": "arrow_right_click",
///       "payload": "arrow_right_click",
///       "topic": "zigbee2mqtt/0x90fd9ffffedf1266/action",
///       "device": {
///           "identifiers": [
///               "zigbee2mqtt_0x90fd9ffffedf1266"
///           ]
///       }
///   }   
///   ```
///
/// - Trigger topic: `zigbee2mqtt/0x90fd9ffffedf1266/action`
/// - Trigger payload: `arrow_right_click`
///
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeviceTrigger {
    /// Replaces `~` with this value in any MQTT topic attribute.
    /// [See Home Assistant documentation](https://www.home-assistant.io/integrations/mqtt/#using-abbreviations-and-base-topic)
    #[serde(rename = "~", skip_serializing_if = "Option::is_none")]
    pub topic_prefix: Option<String>,

    /// It is encouraged to add additional information about the origin that supplies MQTT entities via MQTT discovery by adding the origin option (can be abbreviated to o) to the discovery payload. Note that these options also support abbreviations. Information of the origin will be logged to the core event log when an item is discovered or updated.
    #[serde(rename = "o")]
    pub origin: Origin,

    /// Information about the device this button is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/en/device_registry_index.html). Only works when [`unique_id`](#unique_id) is set. At least one of identifiers or connections must be present to identify the device.
    #[serde(rename = "dev")]
    pub device: DeviceInformation,

    /// A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`.
    #[serde(flatten)]
    pub availability: Availability,

    /// The category of the entity. (optional, default: None)
    #[serde(rename = "ent_cat", skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<EntityCategory>,

    /// The type of automation, must be 'trigger'.
    #[serde(rename = "atype")]
    pub automation_type: String,

    /// Optional payload to match the payload being sent over the topic.
    #[serde(rename = "pl", skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,

    /// Must be `device_automation`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    #[serde(rename = "p")]
    pub platform: String,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// The subtype of the trigger, e.g. `button_1`. Entries supported by the frontend: `turn_on`, `turn_off`, `button_1`, `button_2`, `button_3`, `button_4`, `button_5`, `button_6`. If set to an unsupported value, will render as `subtype type`, e.g. `left_button pressed` with `type` set to `button_short_press` and `subtype` set to `left_button`
    #[serde(rename = "stype")]
    pub subtype: String,

    /// The MQTT topic subscribed to receive trigger events.
    #[serde(rename = "t")]
    pub topic: String,

    /// The type of the trigger, e.g. `button_short_press`. Entries supported by the frontend: `button_short_press`, `button_short_release`, `button_long_press`, `button_long_release`, `button_double_press`, `button_triple_press`, `button_quadruple_press`, `button_quintuple_press`. If set to an unsupported value, will render as `subtype type`, e.g. `button_1 spammed` with `type` set to `spammed` and `subtype` set to `button_1`
    #[serde(rename = "type")]
    pub r#type: String,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the value.
    #[serde(rename = "val_tpl", skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

impl DeviceTrigger {
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
    pub fn device(mut self, device: DeviceInformation) -> Self {
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

    /// The type of automation, must be 'trigger'.
    pub fn automation_type<T: Into<String>>(mut self, automation_type: T) -> Self {
        self.automation_type = automation_type.into();
        self
    }

    /// Optional payload to match the payload being sent over the topic.
    pub fn payload<T: Into<String>>(mut self, payload: T) -> Self {
        self.payload = Some(payload.into());
        self
    }

    /// Must be `device_automation`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    pub fn platform<T: Into<String>>(mut self, platform: T) -> Self {
        self.platform = platform.into();
        self
    }

    /// The maximum QoS level to be used when receiving and publishing messages.
    pub fn qos(mut self, qos: Qos) -> Self {
        self.qos = Some(qos);
        self
    }

    /// The subtype of the trigger, e.g. `button_1`. Entries supported by the frontend: `turn_on`, `turn_off`, `button_1`, `button_2`, `button_3`, `button_4`, `button_5`, `button_6`. If set to an unsupported value, will render as `subtype type`, e.g. `left_button pressed` with `type` set to `button_short_press` and `subtype` set to `left_button`
    pub fn subtype<T: Into<String>>(mut self, subtype: T) -> Self {
        self.subtype = subtype.into();
        self
    }

    /// The MQTT topic subscribed to receive trigger events.
    pub fn topic<T: Into<String>>(mut self, topic: T) -> Self {
        self.topic = topic.into();
        self
    }

    /// The type of the trigger, e.g. `button_short_press`. Entries supported by the frontend: `button_short_press`, `button_short_release`, `button_long_press`, `button_long_release`, `button_double_press`, `button_triple_press`, `button_quadruple_press`, `button_quintuple_press`. If set to an unsupported value, will render as `subtype type`, e.g. `button_1 spammed` with `type` set to `spammed` and `subtype` set to `button_1`
    pub fn r#type<T: Into<String>>(mut self, r#type: T) -> Self {
        self.r#type = r#type.into();
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the value.
    pub fn value_template<T: Into<String>>(mut self, value_template: T) -> Self {
        self.value_template = Some(value_template.into());
        self
    }
}

impl Default for DeviceTrigger {
    fn default() -> Self {
        Self {
            topic_prefix: Default::default(),
            origin: Default::default(),
            device: Default::default(),
            entity_category: Default::default(),
            availability: Default::default(),
            automation_type: Default::default(),
            payload: Default::default(),
            platform: "device_trigger".to_string(),
            qos: Default::default(),
            subtype: Default::default(),
            topic: Default::default(),
            r#type: Default::default(),
            value_template: Default::default(),
        }
    }
}

impl From<DeviceTrigger> for Entity {
    fn from(value: DeviceTrigger) -> Self {
        Entity::DeviceTrigger(value)
    }
}
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
    pub device: DeviceInformation,

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
    pub fn device(mut self, device: DeviceInformation) -> Self {
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
/// ---
/// title: "MQTT Number"
/// description: "Instructions on how to interact with a device exposing a Number through MQTT from within Home Assistant."
/// ha_category:
///   - Number
/// ha_release: 2021.2
/// ha_iot_class: Configurable
/// ha_domain: mqtt
/// ---
///
/// The `mqtt` Number platform allows you to integrate devices that might expose configuration options through MQTT into Home Assistant as a Number. Every time a message under the `topic` in the configuration is received, the number entity will be updated in Home Assistant and vice-versa, keeping the device and Home Assistant in-sync.
///
/// ## Configuration
///
/// To use an MQTT number entity in your installation, add the following to your `configuration.yaml` file.
/// {% include integrations/restart_ha_after_config_inclusion.md %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - number:
///       command_topic: my-device/threshold
/// ```
///
/// Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
///
///
/// ⚠ Important\
/// Make sure that your topic matches exactly. `some-topic/` and `some-topic` are different topics.
///
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Number {
    /// Replaces `~` with this value in any MQTT topic attribute.
    /// [See Home Assistant documentation](https://www.home-assistant.io/integrations/mqtt/#using-abbreviations-and-base-topic)
    #[serde(rename = "~", skip_serializing_if = "Option::is_none")]
    pub topic_prefix: Option<String>,

    /// It is encouraged to add additional information about the origin that supplies MQTT entities via MQTT discovery by adding the origin option (can be abbreviated to o) to the discovery payload. Note that these options also support abbreviations. Information of the origin will be logged to the core event log when an item is discovered or updated.
    #[serde(rename = "o")]
    pub origin: Origin,

    /// Information about the device this button is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/en/device_registry_index.html). Only works when [`unique_id`](#unique_id) is set. At least one of identifiers or connections must be present to identify the device.
    #[serde(rename = "dev")]
    pub device: DeviceInformation,

    /// A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`.
    #[serde(flatten)]
    pub availability: Availability,

    /// The category of the entity. (optional, default: None)
    #[serde(rename = "ent_cat", skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<EntityCategory>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `command_topic`.
    #[serde(rename = "cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub command_template: Option<String>,

    /// The MQTT topic to publish commands to change the number.
    #[serde(rename = "cmd_t")]
    pub command_topic: String,

    /// The [type/class](/integrations/number/#device-class) of the number. The `device_class` can be `null`.
    #[serde(rename = "dev_cla", skip_serializing_if = "Option::is_none")]
    pub device_class: Option<NumberDeviceClass>,

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

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as number attributes. Implies `force_update` of the current number state when a message is received on this topic.
    #[serde(rename = "json_attr_t", skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,

    /// Maximum value.
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: Option<Decimal>,

    /// Minimum value.
    #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
    pub min: Option<Decimal>,

    /// Control how the number should be displayed in the UI. Can be set to `box` or `slider` to force a display mode.
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    /// The name of the Number. Can be set to `null` if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// Flag that defines if number works in optimistic mode.
    #[serde(rename = "opt", skip_serializing_if = "Option::is_none")]
    pub optimistic: Option<bool>,

    /// A special payload that resets the state to `unknown` when received on the `state_topic`.
    #[serde(rename = "pl_rst", skip_serializing_if = "Option::is_none")]
    pub payload_reset: Option<String>,

    /// Must be `number`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    #[serde(rename = "p")]
    pub platform: String,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// If the published message should have the retain flag on or not.
    #[serde(rename = "ret", skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,

    /// The MQTT topic subscribed to receive number values. An empty payload is ignored.
    #[serde(rename = "stat_t", skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,

    /// Step value. Smallest value `0.001`.
    #[serde(rename = "step", skip_serializing_if = "Option::is_none")]
    pub step: Option<Decimal>,

    /// An ID that uniquely identifies this Number. If two Numbers have the same unique ID Home Assistant will raise an exception. Required when used with device-based discovery.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,

    /// Defines the unit of measurement of the sensor, if any. The `unit_of_measurement` can be `null`.
    #[serde(rename = "unit_of_meas", skip_serializing_if = "Option::is_none")]
    pub unit_of_measurement: Option<Unit>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the value.
    #[serde(rename = "val_tpl", skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

impl Number {
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
    pub fn device(mut self, device: DeviceInformation) -> Self {
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

    /// The MQTT topic to publish commands to change the number.
    pub fn command_topic<T: Into<String>>(mut self, command_topic: T) -> Self {
        self.command_topic = command_topic.into();
        self
    }

    /// The [type/class](/integrations/number/#device-class) of the number. The `device_class` can be `null`.
    pub fn device_class(mut self, device_class: NumberDeviceClass) -> Self {
        self.device_class = Some(device_class);
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

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as number attributes. Implies `force_update` of the current number state when a message is received on this topic.
    pub fn json_attributes_topic<T: Into<String>>(mut self, json_attributes_topic: T) -> Self {
        self.json_attributes_topic = Some(json_attributes_topic.into());
        self
    }

    /// Maximum value.
    pub fn max(mut self, max: Decimal) -> Self {
        self.max = Some(max);
        self
    }

    /// Minimum value.
    pub fn min(mut self, min: Decimal) -> Self {
        self.min = Some(min);
        self
    }

    /// Control how the number should be displayed in the UI. Can be set to `box` or `slider` to force a display mode.
    pub fn mode<T: Into<String>>(mut self, mode: T) -> Self {
        self.mode = Some(mode.into());
        self
    }

    /// The name of the Number. Can be set to `null` if only the device name is relevant.
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    /// Flag that defines if number works in optimistic mode.
    pub fn optimistic(mut self, optimistic: bool) -> Self {
        self.optimistic = Some(optimistic);
        self
    }

    /// A special payload that resets the state to `unknown` when received on the `state_topic`.
    pub fn payload_reset<T: Into<String>>(mut self, payload_reset: T) -> Self {
        self.payload_reset = Some(payload_reset.into());
        self
    }

    /// Must be `number`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
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

    /// The MQTT topic subscribed to receive number values. An empty payload is ignored.
    pub fn state_topic<T: Into<String>>(mut self, state_topic: T) -> Self {
        self.state_topic = Some(state_topic.into());
        self
    }

    /// Step value. Smallest value `0.001`.
    pub fn step(mut self, step: Decimal) -> Self {
        self.step = Some(step);
        self
    }

    /// An ID that uniquely identifies this Number. If two Numbers have the same unique ID Home Assistant will raise an exception. Required when used with device-based discovery.
    pub fn unique_id<T: Into<String>>(mut self, unique_id: T) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }

    /// Defines the unit of measurement of the sensor, if any. The `unit_of_measurement` can be `null`.
    pub fn unit_of_measurement<T: Into<Unit>>(mut self, unit_of_measurement: T) -> Self {
        self.unit_of_measurement = Some(unit_of_measurement.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the value.
    pub fn value_template<T: Into<String>>(mut self, value_template: T) -> Self {
        self.value_template = Some(value_template.into());
        self
    }
}

impl Default for Number {
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
            max: Default::default(),
            min: Default::default(),
            mode: Default::default(),
            name: Default::default(),
            object_id: Default::default(),
            optimistic: Default::default(),
            payload_reset: Default::default(),
            platform: "number".to_string(),
            qos: Default::default(),
            retain: Default::default(),
            state_topic: Default::default(),
            step: Default::default(),
            unique_id: Default::default(),
            unit_of_measurement: Default::default(),
            value_template: Default::default(),
        }
    }
}

impl From<Number> for Entity {
    fn from(value: Number) -> Self {
        Entity::Number(value)
    }
}
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
/// To use an MQTT binary sensor in your installation, [add a MQTT device as a subentry](/integrations/mqtt/#configuration), or add the following to your `configuration.yaml` file.
/// {% include integrations/restart_ha_after_config_inclusion.md %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - binary_sensor:
///       state_topic: "basement/window/contact"
/// ```
///
/// Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
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
///       state_topic: "bedroom/window/contact"
///       payload_on: "ON"
///       availability:
///         - topic: "bedroom/window/availability"
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
    pub device: DeviceInformation,

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

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
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
    #[serde(rename = "p")]
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
    pub fn device(mut self, device: DeviceInformation) -> Self {
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

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
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
    pub device: DeviceInformation,

    /// A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`.
    #[serde(flatten)]
    pub availability: Availability,

    /// The category of the entity. (optional, default: None)
    #[serde(rename = "ent_cat", skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<EntityCategory>,

    /// The MQTT topic subscribed to receive tag scanned events.
    #[serde(rename = "t")]
    pub topic: String,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) that returns a tag ID.
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
    pub fn device(mut self, device: DeviceInformation) -> Self {
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

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) that returns a tag ID.
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
    pub device: DeviceInformation,

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
    pub fn device(mut self, device: DeviceInformation) -> Self {
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
/// ---
/// title: "MQTT Fan"
/// description: "Instructions on how to integrate MQTT fans into Home Assistant."
/// ha_category:
///   - Fan
/// ha_release: 0.27
/// ha_iot_class: Configurable
/// ha_domain: mqtt
/// ---
///
/// The `mqtt` fan platform lets you control your MQTT enabled fans.
///
/// ## Configuration
///
/// In an ideal scenario, the MQTT device will have a `state_topic` to publish state changes. If these messages are published with a `RETAIN` flag, the MQTT fan will receive an instant state update after subscription and will start with the correct state. Otherwise, the initial state of the fan will be `unknown`. A MQTT device can reset the current state to `unknown` using a `None` payload.
///
/// When a `state_topic` is not available, the fan will work in optimistic mode. In this mode, the fan will immediately change state after every command. Otherwise, the fan will wait for state confirmation from the device (message from `state_topic`).  The initial state is set to `False` / `off` in optimistic mode.
///
/// Optimistic mode can be forced even if a `state_topic` is available. Try to enable it if you are experiencing incorrect fan operation.
///
/// To use an MQTT fan in your installation, [add a MQTT device as a subentry](/integrations/mqtt/#configuration), or add the following to your `configuration.yaml` file.
/// {% include integrations/restart_ha_after_config_inclusion.md %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - fan:
///       command_topic: "bedroom_fan/on/set"
/// ```
///
/// Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
///
///
/// ⚠ Important\
///
/// Make sure that your topics match exactly. `some-topic/` and `some-topic` are different topics.
///
///
/// ## Examples
///
/// In this section you find some real-life examples of how to use this fan.
///
/// ### Full configuration
///
/// The example below shows a full configuration for a MQTT fan using percentage and preset modes.
/// There are 10 speeds within the speed range, so  `percentage_step` = 100 / 10 steps = 10.0 %.
///
/// ```yaml
/// # Example using percentage based speeds with preset modes configuration.yaml
/// mqtt:
///   - fan:
///       name: "Bedroom Fan"
///       state_topic: "bedroom_fan/on/state"
///       command_topic: "bedroom_fan/on/set"
///       direction_state_topic: "bedroom_fan/direction/state"
///       direction_command_topic: "bedroom_fan/direction/set"
///       oscillation_state_topic: "bedroom_fan/oscillation/state"
///       oscillation_command_topic: "bedroom_fan/oscillation/set"
///       percentage_state_topic: "bedroom_fan/speed/percentage_state"
///       percentage_command_topic: "bedroom_fan/speed/percentage"
///       preset_mode_state_topic: "bedroom_fan/preset/preset_mode_state"
///       preset_mode_command_topic: "bedroom_fan/preset/preset_mode"
///       preset_modes:
///         -  "auto"
///         -  "smart"
///         -  "whoosh"
///         -  "eco"
///         -  "breeze"
///       qos: 0
///       payload_on: "true"
///       payload_off: "false"
///       payload_oscillation_on: "true"
///       payload_oscillation_off: "false"
///       speed_range_min: 1
///       speed_range_max: 10
/// ```
///
/// ### Configuration using command templates
///
/// This example demonstrates how to use command templates with JSON output.
///
///
/// ```yaml
/// # Example configuration.yaml with command templates
/// mqtt:
///   - fan:
///       name: "Bedroom Fan"
///       command_topic: "bedroom_fan/on/set"
///       command_template: "{ state: '{{ value }}'}"
///       direction_command_template: "{{ iif(value == 'forward', 'fwd', 'rev') }}"
///       direction_value_template: "{{ iif(value == 'fwd', 'forward', 'reverse') }}"
///       oscillation_command_topic: "bedroom_fan/oscillation/set"
///       oscillation_command_template: "{ oscillation: '{{ value }}'}"
///       percentage_command_topic: "bedroom_fan/speed/percentage"
///       percentage_command_template: "{ percentage: '{{ value }}'}"
///       preset_mode_command_topic: "bedroom_fan/preset/preset_mode"
///       preset_mode_command_template: "{ preset_mode: '{{ value }}'}"
///       preset_modes:
///         -  "auto"
///         -  "smart"
///         -  "whoosh"
///         -  "eco"
///         -  "breeze"
/// ```
///
///
/// This example shows how to configure a fan that doesn't use `forward` and `backward` as directions.
///
///
/// ```yaml
/// # Example configuration.yaml with direction templates
/// mqtt:
///   - fan:
///       name: "Bedroom Fan"
///       direction_command_template: "{{ iif(value == 'forward', 'fwd', 'rev') }}"
///       direction_value_template: "{{ iif(value == 'fwd', 'forward', 'reverse') }}"
/// ```
///
///
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Fan {
    /// Replaces `~` with this value in any MQTT topic attribute.
    /// [See Home Assistant documentation](https://www.home-assistant.io/integrations/mqtt/#using-abbreviations-and-base-topic)
    #[serde(rename = "~", skip_serializing_if = "Option::is_none")]
    pub topic_prefix: Option<String>,

    /// It is encouraged to add additional information about the origin that supplies MQTT entities via MQTT discovery by adding the origin option (can be abbreviated to o) to the discovery payload. Note that these options also support abbreviations. Information of the origin will be logged to the core event log when an item is discovered or updated.
    #[serde(rename = "o")]
    pub origin: Origin,

    /// Information about the device this button is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/en/device_registry_index.html). Only works when [`unique_id`](#unique_id) is set. At least one of identifiers or connections must be present to identify the device.
    #[serde(rename = "dev")]
    pub device: DeviceInformation,

    /// A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`.
    #[serde(flatten)]
    pub availability: Availability,

    /// The category of the entity. (optional, default: None)
    #[serde(rename = "ent_cat", skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<EntityCategory>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `command_topic`.
    #[serde(rename = "cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub command_template: Option<String>,

    /// The MQTT topic to publish commands to change the fan state.
    #[serde(rename = "cmd_t")]
    pub command_topic: String,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `direction_command_topic`.
    #[serde(rename = "dir_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub direction_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the direction state.
    #[serde(rename = "dir_cmd_t", skip_serializing_if = "Option::is_none")]
    pub direction_command_topic: Option<String>,

    /// The MQTT topic subscribed to receive direction state updates.
    #[serde(rename = "dir_stat_t", skip_serializing_if = "Option::is_none")]
    pub direction_state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract a value from the direction.
    #[serde(rename = "dir_val_tpl", skip_serializing_if = "Option::is_none")]
    pub direction_value_template: Option<String>,

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

    /// The name of the fan. Can be set to `null` if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// Flag that defines if fan works in optimistic mode
    #[serde(rename = "opt", skip_serializing_if = "Option::is_none")]
    pub optimistic: Option<bool>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `oscillation_command_topic`.
    #[serde(rename = "osc_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub oscillation_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the oscillation state.
    #[serde(rename = "osc_cmd_t", skip_serializing_if = "Option::is_none")]
    pub oscillation_command_topic: Option<String>,

    /// The MQTT topic subscribed to receive oscillation state updates.
    #[serde(rename = "osc_stat_t", skip_serializing_if = "Option::is_none")]
    pub oscillation_state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract a value from the oscillation.
    #[serde(rename = "osc_val_tpl", skip_serializing_if = "Option::is_none")]
    pub oscillation_value_template: Option<String>,

    /// The payload that represents the stop state.
    #[serde(rename = "pl_off", skip_serializing_if = "Option::is_none")]
    pub payload_off: Option<String>,

    /// The payload that represents the running state.
    #[serde(rename = "pl_on", skip_serializing_if = "Option::is_none")]
    pub payload_on: Option<String>,

    /// The payload that represents the oscillation off state.
    #[serde(rename = "pl_osc_off", skip_serializing_if = "Option::is_none")]
    pub payload_oscillation_off: Option<String>,

    /// The payload that represents the oscillation on state.
    #[serde(rename = "pl_osc_on", skip_serializing_if = "Option::is_none")]
    pub payload_oscillation_on: Option<String>,

    /// A special payload that resets the `percentage` state attribute to `unknown` when received at the `percentage_state_topic`.
    #[serde(rename = "pl_rst_pct", skip_serializing_if = "Option::is_none")]
    pub payload_reset_percentage: Option<String>,

    /// A special payload that resets the `preset_mode` state attribute to `unknown` when received at the `preset_mode_state_topic`.
    #[serde(rename = "pl_rst_pr_mode", skip_serializing_if = "Option::is_none")]
    pub payload_reset_preset_mode: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `percentage_command_topic`.
    #[serde(rename = "pct_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub percentage_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the fan speed state based on a percentage.
    #[serde(rename = "pct_cmd_t", skip_serializing_if = "Option::is_none")]
    pub percentage_command_topic: Option<String>,

    /// The MQTT topic subscribed to receive fan speed based on percentage.
    #[serde(rename = "pct_stat_t", skip_serializing_if = "Option::is_none")]
    pub percentage_state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the `percentage` value from the payload received on `percentage_state_topic`.
    #[serde(rename = "pct_val_tpl", skip_serializing_if = "Option::is_none")]
    pub percentage_value_template: Option<String>,

    /// Must be `fan`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    #[serde(rename = "p")]
    pub platform: String,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `preset_mode_command_topic`.
    #[serde(rename = "pr_mode_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub preset_mode_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the preset mode.
    #[serde(rename = "pr_mode_cmd_t", skip_serializing_if = "Option::is_none")]
    pub preset_mode_command_topic: Option<String>,

    /// The MQTT topic subscribed to receive fan speed based on presets.
    #[serde(rename = "pr_mode_stat_t", skip_serializing_if = "Option::is_none")]
    pub preset_mode_state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the `preset_mode` value from the payload received on `preset_mode_state_topic`.
    #[serde(rename = "pr_mode_val_tpl", skip_serializing_if = "Option::is_none")]
    pub preset_mode_value_template: Option<String>,

    /// List of preset modes this fan is capable of running at. Common examples include `auto`, `smart`, `whoosh`, `eco` and `breeze`.
    #[serde(rename = "pr_modes", skip_serializing_if = "Option::is_none")]
    pub preset_modes: Option<Vec<String>>,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// If the published message should have the retain flag on or not.
    #[serde(rename = "ret", skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,

    /// The maximum of numeric output range (representing 100 %). The `percentage_step` is defined by `100` / the number of speeds within the speed range.
    #[serde(rename = "spd_rng_max", skip_serializing_if = "Option::is_none")]
    pub speed_range_max: Option<i32>,

    /// The minimum of numeric output range (`off` not included, so `speed_range_min` - `1` represents 0 %). The `percentage_step` is defined by `100` / the number of speeds within the speed range.
    #[serde(rename = "spd_rng_min", skip_serializing_if = "Option::is_none")]
    pub speed_range_min: Option<i32>,

    /// The MQTT topic subscribed to receive state updates. A "None" payload resets to an `unknown` state. An empty payload is ignored. By default, valid state payloads are `OFF` and `ON`. The accepted payloads can be overridden with the `payload_off` and `payload_on` config options.
    #[serde(rename = "stat_t", skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract a value from the state.
    #[serde(rename = "stat_val_tpl", skip_serializing_if = "Option::is_none")]
    pub state_value_template: Option<String>,

    /// An ID that uniquely identifies this fan. If two fans have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
}

impl Fan {
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
    pub fn device(mut self, device: DeviceInformation) -> Self {
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

    /// The MQTT topic to publish commands to change the fan state.
    pub fn command_topic<T: Into<String>>(mut self, command_topic: T) -> Self {
        self.command_topic = command_topic.into();
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `direction_command_topic`.
    pub fn direction_command_template<T: Into<String>>(
        mut self,
        direction_command_template: T,
    ) -> Self {
        self.direction_command_template = Some(direction_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the direction state.
    pub fn direction_command_topic<T: Into<String>>(mut self, direction_command_topic: T) -> Self {
        self.direction_command_topic = Some(direction_command_topic.into());
        self
    }

    /// The MQTT topic subscribed to receive direction state updates.
    pub fn direction_state_topic<T: Into<String>>(mut self, direction_state_topic: T) -> Self {
        self.direction_state_topic = Some(direction_state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract a value from the direction.
    pub fn direction_value_template<T: Into<String>>(
        mut self,
        direction_value_template: T,
    ) -> Self {
        self.direction_value_template = Some(direction_value_template.into());
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

    /// The name of the fan. Can be set to `null` if only the device name is relevant.
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    /// Flag that defines if fan works in optimistic mode
    pub fn optimistic(mut self, optimistic: bool) -> Self {
        self.optimistic = Some(optimistic);
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `oscillation_command_topic`.
    pub fn oscillation_command_template<T: Into<String>>(
        mut self,
        oscillation_command_template: T,
    ) -> Self {
        self.oscillation_command_template = Some(oscillation_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the oscillation state.
    pub fn oscillation_command_topic<T: Into<String>>(
        mut self,
        oscillation_command_topic: T,
    ) -> Self {
        self.oscillation_command_topic = Some(oscillation_command_topic.into());
        self
    }

    /// The MQTT topic subscribed to receive oscillation state updates.
    pub fn oscillation_state_topic<T: Into<String>>(mut self, oscillation_state_topic: T) -> Self {
        self.oscillation_state_topic = Some(oscillation_state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract a value from the oscillation.
    pub fn oscillation_value_template<T: Into<String>>(
        mut self,
        oscillation_value_template: T,
    ) -> Self {
        self.oscillation_value_template = Some(oscillation_value_template.into());
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

    /// The payload that represents the oscillation off state.
    pub fn payload_oscillation_off<T: Into<String>>(mut self, payload_oscillation_off: T) -> Self {
        self.payload_oscillation_off = Some(payload_oscillation_off.into());
        self
    }

    /// The payload that represents the oscillation on state.
    pub fn payload_oscillation_on<T: Into<String>>(mut self, payload_oscillation_on: T) -> Self {
        self.payload_oscillation_on = Some(payload_oscillation_on.into());
        self
    }

    /// A special payload that resets the `percentage` state attribute to `unknown` when received at the `percentage_state_topic`.
    pub fn payload_reset_percentage<T: Into<String>>(
        mut self,
        payload_reset_percentage: T,
    ) -> Self {
        self.payload_reset_percentage = Some(payload_reset_percentage.into());
        self
    }

    /// A special payload that resets the `preset_mode` state attribute to `unknown` when received at the `preset_mode_state_topic`.
    pub fn payload_reset_preset_mode<T: Into<String>>(
        mut self,
        payload_reset_preset_mode: T,
    ) -> Self {
        self.payload_reset_preset_mode = Some(payload_reset_preset_mode.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `percentage_command_topic`.
    pub fn percentage_command_template<T: Into<String>>(
        mut self,
        percentage_command_template: T,
    ) -> Self {
        self.percentage_command_template = Some(percentage_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the fan speed state based on a percentage.
    pub fn percentage_command_topic<T: Into<String>>(
        mut self,
        percentage_command_topic: T,
    ) -> Self {
        self.percentage_command_topic = Some(percentage_command_topic.into());
        self
    }

    /// The MQTT topic subscribed to receive fan speed based on percentage.
    pub fn percentage_state_topic<T: Into<String>>(mut self, percentage_state_topic: T) -> Self {
        self.percentage_state_topic = Some(percentage_state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the `percentage` value from the payload received on `percentage_state_topic`.
    pub fn percentage_value_template<T: Into<String>>(
        mut self,
        percentage_value_template: T,
    ) -> Self {
        self.percentage_value_template = Some(percentage_value_template.into());
        self
    }

    /// Must be `fan`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    pub fn platform<T: Into<String>>(mut self, platform: T) -> Self {
        self.platform = platform.into();
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `preset_mode_command_topic`.
    pub fn preset_mode_command_template<T: Into<String>>(
        mut self,
        preset_mode_command_template: T,
    ) -> Self {
        self.preset_mode_command_template = Some(preset_mode_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the preset mode.
    pub fn preset_mode_command_topic<T: Into<String>>(
        mut self,
        preset_mode_command_topic: T,
    ) -> Self {
        self.preset_mode_command_topic = Some(preset_mode_command_topic.into());
        self
    }

    /// The MQTT topic subscribed to receive fan speed based on presets.
    pub fn preset_mode_state_topic<T: Into<String>>(mut self, preset_mode_state_topic: T) -> Self {
        self.preset_mode_state_topic = Some(preset_mode_state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the `preset_mode` value from the payload received on `preset_mode_state_topic`.
    pub fn preset_mode_value_template<T: Into<String>>(
        mut self,
        preset_mode_value_template: T,
    ) -> Self {
        self.preset_mode_value_template = Some(preset_mode_value_template.into());
        self
    }

    /// List of preset modes this fan is capable of running at. Common examples include `auto`, `smart`, `whoosh`, `eco` and `breeze`.
    pub fn preset_modes<T: Into<String>>(mut self, preset_modes: Vec<T>) -> Self {
        self.preset_modes = Some(preset_modes.into_iter().map(|v| v.into()).collect());
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

    /// The maximum of numeric output range (representing 100 %). The `percentage_step` is defined by `100` / the number of speeds within the speed range.
    pub fn speed_range_max(mut self, speed_range_max: i32) -> Self {
        self.speed_range_max = Some(speed_range_max);
        self
    }

    /// The minimum of numeric output range (`off` not included, so `speed_range_min` - `1` represents 0 %). The `percentage_step` is defined by `100` / the number of speeds within the speed range.
    pub fn speed_range_min(mut self, speed_range_min: i32) -> Self {
        self.speed_range_min = Some(speed_range_min);
        self
    }

    /// The MQTT topic subscribed to receive state updates. A "None" payload resets to an `unknown` state. An empty payload is ignored. By default, valid state payloads are `OFF` and `ON`. The accepted payloads can be overridden with the `payload_off` and `payload_on` config options.
    pub fn state_topic<T: Into<String>>(mut self, state_topic: T) -> Self {
        self.state_topic = Some(state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract a value from the state.
    pub fn state_value_template<T: Into<String>>(mut self, state_value_template: T) -> Self {
        self.state_value_template = Some(state_value_template.into());
        self
    }

    /// An ID that uniquely identifies this fan. If two fans have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    pub fn unique_id<T: Into<String>>(mut self, unique_id: T) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }
}

impl Default for Fan {
    fn default() -> Self {
        Self {
            topic_prefix: Default::default(),
            origin: Default::default(),
            device: Default::default(),
            entity_category: Default::default(),
            availability: Default::default(),
            command_template: Default::default(),
            command_topic: Default::default(),
            direction_command_template: Default::default(),
            direction_command_topic: Default::default(),
            direction_state_topic: Default::default(),
            direction_value_template: Default::default(),
            enabled_by_default: Default::default(),
            encoding: Default::default(),
            entity_picture: Default::default(),
            icon: Default::default(),
            json_attributes_template: Default::default(),
            json_attributes_topic: Default::default(),
            name: Default::default(),
            object_id: Default::default(),
            optimistic: Default::default(),
            oscillation_command_template: Default::default(),
            oscillation_command_topic: Default::default(),
            oscillation_state_topic: Default::default(),
            oscillation_value_template: Default::default(),
            payload_off: Default::default(),
            payload_on: Default::default(),
            payload_oscillation_off: Default::default(),
            payload_oscillation_on: Default::default(),
            payload_reset_percentage: Default::default(),
            payload_reset_preset_mode: Default::default(),
            percentage_command_template: Default::default(),
            percentage_command_topic: Default::default(),
            percentage_state_topic: Default::default(),
            percentage_value_template: Default::default(),
            platform: "fan".to_string(),
            preset_mode_command_template: Default::default(),
            preset_mode_command_topic: Default::default(),
            preset_mode_state_topic: Default::default(),
            preset_mode_value_template: Default::default(),
            preset_modes: Default::default(),
            qos: Default::default(),
            retain: Default::default(),
            speed_range_max: Default::default(),
            speed_range_min: Default::default(),
            state_topic: Default::default(),
            state_value_template: Default::default(),
            unique_id: Default::default(),
        }
    }
}

impl From<Fan> for Entity {
    fn from(value: Fan) -> Self {
        Entity::Fan(value)
    }
}
/// ---
/// title: "MQTT Sensor"
/// description: "Instructions on how to integrate MQTT sensors within Home Assistant."
/// ha_category:
///   - Sensor
/// ha_release: 0.7
/// ha_iot_class: Configurable
/// ha_domain: mqtt
/// ---
///
/// This `mqtt` sensor platform uses the MQTT message payload as the sensor value. If messages in this `state_topic` are published with *RETAIN* flag, the sensor will receive an instant update with last known value. Otherwise, the initial state will be undefined.
///
/// ## Configuration
///
///
/// To use an MQTT sensor in your installation, [add a MQTT device as a subentry](/integrations/mqtt/#configuration), or add the following to your `configuration.yaml` file.
/// {% include integrations/restart_ha_after_config_inclusion.md %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   sensor:
///     - name: "Bedroom Temperature"
///       state_topic: "home/bedroom/temperature"
/// ```
///
/// Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
///
///
/// ## Examples
///
/// In this section, you find some real-life examples showing how to use this sensor.
///
/// ### Processing Unix EPOCH timestamps
///
/// The example below shows how an MQTT sensor can process a Unix EPOCH payload.
///
///
/// Set up via YAML:
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   sensor:
///     - name: "turned on"
///       state_topic: "pump/timestamp_on"
///       device_class: "timestamp"
///       value_template: "{{ as_datetime(value) }}"
///       unique_id: "hp_1231232_ts_on"
///       device:
///         name: "Heat pump"
///         identifiers:
///           - "hp_1231232"
/// ```
///
///
/// Or set up via MQTT discovery:
///
/// Discovery topic: `homeassistant/sensor/hp_1231232/config`
///
///
/// ```json
/// {
///   "name": "turned on",
///   "state_topic": "pump/timestamp_on",
///   "device_class": "timestamp",
///   "value_template": "{{ as_datetime(value) }}",
///   "unique_id": "hp_1231232_ts_on",
///   "device": {
///     "name": "Heat pump",
///     "identifiers": [
///       "hp_1231232"
///     ]
///   }
/// }
/// ```
///
///
/// To test, you can use the command line tool `mosquitto_pub` shipped with `mosquitto` or the `mosquitto-clients` package to send MQTT messages.
///
/// Payload topic: `pump/timestamp_on`
/// Payload: `1707294116`
///
/// To set the state of the sensor manually:
///
/// ```bash
/// mosquitto_pub -h 127.0.0.1 -p 1883 -u username -P some_password -t pump/timestamp_on -m '1707294116'
/// ```
///
/// Make sure the IP address of your MQTT broker is used and that user credentials have been set up correctly.
///
/// The `value_template` will render the Unix EPOCH timestamp to correct format: `2024-02-07 08:21:56+00:00`.
///
/// ### JSON attributes topic configuration
///
/// The example sensor below shows a configuration example which uses the following separate topic and JSON structure to add extra attributes.
///
/// Topic: `home/sensor1/attributes`
///  ```json
///  {
///     "ClientName": <string>,
///     "IP": <string>,
///     "MAC": <string>,
///     "RSSI": <string>,
///     "HostName": <string>,
///     "ConnectedSSID": <string>
/// }
///  ```
///  It also makes use of the `availability` topic.
///
/// Extra attributes will be displayed in the frontend and can also be extracted in [Templates](/docs/configuration/templating/#attributes). For example, to extract the `ClientName` attribute from the sensor below, use a template similar to: {% raw %}`{{ state_attr('sensor.bs_rssi', 'ClientName') }}`{% endraw %}.
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   sensor:
///     - name: "RSSI"
///       state_topic: "home/sensor1/infojson"
///       unit_of_measurement: "dBm"
///       value_template: "{{ value_json.RSSI }}"
///       availability:
///         - topic: "home/sensor1/status"
///       payload_available: "online"
///       payload_not_available: "offline"
///       json_attributes_topic: "home/sensor1/attributes"
/// ```
///
///
/// ### JSON attributes template configuration
///
/// The example sensor below shows a configuration example which uses the following topic and JSON structure with a template to add `Timer1.Arm` and `Timer1.Time` as extra attributes.
///
/// Topic: `tele/sonoff/sensor`
/// ```json
/// {
///     "Timer1": {
///         "Arm": <status>,
///         "Time": <time>
///     },
///     "Timer2": {
///         "Arm": <status>,
///         "Time": <time>
///     }
/// }
/// ```
/// To instead only add `Timer1.Arm`as an extra attribute, change `json_attributes_template` to: {% raw %}`"{{ {'Arm': value_json.Timer1} | tojson }}"`{% endraw %}.
///
/// Extra attributes will be displayed in the frontend and can also be extracted in [Templates](/docs/configuration/templating/#attributes). For example, to extract the `Arm` attribute from the sensor below, use a template similar to: {% raw %}`{{ state_attr('sensor.timer1', 'Arm') }}`{% endraw %}.
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   sensor:
///     - name: "Timer 1"
///       state_topic: "tele/sonoff/sensor"
///       value_template: "{{ value_json.Timer1.Arm }}"
///       json_attributes_topic: "tele/sonoff/sensor"
///       json_attributes_template: "{{ value_json.Timer1 | tojson }}"
///
///     - name: "Timer 2"
///       state_topic: "tele/sonoff/sensor"
///       value_template: "{{ value_json.Timer2.Arm }}"
///       json_attributes_topic: "tele/sonoff/sensor"
///       json_attributes_template: "{{ value_json.Timer2 | tojson }}"
/// ```
///
///
/// {% warning %}
/// If `json_attributes_topic` and `state_topic` share the same topic, a state update will happen only once, unless the state update did not change the state or `force_update` was set to `true`.
///
/// Setting up MQTT sensor's with extra state attributes that contain values that change at every update, like timestamps, or enabling the `force_update` option, is discouraged, as this will trigger state writes for every update. This can have a serious impact on the total system performance. A better option is creating separate sensors instead.
/// {% endwarning %}
///
/// ### Usage of `entity_id` in the template
///
/// The example below shows how a simple filter, that calculates the value by adding 90% of the new value and 10% of the previous value, can be implemented in a template.
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   sensor:
///     - name: "Temp 1"
///       state_topic: "sensor/temperature"
///       value_template: |-
///         {% if states(entity_id) == None %}
///           {{ value | round(2) }}
///         {% else %}
///           {{ value | round(2) * 0.9 + states(entity_id) * 0.1 }}
///         {% endif %}
/// ```
///
///
/// ### Owntracks battery level sensor
///
/// If you are using the [OwnTracks](/integrations/owntracks) and enable the reporting of the battery level then you can use an MQTT sensor to keep track of your battery. A regular MQTT message from OwnTracks looks like this:
///
/// Topic: `owntracks/tablet/tablet`
/// ```json
/// {
///     "_type": "location",
///     "lon": 7.21,
///     "t": "u",
///     "batt": 92,
///     "tst": 144995643,
///     "tid": "ta",
///     "acc": 27,
///     "lat": 46.12
/// }
/// ```
///
/// Thus the trick is extracting the battery level from the payload.
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   sensor:
///     - name: "Battery Tablet"
///       state_topic: "owntracks/tablet/tablet"
///       unit_of_measurement: "%"
///       value_template: "{{ value_json.batt }}"
/// ```
///
///
/// ### Temperature and humidity sensors
///
/// If you are using a DHT sensor and a NodeMCU board (esp8266), you can retrieve temperature and humidity with a MQTT sensor. A code example can be found [here](https://github.com/mertenats/open-home-automation/tree/master/ha_mqtt_sensor_dht22). A regular MQTT message from this example looks like this:
///
/// Topic: `office/sensor1`
/// ```json
///   {
///     "temperature": 23.20,
///     "humidity": 43.70
///   }
/// ```
///
/// Then use this configuration example to extract the data from the payload:
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   sensor:
///     - name: "Temperature"
///       state_topic: "office/sensor1"
///       suggested_display_precision: 1
///       unit_of_measurement: "°C"
///       value_template: "{{ value_json.temperature }}"
///     - name: "Humidity"
///       state_topic: "office/sensor1"
///       unit_of_measurement: "%"
///       value_template: "{{ value_json.humidity }}"
/// ```
///
///
/// ### Get sensor value from a device with ESPEasy
///
/// Assuming that you have flashed your ESP8266 unit with [ESPEasy](https://github.com/letscontrolit/ESPEasy). Under "Config" set a name ("Unit Name:") for your device (here it's "bathroom"). A "Controller" for MQTT with the protocol "OpenHAB MQTT" is present and the entries ("Controller Subscribe:" and "Controller Publish:") are adjusted to match your needs. In this example the topics are prefixed with "home". Please keep in mind that the ESPEasy default topics start with a `/` and only contain the name when writing your entry for the `configuration.yaml` file.
///
/// - **Controller Subscribe**: `home/%sysname%/#` (instead of `/%sysname%/#`)
/// - **Controller Publish**: `home/%sysname%/%tskname%/%valname%` (instead of `/%sysname%/%tskname%/%valname%`)
///
/// Also, add a sensor in the "Devices" tap with the name "analog" and "brightness" as value.
///
/// As soon as the unit is online, you will get the state of the sensor.
///
/// ```bash
/// home/bathroom/status Connected
/// ...
/// home/bathroom/analog/brightness 290.00
/// ```
///
/// The configuration will look like the example below:
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   sensor:
///     - name: "Brightness"
///       state_topic: "home/bathroom/analog/brightness"
/// ```
///
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Sensor {
    /// Replaces `~` with this value in any MQTT topic attribute.
    /// [See Home Assistant documentation](https://www.home-assistant.io/integrations/mqtt/#using-abbreviations-and-base-topic)
    #[serde(rename = "~", skip_serializing_if = "Option::is_none")]
    pub topic_prefix: Option<String>,

    /// It is encouraged to add additional information about the origin that supplies MQTT entities via MQTT discovery by adding the origin option (can be abbreviated to o) to the discovery payload. Note that these options also support abbreviations. Information of the origin will be logged to the core event log when an item is discovered or updated.
    #[serde(rename = "o")]
    pub origin: Origin,

    /// Information about the device this button is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/en/device_registry_index.html). Only works when [`unique_id`](#unique_id) is set. At least one of identifiers or connections must be present to identify the device.
    #[serde(rename = "dev")]
    pub device: DeviceInformation,

    /// A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`.
    #[serde(flatten)]
    pub availability: Availability,

    /// The category of the entity. (optional, default: None)
    #[serde(rename = "ent_cat", skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<EntityCategory>,

    /// The [type/class](/integrations/sensor/#device-class) of the sensor to set the icon in the frontend. The `device_class` can be `null`.
    #[serde(rename = "dev_cla", skip_serializing_if = "Option::is_none")]
    pub device_class: Option<SensorDeviceClass>,

    /// Flag which defines if the entity should be enabled when first added.
    #[serde(rename = "en", skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,

    /// The encoding of the payloads received. Set to `""` to disable decoding of incoming payload.
    #[serde(rename = "e", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,

    /// Picture URL for the entity.
    #[serde(rename = "ent_pic", skip_serializing_if = "Option::is_none")]
    pub entity_picture: Option<String>,

    /// Sends update events even if the value hasn't changed. Useful if you want to have meaningful value graphs in history.
    #[serde(rename = "frc_upd", skip_serializing_if = "Option::is_none")]
    pub force_update: Option<bool>,

    /// [Icon](/docs/configuration/customizing-devices/#icon) for the entity.
    #[serde(rename = "ic", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the JSON dictionary from messages received on the `json_attributes_topic`.
    #[serde(rename = "json_attr_tpl", skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Implies `force_update` of the current sensor state when a message is received on this topic.
    #[serde(rename = "json_attr_t", skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the last_reset. When `last_reset_value_template` is set, the `state_class` option must be `total`. Available variables: `entity_id`. The `entity_id` can be used to reference the entity's attributes.
    #[serde(rename = "lrst_val_tpl", skip_serializing_if = "Option::is_none")]
    pub last_reset_value_template: Option<String>,

    /// The name of the MQTT sensor. Can be set to `null` if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// List of allowed sensor state value. An empty list is not allowed. The sensor's `device_class` must be set to `enum`. The `options` option cannot be used together with `state_class` or `unit_of_measurement`.
    #[serde(rename = "ops", skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,

    /// Must be `sensor`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    #[serde(rename = "p")]
    pub platform: String,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// The [state_class](https://developers.home-assistant.io/docs/core/entity/sensor#available-state-classes) of the sensor.
    #[serde(rename = "stat_cla", skip_serializing_if = "Option::is_none")]
    pub state_class: Option<SensorStateClass>,

    /// The MQTT topic subscribed to receive sensor values. If `device_class`, `state_class`, `unit_of_measurement` or `suggested_display_precision` is set, and a numeric value is expected, an empty value `''` will be ignored and will not update the state, a `'None'` value will set the sensor to an `unknown` state. If a `value_template` is used to parse a JSON payload, a `null` value in the JSON [will be rendered as](/docs/configuration/templating/#using-value-templates-with-mqtt) `'None'`. Note that the `device_class` can be `null`.
    #[serde(rename = "stat_t")]
    pub state_topic: String,

    /// The number of decimals which should be used in the sensor's state after rounding.
    #[serde(rename = "sug_dsp_prc", skip_serializing_if = "Option::is_none")]
    pub suggested_display_precision: Option<i32>,

    /// An ID that uniquely identifies this sensor. If two sensors have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,

    /// Defines the units of measurement of the sensor, if any. The `unit_of_measurement` can be `null`.
    #[serde(rename = "unit_of_meas", skip_serializing_if = "Option::is_none")]
    pub unit_of_measurement: Option<Unit>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the value. If the template throws an error, the current state will be used instead.
    #[serde(rename = "val_tpl", skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

impl Sensor {
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
    pub fn device(mut self, device: DeviceInformation) -> Self {
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

    /// The [type/class](/integrations/sensor/#device-class) of the sensor to set the icon in the frontend. The `device_class` can be `null`.
    pub fn device_class(mut self, device_class: SensorDeviceClass) -> Self {
        self.device_class = Some(device_class);
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

    /// Sends update events even if the value hasn't changed. Useful if you want to have meaningful value graphs in history.
    pub fn force_update(mut self, force_update: bool) -> Self {
        self.force_update = Some(force_update);
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

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Implies `force_update` of the current sensor state when a message is received on this topic.
    pub fn json_attributes_topic<T: Into<String>>(mut self, json_attributes_topic: T) -> Self {
        self.json_attributes_topic = Some(json_attributes_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the last_reset. When `last_reset_value_template` is set, the `state_class` option must be `total`. Available variables: `entity_id`. The `entity_id` can be used to reference the entity's attributes.
    pub fn last_reset_value_template<T: Into<String>>(
        mut self,
        last_reset_value_template: T,
    ) -> Self {
        self.last_reset_value_template = Some(last_reset_value_template.into());
        self
    }

    /// The name of the MQTT sensor. Can be set to `null` if only the device name is relevant.
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    /// List of allowed sensor state value. An empty list is not allowed. The sensor's `device_class` must be set to `enum`. The `options` option cannot be used together with `state_class` or `unit_of_measurement`.
    pub fn options<T: Into<String>>(mut self, options: Vec<T>) -> Self {
        self.options = Some(options.into_iter().map(|v| v.into()).collect());
        self
    }

    /// Must be `sensor`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    pub fn platform<T: Into<String>>(mut self, platform: T) -> Self {
        self.platform = platform.into();
        self
    }

    /// The maximum QoS level to be used when receiving and publishing messages.
    pub fn qos(mut self, qos: Qos) -> Self {
        self.qos = Some(qos);
        self
    }

    /// The [state_class](https://developers.home-assistant.io/docs/core/entity/sensor#available-state-classes) of the sensor.
    pub fn state_class<T: Into<SensorStateClass>>(mut self, state_class: T) -> Self {
        self.state_class = Some(state_class.into());
        self
    }

    /// The MQTT topic subscribed to receive sensor values. If `device_class`, `state_class`, `unit_of_measurement` or `suggested_display_precision` is set, and a numeric value is expected, an empty value `''` will be ignored and will not update the state, a `'None'` value will set the sensor to an `unknown` state. If a `value_template` is used to parse a JSON payload, a `null` value in the JSON [will be rendered as](/docs/configuration/templating/#using-value-templates-with-mqtt) `'None'`. Note that the `device_class` can be `null`.
    pub fn state_topic<T: Into<String>>(mut self, state_topic: T) -> Self {
        self.state_topic = state_topic.into();
        self
    }

    /// The number of decimals which should be used in the sensor's state after rounding.
    pub fn suggested_display_precision(mut self, suggested_display_precision: i32) -> Self {
        self.suggested_display_precision = Some(suggested_display_precision);
        self
    }

    /// An ID that uniquely identifies this sensor. If two sensors have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    pub fn unique_id<T: Into<String>>(mut self, unique_id: T) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }

    /// Defines the units of measurement of the sensor, if any. The `unit_of_measurement` can be `null`.
    pub fn unit_of_measurement<T: Into<Unit>>(mut self, unit_of_measurement: T) -> Self {
        self.unit_of_measurement = Some(unit_of_measurement.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the value. If the template throws an error, the current state will be used instead.
    pub fn value_template<T: Into<String>>(mut self, value_template: T) -> Self {
        self.value_template = Some(value_template.into());
        self
    }
}

impl Default for Sensor {
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
            last_reset_value_template: Default::default(),
            name: Default::default(),
            object_id: Default::default(),
            options: Default::default(),
            platform: "sensor".to_string(),
            qos: Default::default(),
            state_class: Default::default(),
            state_topic: Default::default(),
            suggested_display_precision: Default::default(),
            unique_id: Default::default(),
            unit_of_measurement: Default::default(),
            value_template: Default::default(),
        }
    }
}

impl From<Sensor> for Entity {
    fn from(value: Sensor) -> Self {
        Entity::Sensor(value)
    }
}
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
/// To use an MQTT lawn mower in your installation, add the following to your `configuration.yaml` file.
/// {% include integrations/restart_ha_after_config_inclusion.md %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - lawn_mower:
///       command_topic: topic
///       name: "Test Lawn Mower"
/// ```
///
/// Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
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
    pub device: DeviceInformation,

    /// A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`.
    #[serde(flatten)]
    pub availability: Availability,

    /// The category of the entity. (optional, default: None)
    #[serde(rename = "ent_cat", skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<EntityCategory>,

    /// The MQTT topic subscribed to receive an update of the activity. Valid activities are `mowing`, `paused`, `docked`, and `error`. Use `value_template` to extract the activity state from a custom payload. When payload `none` is received, the activity state will be reset to `unknown`.
    #[serde(rename = "act_stat_t", skip_serializing_if = "Option::is_none")]
    pub activity_state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the value.
    #[serde(rename = "act_val_tpl", skip_serializing_if = "Option::is_none")]
    pub activity_value_template: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `dock_command_topic`. The `value` parameter in the template will be set to `dock`.
    #[serde(rename = "dock_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub dock_command_template: Option<String>,

    /// The MQTT topic that publishes commands when the `lawn_mower.dock` action is performed. The value `dock` is published when the action is used. Use a `dock_command_template` to publish a custom format.
    #[serde(rename = "dock_cmd_t", skip_serializing_if = "Option::is_none")]
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

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// Flag that defines if the lawn mower works in optimistic mode.
    #[serde(rename = "opt", skip_serializing_if = "Option::is_none")]
    pub optimistic: Option<bool>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `pause_command_topic`. The `value` parameter in the template will be set to `pause`.
    #[serde(rename = "pause_mw_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub pause_command_template: Option<String>,

    /// The MQTT topic that publishes commands when the `lawn_mower.pause` action is performed. The value `pause` is published when the action is used. Use a `pause_command_template` to publish a custom format.
    #[serde(rename = "pause_cmd_t", skip_serializing_if = "Option::is_none")]
    pub pause_command_topic: Option<String>,

    /// Must be `lawn_mower`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    #[serde(rename = "p")]
    pub platform: String,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// If the published message should have the retain flag on or not.
    #[serde(rename = "ret", skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,

    /// The MQTT topic that publishes commands when the `lawn_mower.start_mowing` action is performed. The value `start_mowing` is published when the action used. Use a `start_mowing_command_template` to publish a custom format.
    #[serde(rename = "strt_mw_cmd_t", skip_serializing_if = "Option::is_none")]
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
    pub fn device(mut self, device: DeviceInformation) -> Self {
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

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
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
/// To use an MQTT update entity in your installation, add the following to your `configuration.yaml` file.
/// {% include integrations/restart_ha_after_config_inclusion.md %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - update:
///       state_topic: topic-installed
///       latest_version_topic: topic-latest
/// ```
///
/// Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
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
    pub device: DeviceInformation,

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
    #[serde(rename = "dsp_prc", skip_serializing_if = "Option::is_none")]
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

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// The MQTT payload to start installing process.
    #[serde(rename = "pl_inst", skip_serializing_if = "Option::is_none")]
    pub payload_install: Option<String>,

    /// Must be `update`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    #[serde(rename = "p")]
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
    pub fn device(mut self, device: DeviceInformation) -> Self {
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

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
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
/// ---
/// title: "MQTT Switch"
/// description: "Instructions on how to integrate MQTT switches into Home Assistant."
/// ha_category:
///   - Switch
/// ha_release: 0.7
/// ha_iot_class: Configurable
/// ha_domain: mqtt
/// ---
///
/// The `mqtt` switch platform lets you control your MQTT enabled switches.
///
/// ## Configuration
///
/// In an ideal scenario, the MQTT device will have a `state_topic` to publish state changes. If these messages are published with a `RETAIN` flag, the MQTT switch will receive an instant state update after subscription, and will start with the correct state. Otherwise, the initial state of the switch will be `unknown`. A MQTT device can reset the current state to `unknown` using a `None` payload.
///
/// When a `state_topic` is not available, the switch will work in optimistic mode. In this mode, the switch will immediately change state after every command. Otherwise, the switch will wait for state confirmation from the device (message from `state_topic`). The initial state is set to `False` / `off` in optimistic mode.
///
/// Optimistic mode can be forced, even if the `state_topic` is available. Try to enable it, if experiencing incorrect switch operation.
///
/// To use an MQTT switch in your installation, [add a MQTT device as a subentry](/integrations/mqtt/#configuration), or add the following to your `configuration.yaml` file.
/// {% include integrations/restart_ha_after_config_inclusion.md %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - switch:
///       command_topic: "home/bedroom/switch1/set"
/// ```
///
/// Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
///
///
/// ⚠ Important\
/// Make sure that your topic matches exactly. `some-topic/` and `some-topic` are different topics.
///
/// ## Examples
///
/// In this section, you will find some real-life examples of how to use this sensor.
///
/// ### Full configuration
///
/// The example below shows a full configuration for a switch.
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - switch:
///       unique_id: bedroom_switch
///       name: "Bedroom Switch"
///       state_topic: "home/bedroom/switch1"
///       command_topic: "home/bedroom/switch1/set"
///       availability:
///         - topic: "home/bedroom/switch1/available"
///       payload_on: "ON"
///       payload_off: "OFF"
///       state_on: "ON"
///       state_off: "OFF"
///       optimistic: false
///       qos: 0
///       retain: true
/// ```
///
/// For a check, you can use the command line tools `mosquitto_pub` shipped with `mosquitto` to send MQTT messages. This allows you to operate your switch manually. First, we can simulate the availability message sent for the switch:
///
/// ```bash
/// mosquitto_pub -h 127.0.0.1 -t home/bedroom/switch1/available -m "online"
/// ```
///
/// We can simulate the switch being turned on by publishing the `ON` command message:
///
/// ```bash
/// mosquitto_pub -h 127.0.0.1 -t home/bedroom/switch1/set -m "ON"
/// ```
///
/// Finally, we can simulate the switch reporting back the changed state to Home Assistant:
///
/// ```bash
/// mosquitto_pub -h 127.0.0.1 -t home/bedroom/switch1 -m "ON"
/// ```
///
/// ### Set the state of a device with ESPEasy
///
/// Assuming that you have flashed your ESP8266 unit with [ESPEasy](https://github.com/letscontrolit/ESPEasy). Under "Config" is a name ("Unit Name:") set for your device (here it's "bathroom"). A configuration for a "Controller" for MQTT with the protocol "OpenHAB MQTT" is present and the entries ("Controller Subscribe:" and "Controller Publish:") are adjusted to match your needs. In this example, the topics are prefixed with "home". There is no further configuration needed as the [GPIOs](https://espeasy.readthedocs.io/en/latest/Controller/C005.html) can be controlled with MQTT directly.
///
/// Manually you can set pin 13 to high with `mosquitto_pub` or another MQTT tool:
///
/// ```bash
/// mosquitto_pub -h 127.0.0.1 -t home/bathroom/gpio/13 -m "1"
/// ```
///
/// The configuration will look like the example below:
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - switch:
///       name: bathroom
///       state_topic: "home/bathroom/gpio/13"
///       command_topic: "home/bathroom/gpio/13"
///       payload_on: "1"
///       payload_off: "0"
/// ```
///
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Switch {
    /// Replaces `~` with this value in any MQTT topic attribute.
    /// [See Home Assistant documentation](https://www.home-assistant.io/integrations/mqtt/#using-abbreviations-and-base-topic)
    #[serde(rename = "~", skip_serializing_if = "Option::is_none")]
    pub topic_prefix: Option<String>,

    /// It is encouraged to add additional information about the origin that supplies MQTT entities via MQTT discovery by adding the origin option (can be abbreviated to o) to the discovery payload. Note that these options also support abbreviations. Information of the origin will be logged to the core event log when an item is discovered or updated.
    #[serde(rename = "o")]
    pub origin: Origin,

    /// Information about the device this button is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/en/device_registry_index.html). Only works when [`unique_id`](#unique_id) is set. At least one of identifiers or connections must be present to identify the device.
    #[serde(rename = "dev")]
    pub device: DeviceInformation,

    /// A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`.
    #[serde(flatten)]
    pub availability: Availability,

    /// The category of the entity. (optional, default: None)
    #[serde(rename = "ent_cat", skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<EntityCategory>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `command_topic`. The switch command template accepts the parameters `value`. The `value` parameter will contain the configured value for either `payload_on` or `payload_off`.
    #[serde(rename = "cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub command_template: Option<String>,

    /// The MQTT topic to publish commands to change the switch state.
    #[serde(rename = "cmd_t")]
    pub command_topic: String,

    /// The [type/class](/integrations/switch/#device-class) of the switch to set the icon in the frontend. The `device_class` can be `null`.
    #[serde(rename = "dev_cla", skip_serializing_if = "Option::is_none")]
    pub device_class: Option<SwitchDeviceClass>,

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

    /// The name to use when displaying this switch. Can be set to `null` if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// Flag that defines if switch works in optimistic mode.
    #[serde(rename = "opt", skip_serializing_if = "Option::is_none")]
    pub optimistic: Option<bool>,

    /// The payload that represents `off` state. If specified, will be used for both comparing to the value in the `state_topic` (see `value_template` and `state_off` for details) and sending as `off` command to the `command_topic`.
    #[serde(rename = "pl_off", skip_serializing_if = "Option::is_none")]
    pub payload_off: Option<String>,

    /// The payload that represents `on` state. If specified, will be used for both comparing to the value in the `state_topic` (see `value_template` and `state_on`  for details) and sending as `on` command to the `command_topic`.
    #[serde(rename = "pl_on", skip_serializing_if = "Option::is_none")]
    pub payload_on: Option<String>,

    /// Must be `switch`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    #[serde(rename = "p")]
    pub platform: String,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// If the published message should have the retain flag on or not.
    #[serde(rename = "ret", skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,

    /// The payload that represents the `off` state. Used when value that represents `off` state in the `state_topic` is different from value that should be sent to the `command_topic` to turn the device `off`.
    #[serde(rename = "stat_off", skip_serializing_if = "Option::is_none")]
    pub state_off: Option<String>,

    /// The payload that represents the `on` state. Used when value that represents `on` state in the `state_topic` is different from value that should be sent to the `command_topic` to turn the device `on`.
    #[serde(rename = "stat_on", skip_serializing_if = "Option::is_none")]
    pub state_on: Option<String>,

    /// The MQTT topic subscribed to receive state updates. A "None" payload resets to an `unknown` state. An empty payload is ignored.By default, valid state payloads are `OFF` and `ON`. The accepted payloads can be overridden with the `payload_off` and `payload_on` config options.
    #[serde(rename = "stat_t", skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,

    /// An ID that uniquely identifies this switch device. If two switches have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract device's state from the `state_topic`. To determine the switches's state result of this template will be compared to `state_on` and `state_off`.
    #[serde(rename = "val_tpl", skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

impl Switch {
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
    pub fn device(mut self, device: DeviceInformation) -> Self {
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

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `command_topic`. The switch command template accepts the parameters `value`. The `value` parameter will contain the configured value for either `payload_on` or `payload_off`.
    pub fn command_template<T: Into<String>>(mut self, command_template: T) -> Self {
        self.command_template = Some(command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the switch state.
    pub fn command_topic<T: Into<String>>(mut self, command_topic: T) -> Self {
        self.command_topic = command_topic.into();
        self
    }

    /// The [type/class](/integrations/switch/#device-class) of the switch to set the icon in the frontend. The `device_class` can be `null`.
    pub fn device_class(mut self, device_class: SwitchDeviceClass) -> Self {
        self.device_class = Some(device_class);
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

    /// The name to use when displaying this switch. Can be set to `null` if only the device name is relevant.
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    /// Flag that defines if switch works in optimistic mode.
    pub fn optimistic(mut self, optimistic: bool) -> Self {
        self.optimistic = Some(optimistic);
        self
    }

    /// The payload that represents `off` state. If specified, will be used for both comparing to the value in the `state_topic` (see `value_template` and `state_off` for details) and sending as `off` command to the `command_topic`.
    pub fn payload_off<T: Into<String>>(mut self, payload_off: T) -> Self {
        self.payload_off = Some(payload_off.into());
        self
    }

    /// The payload that represents `on` state. If specified, will be used for both comparing to the value in the `state_topic` (see `value_template` and `state_on`  for details) and sending as `on` command to the `command_topic`.
    pub fn payload_on<T: Into<String>>(mut self, payload_on: T) -> Self {
        self.payload_on = Some(payload_on.into());
        self
    }

    /// Must be `switch`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
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

    /// The payload that represents the `off` state. Used when value that represents `off` state in the `state_topic` is different from value that should be sent to the `command_topic` to turn the device `off`.
    pub fn state_off<T: Into<String>>(mut self, state_off: T) -> Self {
        self.state_off = Some(state_off.into());
        self
    }

    /// The payload that represents the `on` state. Used when value that represents `on` state in the `state_topic` is different from value that should be sent to the `command_topic` to turn the device `on`.
    pub fn state_on<T: Into<String>>(mut self, state_on: T) -> Self {
        self.state_on = Some(state_on.into());
        self
    }

    /// The MQTT topic subscribed to receive state updates. A "None" payload resets to an `unknown` state. An empty payload is ignored.By default, valid state payloads are `OFF` and `ON`. The accepted payloads can be overridden with the `payload_off` and `payload_on` config options.
    pub fn state_topic<T: Into<String>>(mut self, state_topic: T) -> Self {
        self.state_topic = Some(state_topic.into());
        self
    }

    /// An ID that uniquely identifies this switch device. If two switches have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    pub fn unique_id<T: Into<String>>(mut self, unique_id: T) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract device's state from the `state_topic`. To determine the switches's state result of this template will be compared to `state_on` and `state_off`.
    pub fn value_template<T: Into<String>>(mut self, value_template: T) -> Self {
        self.value_template = Some(value_template.into());
        self
    }
}

impl Default for Switch {
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
            payload_off: Default::default(),
            payload_on: Default::default(),
            platform: "switch".to_string(),
            qos: Default::default(),
            retain: Default::default(),
            state_off: Default::default(),
            state_on: Default::default(),
            state_topic: Default::default(),
            unique_id: Default::default(),
            value_template: Default::default(),
        }
    }
}

impl From<Switch> for Entity {
    fn from(value: Switch) -> Self {
        Entity::Switch(value)
    }
}
/// ---
/// title: "MQTT Cover"
/// description: "Instructions on how to integrate MQTT covers into Home Assistant."
/// ha_category:
///   - Cover
/// ha_iot_class: Configurable
/// ha_release: 0.18
/// ha_domain: mqtt
/// ---
///
/// The `mqtt` cover platform allows you to control an MQTT cover (such as blinds, a roller shutter or a garage door).
///
/// ## Configuration
///
/// A cover entity can be in states (`open`, `opening`, `closed` or `closing`).
///
/// If a `state_topic` is configured, the entity's state will be updated only after an MQTT message is received on `state_topic` matching `state_open`, `state_opening`, `state_closed` or `state_closing`. For covers that only report 3 states (`opening`, `closing`, `stopped`), a `state_stopped` state can be configured to indicate that the device is not moving. When this payload is received on the `state_topic`, and a `position_topic` is not configured, the cover will be set to state `closed` if its state was `closing` and to state `open` otherwise. If a `position_topic` is set, the cover's position will be used to set the state to either `open` or `closed` state.
///
/// If the cover reports its position, a `position_topic` can be configured for receiving the position. If no `state_topic` is configured, the cover's state will be set to either `open` or `closed` when a position is received.
///
/// If the cover reports its tilt position, a `tilt_status_topic` can be configured for receiving the tilt position.
/// If position topic and state topic are both defined, the device state (`open`, `opening`, `closed` or `closing`) will be set by the state topic and the cover position will be set by the position topic.
///
/// If neither a state topic nor a position topic are defined, the cover will work in optimistic mode. In this mode, the cover will immediately change state (`open` or `closed`) after every command sent by Home Assistant. If a state topic/position topic is defined, the cover will wait for a message on `state_topic` or `position_topic`.
///
/// Optimistic mode can be forced, even if a `state_topic` / `position_topic` is defined. Try to enable it if experiencing incorrect cover operation (Google Assistant gauge may need optimistic mode as it often send request to your Home Assistant immediately after send set_cover_position in which case MQTT could be too slow).
///
/// The `mqtt` cover platform optionally supports a list of `availability` topics to receive online and offline messages (birth and LWT messages) from the MQTT cover device. During normal operation, if the MQTT cover device goes offline (i.e., publishes a matching `payload_not_available` to any `availability` topic), Home Assistant will display the cover as "unavailable". If these messages are published with the `retain` flag set, the cover will receive an instant update after subscription and Home Assistant will display correct availability state of the cover when Home Assistant starts up. If the `retain` flag is not set, Home Assistant will display the cover as "unavailable" when Home Assistant starts up.
///
/// To use an MQTT cover in your installation, [add a MQTT device as a subentry](/integrations/mqtt/#configuration), or add the following to your `configuration.yaml` file.
/// {% include integrations/restart_ha_after_config_inclusion.md %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - cover:
///       command_topic: "living-room-cover/set"
/// ```
///
/// Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
///
///
///
/// 🛈 Note\
/// MQTT cover expects position and tilt values to be in range of 0 to 100, where 0 indicates closed position and 100 indicates fully open position.
/// If position `min` or `max` are set to a different range (e.g. 40 to 140), when sending command to the device the range will be adjusted to the device range (position 0 will send a value of 40 to device) and when position payload is received from the device it will be adjusted back to the 0 to 100 range (device value of 40 will report cover position 0).
/// `min` and `max` can also be used to reverse the direction of the device, if `min` is set to 100 and `max` is set to `0` device operation will be inverted (e.g. when setting position to 40, a value of 60 will be sent to device).
///
/// ## Examples
///
/// In this section you will find some real-life examples of how to use this platform.
///
/// ### Full configuration state topic without tilt
///
/// The example below shows a full configuration for a cover without tilt with state topic only.
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - cover:
///       name: "MQTT Cover"
///       command_topic: "living-room-cover/set"
///       state_topic: "living-room-cover/state"
///       availability:
///         - topic: "living-room-cover/availability"
///       qos: 0
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
///       value_template: "{{ value.x }}"
/// ```
///
///
/// ### Full configuration position topic without tilt
///
/// The example below shows a full configuration for a cover without tilt with position topic.
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - cover:
///       name: "MQTT Cover"
///       command_topic: "living-room-cover/set"
///       position_topic: "living-room-cover/position"
///       availability:
///         - topic: "living-room-cover/availability"
///       set_position_topic: "living-room-cover/set_position"
///       qos: 0
///       retain: true
///       payload_open: "OPEN"
///       payload_close: "CLOSE"
///       payload_stop: "STOP"
///       position_open: 100
///       position_closed: 0
///       payload_available: "online"
///       payload_not_available: "offline"
///       optimistic: false
///       value_template: "{{ value.x }}"
/// ```
///
///
/// ### Full configuration for position, state and tilt
///
/// The example below shows a full configuration for a cover with position, state & tilt.
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - cover:
///       name: "MQTT Cover"
///       command_topic: "living-room-cover/set"
///       state_topic: "living-room-cover/state"
///       position_topic: "living-room-cover/position"
///       availability:
///         - topic: "living-room-cover/availability"
///       qos: 0
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
///       value_template: "{{ value.x }}"
///       position_template: "{{ value.y }}"
///       tilt_command_topic: "living-room-cover/tilt"
///       tilt_status_topic: "living-room-cover/tilt-state"
///       tilt_status_template: "{{ value_json["PWM"]["PWM1"] }}"
///       tilt_min: 0
///       tilt_max: 180
///       tilt_closed_value: 70
///       tilt_opened_value: 180
/// ```
///
///
/// ### Full configuration using stopped state
///
/// The example below shows a full configuration for a cover using stopped state.
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - cover:
///       name: "MQTT Cover"
///       command_topic: "living-room-cover/set"
///       state_topic: "living-room-cover/state"
///       position_topic: "living-room-cover/position"
///       availability:
///         - topic: "living-room-cover/availability"
///       qos: 0
///       retain: true
///       payload_open: "OPEN"
///       payload_close: "CLOSE"
///       payload_stop: "STOP"
///       state_opening: "opening"
///       state_closed: "closed"
///       state_stopped: "stopped"
///       payload_available: "online"
///       payload_not_available: "offline"
///       optimistic: false
///       value_template: "{{ value.x }}"
///       position_template: "{{ value.y }}"
/// ```
///
///
/// ### Configuration for disabling cover commands
///
/// The example below shows a configuration for a cover that does not have a close command.
/// Setting `payload_close` empty or to `null` disables the close command and will not show the close button.
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - cover:
///       payload_open: "on"
///       payload_close:
///       payload_stop: "on"
/// ```
///
/// The following commands can be disabled: `open`, `close`, `stop` by overriding their payloads: `payload_open`, `payload_close`, `payload_stop`
///
/// For auto discovery message the payload needs to be set to `null`, example for cover without close command:
///
/// ```json
/// {
///   "cover": [
///     {
///       "payload_open": "on",
///       "payload_close": null,
///       "payload_stop": "on"
///     }
///   ]
/// }
/// ```
///
///
/// ### Full configuration using `entity_id`- variable in the template
///
/// The example below shows an example of how to correct the state of the blind depending if it moved up, or down.
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - cover:
///       name: "MQTT Cover"
///       command_topic: "living-room-cover/set"
///       state_topic: "living-room-cover/state"
///       position_topic: "living-room-cover/position"
///       set_position_topic: "living-room-cover/position/set"
///       payload_open:  "open"
///       payload_close: "close"
///       payload_stop:  "stop"
///       state_opening: "open"
///       state_closing: "close"
///       state_stopped: "stop"
///       optimistic: false
///       position_template: |-
///         {% if not state_attr(entity_id, "current_position") %}
///           {{ value }}
///         {% elif state_attr(entity_id, "current_position") < (value | int) %}
///           {{ (value | int + 1) }}
///         {% elif state_attr(entity_id, "current_position") > (value | int) %}
///           {{ (value | int - 1) }}
///         {% else %}
///           {{ value }}
///         {% endif %}
/// ```
///
///
/// ### Full configuration using advanced templating
///
/// The `position_template` can accept JSON, where `position` and `tilt_position` is provided at the same time.
///
/// The example below shows a full example of how to set up a venetian blind which has a combined position and tilt topic. The blind in the example has moveable slats which tilt with a position change. In the example, it takes the blind 6% of the movement for a full rotation of the slats.
///
/// Following variable might be used in `position_template`, `set_position_template`, `tilt_command_template` and `tilt_status_template`, `json_attributes_template` (only `entity_id`).
///
/// - `entity_id` - The ID of the entity itself. It can be used to reference its attributes with the help of the [states](/docs/configuration/templating/#states) template function.
/// - `position_open`
/// - `position_closed`
/// - `tilt_min`
/// - `tilt_max`
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - cover:
///       name: "MQTT Cover"
///       command_topic: "living-room-cover/set"
///       state_topic: "living-room-cover/state"
///       position_topic: "living-room-cover/position"
///       set_position_topic: "living-room-cover/position/set"
///       tilt_command_topic: "living-room-cover/position/set" # same as `set_position_topic`
///       qos: 1
///       retain: false
///       payload_open:  "open"
///       payload_close: "close"
///       payload_stop:  "stop"
///       state_opening: "open"
///       state_closing: "close"
///       state_stopped: "stop"
///       position_open: 100
///       position_closed: 0
///       tilt_min: 0
///       tilt_max: 6
///       tilt_opened_value: 3
///       tilt_closed_value: 0
///       optimistic: false
///       position_template: |-
///         {% if not state_attr(entity_id, "current_position") %}
///           {
///             "position" : {{ value }},
///             "tilt_position" : 0
///           }
///         {% else %}
///           {% set old_position = state_attr(entity_id, "current_position") %}
///           {% set old_tilt_percent = (state_attr(entity_id, "current_tilt_position")) %}
///
///           {% set movement = value | int - old_position %}
///           {% set old_tilt_position = (old_tilt_percent / 100 * (tilt_max - tilt_min)) %}
///           {% set new_tilt_position = min(max((old_tilt_position + movement), tilt_min), tilt_max) %}
///   
///           {
///             "position": {{ value }},
///             "tilt_position": {{ new_tilt_position }}
///           }
///         {% endif %}
///     tilt_command_template: >-
///         {% set position = state_attr(entity_id, "current_position") %}
///         {% set tilt = state_attr(entity_id, "current_tilt_position") %}
///         {% set movement = (tilt_position - tilt) / 100 * tilt_max %}
///         {{ position + movement }}
///       payload_open: "on"
///       payload_close:
///       payload_stop: "on"
/// ```
///
///
/// ### Testing your configuration
///
/// To test, you can use the command line tool `mosquitto_pub` shipped with `mosquitto` or the `mosquitto-clients` package to send MQTT messages. This allows you to operate your cover manually:
///
/// ```bash
/// mosquitto_pub -h 127.0.0.1 -t living-room-cover/set -m "CLOSE"
/// ```
///
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Cover {
    /// Replaces `~` with this value in any MQTT topic attribute.
    /// [See Home Assistant documentation](https://www.home-assistant.io/integrations/mqtt/#using-abbreviations-and-base-topic)
    #[serde(rename = "~", skip_serializing_if = "Option::is_none")]
    pub topic_prefix: Option<String>,

    /// It is encouraged to add additional information about the origin that supplies MQTT entities via MQTT discovery by adding the origin option (can be abbreviated to o) to the discovery payload. Note that these options also support abbreviations. Information of the origin will be logged to the core event log when an item is discovered or updated.
    #[serde(rename = "o")]
    pub origin: Origin,

    /// Information about the device this button is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/en/device_registry_index.html). Only works when [`unique_id`](#unique_id) is set. At least one of identifiers or connections must be present to identify the device.
    #[serde(rename = "dev")]
    pub device: DeviceInformation,

    /// A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`.
    #[serde(flatten)]
    pub availability: Availability,

    /// The category of the entity. (optional, default: None)
    #[serde(rename = "ent_cat", skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<EntityCategory>,

    /// The MQTT topic to publish commands to control the cover.
    #[serde(rename = "cmd_t", skip_serializing_if = "Option::is_none")]
    pub command_topic: Option<String>,

    /// Sets the [class of the device](/integrations/cover/#device_class), changing the device state and icon that is displayed on the frontend. The `device_class` can be `null`.
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

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation.
    #[serde(rename = "json_attr_tpl", skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation.
    #[serde(rename = "json_attr_t", skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,

    /// The name of the cover. Can be set to `null` if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// Flag that defines if switch works in optimistic mode.
    #[serde(rename = "opt", skip_serializing_if = "Option::is_none")]
    pub optimistic: Option<bool>,

    /// The command payload that closes the cover.
    #[serde(rename = "pl_cls", skip_serializing_if = "Option::is_none")]
    pub payload_close: Option<String>,

    /// The command payload that opens the cover.
    #[serde(rename = "pl_open", skip_serializing_if = "Option::is_none")]
    pub payload_open: Option<String>,

    /// The command payload that stops the cover.
    #[serde(rename = "pl_stop", skip_serializing_if = "Option::is_none")]
    pub payload_stop: Option<String>,

    /// The command payload that stops the tilt.
    #[serde(rename = "pl_stop_tilt", skip_serializing_if = "Option::is_none")]
    pub payload_stop_tilt: Option<String>,

    /// Must be `cover`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    #[serde(rename = "p")]
    pub platform: String,

    /// Number which represents closed position.
    #[serde(rename = "pos_clsd", skip_serializing_if = "Option::is_none")]
    pub position_closed: Option<i32>,

    /// Number which represents open position.
    #[serde(rename = "pos_open", skip_serializing_if = "Option::is_none")]
    pub position_open: Option<i32>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) that can be used to extract the payload for the `position_topic` topic. Within the template the following variables are available: `entity_id`, `position_open`; `position_closed`; `tilt_min`; `tilt_max`. The `entity_id` can be used to reference the entity's attributes with help of the [states](/docs/configuration/templating/#states) template function;
    #[serde(rename = "pos_tpl", skip_serializing_if = "Option::is_none")]
    pub position_template: Option<String>,

    /// The MQTT topic subscribed to receive cover position messages.
    #[serde(rename = "pos_t", skip_serializing_if = "Option::is_none")]
    pub position_topic: Option<String>,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// Defines if published messages should have the retain flag set.
    #[serde(rename = "ret", skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to define the position to be sent to the `set_position_topic` topic. Incoming position value is available for use in the template `{% raw %}{{ position }}{% endraw %}`. Within the template the following variables are available: `entity_id`, `position`, the target position in percent; `position_open`; `position_closed`; `tilt_min`; `tilt_max`. The `entity_id` can be used to reference the entity's attributes with help of the [states](/docs/configuration/templating/#states) template function;
    #[serde(rename = "set_pos_tpl", skip_serializing_if = "Option::is_none")]
    pub set_position_template: Option<String>,

    /// The MQTT topic to publish position commands to. You need to set position_topic as well if you want to use position topic. Use template if position topic wants different values than within range `position_closed` - `position_open`. If template is not defined and `position_closed != 100` and `position_open != 0` then proper position value is calculated from percentage position.
    #[serde(rename = "set_pos_t", skip_serializing_if = "Option::is_none")]
    pub set_position_topic: Option<String>,

    /// The payload that represents the closed state.
    #[serde(rename = "stat_clsd", skip_serializing_if = "Option::is_none")]
    pub state_closed: Option<String>,

    /// The payload that represents the closing state.
    #[serde(rename = "stat_closing", skip_serializing_if = "Option::is_none")]
    pub state_closing: Option<String>,

    /// The payload that represents the open state.
    #[serde(rename = "stat_open", skip_serializing_if = "Option::is_none")]
    pub state_open: Option<String>,

    /// The payload that represents the opening state.
    #[serde(rename = "stat_opening", skip_serializing_if = "Option::is_none")]
    pub state_opening: Option<String>,

    /// The payload that represents the stopped state (for covers that do not report `open`/`closed` state).
    #[serde(rename = "stat_stopped", skip_serializing_if = "Option::is_none")]
    pub state_stopped: Option<String>,

    /// The MQTT topic subscribed to receive cover state messages. State topic can only read a (`open`, `opening`, `closed`, `closing` or `stopped`) state.  A "None" payload resets to an `unknown` state. An empty payload is ignored.
    #[serde(rename = "stat_t", skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,

    /// The value that will be sent on a `close_cover_tilt` command.
    #[serde(rename = "tilt_clsd_val", skip_serializing_if = "Option::is_none")]
    pub tilt_closed_value: Option<i32>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) that can be used to extract the payload for the `tilt_command_topic` topic. Within the template the following variables are available: `entity_id`, `tilt_position`, the target tilt position in percent; `position_open`; `position_closed`; `tilt_min`; `tilt_max`. The `entity_id` can be used to reference the entity's attributes with help of the [states](/docs/configuration/templating/#states) template function;
    #[serde(rename = "tilt_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub tilt_command_template: Option<String>,

    /// The MQTT topic to publish commands to control the cover tilt.
    #[serde(rename = "tilt_cmd_t", skip_serializing_if = "Option::is_none")]
    pub tilt_command_topic: Option<String>,

    /// The maximum tilt value.
    #[serde(rename = "tilt_max", skip_serializing_if = "Option::is_none")]
    pub tilt_max: Option<i32>,

    /// The minimum tilt value.
    #[serde(rename = "tilt_min", skip_serializing_if = "Option::is_none")]
    pub tilt_min: Option<i32>,

    /// The value that will be sent on an `open_cover_tilt` command.
    #[serde(rename = "tilt_opnd_val", skip_serializing_if = "Option::is_none")]
    pub tilt_opened_value: Option<i32>,

    /// Flag that determines if tilt works in optimistic mode.
    #[serde(rename = "tilt_opt", skip_serializing_if = "Option::is_none")]
    pub tilt_optimistic: Option<bool>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) that can be used to extract the payload for the `tilt_status_topic` topic. Within the template the following variables are available: `entity_id`, `position_open`; `position_closed`; `tilt_min`; `tilt_max`. The `entity_id` can be used to reference the entity's attributes with help of the [states](/docs/configuration/templating/#states) template function;
    #[serde(rename = "tilt_status_tpl", skip_serializing_if = "Option::is_none")]
    pub tilt_status_template: Option<String>,

    /// The MQTT topic subscribed to receive tilt status update values.
    #[serde(rename = "tilt_status_t", skip_serializing_if = "Option::is_none")]
    pub tilt_status_topic: Option<String>,

    /// An ID that uniquely identifies this cover. If two covers have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) that can be used to extract the payload for the `state_topic` topic.
    #[serde(rename = "val_tpl", skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

impl Cover {
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
    pub fn device(mut self, device: DeviceInformation) -> Self {
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

    /// The MQTT topic to publish commands to control the cover.
    pub fn command_topic<T: Into<String>>(mut self, command_topic: T) -> Self {
        self.command_topic = Some(command_topic.into());
        self
    }

    /// Sets the [class of the device](/integrations/cover/#device_class), changing the device state and icon that is displayed on the frontend. The `device_class` can be `null`.
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

    /// The name of the cover. Can be set to `null` if only the device name is relevant.
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    /// Flag that defines if switch works in optimistic mode.
    pub fn optimistic(mut self, optimistic: bool) -> Self {
        self.optimistic = Some(optimistic);
        self
    }

    /// The command payload that closes the cover.
    pub fn payload_close<T: Into<String>>(mut self, payload_close: T) -> Self {
        self.payload_close = Some(payload_close.into());
        self
    }

    /// The command payload that opens the cover.
    pub fn payload_open<T: Into<String>>(mut self, payload_open: T) -> Self {
        self.payload_open = Some(payload_open.into());
        self
    }

    /// The command payload that stops the cover.
    pub fn payload_stop<T: Into<String>>(mut self, payload_stop: T) -> Self {
        self.payload_stop = Some(payload_stop.into());
        self
    }

    /// The command payload that stops the tilt.
    pub fn payload_stop_tilt<T: Into<String>>(mut self, payload_stop_tilt: T) -> Self {
        self.payload_stop_tilt = Some(payload_stop_tilt.into());
        self
    }

    /// Must be `cover`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    pub fn platform<T: Into<String>>(mut self, platform: T) -> Self {
        self.platform = platform.into();
        self
    }

    /// Number which represents closed position.
    pub fn position_closed(mut self, position_closed: i32) -> Self {
        self.position_closed = Some(position_closed);
        self
    }

    /// Number which represents open position.
    pub fn position_open(mut self, position_open: i32) -> Self {
        self.position_open = Some(position_open);
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) that can be used to extract the payload for the `position_topic` topic. Within the template the following variables are available: `entity_id`, `position_open`; `position_closed`; `tilt_min`; `tilt_max`. The `entity_id` can be used to reference the entity's attributes with help of the [states](/docs/configuration/templating/#states) template function;
    pub fn position_template<T: Into<String>>(mut self, position_template: T) -> Self {
        self.position_template = Some(position_template.into());
        self
    }

    /// The MQTT topic subscribed to receive cover position messages.
    pub fn position_topic<T: Into<String>>(mut self, position_topic: T) -> Self {
        self.position_topic = Some(position_topic.into());
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

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to define the position to be sent to the `set_position_topic` topic. Incoming position value is available for use in the template `{% raw %}{{ position }}{% endraw %}`. Within the template the following variables are available: `entity_id`, `position`, the target position in percent; `position_open`; `position_closed`; `tilt_min`; `tilt_max`. The `entity_id` can be used to reference the entity's attributes with help of the [states](/docs/configuration/templating/#states) template function;
    pub fn set_position_template<T: Into<String>>(mut self, set_position_template: T) -> Self {
        self.set_position_template = Some(set_position_template.into());
        self
    }

    /// The MQTT topic to publish position commands to. You need to set position_topic as well if you want to use position topic. Use template if position topic wants different values than within range `position_closed` - `position_open`. If template is not defined and `position_closed != 100` and `position_open != 0` then proper position value is calculated from percentage position.
    pub fn set_position_topic<T: Into<String>>(mut self, set_position_topic: T) -> Self {
        self.set_position_topic = Some(set_position_topic.into());
        self
    }

    /// The payload that represents the closed state.
    pub fn state_closed<T: Into<String>>(mut self, state_closed: T) -> Self {
        self.state_closed = Some(state_closed.into());
        self
    }

    /// The payload that represents the closing state.
    pub fn state_closing<T: Into<String>>(mut self, state_closing: T) -> Self {
        self.state_closing = Some(state_closing.into());
        self
    }

    /// The payload that represents the open state.
    pub fn state_open<T: Into<String>>(mut self, state_open: T) -> Self {
        self.state_open = Some(state_open.into());
        self
    }

    /// The payload that represents the opening state.
    pub fn state_opening<T: Into<String>>(mut self, state_opening: T) -> Self {
        self.state_opening = Some(state_opening.into());
        self
    }

    /// The payload that represents the stopped state (for covers that do not report `open`/`closed` state).
    pub fn state_stopped<T: Into<String>>(mut self, state_stopped: T) -> Self {
        self.state_stopped = Some(state_stopped.into());
        self
    }

    /// The MQTT topic subscribed to receive cover state messages. State topic can only read a (`open`, `opening`, `closed`, `closing` or `stopped`) state.  A "None" payload resets to an `unknown` state. An empty payload is ignored.
    pub fn state_topic<T: Into<String>>(mut self, state_topic: T) -> Self {
        self.state_topic = Some(state_topic.into());
        self
    }

    /// The value that will be sent on a `close_cover_tilt` command.
    pub fn tilt_closed_value(mut self, tilt_closed_value: i32) -> Self {
        self.tilt_closed_value = Some(tilt_closed_value);
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) that can be used to extract the payload for the `tilt_command_topic` topic. Within the template the following variables are available: `entity_id`, `tilt_position`, the target tilt position in percent; `position_open`; `position_closed`; `tilt_min`; `tilt_max`. The `entity_id` can be used to reference the entity's attributes with help of the [states](/docs/configuration/templating/#states) template function;
    pub fn tilt_command_template<T: Into<String>>(mut self, tilt_command_template: T) -> Self {
        self.tilt_command_template = Some(tilt_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to control the cover tilt.
    pub fn tilt_command_topic<T: Into<String>>(mut self, tilt_command_topic: T) -> Self {
        self.tilt_command_topic = Some(tilt_command_topic.into());
        self
    }

    /// The maximum tilt value.
    pub fn tilt_max(mut self, tilt_max: i32) -> Self {
        self.tilt_max = Some(tilt_max);
        self
    }

    /// The minimum tilt value.
    pub fn tilt_min(mut self, tilt_min: i32) -> Self {
        self.tilt_min = Some(tilt_min);
        self
    }

    /// The value that will be sent on an `open_cover_tilt` command.
    pub fn tilt_opened_value(mut self, tilt_opened_value: i32) -> Self {
        self.tilt_opened_value = Some(tilt_opened_value);
        self
    }

    /// Flag that determines if tilt works in optimistic mode.
    pub fn tilt_optimistic(mut self, tilt_optimistic: bool) -> Self {
        self.tilt_optimistic = Some(tilt_optimistic);
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) that can be used to extract the payload for the `tilt_status_topic` topic. Within the template the following variables are available: `entity_id`, `position_open`; `position_closed`; `tilt_min`; `tilt_max`. The `entity_id` can be used to reference the entity's attributes with help of the [states](/docs/configuration/templating/#states) template function;
    pub fn tilt_status_template<T: Into<String>>(mut self, tilt_status_template: T) -> Self {
        self.tilt_status_template = Some(tilt_status_template.into());
        self
    }

    /// The MQTT topic subscribed to receive tilt status update values.
    pub fn tilt_status_topic<T: Into<String>>(mut self, tilt_status_topic: T) -> Self {
        self.tilt_status_topic = Some(tilt_status_topic.into());
        self
    }

    /// An ID that uniquely identifies this cover. If two covers have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    pub fn unique_id<T: Into<String>>(mut self, unique_id: T) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) that can be used to extract the payload for the `state_topic` topic.
    pub fn value_template<T: Into<String>>(mut self, value_template: T) -> Self {
        self.value_template = Some(value_template.into());
        self
    }
}

impl Default for Cover {
    fn default() -> Self {
        Self {
            topic_prefix: Default::default(),
            origin: Default::default(),
            device: Default::default(),
            entity_category: Default::default(),
            availability: Default::default(),
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
            payload_stop_tilt: Default::default(),
            platform: "cover".to_string(),
            position_closed: Default::default(),
            position_open: Default::default(),
            position_template: Default::default(),
            position_topic: Default::default(),
            qos: Default::default(),
            retain: Default::default(),
            set_position_template: Default::default(),
            set_position_topic: Default::default(),
            state_closed: Default::default(),
            state_closing: Default::default(),
            state_open: Default::default(),
            state_opening: Default::default(),
            state_stopped: Default::default(),
            state_topic: Default::default(),
            tilt_closed_value: Default::default(),
            tilt_command_template: Default::default(),
            tilt_command_topic: Default::default(),
            tilt_max: Default::default(),
            tilt_min: Default::default(),
            tilt_opened_value: Default::default(),
            tilt_optimistic: Default::default(),
            tilt_status_template: Default::default(),
            tilt_status_topic: Default::default(),
            unique_id: Default::default(),
            value_template: Default::default(),
        }
    }
}

impl From<Cover> for Entity {
    fn from(value: Cover) -> Self {
        Entity::Cover(value)
    }
}
/// ---
/// title: "MQTT Scene"
/// description: "Instructions on how to integrate MQTT scenes into Home Assistant."
/// ha_category:
///   - Scene
/// ha_release: 2020.12
/// ha_iot_class: Configurable
/// ha_domain: mqtt
/// ---
///
/// The `mqtt` scene platform lets you control your MQTT enabled scenes.
///
/// ## Configuration
///
/// To use an MQTT scene entity in your installation, add the following to your `configuration.yaml` file.
/// {% include integrations/restart_ha_after_config_inclusion.md %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - scene:
///       command_topic: zigbee2mqtt/living_room_group/set
/// ```
///
/// Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
///
///
/// ⚠ Important\
/// Make sure that your topic matches exactly. `some-topic/` and `some-topic` are different topics.
///
/// ## Examples
///
/// In this section, you will find some real-life examples of how to use the MQTT Scene.
///
/// ### Full configuration
///
/// The example below shows a full configuration for a scene.
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - scene:
///       unique_id: living_room_party_scene
///       name: "Party Scene"
///       command_topic: "home/living_room/party_scene/set"
///       availability:
///         - topic: "home/living_room/party_scene/available"
///       payload_on: "ON"
///       qos: 0
///       retain: true
///       device:
///         name: "Living Room"
///         identifiers: "livingroom_lights"
/// ```
///
/// ### Use with a JSON Payload
///
/// The example below shows a configuration using a JSON payload.
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - scene:
///       name: Living Room Blue Scene
///       unique_id: living_room_blue_scene
///       command_topic: "home/living_room/set"
///       payload_on: '{"activate_scene": "Blue Scene"}'
/// ```
///
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Scene {
    /// Replaces `~` with this value in any MQTT topic attribute.
    /// [See Home Assistant documentation](https://www.home-assistant.io/integrations/mqtt/#using-abbreviations-and-base-topic)
    #[serde(rename = "~", skip_serializing_if = "Option::is_none")]
    pub topic_prefix: Option<String>,

    /// It is encouraged to add additional information about the origin that supplies MQTT entities via MQTT discovery by adding the origin option (can be abbreviated to o) to the discovery payload. Note that these options also support abbreviations. Information of the origin will be logged to the core event log when an item is discovered or updated.
    #[serde(rename = "o")]
    pub origin: Origin,

    /// Information about the device this button is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/en/device_registry_index.html). Only works when [`unique_id`](#unique_id) is set. At least one of identifiers or connections must be present to identify the device.
    #[serde(rename = "dev")]
    pub device: DeviceInformation,

    /// A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`.
    #[serde(flatten)]
    pub availability: Availability,

    /// The category of the entity. (optional, default: None)
    #[serde(rename = "ent_cat", skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<EntityCategory>,

    /// The MQTT topic to publish `payload_on` to activate the scene.
    #[serde(rename = "cmd_t", skip_serializing_if = "Option::is_none")]
    pub command_topic: Option<String>,

    /// Flag which defines if the entity should be enabled when first added.
    #[serde(rename = "en", skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,

    /// The encoding of the published messages.
    #[serde(rename = "e", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,

    /// Picture URL for the entity.
    #[serde(rename = "ent_pic", skip_serializing_if = "Option::is_none")]
    pub entity_picture: Option<String>,

    /// Icon for the scene.
    #[serde(rename = "ic", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation.
    #[serde(rename = "json_attr_tpl", skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation.
    #[serde(rename = "json_attr_t", skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,

    /// The name to use when displaying this scene.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// The payload that will be sent to `command_topic` when activating the MQTT scene.
    #[serde(rename = "pl_on", skip_serializing_if = "Option::is_none")]
    pub payload_on: Option<String>,

    /// Must be `scene`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    #[serde(rename = "p")]
    pub platform: String,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// If the published message should have the retain flag on or not.
    #[serde(rename = "ret", skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,

    /// An ID that uniquely identifies this scene entity. If two scenes have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
}

impl Scene {
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
    pub fn device(mut self, device: DeviceInformation) -> Self {
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

    /// The MQTT topic to publish `payload_on` to activate the scene.
    pub fn command_topic<T: Into<String>>(mut self, command_topic: T) -> Self {
        self.command_topic = Some(command_topic.into());
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

    /// Icon for the scene.
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

    /// The name to use when displaying this scene.
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    /// The payload that will be sent to `command_topic` when activating the MQTT scene.
    pub fn payload_on<T: Into<String>>(mut self, payload_on: T) -> Self {
        self.payload_on = Some(payload_on.into());
        self
    }

    /// Must be `scene`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
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

    /// An ID that uniquely identifies this scene entity. If two scenes have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    pub fn unique_id<T: Into<String>>(mut self, unique_id: T) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            topic_prefix: Default::default(),
            origin: Default::default(),
            device: Default::default(),
            entity_category: Default::default(),
            availability: Default::default(),
            command_topic: Default::default(),
            enabled_by_default: Default::default(),
            encoding: Default::default(),
            entity_picture: Default::default(),
            icon: Default::default(),
            json_attributes_template: Default::default(),
            json_attributes_topic: Default::default(),
            name: Default::default(),
            object_id: Default::default(),
            payload_on: Default::default(),
            platform: "scene".to_string(),
            qos: Default::default(),
            retain: Default::default(),
            unique_id: Default::default(),
        }
    }
}

impl From<Scene> for Entity {
    fn from(value: Scene) -> Self {
        Entity::Scene(value)
    }
}
/// ---
/// title: "MQTT Lock"
/// description: "Instructions on how to integrate MQTT locks into Home Assistant."
/// ha_category:
///   - Lock
/// ha_release: 0.15
/// ha_iot_class: Configurable
/// ha_domain: mqtt
/// ---
///
/// The `mqtt` lock platform lets you control your MQTT enabled locks.
///
/// ## Configuration
///
/// In an ideal scenario, the MQTT device will have a `state_topic` to publish state changes. If these messages are published with a `RETAIN` flag, the MQTT lock will receive an instant state update after subscription and will start with correct state. Otherwise, the initial state of the lock will be `false` / unlocked.
///
/// When a `state_topic` is not available, the lock will work in optimistic mode. In this mode, the lock will immediately change state after every command. Otherwise, the lock will wait for state confirmation from the device (message from `state_topic`).
///
/// Optimistic mode can be forced, even if state topic is available. Try to enable it, if experiencing incorrect lock operation.
///
/// It's mandatory for locks to support `lock` and `unlock`. A lock may optionally support `open`, (e.g. to open the bolt in addition to the latch), in this case, `payload_open` is required in the configuration. If the lock is in optimistic mode, it will change states to `unlocked` when handling the `open` command.
///
/// An MQTT lock can also report the intermediate states `unlocking`, `locking` or `jammed` if the motor reports a jammed state.
///
/// To use an MQTT lock in your installation, add the following to your `configuration.yaml` file.
/// {% include integrations/restart_ha_after_config_inclusion.md %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - lock:
///       command_topic: "home/frontdoor/set"
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
/// In this section you will find some real-life examples of how to use this lock.
///
/// ### Full configuration
///
/// The example below shows a full configuration for a MQTT lock.
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - lock:
///       name: Frontdoor
///       state_topic: "home-assistant/frontdoor/state"
///       code_format: "^\\d{4}$"
///       command_topic: "home-assistant/frontdoor/set"
///       command_template: '{ "action": "{{ value }}", "code":"{{ code }}" }'
///       payload_lock: "LOCK"
///       payload_unlock: "UNLOCK"
///       state_locked: "LOCK"
///       state_unlocked: "UNLOCK"
///       state_locking: "LOCKING"
///       state_unlocking: "UNLOCKING"
///       state_jammed: "MOTOR_JAMMED"
///       state_ok: "MOTOR_OK"
///       optimistic: false
///       qos: 1
///       retain: true
///       value_template: "{{ value.x }}"
/// ```
///
///
/// Keep an eye on retaining messages to keep the state as you don't want to unlock your door by accident when you restart something.
///
/// For a check you can use the command line tools `mosquitto_pub` shipped with `mosquitto` to send MQTT messages. This allows you to operate your lock manually:
///
/// ```bash
/// mosquitto_pub -h 127.0.0.1 -t home-assistant/frontdoor/set -m "LOCK"
/// ```
///
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Lock {
    /// Replaces `~` with this value in any MQTT topic attribute.
    /// [See Home Assistant documentation](https://www.home-assistant.io/integrations/mqtt/#using-abbreviations-and-base-topic)
    #[serde(rename = "~", skip_serializing_if = "Option::is_none")]
    pub topic_prefix: Option<String>,

    /// It is encouraged to add additional information about the origin that supplies MQTT entities via MQTT discovery by adding the origin option (can be abbreviated to o) to the discovery payload. Note that these options also support abbreviations. Information of the origin will be logged to the core event log when an item is discovered or updated.
    #[serde(rename = "o")]
    pub origin: Origin,

    /// Information about the device this button is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/en/device_registry_index.html). Only works when [`unique_id`](#unique_id) is set. At least one of identifiers or connections must be present to identify the device.
    #[serde(rename = "dev")]
    pub device: DeviceInformation,

    /// A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`.
    #[serde(flatten)]
    pub availability: Availability,

    /// The category of the entity. (optional, default: None)
    #[serde(rename = "ent_cat", skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<EntityCategory>,

    /// A regular expression to validate a supplied code when it is set during the action to `open`, `lock` or `unlock` the MQTT lock.
    #[serde(rename = "cod_form", skip_serializing_if = "Option::is_none")]
    pub code_format: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `command_topic`. The lock command template accepts the parameters `value` and `code`. The `value` parameter will contain the configured value for either `payload_open`, `payload_lock` or `payload_unlock`. The `code` parameter is set during the action to `open`, `lock` or `unlock` the MQTT lock and will be set `None` if no code was passed.
    #[serde(rename = "cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub command_template: Option<String>,

    /// The MQTT topic to publish commands to change the lock state.
    #[serde(rename = "cmd_t")]
    pub command_topic: String,

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

    /// The name of the lock. Can be set to `null` if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// Flag that defines if lock works in optimistic mode.
    #[serde(rename = "opt", skip_serializing_if = "Option::is_none")]
    pub optimistic: Option<bool>,

    /// The payload sent to the lock to lock it.
    #[serde(rename = "pl_lock", skip_serializing_if = "Option::is_none")]
    pub payload_lock: Option<String>,

    /// The payload sent to the lock to open it.
    #[serde(rename = "pl_open", skip_serializing_if = "Option::is_none")]
    pub payload_open: Option<String>,

    /// A special payload that resets the state to `unknown` when received on the `state_topic`.
    #[serde(rename = "pl_rst", skip_serializing_if = "Option::is_none")]
    pub payload_reset: Option<String>,

    /// The payload sent to the lock to unlock it.
    #[serde(rename = "pl_unlk", skip_serializing_if = "Option::is_none")]
    pub payload_unlock: Option<String>,

    /// Must be `lock`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    #[serde(rename = "p")]
    pub platform: String,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// If the published message should have the retain flag on or not.
    #[serde(rename = "ret", skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,

    /// The payload sent to `state_topic` by the lock when it's jammed.
    #[serde(rename = "stat_jam", skip_serializing_if = "Option::is_none")]
    pub state_jammed: Option<String>,

    /// The payload sent to `state_topic` by the lock when it's locked.
    #[serde(rename = "stat_locked", skip_serializing_if = "Option::is_none")]
    pub state_locked: Option<String>,

    /// The payload sent to `state_topic` by the lock when it's locking.
    #[serde(rename = "stat_locking", skip_serializing_if = "Option::is_none")]
    pub state_locking: Option<String>,

    /// The MQTT topic subscribed to receive state updates. It accepts states configured with `state_jammed`, `state_locked`, `state_unlocked`, `state_locking` or `state_unlocking`. A "None" payload resets to an `unknown` state. An empty payload is ignored.
    #[serde(rename = "stat_t", skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,

    /// The payload sent to `state_topic` by the lock when it's unlocked.
    #[serde(rename = "stat_unlocked", skip_serializing_if = "Option::is_none")]
    pub state_unlocked: Option<String>,

    /// The payload sent to `state_topic` by the lock when it's unlocking.
    #[serde(rename = "stat_unlocking", skip_serializing_if = "Option::is_none")]
    pub state_unlocking: Option<String>,

    /// An ID that uniquely identifies this lock. If two locks have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract a state value from the payload.
    #[serde(rename = "val_tpl", skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

impl Lock {
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
    pub fn device(mut self, device: DeviceInformation) -> Self {
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

    /// A regular expression to validate a supplied code when it is set during the action to `open`, `lock` or `unlock` the MQTT lock.
    pub fn code_format<T: Into<String>>(mut self, code_format: T) -> Self {
        self.code_format = Some(code_format.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `command_topic`. The lock command template accepts the parameters `value` and `code`. The `value` parameter will contain the configured value for either `payload_open`, `payload_lock` or `payload_unlock`. The `code` parameter is set during the action to `open`, `lock` or `unlock` the MQTT lock and will be set `None` if no code was passed.
    pub fn command_template<T: Into<String>>(mut self, command_template: T) -> Self {
        self.command_template = Some(command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the lock state.
    pub fn command_topic<T: Into<String>>(mut self, command_topic: T) -> Self {
        self.command_topic = command_topic.into();
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

    /// The name of the lock. Can be set to `null` if only the device name is relevant.
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    /// Flag that defines if lock works in optimistic mode.
    pub fn optimistic(mut self, optimistic: bool) -> Self {
        self.optimistic = Some(optimistic);
        self
    }

    /// The payload sent to the lock to lock it.
    pub fn payload_lock<T: Into<String>>(mut self, payload_lock: T) -> Self {
        self.payload_lock = Some(payload_lock.into());
        self
    }

    /// The payload sent to the lock to open it.
    pub fn payload_open<T: Into<String>>(mut self, payload_open: T) -> Self {
        self.payload_open = Some(payload_open.into());
        self
    }

    /// A special payload that resets the state to `unknown` when received on the `state_topic`.
    pub fn payload_reset<T: Into<String>>(mut self, payload_reset: T) -> Self {
        self.payload_reset = Some(payload_reset.into());
        self
    }

    /// The payload sent to the lock to unlock it.
    pub fn payload_unlock<T: Into<String>>(mut self, payload_unlock: T) -> Self {
        self.payload_unlock = Some(payload_unlock.into());
        self
    }

    /// Must be `lock`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
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

    /// The payload sent to `state_topic` by the lock when it's jammed.
    pub fn state_jammed<T: Into<String>>(mut self, state_jammed: T) -> Self {
        self.state_jammed = Some(state_jammed.into());
        self
    }

    /// The payload sent to `state_topic` by the lock when it's locked.
    pub fn state_locked<T: Into<String>>(mut self, state_locked: T) -> Self {
        self.state_locked = Some(state_locked.into());
        self
    }

    /// The payload sent to `state_topic` by the lock when it's locking.
    pub fn state_locking<T: Into<String>>(mut self, state_locking: T) -> Self {
        self.state_locking = Some(state_locking.into());
        self
    }

    /// The MQTT topic subscribed to receive state updates. It accepts states configured with `state_jammed`, `state_locked`, `state_unlocked`, `state_locking` or `state_unlocking`. A "None" payload resets to an `unknown` state. An empty payload is ignored.
    pub fn state_topic<T: Into<String>>(mut self, state_topic: T) -> Self {
        self.state_topic = Some(state_topic.into());
        self
    }

    /// The payload sent to `state_topic` by the lock when it's unlocked.
    pub fn state_unlocked<T: Into<String>>(mut self, state_unlocked: T) -> Self {
        self.state_unlocked = Some(state_unlocked.into());
        self
    }

    /// The payload sent to `state_topic` by the lock when it's unlocking.
    pub fn state_unlocking<T: Into<String>>(mut self, state_unlocking: T) -> Self {
        self.state_unlocking = Some(state_unlocking.into());
        self
    }

    /// An ID that uniquely identifies this lock. If two locks have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    pub fn unique_id<T: Into<String>>(mut self, unique_id: T) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract a state value from the payload.
    pub fn value_template<T: Into<String>>(mut self, value_template: T) -> Self {
        self.value_template = Some(value_template.into());
        self
    }
}

impl Default for Lock {
    fn default() -> Self {
        Self {
            topic_prefix: Default::default(),
            origin: Default::default(),
            device: Default::default(),
            entity_category: Default::default(),
            availability: Default::default(),
            code_format: Default::default(),
            command_template: Default::default(),
            command_topic: Default::default(),
            enabled_by_default: Default::default(),
            encoding: Default::default(),
            entity_picture: Default::default(),
            icon: Default::default(),
            json_attributes_template: Default::default(),
            json_attributes_topic: Default::default(),
            name: Default::default(),
            object_id: Default::default(),
            optimistic: Default::default(),
            payload_lock: Default::default(),
            payload_open: Default::default(),
            payload_reset: Default::default(),
            payload_unlock: Default::default(),
            platform: "lock".to_string(),
            qos: Default::default(),
            retain: Default::default(),
            state_jammed: Default::default(),
            state_locked: Default::default(),
            state_locking: Default::default(),
            state_topic: Default::default(),
            state_unlocked: Default::default(),
            state_unlocking: Default::default(),
            unique_id: Default::default(),
            value_template: Default::default(),
        }
    }
}

impl From<Lock> for Entity {
    fn from(value: Lock) -> Self {
        Entity::Lock(value)
    }
}
/// ---
/// title: "MQTT Text"
/// description: "Instructions on how to interact with a device exposing text capability through MQTT from within Home Assistant."
/// ha_category:
///   - Text
/// ha_release: "2022.12"
/// ha_iot_class: Configurable
/// ha_domain: mqtt
/// ---
///
/// The `mqtt` Text platform allows you to integrate devices that show text that can be set remotely. Optionally the text state can be monitored too using MQTT.
///
/// ## Configuration
///
/// To use an MQTT text entity in your installation, add the following to your `configuration.yaml` file.
/// {% include integrations/restart_ha_after_config_inclusion.md %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - text:
///       command_topic: command-topic
/// ```
///
/// Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
///
///
/// ⚠ Important\
/// Make sure that your topic matches exactly. `some-topic/` and `some-topic` are different topics.
///
/// ## Examples
///
/// This is an example of a manual configured MQTT `text` item.
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - text:
///       name: "Remote LCD screen"
///       icon: mdi:ab-testing
///       mode: "text"
///       command_topic: "txt/cmd"
///       state_topic: "txt/state"
///       min: 2
///       max: 20
/// ```
///
///
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Text {
    /// Replaces `~` with this value in any MQTT topic attribute.
    /// [See Home Assistant documentation](https://www.home-assistant.io/integrations/mqtt/#using-abbreviations-and-base-topic)
    #[serde(rename = "~", skip_serializing_if = "Option::is_none")]
    pub topic_prefix: Option<String>,

    /// It is encouraged to add additional information about the origin that supplies MQTT entities via MQTT discovery by adding the origin option (can be abbreviated to o) to the discovery payload. Note that these options also support abbreviations. Information of the origin will be logged to the core event log when an item is discovered or updated.
    #[serde(rename = "o")]
    pub origin: Origin,

    /// Information about the device this button is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/en/device_registry_index.html). Only works when [`unique_id`](#unique_id) is set. At least one of identifiers or connections must be present to identify the device.
    #[serde(rename = "dev")]
    pub device: DeviceInformation,

    /// A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`.
    #[serde(flatten)]
    pub availability: Availability,

    /// The category of the entity. (optional, default: None)
    #[serde(rename = "ent_cat", skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<EntityCategory>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `command_topic`.
    #[serde(rename = "cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub command_template: Option<String>,

    /// The MQTT topic to publish the text value that is set.
    #[serde(rename = "cmd_t")]
    pub command_topic: String,

    /// Flag which defines if the entity should be enabled when first added.
    #[serde(rename = "en", skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,

    /// The encoding of the payloads received and published messages. Set to `""` to disable decoding of incoming payload.
    #[serde(rename = "e", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,

    /// Picture URL for the entity.
    #[serde(rename = "ent_pic", skip_serializing_if = "Option::is_none")]
    pub entity_picture: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the JSON dictionary from messages received on the `json_attributes_topic`.
    #[serde(rename = "json_attr_tpl", skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as entity attributes. Implies `force_update` of the current select state when a message is received on this topic.
    #[serde(rename = "json_attr_t", skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,

    /// The maximum size of a text being set or received (maximum is 255).
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,

    /// The minimum size of a text being set or received.
    #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,

    /// The mode off the text entity. Must be either `text` or `password`.
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    /// The name of the text entity. Can be set to `null` if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// A valid regular expression the text being set or received must match with.
    #[serde(rename = "ptrn", skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,

    /// Must be `text`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    #[serde(rename = "p")]
    pub platform: String,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// If the published message should have the retain flag on or not.
    #[serde(rename = "ret", skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,

    /// The MQTT topic subscribed to receive text state updates. Text state updates should match the `pattern` (if set) and meet the size constraints `min` and `max`. Can be used with `value_template` to render the incoming payload to a text update.
    #[serde(rename = "stat_t", skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,

    /// An ID that uniquely identifies this Select. If two Selects have the same unique ID Home Assistant will raise an exception. Required when used with device-based discovery.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the text state value from the payload received on `state_topic`.
    #[serde(rename = "val_tpl", skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

impl Text {
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
    pub fn device(mut self, device: DeviceInformation) -> Self {
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

    /// The MQTT topic to publish the text value that is set.
    pub fn command_topic<T: Into<String>>(mut self, command_topic: T) -> Self {
        self.command_topic = command_topic.into();
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

    /// The maximum size of a text being set or received (maximum is 255).
    pub fn max(mut self, max: i32) -> Self {
        self.max = Some(max);
        self
    }

    /// The minimum size of a text being set or received.
    pub fn min(mut self, min: i32) -> Self {
        self.min = Some(min);
        self
    }

    /// The mode off the text entity. Must be either `text` or `password`.
    pub fn mode<T: Into<String>>(mut self, mode: T) -> Self {
        self.mode = Some(mode.into());
        self
    }

    /// The name of the text entity. Can be set to `null` if only the device name is relevant.
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    /// A valid regular expression the text being set or received must match with.
    pub fn pattern<T: Into<String>>(mut self, pattern: T) -> Self {
        self.pattern = Some(pattern.into());
        self
    }

    /// Must be `text`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
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

    /// The MQTT topic subscribed to receive text state updates. Text state updates should match the `pattern` (if set) and meet the size constraints `min` and `max`. Can be used with `value_template` to render the incoming payload to a text update.
    pub fn state_topic<T: Into<String>>(mut self, state_topic: T) -> Self {
        self.state_topic = Some(state_topic.into());
        self
    }

    /// An ID that uniquely identifies this Select. If two Selects have the same unique ID Home Assistant will raise an exception. Required when used with device-based discovery.
    pub fn unique_id<T: Into<String>>(mut self, unique_id: T) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the text state value from the payload received on `state_topic`.
    pub fn value_template<T: Into<String>>(mut self, value_template: T) -> Self {
        self.value_template = Some(value_template.into());
        self
    }
}

impl Default for Text {
    fn default() -> Self {
        Self {
            topic_prefix: Default::default(),
            origin: Default::default(),
            device: Default::default(),
            entity_category: Default::default(),
            availability: Default::default(),
            command_template: Default::default(),
            command_topic: Default::default(),
            enabled_by_default: Default::default(),
            encoding: Default::default(),
            entity_picture: Default::default(),
            json_attributes_template: Default::default(),
            json_attributes_topic: Default::default(),
            max: Default::default(),
            min: Default::default(),
            mode: Default::default(),
            name: Default::default(),
            object_id: Default::default(),
            pattern: Default::default(),
            platform: "text".to_string(),
            qos: Default::default(),
            retain: Default::default(),
            state_topic: Default::default(),
            unique_id: Default::default(),
            value_template: Default::default(),
        }
    }
}

impl From<Text> for Entity {
    fn from(value: Text) -> Self {
        Entity::Text(value)
    }
}
/// ---
/// title: "MQTT notify"
/// description: "Instructions on how to integrate MQTT notify entities into Home Assistant."
/// ha_category:
///   - Notifications
/// ha_release: 2024.5
/// ha_iot_class: Configurable
/// ha_domain: mqtt
/// ---
///
/// The **MQTT notify** platform lets you send an MQTT message when the `send_message` action is called. This can be used to expose a action of a remote device that allows processing a message, such as showing it on a screen.
///
/// ## Configuration
///
/// To use an MQTT notify entity in your installation, [add a MQTT device as a subentry](/integrations/mqtt/#configuration), or add the following to your `configuration.yaml` file.
/// {% include integrations/restart_ha_after_config_inclusion.md %}
///
/// To use an MQTT notify entity in your installation, add the following to your `configuration.yaml` file.
/// {% include integrations/restart_ha_after_config_inclusion.md %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - notify:
///       command_topic: "home/living_room/status_screen/notifications"
/// ```
///
/// Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
///
///
/// ⚠ Important\
/// Make sure that your topic matches exactly. `some-topic/` and `some-topic` are different topics.
///
/// ## Examples
///
/// In this section, you will find some real-life examples of how to use this feature.
///
/// ### Full configuration
///
/// The example below shows a full configuration for a notify entity.
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - notify:
///       unique_id: living_room_stat_scr01
///       name: "Living room status screen"
///       command_topic: "home/living_room/status_screen/notifications"
///       availability:
///         - topic: "home/living_room/status_screen/available"
///       qos: 0
///       retain: false
/// ```
///
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Notify {
    /// Replaces `~` with this value in any MQTT topic attribute.
    /// [See Home Assistant documentation](https://www.home-assistant.io/integrations/mqtt/#using-abbreviations-and-base-topic)
    #[serde(rename = "~", skip_serializing_if = "Option::is_none")]
    pub topic_prefix: Option<String>,

    /// It is encouraged to add additional information about the origin that supplies MQTT entities via MQTT discovery by adding the origin option (can be abbreviated to o) to the discovery payload. Note that these options also support abbreviations. Information of the origin will be logged to the core event log when an item is discovered or updated.
    #[serde(rename = "o")]
    pub origin: Origin,

    /// Information about the device this button is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/en/device_registry_index.html). Only works when [`unique_id`](#unique_id) is set. At least one of identifiers or connections must be present to identify the device.
    #[serde(rename = "dev")]
    pub device: DeviceInformation,

    /// A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`.
    #[serde(flatten)]
    pub availability: Availability,

    /// The category of the entity. (optional, default: None)
    #[serde(rename = "ent_cat", skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<EntityCategory>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `command_topic`.
    #[serde(rename = "cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub command_template: Option<String>,

    /// The MQTT topic to publish send message commands at.
    #[serde(rename = "cmd_t", skip_serializing_if = "Option::is_none")]
    pub command_topic: Option<String>,

    /// Flag which defines if the entity should be enabled when first added.
    #[serde(rename = "en", skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,

    /// The encoding of the published messages.
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

    /// The name to use when displaying this notify entity. Can be set to `null` if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// If the published message should have the retain flag on or not.
    #[serde(rename = "ret", skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,

    /// An ID that uniquely identifies this notify entity. If two notify entities have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
}

impl Notify {
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
    pub fn device(mut self, device: DeviceInformation) -> Self {
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

    /// The MQTT topic to publish send message commands at.
    pub fn command_topic<T: Into<String>>(mut self, command_topic: T) -> Self {
        self.command_topic = Some(command_topic.into());
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

    /// The name to use when displaying this notify entity. Can be set to `null` if only the device name is relevant.
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
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

    /// An ID that uniquely identifies this notify entity. If two notify entities have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    pub fn unique_id<T: Into<String>>(mut self, unique_id: T) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }
}

impl Default for Notify {
    fn default() -> Self {
        Self {
            topic_prefix: Default::default(),
            origin: Default::default(),
            device: Default::default(),
            entity_category: Default::default(),
            availability: Default::default(),
            command_template: Default::default(),
            command_topic: Default::default(),
            enabled_by_default: Default::default(),
            encoding: Default::default(),
            entity_picture: Default::default(),
            icon: Default::default(),
            json_attributes_template: Default::default(),
            json_attributes_topic: Default::default(),
            name: Default::default(),
            object_id: Default::default(),
            qos: Default::default(),
            retain: Default::default(),
            unique_id: Default::default(),
        }
    }
}

impl From<Notify> for Entity {
    fn from(value: Notify) -> Self {
        Entity::Notify(value)
    }
}
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
    pub device: DeviceInformation,

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
    pub fn device(mut self, device: DeviceInformation) -> Self {
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
/// ---
/// title: "MQTT Siren"
/// description: "Instructions on how to integrate MQTT sirens into Home Assistant."
/// ha_category:
///   - Siren
/// ha_release: 2022.3
/// ha_iot_class: Configurable
/// ha_domain: mqtt
/// ---
///
/// The `mqtt` siren platform lets you control your MQTT enabled sirens and text based notification devices.
///
/// ## Configuration
///
/// In an ideal scenario, the MQTT device will have a `state_topic` to publish state changes. If these messages are published with a `RETAIN` flag, the MQTT siren will receive an instant state update after subscription, and will start with the correct state. Otherwise, the initial state of the siren will be `false` / `off`.
///
/// When a `state_topic` is not available, the siren will work in optimistic mode. In this mode, the siren will immediately change state after every command. Otherwise, the siren will wait for state confirmation from the device (message from `state_topic`).
///
/// Optimistic mode can be forced, even if the `state_topic` is available. Try to enable it, if experiencing incorrect operation.
///
/// To use an MQTT siren in your installation, add the following to your `configuration.yaml` file.
/// {% include integrations/restart_ha_after_config_inclusion.md %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - siren:
///       command_topic: "home/bedroom/siren/set"
/// ```
///
/// Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
///
///
/// ⚠ Important\
/// Make sure that your topic matches exactly. `some-topic/` and `some-topic` are different topics.
///
/// ## Examples
///
/// In this section, you will find an example of how to use this siren platform.
///
/// ### Full configuration
///
/// The example below shows a full configuration for a siren.
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - siren:
///       unique_id: custom_siren
///       name: "Intrusion siren"
///       state_topic: "home/alarm/siren1"
///       command_topic: "home/alarm/siren1/set"
///       available_tones:
///         - ping
///         - siren
///       availability:
///         - topic: "home/alarm/siren1/available"
///       payload_on: "ON"
///       payload_off: "OFF"
///       state_on: "ON"
///       state_off: "OFF"
///       optimistic: false
///       qos: 0
///       retain: true
/// ```
///
///
/// ### On/Off only siren controlling a Tasmota relay
///
/// The example below shows a configuration for an On/Off type siren, which does not accept JSON commands.
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - siren:
///       unique_id: tasmota_siren
///       name: "garage"
///       state_topic: "stat/SIREN/RESULT"
///       command_topic: "cmnd/SIREN/POWER"
///       availability_topic: "tele/SIREN/LWT"
///       command_template: "{{ value }}"
///       state_value_template: "{{ value_json.POWER }}"
///       payload_on: "ON"
///       payload_off: "OFF"
///       payload_available: "Online"
///       payload_not_available: "Offline"
/// ```
///
///
/// For a check, you can use the command line tools `mosquitto_pub` shipped with `mosquitto` to send MQTT messages. This allows you to operate your siren manually:
///
/// ```bash
/// mosquitto_pub -h 127.0.0.1 -t home/alarm/siren1 -m "ON"
/// ```
///
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Siren {
    /// Replaces `~` with this value in any MQTT topic attribute.
    /// [See Home Assistant documentation](https://www.home-assistant.io/integrations/mqtt/#using-abbreviations-and-base-topic)
    #[serde(rename = "~", skip_serializing_if = "Option::is_none")]
    pub topic_prefix: Option<String>,

    /// It is encouraged to add additional information about the origin that supplies MQTT entities via MQTT discovery by adding the origin option (can be abbreviated to o) to the discovery payload. Note that these options also support abbreviations. Information of the origin will be logged to the core event log when an item is discovered or updated.
    #[serde(rename = "o")]
    pub origin: Origin,

    /// Information about the device this button is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/en/device_registry_index.html). Only works when [`unique_id`](#unique_id) is set. At least one of identifiers or connections must be present to identify the device.
    #[serde(rename = "dev")]
    pub device: DeviceInformation,

    /// A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`.
    #[serde(flatten)]
    pub availability: Availability,

    /// The category of the entity. (optional, default: None)
    #[serde(rename = "ent_cat", skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<EntityCategory>,

    /// A list of available tones the siren supports. When configured, this enables the support for setting a `tone` and enables the `tone` state attribute.
    #[serde(rename = "av_tones", skip_serializing_if = "Option::is_none")]
    pub available_tones: Option<Vec<String>>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate a custom payload to send to `command_topic` when the siren turn off action is called. By default `command_template` will be used as template for action turn off. The variable `value` will be assigned with the configured `payload_off` setting.
    #[serde(rename = "cmd_off_tpl", skip_serializing_if = "Option::is_none")]
    pub command_off_template: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate a custom payload to send to `command_topic`. The variable `value` will be assigned with the configured `payload_on` or `payload_off` setting. The siren turn on action parameters `tone`, `volume_level` or `duration` can be used as variables in the template. When operation in optimistic mode the corresponding state attributes will be set. Turn on parameters will be filtered if a device misses the support.
    #[serde(rename = "cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub command_template: Option<String>,

    /// The MQTT topic to publish commands to change the siren state. Without command templates, a default JSON payload like `{"state":"ON", "tone": "bell", "duration": 10, "volume_level": 0.5 }` is published. When the siren turn on action is performed, the startup parameters will be added to the JSON payload. The `state` value of the JSON payload will be set to the the `payload_on` or `payload_off` configured payload.
    ///
    #[serde(rename = "cmd_t", skip_serializing_if = "Option::is_none")]
    pub command_topic: Option<String>,

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

    /// The name to use when displaying this siren. Can be set to `null` if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// Flag that defines if siren works in optimistic mode.
    #[serde(rename = "opt", skip_serializing_if = "Option::is_none")]
    pub optimistic: Option<bool>,

    /// The payload that represents `off` state. If specified, will be used for both comparing to the value in the `state_topic` (see `value_template` and `state_off` for details) and sending as `off` command to the `command_topic`.
    #[serde(rename = "pl_off", skip_serializing_if = "Option::is_none")]
    pub payload_off: Option<String>,

    /// The payload that represents `on` state. If specified, will be used for both comparing to the value in the `state_topic` (see `value_template` and `state_on`  for details) and sending as `on` command to the `command_topic`.
    #[serde(rename = "pl_on", skip_serializing_if = "Option::is_none")]
    pub payload_on: Option<String>,

    /// Must be `siren`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    #[serde(rename = "p")]
    pub platform: String,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// If the published message should have the retain flag on or not.
    #[serde(rename = "ret", skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,

    /// The payload that represents the `off` state. Used when value that represents `off` state in the `state_topic` is different from value that should be sent to the `command_topic` to turn the device `off`.
    #[serde(rename = "stat_off", skip_serializing_if = "Option::is_none")]
    pub state_off: Option<String>,

    /// The payload that represents the `on` state. Used when value that represents `on` state in the `state_topic` is different from value that should be sent to the `command_topic` to turn the device `on`.
    #[serde(rename = "stat_on", skip_serializing_if = "Option::is_none")]
    pub state_on: Option<String>,

    /// The MQTT topic subscribed to receive state updates. The state update may be either JSON or a simple string. When a JSON payload is detected, the `state` value of the JSON payload should supply the `payload_on` or `payload_off` defined payload to turn the siren on or off. Additionally, the state attributes `duration`, `tone` and `volume_level` can be updated. Use `value_template` to transform the received state update to a compliant JSON payload. Attributes will only be set if the function is supported by the device and a valid value is supplied. When a non JSON payload is detected, it should be either of the `payload_on` or `payload_off` defined payloads or `None` to reset the siren's state to `unknown`. The initial state will be `unknown`. The state will be reset to `unknown` if a `None` payload or `null` JSON value is received as a state update.
    #[serde(rename = "stat_t", skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract device's state from the `state_topic`. To determine the siren's state result of this template will be compared to `state_on` and `state_off`. Alternatively `value_template` can be used to render to a valid JSON payload.
    #[serde(rename = "stat_val_tpl", skip_serializing_if = "Option::is_none")]
    pub state_value_template: Option<String>,

    /// Set to `true` if the MQTT siren supports the `duration` turn on action parameter and enables the `duration` state attribute.
    #[serde(rename = "sup_dur", skip_serializing_if = "Option::is_none")]
    pub support_duration: Option<bool>,

    /// Set to `true` if the MQTT siren supports the `volume_set` turn on action parameter and enables the `volume_level` state attribute.
    #[serde(rename = "sup_vol", skip_serializing_if = "Option::is_none")]
    pub support_volume_set: Option<bool>,

    /// An ID that uniquely identifies this siren device. If two sirens have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
}

impl Siren {
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
    pub fn device(mut self, device: DeviceInformation) -> Self {
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

    /// A list of available tones the siren supports. When configured, this enables the support for setting a `tone` and enables the `tone` state attribute.
    pub fn available_tones<T: Into<String>>(mut self, available_tones: Vec<T>) -> Self {
        self.available_tones = Some(available_tones.into_iter().map(|v| v.into()).collect());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate a custom payload to send to `command_topic` when the siren turn off action is called. By default `command_template` will be used as template for action turn off. The variable `value` will be assigned with the configured `payload_off` setting.
    pub fn command_off_template<T: Into<String>>(mut self, command_off_template: T) -> Self {
        self.command_off_template = Some(command_off_template.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate a custom payload to send to `command_topic`. The variable `value` will be assigned with the configured `payload_on` or `payload_off` setting. The siren turn on action parameters `tone`, `volume_level` or `duration` can be used as variables in the template. When operation in optimistic mode the corresponding state attributes will be set. Turn on parameters will be filtered if a device misses the support.
    pub fn command_template<T: Into<String>>(mut self, command_template: T) -> Self {
        self.command_template = Some(command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the siren state. Without command templates, a default JSON payload like `{"state":"ON", "tone": "bell", "duration": 10, "volume_level": 0.5 }` is published. When the siren turn on action is performed, the startup parameters will be added to the JSON payload. The `state` value of the JSON payload will be set to the the `payload_on` or `payload_off` configured payload.
    ///
    pub fn command_topic<T: Into<String>>(mut self, command_topic: T) -> Self {
        self.command_topic = Some(command_topic.into());
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

    /// The name to use when displaying this siren. Can be set to `null` if only the device name is relevant.
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    /// Flag that defines if siren works in optimistic mode.
    pub fn optimistic(mut self, optimistic: bool) -> Self {
        self.optimistic = Some(optimistic);
        self
    }

    /// The payload that represents `off` state. If specified, will be used for both comparing to the value in the `state_topic` (see `value_template` and `state_off` for details) and sending as `off` command to the `command_topic`.
    pub fn payload_off<T: Into<String>>(mut self, payload_off: T) -> Self {
        self.payload_off = Some(payload_off.into());
        self
    }

    /// The payload that represents `on` state. If specified, will be used for both comparing to the value in the `state_topic` (see `value_template` and `state_on`  for details) and sending as `on` command to the `command_topic`.
    pub fn payload_on<T: Into<String>>(mut self, payload_on: T) -> Self {
        self.payload_on = Some(payload_on.into());
        self
    }

    /// Must be `siren`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
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

    /// The payload that represents the `off` state. Used when value that represents `off` state in the `state_topic` is different from value that should be sent to the `command_topic` to turn the device `off`.
    pub fn state_off<T: Into<String>>(mut self, state_off: T) -> Self {
        self.state_off = Some(state_off.into());
        self
    }

    /// The payload that represents the `on` state. Used when value that represents `on` state in the `state_topic` is different from value that should be sent to the `command_topic` to turn the device `on`.
    pub fn state_on<T: Into<String>>(mut self, state_on: T) -> Self {
        self.state_on = Some(state_on.into());
        self
    }

    /// The MQTT topic subscribed to receive state updates. The state update may be either JSON or a simple string. When a JSON payload is detected, the `state` value of the JSON payload should supply the `payload_on` or `payload_off` defined payload to turn the siren on or off. Additionally, the state attributes `duration`, `tone` and `volume_level` can be updated. Use `value_template` to transform the received state update to a compliant JSON payload. Attributes will only be set if the function is supported by the device and a valid value is supplied. When a non JSON payload is detected, it should be either of the `payload_on` or `payload_off` defined payloads or `None` to reset the siren's state to `unknown`. The initial state will be `unknown`. The state will be reset to `unknown` if a `None` payload or `null` JSON value is received as a state update.
    pub fn state_topic<T: Into<String>>(mut self, state_topic: T) -> Self {
        self.state_topic = Some(state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract device's state from the `state_topic`. To determine the siren's state result of this template will be compared to `state_on` and `state_off`. Alternatively `value_template` can be used to render to a valid JSON payload.
    pub fn state_value_template<T: Into<String>>(mut self, state_value_template: T) -> Self {
        self.state_value_template = Some(state_value_template.into());
        self
    }

    /// Set to `true` if the MQTT siren supports the `duration` turn on action parameter and enables the `duration` state attribute.
    pub fn support_duration(mut self, support_duration: bool) -> Self {
        self.support_duration = Some(support_duration);
        self
    }

    /// Set to `true` if the MQTT siren supports the `volume_set` turn on action parameter and enables the `volume_level` state attribute.
    pub fn support_volume_set(mut self, support_volume_set: bool) -> Self {
        self.support_volume_set = Some(support_volume_set);
        self
    }

    /// An ID that uniquely identifies this siren device. If two sirens have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    pub fn unique_id<T: Into<String>>(mut self, unique_id: T) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }
}

impl Default for Siren {
    fn default() -> Self {
        Self {
            topic_prefix: Default::default(),
            origin: Default::default(),
            device: Default::default(),
            entity_category: Default::default(),
            availability: Default::default(),
            available_tones: Default::default(),
            command_off_template: Default::default(),
            command_template: Default::default(),
            command_topic: Default::default(),
            enabled_by_default: Default::default(),
            encoding: Default::default(),
            entity_picture: Default::default(),
            icon: Default::default(),
            json_attributes_template: Default::default(),
            json_attributes_topic: Default::default(),
            name: Default::default(),
            object_id: Default::default(),
            optimistic: Default::default(),
            payload_off: Default::default(),
            payload_on: Default::default(),
            platform: "siren".to_string(),
            qos: Default::default(),
            retain: Default::default(),
            state_off: Default::default(),
            state_on: Default::default(),
            state_topic: Default::default(),
            state_value_template: Default::default(),
            support_duration: Default::default(),
            support_volume_set: Default::default(),
            unique_id: Default::default(),
        }
    }
}

impl From<Siren> for Entity {
    fn from(value: Siren) -> Self {
        Entity::Siren(value)
    }
}
/// ---
/// title: "MQTT Light"
/// description: "Instructions on how to setup MQTT lights using default schema within Home Assistant."
/// ha_category:
///   - Light
/// ha_iot_class: Configurable
/// ha_release: 0.8
/// ha_domain: mqtt
/// ---
///
/// The `mqtt` light platform lets you control your MQTT enabled lights through one of the supported message schemas, `default`, `json` or `template`.
///
/// ## Comparison of light MQTT schemas
///
/// | Function          | [`default`](#default-schema) | [`json`](#json-schema) | [`template`](#template-schema) |
/// | ----------------- | ---------------------------- | ---------------------- | ------------------------------ |
/// | Brightness        | ✔                            | ✔                      | ✔                              |
/// | Color mode        | ✔                            | ✔                      | ✘                              |
/// | Color temperature | ✔                            | ✔                      | ✔                              |
/// | Effects           | ✔                            | ✔                      | ✔                              |
/// | Flashing          | ✘                            | ✔                      | ✔                              |
/// | HS Color          | ✔                            | ✔                      | ✔                              |
/// | RGB Color         | ✔                            | ✔                      | ✔                              |
/// | RGBW Color        | ✔                            | ✔                      | ✘                              |
/// | RGBWW Color       | ✔                            | ✔                      | ✘                              |
/// | Transitions       | ✘                            | ✔                      | ✔                              |
/// | White             | ✔                            | ✔                      | ✘                              |
/// | XY Color          | ✔                            | ✔                      | ✘                              |
///
/// ## Default schema
///
/// The `mqtt` light platform with default schema lets you control your MQTT enabled lights. It supports setting brightness, color temperature, effects, on/off, RGB colors, XY colors and white.
///
/// ## Default schema - Configuration
///
/// In an ideal scenario, the MQTT device will have a state topic to publish state changes. If these messages are published with a `RETAIN` flag, the MQTT light will receive an instant state update after subscription and will start with the correct state. Otherwise, the initial state of the switch will be `unknown`. A MQTT device can reset the current state to `unknown` using a `None` payload.
///
/// When a state topic is not available, the light will work in optimistic mode. In this mode, the light will immediately change state after every command. Otherwise, the light will wait for state confirmation from the device (message from `state_topic`). The initial state is set to `False` / `off` in optimistic mode.
///
/// Optimistic mode can be forced, even if the `state_topic` is available. Try to enable it, if experiencing incorrect light operation.
///
/// Home Assistant internally assumes that a light's state corresponds to a defined `color_mode`.
/// The state of MQTT lights with default schema and support for both color and color temperature will set the `color_mode` according to the last received valid color or color temperature. Optionally, a `color_mode_state_topic` can be configured for explicit control of the `color_mode`.
///
/// To use an MQTT basic light in your installation, [add a MQTT device as a subentry](/integrations/mqtt/#configuration), or add the following to your `configuration.yaml` file.
/// {% include integrations/restart_ha_after_config_inclusion.md %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - light:
///       command_topic: "office/rgb1/light/switch"
/// ```
///
/// Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
///
///
/// ⚠ Important\
/// Make sure that your topics match exactly. `some-topic/` and `some-topic` are different topics.
///
/// 🛈 Note\
/// XY and RGB can not be used at the same time. If both are provided, XY overrides RGB.
///
/// ## Default schema - Examples
///
/// In this section you will find some real-life examples of how to use this sensor.
///
/// ### Brightness and RGB support
///
/// To enable a light with brightness and RGB support in your installation, add the following to your `configuration.yaml` file:
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - light:
///       name: "Office Light RGB"
///       state_topic: "office/rgb1/light/status"
///       command_topic: "office/rgb1/light/switch"
///       brightness_state_topic: "office/rgb1/brightness/status"
///       brightness_command_topic: "office/rgb1/brightness/set"
///       rgb_state_topic: "office/rgb1/rgb/status"
///       rgb_command_topic: "office/rgb1/rgb/set"
///       state_value_template: "{{ value_json.state }}"
///       brightness_value_template: "{{ value_json.brightness }}"
///       rgb_value_template: "{{ value_json.rgb | join(',') }}"
///       qos: 0
///       payload_on: "ON"
///       payload_off: "OFF"
///       optimistic: false
/// ```
///
///
/// ### Brightness and no RGB support
///
/// To enable a light with brightness (no RGB version) in your installation, add the following to your `configuration.yaml` file:
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - light:
///       name: "Office light"
///       state_topic: "office/light/status"
///       command_topic: "office/light/switch"
///       brightness_state_topic: 'office/light/brightness'
///       brightness_command_topic: 'office/light/brightness/set'
///       qos: 0
///       payload_on: "ON"
///       payload_off: "OFF"
///       optimistic: false
/// ```
///
/// ### Brightness without on commands
///
/// To enable a light that sends only brightness topics to turn it on, add the following to your `configuration.yaml` file. The `command_topic` is only used to send an off command in this case:
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - light:
///       name: "Brightness light"
///       state_topic: "office/light/status"
///       command_topic: "office/light/switch"
///       payload_off: "OFF"
///       brightness_state_topic: 'office/light/brightness'
///       brightness_command_topic: 'office/light/brightness/set'
///       on_command_type: 'brightness'
/// ```
///
/// ## Default schema - Implementations
///
/// - A [basic example](https://github.com/mertenats/open-home-automation/tree/master/ha_mqtt_light) using a NodeMCU board (ESP8266) to control its built-in LED (on/off).
/// - Another [example](https://github.com/mertenats/open-home-automation/tree/master/ha_mqtt_rgb_light) to control a RGB LED (on/off, brightness, and colors).
/// - [Integration guide](https://github.com/xoseperez/espurna/wiki/HomeAssistant) for the ESPUrna firmware (ESP8285/ESP8266).
///
/// ## JSON schema
///
/// The `mqtt` light platform with JSON schema lets you control a MQTT-enabled light that can receive [JSON](https://en.wikipedia.org/wiki/JSON) messages.
///
/// This schema supports on/off, brightness, RGB colors, XY colors, color temperature, transitions and short/long flashing. The messages sent to/from the lights look similar to this, omitting fields when they aren't needed. The `color_mode` will not be present in messages sent to the light. It is optional in messages received from the light, but can be used to disambiguate the current mode in the light. In the example below, `color_mode` is set to `rgb` and `color_temp`, `color.c`, `color.w`, `color.x`, `color.y`, `color.h`, `color.s` will all be ignored by Home Assistant:
///
/// ```json
/// {
///   "brightness": 255,
///   "color_mode": "rgb",
///   "color_temp": 155,
///   "color": {
///     "r": 255,
///     "g": 180,
///     "b": 200,
///     "c": 100,
///     "w": 50,
///     "x": 0.406,
///     "y": 0.301,
///     "h": 344.0,
///     "s": 29.412
///   },
///   "effect": "colorloop",
///   "state": "ON",
///   "transition": 2,
/// }
/// ```
///
/// ## JSON schema - Configuration
///
/// In an ideal scenario, the MQTT device will have a state topic to publish state changes. If these messages are published with the RETAIN flag, the MQTT light will receive an instant state update after subscription and will start with the correct state. Otherwise, the initial state of the light will be off.
///
/// When a state topic is not available, the light will work in optimistic mode. In this mode, the light will immediately change state after every command. Otherwise, the light will wait for state confirmation from the device (message from `state_topic`).
///
/// Optimistic mode can be forced, even if state topic is available. Try enabling it if the light is operating incorrectly.
///
/// To use an MQTT JSON light in your installation, [add a MQTT device as a subentry](/integrations/mqtt/#configuration), or add the following to your `configuration.yaml` file.
/// {% include integrations/restart_ha_after_config_inclusion.md %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - light:
///       schema: json
///       command_topic: "home/rgb1/set"
/// ```
///
/// Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
///
///
/// *The `color` attribute dict in the JSON state payload should contain the following keys based on the `color_mode`:
///
/// - `hs`:
///   - `h`: The hue value
///   - `s`: The saturation value
/// - `xy`:
///   - `x`: X color value
///   - `y`: Y color value
/// - `rgb`:
///   - `r`: Red color value
///   - `g`: Green color value
///   - `b`: Blue color value
/// - `rgbw`:
///   - `r`: Red color value
///   - `g`: Green color value
///   - `b`: Blue color value
///   - `w`: White value
/// - `rgbww`:
///   - `r`: Red color value
///   - `g`: Green color value
///   - `b`: Blue color value
///   - `c`: Cool white value
///   - `w`: Warm white value
///
/// More details about the different colors, color modes and range values [can be found here](https://www.home-assistant.io/integrations/light/).
///
/// ⚠ Important\
/// Make sure that your topics match exact. `some-topic/` and `some-topic` are different topics.
///
/// 🛈 Note\
/// RGB, XY and HSV can not be used at the same time in `state_topic` messages. Make sure that only one of the color models is in the "color" section of the state MQTT payload.
///
/// ## JSON schema - Examples
///
/// In this section you find some real-life examples of how to use this sensor.
///
/// ### Brightness and RGB support
///
/// To enable a light with brightness and RGB support in your installation, add the following to your `configuration.yaml` file:
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - light:
///       schema: json
///       name: mqtt_json_light_1
///       state_topic: "home/rgb1"
///       command_topic: "home/rgb1/set"
///       brightness: true
///       supported_color_modes: ["rgb"]
/// ```
///
/// ### Brightness and no RGB support
///
/// To enable a light with brightness (but no color support) in your installation, add the following to your `configuration.yaml` file:
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - light:
///       schema: json
///       name: mqtt_json_light_1
///       state_topic: "home/rgb1"
///       command_topic: "home/rgb1/set"
///       brightness: true
///       supported_color_modes: ["brightness"]
/// ```
///
/// ### Brightness scaled
///
/// To enable a light using a brightness scale other than 8bit the `brightness_scale` option may be added to denote the "fully on" value:
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - light:
///       schema: json
///       name: mqtt_json_light_1
///       state_topic: "home/light"
///       command_topic: "home/light/set"
///       brightness: true
///       brightness_scale: 4095
///       supported_color_modes: ["brightness"]
/// ```
///
/// Home Assistant will then convert its 8bit value in the message to and from the device:
///
/// ```json
/// {
///   "brightness": 4095,
///   "state": "ON"
/// }
/// ```
///
/// ### HS color
///
/// To use a light with hue+saturation as the color model, set `supported_color_modes` to `["hs"]` in the platform configuration:
///
/// ```yaml
/// mqtt:
///   - light:
///       schema: json
///       name: mqtt_json_hs_light
///       state_topic: "home/light"
///       command_topic: "home/light/set"
///       supported_color_modes: ["hs"]
/// ```
///
/// Home Assistant expects the hue values to be in the range 0 to 360 and the saturation values to be scaled from 0 to 100. For example, the following is a blue color shade:
///
/// ```json
/// {
///   "state": "ON",
///   "color_mode": "hs",
///   "color": {
///     "h": 24.0,
///     "s": 100.0
///   }
/// }
/// ```
///
/// ### Brightness and RGBW support
///
/// To enable a light with brightness, RGB support and a separate white channel (RGBW) in your installation, add the following to your `configuration.yaml` file:
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - light:
///       schema: json
///       name: mqtt_json_light_1
///       state_topic: "home/rgbw1"
///       command_topic: "home/rgbw1/set"
///       brightness: true
///       supported_color_modes: ["rgbw"]
/// ```
///
/// ## Implementations
///
/// - A full example of custom lighting using this platform and an ESP8266 microcontroller can be found [here](https://github.com/corbanmailloux/esp-mqtt-rgb-led). It supports on/off, brightness, transitions, RGB colors, and flashing.
///
/// - There is also another implementation forked from the above repository, it supports all the same features but is made for addressable LED strips using FastLED on a NodeMCU V3 it can be found [here](https://github.com/JammyDodger231/nodemcu-mqtt-rgb-led).
///
/// - [McLighting](https://github.com/toblum/McLighting) is another ESP8266 firmware for WS2812 addressable LEDs.
///
/// - [MQTT JSON Light](https://github.com/mertenats/Open-Home-Automation/tree/master/ha_mqtt_rgbw_light_with_discovery) is another implementation for ESP8266 including [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
///
/// - [ESPHome](https://esphome.io) implements the JSON schema for MQTT based installs and supports [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
///
/// - [AiLight](https://github.com/stelgenhof/AiLight) is a custom firmware for the Ai-Thinker (and equivalent) RGBW WiFi light bulbs that has an ESP8266 onboard and controlled by the MY9291 LED driver. It implements the [MQTT JSON light](/integrations/light.mqtt) platform and supports ON/OFF, RGBW colours, brightness, color temperature, flashing and transitions. Also it includes [MQTT Auto Discovery](/integrations/mqtt/#mqtt-discovery)) and the MQTT Last Will and Testament is enabled as well.
///
/// - [h801-mqtt-json](https://github.com/starkillerOG/h801-mqtt-json) is a custom firmware for the H801 LED dimmer, a 5 channel (RGBWWCW)  WiFi LED strip controller for 12V LED strips. The firmware is meant to control the 5 channels of the H801 to simultaneously control an RGB and a Warm-white/Cold-white LED strip such as a 5050 RGB LED strip and a 5025 Dual White strip. It implements the [MQTT JSON light](/integrations/light.mqtt) platform and supports ON/OFF, RGBW colours (RGB strip), brightness, color temperature (CW/WW strip) and transitions.
///
/// ## Template schema
///
/// The `mqtt` light platform with template schema lets you control a MQTT-enabled light that receive commands on a command topic and optionally sends status update on a state topic.
/// It is format-agnostic so you can use any data format you want (i.e., string, JSON), just configure it with templating.
///
/// This schema supports on/off, brightness, RGB colors, XY colors, HS Color, color temperature, transitions, short/long flashing and effects.
///
/// ## Template schema - Configuration
///
/// In an ideal scenario, the MQTT device will have a state topic to publish state changes. If these messages are published with the RETAIN flag, the MQTT light will receive an instant state update after subscription and will start with the correct state. Otherwise, the initial state of the light will be off.
///
/// When a state topic is not available, the light will work in optimistic mode. In this mode, the light will immediately change state after every command. Otherwise, the light will wait for state confirmation from the device (message from `state_topic`).
///
/// Optimistic mode can be forced, even if state topic is available. Try enabling it if the light is operating incorrectly.
///
/// To use an MQTT template light in your installation, [add a MQTT device as a subentry](/integrations/mqtt/#configuration), or add the following to your `configuration.yaml` file.
/// {% include integrations/restart_ha_after_config_inclusion.md %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - light:
///       schema: template
///       command_topic: "home/rgb1/set"
///       command_on_template: "on"
///       command_off_template: "off"
/// ```
///
/// Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
///
///
/// ⚠ Important\
/// Make sure that your topics match exact. `some-topic/` and `some-topic` are different topics.
///
/// ## Template schema - Examples
///
/// In this section you find some real-life examples of how to use this light.
///
/// ### Simple string payload
///
/// For a simple string payload with the format `state,brightness,r-g-b,h-s` (e.g., `on,255,255-255-255,360-100`), add the following to your `configuration.yaml` file:
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - light:
///       schema: template
///       command_topic: "home/rgb1/set"
///       state_topic: "home/rgb1/status"
///       command_on_template: "on,{{ brightness|d }},{{ red|d }}-{{ green|d }}-{{ blue|d }},{{ hue|d }}-{{ sat|d }}"
///       command_off_template: "off"
///       state_template: "{{ value.split(',')[0] }}"  # must return `on` or `off`
///       brightness_template: "{{ value.split(',')[1] }}"
///       red_template: "{{ value.split(',')[2].split('-')[0] }}"
///       green_template: "{{ value.split(',')[2].split('-')[1] }}"
///       blue_template: "{{ value.split(',')[2].split('-')[2] }}"
/// ```
///
///
/// ### JSON payload
///
/// For a JSON payload with the format `{"state": "on", "brightness": 255, "color": [255, 255, 255], "effect": "rainbow"}`, add the following to your `configuration.yaml` file:
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - light:
///       schema: template
///       effect_list:
///         - rainbow
///         - colorloop
///       command_topic: "home/rgb1/set"
///       state_topic: "home/rgb1/status"
///       command_on_template: >
///         {"state": "on"
///         {%- if brightness is defined -%}
///         , "brightness": {{ brightness }}
///         {%- endif -%}
///         {%- if red is defined and green is defined and blue is defined -%}
///         , "color": [{{ red }}, {{ green }}, {{ blue }}]
///         {%- endif -%}
///         {%- if hue is defined and sat is defined -%}
///         , "huesat": [{{ hue }}, {{ sat }}]
///         {%- endif -%}
///         {%- if effect is defined -%}
///         , "effect": "{{ effect }}"
///         {%- endif -%}
///         }
///       command_off_template: '{"state": "off"}'
///       state_template: '{{ value_json.state }}'
///       brightness_template: '{{ value_json.brightness }}'
///       red_template: '{{ value_json.color[0] }}'
///       green_template: '{{ value_json.color[1] }}'
///       blue_template: '{{ value_json.color[2] }}'
///       effect_template: '{{ value_json.effect }}'
/// ```
///
///
/// ### CCT light (brightness and temperature)
///
/// This example comes from a configuration of Shelly RGBW Bulb working in White mode.
/// `max_mireds` and `min_mireds` set color temperature boundaries to 3000K - 6500K. Notice the same limits are applied in `command_on_template`, but in kelvin units this time. It's due to conversion from mired to kelvin which causes exceeding boundary values accepted by the device.
/// The code also ensures bi-directional conversion of brightness scale between 0-100 (required by the device) and 0-255 (required by Home Assistant).
/// Add the following to your `configuration.yaml` file:
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - light:
///       schema: template
///       name: "Bulb-white"
///       command_topic: "shellies/bulb/color/0/set"
///       state_topic: "shellies/bulb/color/0/status"
///       availability_topic: "shellies/bulb/online"
///       command_on_template: >
///         {"turn": "on", "mode": "white"
///         {%- if brightness is defined -%}
///         , "brightness": {{brightness | float | multiply(0.39215686) | round(0)}}
///         {%- endif -%}
///         {%- if color_temp is defined -%}
///         , "temp": {{ [[(1000000 / color_temp | float) | round(0), 3000] | max, 6500] | min }}
///         {%- endif -%}
///         }
///       command_off_template: '{"turn":"off", "mode": "white"}'
///       state_template: "{% if value_json.ison and value_json.mode == 'white' %}on{% else %}off{% endif %}"
///       brightness_template: "{{ value_json.brightness | float | multiply(2.55) | round(0) }}"
///       color_temp_template: "{{ (1000000 / value_json.temp | float) | round(0) }}"
///       payload_available: "true"
///       payload_not_available: "false"
///       max_mireds: 334
///       min_mireds: 153
///       qos: 1
///       retain: false
///       optimistic: false  
/// ```
///
///
/// ### Template schema - No brightness or color support
///
/// If you don't want brightness, color or effect support, just omit the corresponding configuration sections.
///
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Light {
    /// Replaces `~` with this value in any MQTT topic attribute.
    /// [See Home Assistant documentation](https://www.home-assistant.io/integrations/mqtt/#using-abbreviations-and-base-topic)
    #[serde(rename = "~", skip_serializing_if = "Option::is_none")]
    pub topic_prefix: Option<String>,

    /// It is encouraged to add additional information about the origin that supplies MQTT entities via MQTT discovery by adding the origin option (can be abbreviated to o) to the discovery payload. Note that these options also support abbreviations. Information of the origin will be logged to the core event log when an item is discovered or updated.
    #[serde(rename = "o")]
    pub origin: Origin,

    /// Information about the device this button is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/en/device_registry_index.html). Only works when [`unique_id`](#unique_id) is set. At least one of identifiers or connections must be present to identify the device.
    #[serde(rename = "dev")]
    pub device: DeviceInformation,

    /// A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`.
    #[serde(flatten)]
    pub availability: Availability,

    /// The category of the entity. (optional, default: None)
    #[serde(rename = "ent_cat", skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<EntityCategory>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to compose message which will be sent to `brightness_command_topic`. Available variables: `value`.
    #[serde(rename = "bri_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub brightness_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the light’s brightness.
    #[serde(rename = "bri_cmd_t", skip_serializing_if = "Option::is_none")]
    pub brightness_command_topic: Option<String>,

    /// Defines the maximum brightness value (i.e., 100%) of the MQTT device.
    #[serde(rename = "bri_scl", skip_serializing_if = "Option::is_none")]
    pub brightness_scale: Option<i32>,

    /// The MQTT topic subscribed to receive brightness state updates.
    #[serde(rename = "bri_stat_t", skip_serializing_if = "Option::is_none")]
    pub brightness_state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the brightness value.
    #[serde(rename = "bri_val_tpl", skip_serializing_if = "Option::is_none")]
    pub brightness_value_template: Option<String>,

    /// The MQTT topic subscribed to receive color mode updates. If this is not configured, `color_mode` will be automatically set according to the last received valid color or color temperature. The unit used is mireds, or if `color_temp_kelvin` is set to `true`, in Kelvin.
    #[serde(rename = "clrm_stat_t", skip_serializing_if = "Option::is_none")]
    pub color_mode_state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the color mode.
    #[serde(rename = "clrm_val_tpl", skip_serializing_if = "Option::is_none")]
    pub color_mode_value_template: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to compose message which will be sent to `color_temp_command_topic`. Available variables: `value`.
    #[serde(rename = "clr_temp_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub color_temp_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the light’s color temperature state. By default the color temperature command slider has a range of 153 to 500 mireds (micro reciprocal degrees) or a range of 2000 to 6535 Kelvin if `color_temp_kelvin` is set to `true`.
    #[serde(rename = "clr_temp_cmd_t", skip_serializing_if = "Option::is_none")]
    pub color_temp_command_topic: Option<String>,

    /// When set to `true`, `color_temp_command_topic` will publish color mode updates in Kelvin and process `color_temp_state_topic` will process state updates in Kelvin. When not set the `color_temp` values are converted to mireds.
    #[serde(rename = "clr_temp_k", skip_serializing_if = "Option::is_none")]
    pub color_temp_kelvin: Option<bool>,

    /// The MQTT topic subscribed to receive color temperature state updates.
    #[serde(rename = "clr_temp_stat_t", skip_serializing_if = "Option::is_none")]
    pub color_temp_state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the color temperature value.
    #[serde(rename = "clr_temp_val_tpl", skip_serializing_if = "Option::is_none")]
    pub color_temp_value_template: Option<String>,

    /// The MQTT topic to publish commands to change the switch state.
    #[serde(rename = "cmd_t")]
    pub command_topic: String,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to compose message which will be sent to `effect_command_topic`. Available variables: `value`.
    #[serde(rename = "fx_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub effect_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the light's effect state.
    #[serde(rename = "fx_cmd_t", skip_serializing_if = "Option::is_none")]
    pub effect_command_topic: Option<String>,

    /// The list of effects the light supports.
    #[serde(rename = "fx_list", skip_serializing_if = "Option::is_none")]
    pub effect_list: Option<Vec<String>>,

    /// The MQTT topic subscribed to receive effect state updates.
    #[serde(rename = "fx_stat_t", skip_serializing_if = "Option::is_none")]
    pub effect_state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the effect value.
    #[serde(rename = "fx_val_tpl", skip_serializing_if = "Option::is_none")]
    pub effect_value_template: Option<String>,

    /// Flag which defines if the entity should be enabled when first added.
    #[serde(rename = "en", skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,

    /// The encoding of the payloads received and published messages. Set to `""` to disable decoding of incoming payload.
    #[serde(rename = "e", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,

    /// Picture URL for the entity.
    #[serde(rename = "ent_pic", skip_serializing_if = "Option::is_none")]
    pub entity_picture: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to compose message which will be sent to `hs_command_topic`. Available variables: `hue` and `sat`.
    #[serde(rename = "hs_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub hs_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the light's color state in HS format (Hue Saturation). Range for Hue: 0° .. 360°, Range of Saturation: 0..100. Note: Brightness is sent separately in the `brightness_command_topic`.
    #[serde(rename = "hs_cmd_t", skip_serializing_if = "Option::is_none")]
    pub hs_command_topic: Option<String>,

    /// The MQTT topic subscribed to receive color state updates in HS format. The expected payload is the hue and saturation values separated by commas, for example, `359.5,100.0`. Note: Brightness is received separately in the `brightness_state_topic`.
    #[serde(rename = "hs_stat_t", skip_serializing_if = "Option::is_none")]
    pub hs_state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the HS value.
    #[serde(rename = "hs_val_tpl", skip_serializing_if = "Option::is_none")]
    pub hs_value_template: Option<String>,

    /// [Icon](/docs/configuration/customizing-devices/#icon) for the entity.
    #[serde(rename = "ic", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation.
    #[serde(rename = "json_attr_tpl", skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation.
    #[serde(rename = "json_attr_t", skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,

    /// The maximum color temperature in Kelvin.
    #[serde(rename = "max_k", skip_serializing_if = "Option::is_none")]
    pub max_kelvin: Option<i32>,

    /// The maximum color temperature in mireds.
    #[serde(rename = "max_mirs", skip_serializing_if = "Option::is_none")]
    pub max_mireds: Option<i32>,

    /// The minimum color temperature in Kelvin.
    #[serde(rename = "min_k", skip_serializing_if = "Option::is_none")]
    pub min_kelvin: Option<i32>,

    /// The minimum color temperature in mireds.
    #[serde(rename = "min_mirs", skip_serializing_if = "Option::is_none")]
    pub min_mireds: Option<i32>,

    /// The name of the light. Can be set to `null` if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// Defines when on the payload_on is sent. Using `last` (the default) will send any style (brightness, color, etc) topics first and then a `payload_on` to the `command_topic`. Using `first` will send the `payload_on` and then any style topics. Using `brightness` will only send brightness commands instead of the `payload_on` to turn the light on.
    #[serde(rename = "on_cmd_type", skip_serializing_if = "Option::is_none")]
    pub on_command_type: Option<String>,

    /// Flag that defines if switch works in optimistic mode.
    #[serde(rename = "opt", skip_serializing_if = "Option::is_none")]
    pub optimistic: Option<bool>,

    /// The payload that represents the off state.
    #[serde(rename = "pl_off", skip_serializing_if = "Option::is_none")]
    pub payload_off: Option<String>,

    /// The payload that represents the on state.
    #[serde(rename = "pl_on", skip_serializing_if = "Option::is_none")]
    pub payload_on: Option<String>,

    /// Must be `light`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    #[serde(rename = "p")]
    pub platform: String,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// If the published message should have the retain flag on or not.
    #[serde(rename = "ret", skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to compose message which will be sent to `rgb_command_topic`. Available variables: `red`, `green` and `blue`.
    #[serde(rename = "rgb_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub rgb_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the light's RGB state.
    #[serde(rename = "rgb_cmd_t", skip_serializing_if = "Option::is_none")]
    pub rgb_command_topic: Option<String>,

    /// The MQTT topic subscribed to receive RGB state updates. The expected payload is the RGB values separated by commas, for example, `255,0,127`.
    #[serde(rename = "rgb_stat_t", skip_serializing_if = "Option::is_none")]
    pub rgb_state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the RGB value.
    #[serde(rename = "rgb_val_tpl", skip_serializing_if = "Option::is_none")]
    pub rgb_value_template: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to compose message which will be sent to `rgbw_command_topic`. Available variables: `red`, `green`, `blue` and `white`.
    #[serde(rename = "rgbw_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub rgbw_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the light's RGBW state.
    #[serde(rename = "rgbw_cmd_t", skip_serializing_if = "Option::is_none")]
    pub rgbw_command_topic: Option<String>,

    /// The MQTT topic subscribed to receive RGBW state updates. The expected payload is the RGBW values separated by commas, for example, `255,0,127,64`.
    #[serde(rename = "rgbw_stat_t", skip_serializing_if = "Option::is_none")]
    pub rgbw_state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the RGBW value.
    #[serde(rename = "rgbw_val_tpl", skip_serializing_if = "Option::is_none")]
    pub rgbw_value_template: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to compose message which will be sent to `rgbww_command_topic`. Available variables: `red`, `green`, `blue`, `cold_white` and `warm_white`.
    #[serde(rename = "rgbww_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub rgbww_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the light's RGBWW state.
    #[serde(rename = "rgbww_cmd_t", skip_serializing_if = "Option::is_none")]
    pub rgbww_command_topic: Option<String>,

    /// The MQTT topic subscribed to receive RGBWW state updates. The expected payload is the RGBWW values separated by commas, for example, `255,0,127,64,32`.
    #[serde(rename = "rgbww_stat_t", skip_serializing_if = "Option::is_none")]
    pub rgbww_state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the RGBWW value.
    #[serde(rename = "rgbww_val_tpl", skip_serializing_if = "Option::is_none")]
    pub rgbww_value_template: Option<String>,

    /// The schema to use. Must be `basic` or omitted to select the default schema.
    #[serde(rename = "schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,

    /// The MQTT topic subscribed to receive state updates. A "None" payload resets to an `unknown` state. An empty payload is ignored. By default, valid state payloads are `OFF` and `ON`. The accepted payloads can be overridden with the `payload_off` and `payload_on` config options.
    #[serde(rename = "stat_t", skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the state value. The template should return the values defined by `payload_on` (defaults to "ON") and `payload_off` (defaults to "OFF") settings, or "None".
    #[serde(rename = "stat_val_tpl", skip_serializing_if = "Option::is_none")]
    pub state_value_template: Option<String>,

    /// An ID that uniquely identifies this light. If two lights have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,

    /// The MQTT topic to publish commands to change the light to white mode with a given brightness.
    #[serde(rename = "whit_cmd_t", skip_serializing_if = "Option::is_none")]
    pub white_command_topic: Option<String>,

    /// Defines the maximum white level (i.e., 100%) of the MQTT device.
    #[serde(rename = "whit_scl", skip_serializing_if = "Option::is_none")]
    pub white_scale: Option<i32>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to compose message which will be sent to `xy_command_topic`. Available variables: `x` and `y`.
    #[serde(rename = "xy_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub xy_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the light's XY state.
    #[serde(rename = "xy_cmd_t", skip_serializing_if = "Option::is_none")]
    pub xy_command_topic: Option<String>,

    /// The MQTT topic subscribed to receive XY state updates. The expected payload is the X and Y color values separated by commas, for example, `0.675,0.322`.
    #[serde(rename = "xy_stat_t", skip_serializing_if = "Option::is_none")]
    pub xy_state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the XY value.
    #[serde(rename = "xy_val_tpl", skip_serializing_if = "Option::is_none")]
    pub xy_value_template: Option<String>,
}

impl Light {
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
    pub fn device(mut self, device: DeviceInformation) -> Self {
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

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to compose message which will be sent to `brightness_command_topic`. Available variables: `value`.
    pub fn brightness_command_template<T: Into<String>>(
        mut self,
        brightness_command_template: T,
    ) -> Self {
        self.brightness_command_template = Some(brightness_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the light’s brightness.
    pub fn brightness_command_topic<T: Into<String>>(
        mut self,
        brightness_command_topic: T,
    ) -> Self {
        self.brightness_command_topic = Some(brightness_command_topic.into());
        self
    }

    /// Defines the maximum brightness value (i.e., 100%) of the MQTT device.
    pub fn brightness_scale(mut self, brightness_scale: i32) -> Self {
        self.brightness_scale = Some(brightness_scale);
        self
    }

    /// The MQTT topic subscribed to receive brightness state updates.
    pub fn brightness_state_topic<T: Into<String>>(mut self, brightness_state_topic: T) -> Self {
        self.brightness_state_topic = Some(brightness_state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the brightness value.
    pub fn brightness_value_template<T: Into<String>>(
        mut self,
        brightness_value_template: T,
    ) -> Self {
        self.brightness_value_template = Some(brightness_value_template.into());
        self
    }

    /// The MQTT topic subscribed to receive color mode updates. If this is not configured, `color_mode` will be automatically set according to the last received valid color or color temperature. The unit used is mireds, or if `color_temp_kelvin` is set to `true`, in Kelvin.
    pub fn color_mode_state_topic<T: Into<String>>(mut self, color_mode_state_topic: T) -> Self {
        self.color_mode_state_topic = Some(color_mode_state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the color mode.
    pub fn color_mode_value_template<T: Into<String>>(
        mut self,
        color_mode_value_template: T,
    ) -> Self {
        self.color_mode_value_template = Some(color_mode_value_template.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to compose message which will be sent to `color_temp_command_topic`. Available variables: `value`.
    pub fn color_temp_command_template<T: Into<String>>(
        mut self,
        color_temp_command_template: T,
    ) -> Self {
        self.color_temp_command_template = Some(color_temp_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the light’s color temperature state. By default the color temperature command slider has a range of 153 to 500 mireds (micro reciprocal degrees) or a range of 2000 to 6535 Kelvin if `color_temp_kelvin` is set to `true`.
    pub fn color_temp_command_topic<T: Into<String>>(
        mut self,
        color_temp_command_topic: T,
    ) -> Self {
        self.color_temp_command_topic = Some(color_temp_command_topic.into());
        self
    }

    /// When set to `true`, `color_temp_command_topic` will publish color mode updates in Kelvin and process `color_temp_state_topic` will process state updates in Kelvin. When not set the `color_temp` values are converted to mireds.
    pub fn color_temp_kelvin(mut self, color_temp_kelvin: bool) -> Self {
        self.color_temp_kelvin = Some(color_temp_kelvin);
        self
    }

    /// The MQTT topic subscribed to receive color temperature state updates.
    pub fn color_temp_state_topic<T: Into<String>>(mut self, color_temp_state_topic: T) -> Self {
        self.color_temp_state_topic = Some(color_temp_state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the color temperature value.
    pub fn color_temp_value_template<T: Into<String>>(
        mut self,
        color_temp_value_template: T,
    ) -> Self {
        self.color_temp_value_template = Some(color_temp_value_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the switch state.
    pub fn command_topic<T: Into<String>>(mut self, command_topic: T) -> Self {
        self.command_topic = command_topic.into();
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to compose message which will be sent to `effect_command_topic`. Available variables: `value`.
    pub fn effect_command_template<T: Into<String>>(mut self, effect_command_template: T) -> Self {
        self.effect_command_template = Some(effect_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the light's effect state.
    pub fn effect_command_topic<T: Into<String>>(mut self, effect_command_topic: T) -> Self {
        self.effect_command_topic = Some(effect_command_topic.into());
        self
    }

    /// The list of effects the light supports.
    pub fn effect_list<T: Into<String>>(mut self, effect_list: Vec<T>) -> Self {
        self.effect_list = Some(effect_list.into_iter().map(|v| v.into()).collect());
        self
    }

    /// The MQTT topic subscribed to receive effect state updates.
    pub fn effect_state_topic<T: Into<String>>(mut self, effect_state_topic: T) -> Self {
        self.effect_state_topic = Some(effect_state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the effect value.
    pub fn effect_value_template<T: Into<String>>(mut self, effect_value_template: T) -> Self {
        self.effect_value_template = Some(effect_value_template.into());
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

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to compose message which will be sent to `hs_command_topic`. Available variables: `hue` and `sat`.
    pub fn hs_command_template<T: Into<String>>(mut self, hs_command_template: T) -> Self {
        self.hs_command_template = Some(hs_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the light's color state in HS format (Hue Saturation). Range for Hue: 0° .. 360°, Range of Saturation: 0..100. Note: Brightness is sent separately in the `brightness_command_topic`.
    pub fn hs_command_topic<T: Into<String>>(mut self, hs_command_topic: T) -> Self {
        self.hs_command_topic = Some(hs_command_topic.into());
        self
    }

    /// The MQTT topic subscribed to receive color state updates in HS format. The expected payload is the hue and saturation values separated by commas, for example, `359.5,100.0`. Note: Brightness is received separately in the `brightness_state_topic`.
    pub fn hs_state_topic<T: Into<String>>(mut self, hs_state_topic: T) -> Self {
        self.hs_state_topic = Some(hs_state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the HS value.
    pub fn hs_value_template<T: Into<String>>(mut self, hs_value_template: T) -> Self {
        self.hs_value_template = Some(hs_value_template.into());
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

    /// The maximum color temperature in Kelvin.
    pub fn max_kelvin(mut self, max_kelvin: i32) -> Self {
        self.max_kelvin = Some(max_kelvin);
        self
    }

    /// The maximum color temperature in mireds.
    pub fn max_mireds(mut self, max_mireds: i32) -> Self {
        self.max_mireds = Some(max_mireds);
        self
    }

    /// The minimum color temperature in Kelvin.
    pub fn min_kelvin(mut self, min_kelvin: i32) -> Self {
        self.min_kelvin = Some(min_kelvin);
        self
    }

    /// The minimum color temperature in mireds.
    pub fn min_mireds(mut self, min_mireds: i32) -> Self {
        self.min_mireds = Some(min_mireds);
        self
    }

    /// The name of the light. Can be set to `null` if only the device name is relevant.
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    /// Defines when on the payload_on is sent. Using `last` (the default) will send any style (brightness, color, etc) topics first and then a `payload_on` to the `command_topic`. Using `first` will send the `payload_on` and then any style topics. Using `brightness` will only send brightness commands instead of the `payload_on` to turn the light on.
    pub fn on_command_type<T: Into<String>>(mut self, on_command_type: T) -> Self {
        self.on_command_type = Some(on_command_type.into());
        self
    }

    /// Flag that defines if switch works in optimistic mode.
    pub fn optimistic(mut self, optimistic: bool) -> Self {
        self.optimistic = Some(optimistic);
        self
    }

    /// The payload that represents the off state.
    pub fn payload_off<T: Into<String>>(mut self, payload_off: T) -> Self {
        self.payload_off = Some(payload_off.into());
        self
    }

    /// The payload that represents the on state.
    pub fn payload_on<T: Into<String>>(mut self, payload_on: T) -> Self {
        self.payload_on = Some(payload_on.into());
        self
    }

    /// Must be `light`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
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

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to compose message which will be sent to `rgb_command_topic`. Available variables: `red`, `green` and `blue`.
    pub fn rgb_command_template<T: Into<String>>(mut self, rgb_command_template: T) -> Self {
        self.rgb_command_template = Some(rgb_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the light's RGB state.
    pub fn rgb_command_topic<T: Into<String>>(mut self, rgb_command_topic: T) -> Self {
        self.rgb_command_topic = Some(rgb_command_topic.into());
        self
    }

    /// The MQTT topic subscribed to receive RGB state updates. The expected payload is the RGB values separated by commas, for example, `255,0,127`.
    pub fn rgb_state_topic<T: Into<String>>(mut self, rgb_state_topic: T) -> Self {
        self.rgb_state_topic = Some(rgb_state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the RGB value.
    pub fn rgb_value_template<T: Into<String>>(mut self, rgb_value_template: T) -> Self {
        self.rgb_value_template = Some(rgb_value_template.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to compose message which will be sent to `rgbw_command_topic`. Available variables: `red`, `green`, `blue` and `white`.
    pub fn rgbw_command_template<T: Into<String>>(mut self, rgbw_command_template: T) -> Self {
        self.rgbw_command_template = Some(rgbw_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the light's RGBW state.
    pub fn rgbw_command_topic<T: Into<String>>(mut self, rgbw_command_topic: T) -> Self {
        self.rgbw_command_topic = Some(rgbw_command_topic.into());
        self
    }

    /// The MQTT topic subscribed to receive RGBW state updates. The expected payload is the RGBW values separated by commas, for example, `255,0,127,64`.
    pub fn rgbw_state_topic<T: Into<String>>(mut self, rgbw_state_topic: T) -> Self {
        self.rgbw_state_topic = Some(rgbw_state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the RGBW value.
    pub fn rgbw_value_template<T: Into<String>>(mut self, rgbw_value_template: T) -> Self {
        self.rgbw_value_template = Some(rgbw_value_template.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to compose message which will be sent to `rgbww_command_topic`. Available variables: `red`, `green`, `blue`, `cold_white` and `warm_white`.
    pub fn rgbww_command_template<T: Into<String>>(mut self, rgbww_command_template: T) -> Self {
        self.rgbww_command_template = Some(rgbww_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the light's RGBWW state.
    pub fn rgbww_command_topic<T: Into<String>>(mut self, rgbww_command_topic: T) -> Self {
        self.rgbww_command_topic = Some(rgbww_command_topic.into());
        self
    }

    /// The MQTT topic subscribed to receive RGBWW state updates. The expected payload is the RGBWW values separated by commas, for example, `255,0,127,64,32`.
    pub fn rgbww_state_topic<T: Into<String>>(mut self, rgbww_state_topic: T) -> Self {
        self.rgbww_state_topic = Some(rgbww_state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the RGBWW value.
    pub fn rgbww_value_template<T: Into<String>>(mut self, rgbww_value_template: T) -> Self {
        self.rgbww_value_template = Some(rgbww_value_template.into());
        self
    }

    /// The schema to use. Must be `basic` or omitted to select the default schema.
    pub fn schema<T: Into<String>>(mut self, schema: T) -> Self {
        self.schema = Some(schema.into());
        self
    }

    /// The MQTT topic subscribed to receive state updates. A "None" payload resets to an `unknown` state. An empty payload is ignored. By default, valid state payloads are `OFF` and `ON`. The accepted payloads can be overridden with the `payload_off` and `payload_on` config options.
    pub fn state_topic<T: Into<String>>(mut self, state_topic: T) -> Self {
        self.state_topic = Some(state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the state value. The template should return the values defined by `payload_on` (defaults to "ON") and `payload_off` (defaults to "OFF") settings, or "None".
    pub fn state_value_template<T: Into<String>>(mut self, state_value_template: T) -> Self {
        self.state_value_template = Some(state_value_template.into());
        self
    }

    /// An ID that uniquely identifies this light. If two lights have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    pub fn unique_id<T: Into<String>>(mut self, unique_id: T) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }

    /// The MQTT topic to publish commands to change the light to white mode with a given brightness.
    pub fn white_command_topic<T: Into<String>>(mut self, white_command_topic: T) -> Self {
        self.white_command_topic = Some(white_command_topic.into());
        self
    }

    /// Defines the maximum white level (i.e., 100%) of the MQTT device.
    pub fn white_scale(mut self, white_scale: i32) -> Self {
        self.white_scale = Some(white_scale);
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to compose message which will be sent to `xy_command_topic`. Available variables: `x` and `y`.
    pub fn xy_command_template<T: Into<String>>(mut self, xy_command_template: T) -> Self {
        self.xy_command_template = Some(xy_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the light's XY state.
    pub fn xy_command_topic<T: Into<String>>(mut self, xy_command_topic: T) -> Self {
        self.xy_command_topic = Some(xy_command_topic.into());
        self
    }

    /// The MQTT topic subscribed to receive XY state updates. The expected payload is the X and Y color values separated by commas, for example, `0.675,0.322`.
    pub fn xy_state_topic<T: Into<String>>(mut self, xy_state_topic: T) -> Self {
        self.xy_state_topic = Some(xy_state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the XY value.
    pub fn xy_value_template<T: Into<String>>(mut self, xy_value_template: T) -> Self {
        self.xy_value_template = Some(xy_value_template.into());
        self
    }
}

impl Default for Light {
    fn default() -> Self {
        Self {
            topic_prefix: Default::default(),
            origin: Default::default(),
            device: Default::default(),
            entity_category: Default::default(),
            availability: Default::default(),
            brightness_command_template: Default::default(),
            brightness_command_topic: Default::default(),
            brightness_scale: Default::default(),
            brightness_state_topic: Default::default(),
            brightness_value_template: Default::default(),
            color_mode_state_topic: Default::default(),
            color_mode_value_template: Default::default(),
            color_temp_command_template: Default::default(),
            color_temp_command_topic: Default::default(),
            color_temp_kelvin: Default::default(),
            color_temp_state_topic: Default::default(),
            color_temp_value_template: Default::default(),
            command_topic: Default::default(),
            effect_command_template: Default::default(),
            effect_command_topic: Default::default(),
            effect_list: Default::default(),
            effect_state_topic: Default::default(),
            effect_value_template: Default::default(),
            enabled_by_default: Default::default(),
            encoding: Default::default(),
            entity_picture: Default::default(),
            hs_command_template: Default::default(),
            hs_command_topic: Default::default(),
            hs_state_topic: Default::default(),
            hs_value_template: Default::default(),
            icon: Default::default(),
            json_attributes_template: Default::default(),
            json_attributes_topic: Default::default(),
            max_kelvin: Default::default(),
            max_mireds: Default::default(),
            min_kelvin: Default::default(),
            min_mireds: Default::default(),
            name: Default::default(),
            object_id: Default::default(),
            on_command_type: Default::default(),
            optimistic: Default::default(),
            payload_off: Default::default(),
            payload_on: Default::default(),
            platform: "light".to_string(),
            qos: Default::default(),
            retain: Default::default(),
            rgb_command_template: Default::default(),
            rgb_command_topic: Default::default(),
            rgb_state_topic: Default::default(),
            rgb_value_template: Default::default(),
            rgbw_command_template: Default::default(),
            rgbw_command_topic: Default::default(),
            rgbw_state_topic: Default::default(),
            rgbw_value_template: Default::default(),
            rgbww_command_template: Default::default(),
            rgbww_command_topic: Default::default(),
            rgbww_state_topic: Default::default(),
            rgbww_value_template: Default::default(),
            schema: Default::default(),
            state_topic: Default::default(),
            state_value_template: Default::default(),
            unique_id: Default::default(),
            white_command_topic: Default::default(),
            white_scale: Default::default(),
            xy_command_template: Default::default(),
            xy_command_topic: Default::default(),
            xy_state_topic: Default::default(),
            xy_value_template: Default::default(),
        }
    }
}

impl From<Light> for Entity {
    fn from(value: Light) -> Self {
        Entity::Light(value)
    }
}
/// ---
/// title: "MQTT Select"
/// description: "Instructions on how to interact with a device exposing a Select through MQTT from within Home Assistant."
/// ha_category:
///   - Select
/// ha_release: 2021.7
/// ha_iot_class: Configurable
/// ha_domain: mqtt
/// ---
///
/// The `mqtt` Select platform allows you to integrate devices that might expose configuration options through MQTT into Home Assistant as a Select. Every time a message under the `topic` in the configuration is received, the select entity will be updated in Home Assistant and vice-versa, keeping the device and Home Assistant in sync.
///
/// ## Configuration
///
/// To use an MQTT select entity in your installation, add the following to your `configuration.yaml` file.
/// {% include integrations/restart_ha_after_config_inclusion.md %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - select:
///       command_topic: topic
///       name: "Test Select"
///       options:
///         - "Option 1"
///         - "Option 2"
/// ```
///
/// Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
///
///
/// ⚠ Important\
/// Make sure that your topic matches exactly. `some-topic/` and `some-topic` are different topics.
///
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Select {
    /// Replaces `~` with this value in any MQTT topic attribute.
    /// [See Home Assistant documentation](https://www.home-assistant.io/integrations/mqtt/#using-abbreviations-and-base-topic)
    #[serde(rename = "~", skip_serializing_if = "Option::is_none")]
    pub topic_prefix: Option<String>,

    /// It is encouraged to add additional information about the origin that supplies MQTT entities via MQTT discovery by adding the origin option (can be abbreviated to o) to the discovery payload. Note that these options also support abbreviations. Information of the origin will be logged to the core event log when an item is discovered or updated.
    #[serde(rename = "o")]
    pub origin: Origin,

    /// Information about the device this button is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/en/device_registry_index.html). Only works when [`unique_id`](#unique_id) is set. At least one of identifiers or connections must be present to identify the device.
    #[serde(rename = "dev")]
    pub device: DeviceInformation,

    /// A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`.
    #[serde(flatten)]
    pub availability: Availability,

    /// The category of the entity. (optional, default: None)
    #[serde(rename = "ent_cat", skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<EntityCategory>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to generate the payload to send to `command_topic`.
    #[serde(rename = "cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub command_template: Option<String>,

    /// The MQTT topic to publish commands to change the selected option.
    #[serde(rename = "cmd_t")]
    pub command_topic: String,

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

    /// The name of the Select. Can be set to `null` if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// Flag that defines if the select works in optimistic mode.
    #[serde(rename = "opt", skip_serializing_if = "Option::is_none")]
    pub optimistic: Option<bool>,

    /// List of options that can be selected. An empty list or a list with a single item is allowed.
    #[serde(rename = "ops")]
    pub options: Vec<String>,

    /// Must be `select`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    #[serde(rename = "p")]
    pub platform: String,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// If the published message should have the retain flag on or not.
    #[serde(rename = "ret", skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,

    /// The MQTT topic subscribed to receive update of the selected option. A "None" payload resets to an `unknown` state. An empty payload is ignored.
    #[serde(rename = "stat_t", skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,

    /// An ID that uniquely identifies this Select. If two Selects have the same unique ID Home Assistant will raise an exception. Required when used with device-based discovery.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the value.
    #[serde(rename = "val_tpl", skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

impl Select {
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
    pub fn device(mut self, device: DeviceInformation) -> Self {
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

    /// The MQTT topic to publish commands to change the selected option.
    pub fn command_topic<T: Into<String>>(mut self, command_topic: T) -> Self {
        self.command_topic = command_topic.into();
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

    /// The name of the Select. Can be set to `null` if only the device name is relevant.
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    /// Flag that defines if the select works in optimistic mode.
    pub fn optimistic(mut self, optimistic: bool) -> Self {
        self.optimistic = Some(optimistic);
        self
    }

    /// List of options that can be selected. An empty list or a list with a single item is allowed.
    pub fn options<T: Into<String>>(mut self, options: Vec<T>) -> Self {
        self.options = options.into_iter().map(|v| v.into()).collect();
        self
    }

    /// Must be `select`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
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

    /// The MQTT topic subscribed to receive update of the selected option. A "None" payload resets to an `unknown` state. An empty payload is ignored.
    pub fn state_topic<T: Into<String>>(mut self, state_topic: T) -> Self {
        self.state_topic = Some(state_topic.into());
        self
    }

    /// An ID that uniquely identifies this Select. If two Selects have the same unique ID Home Assistant will raise an exception. Required when used with device-based discovery.
    pub fn unique_id<T: Into<String>>(mut self, unique_id: T) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the value.
    pub fn value_template<T: Into<String>>(mut self, value_template: T) -> Self {
        self.value_template = Some(value_template.into());
        self
    }
}

impl Default for Select {
    fn default() -> Self {
        Self {
            topic_prefix: Default::default(),
            origin: Default::default(),
            device: Default::default(),
            entity_category: Default::default(),
            availability: Default::default(),
            command_template: Default::default(),
            command_topic: Default::default(),
            enabled_by_default: Default::default(),
            encoding: Default::default(),
            entity_picture: Default::default(),
            icon: Default::default(),
            json_attributes_template: Default::default(),
            json_attributes_topic: Default::default(),
            name: Default::default(),
            object_id: Default::default(),
            optimistic: Default::default(),
            options: Default::default(),
            platform: "select".to_string(),
            qos: Default::default(),
            retain: Default::default(),
            state_topic: Default::default(),
            unique_id: Default::default(),
            value_template: Default::default(),
        }
    }
}

impl From<Select> for Entity {
    fn from(value: Select) -> Self {
        Entity::Select(value)
    }
}
