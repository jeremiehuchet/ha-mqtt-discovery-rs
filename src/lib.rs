#![recursion_limit = "256"]

use anyhow::{Result, anyhow};
use bon::bon;
pub use rumqttc::v5;
use rumqttc::v5::{
    AsyncClient,
    mqttbytes::{QoS::AtLeastOnce, v5::PublishProperties},
};
use serde::Serialize;
use std::collections::HashMap;

pub mod common;

mod generated;
use crate::common::{Availability, DeviceInformation, Origin, Qos};
pub use generated::entities::*;
pub use generated::*;
use regex::Regex;
use unicode_normalization::UnicodeNormalization;

const ONE_WEEK_SECONDS: u32 = 60 * 60 * 24 * 7;

#[derive(Clone)]
pub struct HomeAssistantMqtt {
    client: AsyncClient,
    discovery_prefix: String,
}

impl HomeAssistantMqtt {
    pub fn new<S: Into<String>>(client: AsyncClient, discovery_prefix: S) -> Self {
        Self {
            client,
            discovery_prefix: discovery_prefix.into(),
        }
    }

    /// The discovery topic needs to follow a specific format:
    /// `<discovery_prefix>/<component>/[<node_id>/]<object_id>/config`
    ///
    /// - `<discovery_prefix>`: The Discovery Prefix defaults to `homeassistant`. This prefix can be changed.
    /// - `<component>`: One of the supported MQTT integrations, eg. `binary_sensor`.
    /// - `<node_id>` (Optional): ID of the node providing the topic, this is not used by Home Assistant but may be used to structure the MQTT topic. The ID of the node must only consist of characters from the character class [a-zA-Z0-9_-] (alphanumerics, underscore and hyphen).
    /// - `<object_id>`: The ID of the device. This is only to allow for separate topics for each device and is not used for the `entity_id`. The ID of the device must only consist of characters from the character class [a-zA-Z0-9_-] (alphanumerics, underscore and hyphen).
    ///
    /// The `<node_id>` level can be used by clients to only subscribe to their own (command) topics by using one wildcard topic like <discovery_prefix>/+/<node_id>/+/set.
    ///
    /// Best practice for entities with a unique_id is to set `<object_id>` to unique_id and omit the `<node_id>`.
    pub async fn publish_entity(&self, entity: Entity) -> Result<()> {
        let component = entity.get_platform();
        let unique_id = entity
            .get_unique_id()
            .expect("'uniq_id' attribute should be defined");
        let prefix = self
            .discovery_prefix
            .strip_suffix("/")
            .unwrap_or(&self.discovery_prefix);
        let topic = format!("{prefix}/{component}/{unique_id}/config");
        let payload = serde_json::ser::to_string(&entity).unwrap();
        let props = PublishProperties {
            //payload_format_indicator: Some(1),
            message_expiry_interval: Some(ONE_WEEK_SECONDS),
            content_type: Some("application/json".to_string()),
            ..Default::default()
        };
        Ok(self
            .client
            .publish_with_properties(topic, AtLeastOnce, true, payload, props)
            .await?)
    }

    /// The discovery topic needs to follow a specific format:
    /// `<discovery_prefix>/<component>/[<node_id>/]<object_id>/config`
    ///
    /// - `<discovery_prefix>`: The Discovery Prefix defaults to `homeassistant` and this prefix can be changed.
    /// - `<component>`: One of the supported MQTT integrations, e.g., `binary_sensor`, or `device` in case of a device discovery.
    /// - `<node_id>`: (Optional): ID of the node providing the topic, this is not used by Home Assistant but may be used to structure the MQTT topic. The ID of the node must only consist of characters from the character class `[a-zA-Z0-9_-]` (alphanumerics, underscore and hyphen).
    /// - `<object_id>`: The ID of the device. This is only to allow for separate topics for each device and is not used for the `entity_id`. The ID of the device must only consist of characters from the character class `[a-zA-Z0-9_-]` (alphanumerics, underscore and hyphen).
    ///
    /// The <node_id> level can be used by clients to only subscribe to their own (command) topics by using one wildcard topic like `<discovery_prefix>/+/<node_id>/+/set`.
    ///
    /// Best practice for entities with a unique_id is to set <object_id> to unique_id and omit the <node_id>.
    pub async fn publish_device(&self, device: DeviceComponents) -> Result<()> {
        let prefix = self
            .discovery_prefix
            .strip_suffix("/")
            .unwrap_or(&self.discovery_prefix);
        let unique_id = device.unique_id();
        let topic = format!("{prefix}/device/{unique_id}/config");
        let payload = serde_json::ser::to_string(&device)?;
        let props = PublishProperties {
            //payload_format_indicator: Some(1),
            message_expiry_interval: Some(ONE_WEEK_SECONDS),
            content_type: Some("application/json".to_string()),
            ..Default::default()
        };
        Ok(self
            .client
            .publish_with_properties(topic, AtLeastOnce, true, payload, props)
            .await?)
    }

