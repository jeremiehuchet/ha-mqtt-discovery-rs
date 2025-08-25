use super::common::Qos;
use super::common::{Availability, Device, EntityCategory, Origin};
use crate::Entity;
use serde_derive::Serialize;

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
    pub device: Device,

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
