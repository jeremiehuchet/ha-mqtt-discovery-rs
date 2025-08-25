use serde::ser::SerializeSeq;
use serde_derive::Serialize;

/// Classification of a non-primary entity.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum EntityCategory {
    /// The entity allows changing the configuration of a device,
    /// for example a switch entity making it possible to turn the background illumination of a switch on and off.
    #[serde(rename = "config")]
    Config,

    /// THe entity exposes some configuration parameter or diagnostics of a device but does not allow changing it,
    /// for example a sensor showing RSSI or MAC-address.
    #[serde(rename = "diagnostic")]
    Diagnostic,
}

/// It is encouraged to add additional information about the origin that supplies MQTT entities via MQTT discovery by adding the origin option (can be abbreviated to o) to the discovery payload. Note that these options also support abbreviations. Information of the origin will be logged to the core event log when an item is discovered or updated.
#[derive(Clone, Debug, PartialEq, Serialize, Default)]
pub struct Origin {
    /// The name of the application that is the origin the discovered MQTT item. This option is required.
    #[serde(rename = "name")]
    pub name: String,
    /// Software version of the application that supplies the discovered MQTT item.
    #[serde(rename = "sw", skip_serializing_if = "Option::is_none")]
    pub sw_version: Option<String>,
    /// Support URL of the application that supplies the discovered MQTT item.
    #[serde(rename = "support_url", skip_serializing_if = "Option::is_none")]
    pub support_url: Option<String>,
}

impl Origin {
    /// The name of the application that is the origin the discovered MQTT item. This option is required.
    pub fn new<S: Into<String>>(name: S) -> Self {
        Origin {
            name: name.into(),
            ..Default::default()
        }
    }

    /// Software version of the application that supplies the discovered MQTT item.
    pub fn with_sw_version<S: Into<String>>(mut self, sw_version: S) -> Self {
        self.sw_version = Some(sw_version.into());
        self
    }

    /// Support URL of the application that supplies the discovered MQTT item.
    pub fn with_support_url<S: Into<String>>(mut self, support_url: S) -> Self {
        self.support_url = Some(support_url.into());
        self
    }
}

/// Information about the device this sensor is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/device_registry_index/). Only works when `unique_id` is set. At least one of identifiers or connections must be present to identify the device.
#[derive(Clone, Debug, PartialEq, Serialize, Default)]
pub struct Device {
    /// The name of the device.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A list of IDs that uniquely identify the device. For example a serial number.
    #[serde(rename = "ids", skip_serializing_if = "Vec::is_empty")]
    pub identifiers: Vec<String>,
    /// A list of connections of the device to the outside world as a list of tuples `[connection_type, connection_identifier]`. For example the MAC address of a network interface: `"connections": [["mac", "02:5b:26:a8:dc:12"]]`.
    #[serde(rename = "cns", skip_serializing_if = "Vec::is_empty")]
    pub connections: Vec<DeviceConnection>,
    /// A link to the webpage that can manage the configuration of this device. Can be either an `http://`, `https://` or an internal `homeassistant://` URL.
    #[serde(rename = "cu", skip_serializing_if = "Option::is_none")]
    pub configuration_url: Option<String>,
    /// The manufacturer of the device.
    #[serde(rename = "mf", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// The model of the device.
    #[serde(rename = "mdl", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Suggest an area if the device isn’t in one yet.
    #[serde(rename = "sa", skip_serializing_if = "Option::is_none")]
    pub suggested_area: Option<String>,
    /// The firmware version of the device.
    #[serde(rename = "sw", skip_serializing_if = "Option::is_none")]
    pub sw_version: Option<String>,
    /// The hardware version of the device.
    #[serde(rename = "hw", skip_serializing_if = "Option::is_none")]
    pub hw_version: Option<String>,
    /// Identifier of a device that routes messages between this device and Home Assistant. Examples of such devices are hubs, or parent devices of a sub-device. This is used to show device topology in Home Assistant.
    #[serde(rename = "via_device", skip_serializing_if = "Option::is_none")]
    pub via_device: Option<String>,
}

impl Device {
    /// The name of the device.
    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Add an ID that uniquely identify the device. For example a serial number.
    pub fn add_identifier<S: Into<String>>(mut self, identifier: S) -> Self {
        self.identifiers.push(identifier.into());
        self
    }

    /// Add an ID that uniquely identify the device. For example a serial number.
    pub fn add_connection(mut self, connection: DeviceConnection) -> Self {
        self.connections.push(connection);
        self
    }

    /// A link to the webpage that can manage the configuration of this device. Can be either an `http://`, `https://` or an internal `homeassistant://` URL.
    pub fn configuration_url<S: Into<String>>(mut self, configuration_url: S) -> Self {
        self.configuration_url = Some(configuration_url.into());
        self
    }

    /// The manufacturer of the device.
    pub fn manufacturer<S: Into<String>>(mut self, manufacturer: S) -> Self {
        self.manufacturer = Some(manufacturer.into());
        self
    }

