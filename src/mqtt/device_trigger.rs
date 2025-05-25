use super::common::Qos;
use super::common::{Availability, Device, EntityCategory, Origin};
use crate::Entity;
use serde_derive::Serialize;

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
/// MQTT device triggers are only supported through [MQTT discovery](/integrations/mqtt/#mqtt-discovery), manual setup through {% term "`configuration.yaml`" %} is not supported.
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
    pub device: Device,

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
    #[serde(rename = "platform")]
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
