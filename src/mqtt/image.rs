use super::common::{Availability, Device, EntityCategory, Origin};
use crate::Entity;
use serde_derive::Serialize;

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
/// To enable this image in your installation, add the following to your {% term "`configuration.yaml`" %} file:
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - image:
///       url_topic: mynas/status/url
/// ```
///
///
/// ### Example receiving images from a URL
///
/// Add the configuration below to your {% term "`configuration.yaml`" %}.
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
/// Add the configuration below to your {% term "`configuration.yaml`" %}.
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
    pub device: Device,

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

    /// Used instead of `name` for automatic generation of `entity_id`
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

    /// Used instead of `name` for automatic generation of `entity_id`
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