    /// The model of the device.
    pub fn model<S: Into<String>>(mut self, model: S) -> Self {
        self.model = Some(model.into());
        self
    }

    /// Suggest an area if the device isn’t in one yet.
    pub fn suggested_area<S: Into<String>>(mut self, suggested_area: S) -> Self {
        self.suggested_area = Some(suggested_area.into());
        self
    }

    /// The firmware version of the device.
    pub fn sw_version<S: Into<String>>(mut self, sw_version: S) -> Self {
        self.sw_version = Some(sw_version.into());
        self
    }

    /// The hardware version of the device.
    pub fn hw_version<S: Into<String>>(mut self, hw_version: S) -> Self {
        self.hw_version = Some(hw_version.into());
        self
    }

    /// Identifier of a device that routes messages between this device and Home Assistant. Examples of such devices are hubs, or parent devices of a sub-device. This is used to show device topology in Home Assistant.
    pub fn via_device<S: Into<String>>(mut self, via_device: S) -> Self {
        self.via_device = Some(via_device.into());
        self
    }
}

/// A tuple `[connection_type, connection_identifier]`.
/// For example the MAC address of a network interface: `["mac", "02:5b:26:a8:dc:12"]`.
#[derive(Clone, Debug, PartialEq)]
pub struct DeviceConnection {
    pub r#type: String,
    pub identifier: String,
}

impl DeviceConnection {
    pub fn mac<S: Into<String>>(mac_address: S) -> Self {
        DeviceConnection {
            r#type: "mac".to_string(),
            identifier: mac_address.into(),
        }
    }
}

impl serde::ser::Serialize for DeviceConnection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&self.r#type)?;
        seq.serialize_element(&self.identifier)?;
        seq.end()
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum SensorStateClass {
    /// The state represents a measurement in present time, not a historical aggregation such as statistics or a prediction of the future.
    ///
    /// Examples of what should be classified measurement are: current temperature, humidify or electric power.
    /// Examples of what should not be classified as measurement: Forecasted temperature for tomorrow, yesterday's energy consumption or anything else that doesn't include the current measurement.
    /// For supported sensors, statistics of hourly min, max and average sensor readings is updated every 5 minutes.
    #[serde(rename = "measurement")]
    Measurement,

    /// The state represents a total amount that can both increase and decrease, e.g. a net energy meter.
    ///
    /// Statistics of the accumulated growth or decline of the sensor's value since it was first added is updated every 5 minutes.
    /// This state class should not be used for sensors where the absolute value is interesting instead of the accumulated growth or decline, for example remaining battery capacity or CPU load; in such cases state class measurement should be used instead.
    #[serde(rename = "total")]
    Total,

    /// Similar to total, with the restriction that the state represents a monotonically increasing positive total which periodically restarts counting from 0, e.g. a daily amount of consumed gas, weekly water consumption or lifetime energy consumption.
    ///
    /// Statistics of the accumulated growth of the sensor's value since it was first added is updated every 5 minutes.
    /// A decreasing value is interpreted as the start of a new meter cycle or the replacement of the meter.
    #[serde(rename = "total_increasing")]
    TotalIncreasing,
}

#[derive(Clone, Debug, PartialEq, Serialize, Default)]
pub struct Availability {
    /// Controls the conditions needed to set the entity to `available`.
    #[serde(rename = "avty_mode")]
    pub mode: AvailabilityMode,
    /// A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`.
    #[serde(rename = "avty")]
    pub availability: Vec<AvailabilityCheck>,
    /// If set, it defines the number of seconds after the sensor’s state expires, if it’s not updated.
    /// After expiry, the sensor’s state becomes unavailable. Default the sensors state never expires.
    /// (optional, default: 0)
    #[serde(rename = "exp_aft", skip_serializing_if = "Option::is_none")]
    pub expire_after: Option<u64>,
}

#[allow(dead_code)]
impl Availability {
    /// An availability checker using a single topic and the default `online` and `offline` payloads.
    pub fn single_topic(topic: &str) -> Self {
        Self::single(AvailabilityCheck {
            payload_available: None,
            payload_not_available: None,
            topic: topic.to_string(),
            value_template: None,
        })
    }

    /// An availability checker using a single check.
    pub fn single(availability: AvailabilityCheck) -> Self {
        Self {
            mode: AvailabilityMode::All,
            availability: vec![availability],
            expire_after: None,
        }
    }

    /// An availability checker requiring all the given checks.
    pub fn all(checks: Vec<AvailabilityCheck>) -> Self {
        Self {
            mode: AvailabilityMode::All,
            availability: checks,
            expire_after: None,
        }
    }

    /// An availability checker requiring any the given checks.
    pub fn any(checks: Vec<AvailabilityCheck>) -> Self {
        Self {
            mode: AvailabilityMode::Any,
            availability: checks,
            expire_after: None,
        }
    }

