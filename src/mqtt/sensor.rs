use super::common::Qos;
use super::common::SensorStateClass;
use super::common::{Availability, Device, EntityCategory, Origin};
use super::device_classes::SensorDeviceClass;
use super::units::Unit;
use serde_derive::Serialize;

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
/// To use your MQTT sensor in your installation, add the following to your `configuration.yaml` file:
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   sensor:
///     - name: "Bedroom Temperature"
///       state_topic: "home/bedroom/temperature"
/// ```
///
/// {% configuration %}
/// availability:
///   description: A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`.
///   required: false
///   type: list
///   keys:
///     payload_available:
///       description: The payload that represents the available state.
///       required: false
///       type: string
///       default: online
///     payload_not_available:
///       description: The payload that represents the unavailable state.
///       required: false
///       type: string
///       default: offline
///     topic:
///       description: An MQTT topic subscribed to receive availability (online/offline) updates.
///       required: true
///       type: string
///     value_template:
///       description: "Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract device's availability from the `topic`. To determine the devices's availability result of this template will be compared to `payload_available` and `payload_not_available`."
///       required: false
///       type: template
/// availability_mode:
///   description: When `availability` is configured, this controls the conditions needed to set the entity to `available`. Valid entries are `all`, `any`, and `latest`. If set to `all`, `payload_available` must be received on all configured availability topics before the entity is marked as online. If set to `any`, `payload_available` must be received on at least one configured availability topic before the entity is marked as online. If set to `latest`, the last `payload_available` or `payload_not_available` received on any configured availability topic controls the availability.
///   required: false
///   type: string
///   default: latest
/// availability_template:
///   description: "Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract device's availability from the `availability_topic`. To determine the devices's availability result of this template will be compared to `payload_available` and `payload_not_available`."
///   required: false
///   type: template
/// availability_topic:
///   description: The MQTT topic subscribed to receive availability (online/offline) updates.
///   required: false
///   type: string
/// device:
///   description: "Information about the device this sensor is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/device_registry_index/). Only works when [`unique_id`](#unique_id) is set. At least one of identifiers or connections must be present to identify the device."
///   required: false
///   type: map
///   keys:
///     configuration_url:
///       description: 'A link to the webpage that can manage the configuration of this device. Can be either an `http://`, `https://` or an internal `homeassistant://` URL.'
///       required: false
///       type: string
///     connections:
///       description: 'A list of connections of the device to the outside world as a list of tuples `[connection_type, connection_identifier]`. For example the MAC address of a network interface: `"connections": [["mac", "02:5b:26:a8:dc:12"]]`.'
///       required: false
///       type: list
///     hw_version:
///       description: The hardware version of the device.
///       required: false
///       type: string
///     identifiers:
///       description: A list of IDs that uniquely identify the device. For example a serial number.
///       required: false
///       type: [string, list]
///     manufacturer:
///       description: The manufacturer of the device.
///       required: false
///       type: string
///     model:
///       description: The model of the device.
///       required: false
///       type: string
///     name:
///       description: The name of the device.
///       required: false
///       type: string
///     serial_number:
///       description: "The serial number of the device."
///       required: false
///       type: string
///     suggested_area:
///       description: 'Suggest an area if the device isn’t in one yet.'
///       required: false
///       type: string
///     sw_version:
///       description: The firmware version of the device.
///       required: false
///       type: string
///     via_device:
///       description: 'Identifier of a device that routes messages between this device and Home Assistant. Examples of such devices are hubs, or parent devices of a sub-device. This is used to show device topology in Home Assistant.'
///       required: false
///       type: string
/// device_class:
///   description: The [type/class](/integrations/sensor/#device-class) of the sensor to set the icon in the frontend. The `device_class` can be `null`.
///   required: false
///   type: device_class
/// enabled_by_default:
///   description: Flag which defines if the entity should be enabled when first added.
///   required: false
///   type: boolean
///   default: true
/// encoding:
///   description: The encoding of the payloads received. Set to `""` to disable decoding of incoming payload.
///   required: false
///   type: string
///   default: "utf-8"
/// entity_category:
///   description: The [category](https://developers.home-assistant.io/docs/core/entity#generic-properties) of the entity. When set, the entity category must be `diagnostic` for sensors.
///   required: false
///   type: string
/// expire_after:
///   description: If set, it defines the number of seconds after the sensor's state expires, if it's not updated. After expiry, the sensor's state becomes `unavailable`. Default the sensors state never expires.
///   required: false
///   type: integer
///   default: 0
/// force_update:
///   description: Sends update events even if the value hasn't changed. Useful if you want to have meaningful value graphs in history.
///   required: false
///   type: boolean
///   default: false
/// icon:
///   description: "[Icon](/docs/configuration/customizing-devices/#icon) for the entity."
///   required: false
///   type: icon
/// json_attributes_template:
///   description: "Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`."
///   required: false
///   type: template
/// json_attributes_topic:
///   description: The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Implies `force_update` of the current sensor state when a message is received on this topic.
///   required: false
///   type: string
/// last_reset_value_template:
///   description: "Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the last_reset. When `last_reset_value_template` is set, the `state_class` option must be `total`. Available variables: `entity_id`. The `entity_id` can be used to reference the entity's attributes."
///   required: false
///   type: template
/// name:
///   description: The name of the MQTT sensor. Can be set to `null` if only the device name is relevant.
///   required: false
///   type: string
///   default: MQTT Sensor
/// object_id:
///   description: Used instead of `name` for automatic generation of `entity_id`
///   required: false
///   type: string
/// payload_available:
///   description: The payload that represents the available state.
///   required: false
///   type: string
///   default: online
/// payload_not_available:
///   description: The payload that represents the unavailable state.
///   required: false
///   type: string
///   default: offline
/// suggested_display_precision:
///   description: The number of decimals which should be used in the sensor's state after rounding.
///   required: false
///   type: integer
/// qos:
///   description: The maximum QoS level to be used when receiving and publishing messages.
///   required: false
///   type: integer
///   default: 0
/// state_class:
///   description: The [state_class](https://developers.home-assistant.io/docs/core/entity/sensor#available-state-classes) of the sensor.
///   required: false
///   type: string
/// state_topic:
///   description: The MQTT topic subscribed to receive sensor values. If `device_class`, `state_class`, `unit_of_measurement` or `suggested_display_precision` is set, and a numeric value is expected, an empty value `''` will be ignored and will not update the state, a `'null'` value will set the sensor to an `unknown` state. The `device_class` can be `null`.
///   required: true
///   type: string
/// unique_id:
///   description: "An ID that uniquely identifies this sensor. If two sensors have the same unique ID, Home Assistant will raise an exception."
///   required: false
///   type: string
/// unit_of_measurement:
///   description: Defines the units of measurement of the sensor, if any. The `unit_of_measurement` can be `null`.
///   required: false
///   type: string
/// value_template:
///   description: "Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the value. If the template throws an error, the current state will be used instead."
///   required: false
///   type: template
/// {% endconfiguration %}
///
/// ## Examples
///
/// In this section, you find some real-life examples showing how to use this sensor.
///
/// ### Processing Unix EPOCH timestamps
///
/// The example below shows how an MQTT sensor can process a Unix EPOCH payload.
///
/// {% raw %}
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
/// {% endraw %}
///
/// Or set up via MQTT discovery:
///
/// Discovery topic: `homeassistant/sensor/hp_1231232/config`
///
/// {% raw %}
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
/// {% endraw %}
///
/// To test, you can use the command line tool `mosquitto_pub` shipped with `mosquitto` or the `mosquitto-clients` package to send MQTT messages.
///
/// Payload topic: `pump/timestamp_on`
/// Payload: `1707294116`
///
/// To set the state of the sensor manually:
///
/// ```bash
/// mosquitto_pub -h 127.0.0.1 -u username -p some_password -t pump/timestamp_on -m '1707294116'
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
/// {% raw %}
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
/// {% endraw %}
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
/// {% raw %}
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
/// {% endraw %}
///
/// The state and the attributes of the sensor by design do not update in a synchronous manner if they share the same MQTT topic. Temporal mismatches between the state and the attribute data may occur if both the state and the attributes are changed simultaneously by the same MQTT message. An automation that triggers on any state change of the sensor will also trigger both on the change of the state or a change of the attributes. Such automations will be triggered twice if both the state and the attributes change. Please use a [MQTT trigger](/docs/automation/trigger/#mqtt-trigger) and process the JSON in the automation directly via the {% raw %}`{{ trigger.payload_json }}`{% endraw %} [trigger data](/docs/automation/templating/#mqtt) for automations that must synchronously handle multiple JSON values within the same MQTT message.
///
/// ### Usage of `entity_id` in the template
///
/// The example below shows how a simple filter, that calculates the value by adding 90% of the new value and 10% of the previous value, can be implemented in a template.
///
/// {% raw %}
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
/// {% endraw %}
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
/// {% raw %}
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
/// {% endraw %}
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
/// {% raw %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   sensor:
///     - name: "Temperature"
///       state_topic: "office/sensor1"
///       suggested_display_precision: 1
///       unit_of_measurement: "°C"
///       value_template: "{{ value_json.temperature }}
///     - name: "Humidity"
///       state_topic: "office/sensor1"
///       unit_of_measurement: "%"
///       value_template: "{{ value_json.humidity }}"
/// ```
///
/// {% endraw %}
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
#[derive(Clone, Debug, PartialEq, Serialize, Default)]
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
    pub device: Device,

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

    /// Sends update events even if the value hasn't changed. Useful if you want to have meaningful value graphs in history.
    #[serde(rename = "frc_upd", skip_serializing_if = "Option::is_none")]
    pub force_update: Option<bool>,

    /// [Icon](/docs/configuration/customizing-devices/#icon) for the entity.
    #[serde(rename = "ic", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`.
    #[serde(rename = "json_attr_tpl", skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Implies `force_update` of the current sensor state when a message is received on this topic.
    #[serde(rename = "json_attr_t", skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the last_reset. When `last_reset_value_template` is set, the `state_class` option must be `total`. Available variables: `entity_id`. The `entity_id` can be used to reference the entity's attributes.
    #[serde(rename = "lrst_val_tpl", skip_serializing_if = "Option::is_none")]
    pub last_reset_value_template: Option<String>,

    /// The name of the MQTT sensor. Can be set to `null` if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used instead of `name` for automatic generation of `entity_id`
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// The number of decimals which should be used in the sensor's state after rounding.
    #[serde(rename = "sug_dsp_prc", skip_serializing_if = "Option::is_none")]
    pub suggested_display_precision: Option<i32>,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// The [state_class](https://developers.home-assistant.io/docs/core/entity/sensor#available-state-classes) of the sensor.
    #[serde(rename = "stat_cla", skip_serializing_if = "Option::is_none")]
    pub state_class: Option<SensorStateClass>,

    /// The MQTT topic subscribed to receive sensor values. If `device_class`, `state_class`, `unit_of_measurement` or `suggested_display_precision` is set, and a numeric value is expected, an empty value `''` will be ignored and will not update the state, a `'null'` value will set the sensor to an `unknown` state. The `device_class` can be `null`.
    #[serde(rename = "stat_t")]
    pub state_topic: String,

    /// An ID that uniquely identifies this sensor. If two sensors have the same unique ID, Home Assistant will raise an exception.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,

    /// Defines the units of measurement of the sensor, if any. The `unit_of_measurement` can be `null`.
    #[serde(rename = "unit_of_meas", skip_serializing_if = "Option::is_none")]
    pub unit_of_measurement: Option<Unit>,

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the value. If the template throws an error, the current state will be used instead.
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

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`.
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

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the last_reset. When `last_reset_value_template` is set, the `state_class` option must be `total`. Available variables: `entity_id`. The `entity_id` can be used to reference the entity's attributes.
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

    /// Used instead of `name` for automatic generation of `entity_id`
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    /// The number of decimals which should be used in the sensor's state after rounding.
    pub fn suggested_display_precision(mut self, suggested_display_precision: i32) -> Self {
        self.suggested_display_precision = Some(suggested_display_precision);
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

    /// The MQTT topic subscribed to receive sensor values. If `device_class`, `state_class`, `unit_of_measurement` or `suggested_display_precision` is set, and a numeric value is expected, an empty value `''` will be ignored and will not update the state, a `'null'` value will set the sensor to an `unknown` state. The `device_class` can be `null`.
    pub fn state_topic<T: Into<String>>(mut self, state_topic: T) -> Self {
        self.state_topic = state_topic.into();
        self
    }

    /// An ID that uniquely identifies this sensor. If two sensors have the same unique ID, Home Assistant will raise an exception.
    pub fn unique_id<T: Into<String>>(mut self, unique_id: T) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }

    /// Defines the units of measurement of the sensor, if any. The `unit_of_measurement` can be `null`.
    pub fn unit_of_measurement<T: Into<Unit>>(mut self, unit_of_measurement: T) -> Self {
        self.unit_of_measurement = Some(unit_of_measurement.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the value. If the template throws an error, the current state will be used instead.
    pub fn value_template<T: Into<String>>(mut self, value_template: T) -> Self {
        self.value_template = Some(value_template.into());
        self
    }
}
