use super::common::Qos;
use super::common::{Availability, Device, EntityCategory, Origin};
use crate::Entity;
use serde_derive::Serialize;

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
/// To enable MQTT locks in your installation, add the following to your {% term "`configuration.yaml`" %} file:
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - lock:
///       command_topic: "home/frontdoor/set"
/// ```
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
    pub device: Device,

    /// A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`.
    #[serde(flatten)]
    pub availability: Availability,

    /// The category of the entity. (optional, default: None)
    #[serde(rename = "ent_cat", skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<EntityCategory>,

    /// A regular expression to validate a supplied code when it is set during the action to `open`, `lock` or `unlock` the MQTT lock.
    #[serde(rename = "code_format", skip_serializing_if = "Option::is_none")]
    pub code_format: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `command_topic`. The lock command template accepts the parameters `value` and `code`. The `value` parameter will contain the configured value for either `payload_open`, `payload_lock` or `payload_unlock`. The `code` parameter is set during the action to `open`, `lock` or `unlock` the MQTT lock and will be set `None` if no code was passed.
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

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation.
    #[serde(rename = "json_attr_tpl", skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation.
    #[serde(rename = "json_attr_t", skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,

    /// The name of the lock. Can be set to `null` if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used instead of `name` for automatic generation of `entity_id`
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
    #[serde(rename = "platform")]
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

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract a state value from the payload.
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

    /// A regular expression to validate a supplied code when it is set during the action to `open`, `lock` or `unlock` the MQTT lock.
    pub fn code_format<T: Into<String>>(mut self, code_format: T) -> Self {
        self.code_format = Some(code_format.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `command_topic`. The lock command template accepts the parameters `value` and `code`. The `value` parameter will contain the configured value for either `payload_open`, `payload_lock` or `payload_unlock`. The `code` parameter is set during the action to `open`, `lock` or `unlock` the MQTT lock and will be set `None` if no code was passed.
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

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation.
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

    /// Used instead of `name` for automatic generation of `entity_id`
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

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract a state value from the payload.
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