    /// See `AvailabilityCheck::Latest`
    pub fn latest(checks: Vec<AvailabilityCheck>) -> Self {
        Self {
            mode: AvailabilityMode::Latest,
            availability: checks,
            expire_after: None,
        }
    }

    /// Sets the number of seconds after the sensor’s state expires, if it’s not updated. After expiry, the sensor’s state becomes unavailable. Default the sensors state never expires.
    pub fn expire_after(mut self, expire_after: u64) -> Self {
        self.expire_after = Some(expire_after);
        self
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize, Default)]
pub enum AvailabilityMode {
    /// `payload_available` must be received on all configured availability topics before the entity is marked as online.
    #[serde(rename = "all")]
    #[default]
    All,
    /// `payload_available` must be received on at least one configured availability topic before the entity is marked as online.
    #[serde(rename = "any")]
    Any,
    /// the last `payload_available` or `payload_not_available` received on any configured availability topic controls the availability.
    #[serde(rename = "latest")]
    Latest,
}

#[derive(Clone, Debug, PartialEq, Serialize, Default)]
pub struct AvailabilityCheck {
    /// The payload that represents the available state. (optional, default: `online`)
    #[serde(rename = "pl_avail", skip_serializing_if = "Option::is_none")]
    pub payload_available: Option<String>,
    /// The payload that represents the unavailable state. (optional, default: `offline`)
    #[serde(rename = "pl_not_avail", skip_serializing_if = "Option::is_none")]
    pub payload_not_available: Option<String>,
    /// An MQTT topic subscribed to receive availability (online/offline) updates.
    #[serde(rename = "t")]
    pub topic: String,
    /// Defines a template to extract device’s availability from the topic. To determine the devices’s availability result of this template will be compared to payload_available and payload_not_available.
    #[serde(rename = "val_tpl", skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

impl AvailabilityCheck {
    /// An MQTT topic subscribed to receive availability (online/offline) updates.
    pub fn topic<S: Into<String>>(topic: S) -> Self {
        Self {
            topic: topic.into(),
            ..Default::default()
        }
    }

    /// The payload that represents the available state. (optional, default: `online`)
    pub fn payload_available<S: Into<String>>(mut self, payload_available: S) -> Self {
        self.payload_available = Some(payload_available.into());
        self
    }

    /// The payload that represents the unavailable state. (optional, default: `offline`)
    pub fn payload_not_available<S: Into<String>>(mut self, payload_not_available: S) -> Self {
        self.payload_not_available = Some(payload_not_available.into());
        self
    }

    /// Defines a template to extract device’s availability from the topic. To determine the devices’s availability result of this template will be compared to payload_available and payload_not_available.
    pub fn value_template<S: Into<String>>(mut self, value_template: S) -> Self {
        self.value_template = Some(value_template.into());
        self
    }
}

/// The maximum QoS level to be used when receiving and publishing messages.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum Qos {
    /// At most once (QoS 0)
    #[serde(rename = "0")]
    AtMostOnce,

    /// At least once (QoS 1)
    #[serde(rename = "1")]
    AtLeastOnce,

    /// Exactly once (QoS 2)
    #[serde(rename = "2")]
    ExactlyOnce,
}

/// Defines the temperature unit of the device, `C` or `F`. If this is not set, the temperature unit is set to the system temperature unit.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum TemperatureUnit {
    #[serde(rename = "C")]
    Celcius,

    #[serde(rename = "F")]
    Fahrenheit,
}

#[cfg(test)]
mod tests {
    use assert_json_diff::assert_json_eq;
    use serde_json::json;

    use super::*;

    #[test]
    fn can_serialize_origin() {
        let origin = Origin {
            name: "application name".to_string(),
            sw_version: Some("software version".to_string()),
            support_url: Some("https://github.com".to_string()),
        };
        assert_json_eq!(
            json! ({
              "name": "application name",
              "sw": "software version",
              "support_url": "https://github.com"
            }),
            serde_json::to_value(&origin).unwrap()
        );
    }

    #[test]
    fn can_serialize_device() {
        let device = Device {
            name: Some("device name".to_string()),
            identifiers: vec!["device id".to_string()],
            connections: vec![DeviceConnection::mac("connection id")],
            configuration_url: Some("http://config.url".to_string()),
            manufacturer: Some("device manufacturer".to_string()),
            model: Some("device model".to_string()),
            suggested_area: Some("area".to_string()),
            sw_version: Some("sw_v".to_string()),
            hw_version: Some("hw_v".to_string()),
            via_device: Some("via".to_string()),
        };
        assert_json_eq!(
            json! (
            {
              "name": "device name",
              "ids": [
                "device id"
              ],
              "cns": [
                [
                  "mac",
                  "connection id"
                ]
              ],
              "cu": "http://config.url",
              "mf": "device manufacturer",
              "mdl": "device model",
              "sa": "area",
              "sw": "sw_v",
              "hw": "hw_v",
              "via_device": "via"
            }),
            serde_json::to_value(&device).unwrap()
        );
    }
}