    pub async fn publish_data<S: Serialize>(
        &self,
        topic: &String,
        payload: &S,
        message_expiry_interval: Option<u32>,
    ) -> Result<()> {
        let payload = serde_json::ser::to_string(payload).unwrap();
        let props = PublishProperties {
            message_expiry_interval,
            content_type: Some("application/json".to_string()),
            ..Default::default()
        };
        Ok(self
            .client
            .publish_with_properties(topic, AtLeastOnce, true, payload, props)
            .await?)
    }
}

/// A device with multiple components declared at once.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DeviceComponents {
    /// It is encouraged to add additional information about the origin that supplies MQTT entities via MQTT discovery by adding the origin option (can be abbreviated to o) to the discovery payload. Note that these options also support abbreviations. Information of the origin will be logged to the core event log when an item is discovered or updated.
    #[serde(rename = "o")]
    pub origin: Origin,

    /// Information about the device this button is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/en/device_registry_index.html). Only works when [`unique_id`](#unique_id) is set. At least one of identifiers or connections must be present to identify the device.
    #[serde(rename = "dev")]
    pub device: DeviceInformation,

    /// A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub availability: Option<Availability>,

    /// Components of the device.
    #[serde(rename = "cmps")]
    pub components: HashMap<String, Entity>,

    /// Replaces `~` with this value in any MQTT topic attribute.
    /// [See Home Assistant documentation](https://www.home-assistant.io/integrations/mqtt/#using-abbreviations-and-base-topic)
    #[serde(rename = "~", skip_serializing_if = "Option::is_none")]
    pub topic_prefix: Option<String>,

    /// The MQTT topic subscribed to receive state updates. A "None" payload resets to an `unknown` state. An empty payload is ignored. Valid state payloads are `OFF` and `ON`. Custom `OFF` and `ON` values can be set with the `payload_off` and `payload_on` config options.
    #[serde(rename = "stat_t", skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,

    /// The MQTT topic to publish commands to change the alarm state.
    #[serde(rename = "cmd_t", skip_serializing_if = "Option::is_none")]
    pub command_topic: Option<String>,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// The encoding of the payloads received and published messages. Set to `""` to disable decoding of incoming payload.
    #[serde(rename = "e", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
}

#[bon]
impl DeviceComponents {
    #[builder]
    pub fn new(
        #[builder(field)] components: HashMap<String, Entity>,
        origin: Origin,
        device: DeviceInformation,
        availability: Option<Availability>,
        #[builder(into)] topic_prefix: Option<String>,
        #[builder(into)] state_topic: Option<String>,
        #[builder(into)] command_topic: Option<String>,
        qos: Option<Qos>,
        #[builder(into)] encoding: Option<String>,
    ) -> Self {
        DeviceComponents {
            origin,
            device,
            availability,
            components: components.into_iter().map(|(k, v)| (k.into(), v)).collect(),
            topic_prefix,
            state_topic,
            command_topic,
            qos,
            encoding,
        }
    }

    fn unique_id(&self) -> String {
        slug(
            self.device
                .identifiers
                .first()
                .expect("a device must have at least one identifier"),
        )
    }
}

impl<S: device_components_builder::State> DeviceComponentsBuilder<S> {
    pub fn component<N: Into<String>>(mut self, name: N, value: Entity) -> Self {
        // `self.levels` is accessible in the builder
        self.components.insert(name.into(), value);
        self
    }
}

fn slug(string: &String) -> String {
    let nfkd = string.nfkd().to_string();
    let without_diacritics = Regex::new(r"\p{M}").unwrap().replace_all(&nfkd, "");
    Regex::new(r"[^a-zA-Z0-9_-]")
        .unwrap()
        .replace_all(&without_diacritics.to_string(), "_")
        .to_string()
}
