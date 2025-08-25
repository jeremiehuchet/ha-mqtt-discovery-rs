#![recursion_limit = "256"]

use anyhow::{Result, anyhow};
use rumqttc::v5::{
    AsyncClient,
    mqttbytes::{QoS::AtLeastOnce, v5::PublishProperties},
};
use serde::Serialize;

pub use rumqttc::v5;
use serde_json::Value;

pub mod common;

mod generated;
pub use generated::entities::*;
pub use generated::*;

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
    /// - `<discovery_prefix>`: The Discovery Prefix defaults to homeassistant. This prefix can be changed.
    /// - `<component>`: One of the supported MQTT integrations, eg. binary_sensor.
    /// - `<node_id>` (Optional): ID of the node providing the topic, this is not used by Home Assistant but may be used to structure the MQTT topic. The ID of the node must only consist of characters from the character class [a-zA-Z0-9_-] (alphanumerics, underscore and hyphen).
    /// - `<object_id>`: The ID of the device. This is only to allow for separate topics for each device and is not used for the entity_id. The ID of the device must only consist of characters from the character class [a-zA-Z0-9_-] (alphanumerics, underscore and hyphen).
    ///
    /// The `<node_id>` level can be used by clients to only subscribe to their own (command) topics by using one wildcard topic like <discovery_prefix>/+/<node_id>/+/set.
    ///
    /// Best practice for entities with a unique_id is to set `<object_id>` to unique_id and omit the `<node_id>`.
    pub async fn publish_entity(&self, entity: Entity) -> Result<()> {
        let component = entity.get_component_name();
        let attributes = entity.get_attributes()?;
        let object_id = attributes
            .as_object()
            .ok_or(anyhow!("entity configuration should be an object"))?
            .get("uniq_id")
            .ok_or(anyhow!(
                "entity configuration should have an attribute 'uniq_id'"
            ))?
            .as_str()
            .ok_or(anyhow!("'uniq_id' attribute should be a string"))?;
        let prefix = self
            .discovery_prefix
            .strip_suffix("/")
            .unwrap_or(&self.discovery_prefix);
        let topic = format!("{prefix}/{component}/{object_id}/config");
        let payload = serde_json::ser::to_string(&attributes).unwrap();
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

#[derive(Clone)]
pub enum Entity {
    AlarmControlPanel(AlarmControlPanel),
    BinarySensor(BinarySensor),
    Button(Button),
    Camera(Camera),
    Climate(Climate),
    Cover(Cover),
    DeviceTracker(DeviceTracker),
    DeviceTrigger(DeviceTrigger),
    Event(Event),
    Fan(Fan),
    Humidifier(Humidifier),
    Image(Image),
    LawnMower(LawnMower),
    Light(Light),
    Lock(Lock),
    Notify(Notify),
    Number(Number),
    Scene(Scene),
    Select(Select),
    Sensor(Sensor),
    Siren(Siren),
    Switch(Switch),
    Tag(Tag),
    Text(Text),
    Update(Update),
    Vacuum(Vacuum),
    Valve(Valve),
    WaterHeater(WaterHeater),
}

impl Entity {
    fn get_component_name(&self) -> &str {
        match self {
            Entity::AlarmControlPanel(_) => "alarm_control_panel",
            Entity::BinarySensor(_) => "binary_sensor",
            Entity::Button(_) => "button",
            Entity::Camera(_) => "camera",
            Entity::Climate(_) => "climate",
            Entity::Cover(_) => "cover",
            Entity::DeviceTracker(_) => "device_tracker",
            Entity::DeviceTrigger(_) => "device_trigger",
            Entity::Event(_) => "event",
            Entity::Fan(_) => "fan",
            Entity::Humidifier(_) => "humidifier",
            Entity::Image(_) => "image",
            Entity::LawnMower(_) => "lawn_mower",
            Entity::Light(_) => "light",
            Entity::Lock(_) => "lock",
            Entity::Notify(_) => "notify",
            Entity::Number(_) => "number",
            Entity::Scene(_) => "scene",
            Entity::Select(_) => "select",
            Entity::Sensor(_) => "sensor",
            Entity::Siren(_) => "siren",
            Entity::Switch(_) => "switch",
            Entity::Tag(_) => "tag",
            Entity::Text(_) => "text",
            Entity::Update(_) => "update",
            Entity::Vacuum(_) => "vacuum",
            Entity::Valve(_) => "valve",
            Entity::WaterHeater(_) => "water_heater",
        }
    }

    fn get_attributes(&self) -> Result<Value> {
        let attributes = match self {
            Entity::AlarmControlPanel(alarm_control_panel) => {
                serde_json::to_value(alarm_control_panel)?
            }
            Entity::BinarySensor(binary_sensor) => serde_json::to_value(binary_sensor)?,
            Entity::Button(button) => serde_json::to_value(button)?,
            Entity::Camera(camera) => serde_json::to_value(camera)?,
            Entity::Climate(climate) => serde_json::to_value(climate)?,
            Entity::Cover(cover) => serde_json::to_value(cover)?,
            Entity::DeviceTracker(device_tracker) => serde_json::to_value(device_tracker)?,
            Entity::DeviceTrigger(device_trigger) => serde_json::to_value(device_trigger)?,
            Entity::Event(event) => serde_json::to_value(event)?,
            Entity::Fan(fan) => serde_json::to_value(fan)?,
            Entity::Humidifier(humidifier) => serde_json::to_value(humidifier)?,
            Entity::Image(image) => serde_json::to_value(image)?,
            Entity::LawnMower(lawn_mower) => serde_json::to_value(lawn_mower)?,
            Entity::Light(light) => serde_json::to_value(light)?,
            Entity::Lock(lock) => serde_json::to_value(lock)?,
            Entity::Notify(notify) => serde_json::to_value(notify)?,
            Entity::Number(number) => serde_json::to_value(number)?,
            Entity::Scene(scene) => serde_json::to_value(scene)?,
            Entity::Select(select) => serde_json::to_value(select)?,
            Entity::Sensor(sensor) => serde_json::to_value(sensor)?,
            Entity::Siren(siren) => serde_json::to_value(siren)?,
            Entity::Switch(switch) => serde_json::to_value(switch)?,
            Entity::Tag(tag) => serde_json::to_value(tag)?,
            Entity::Text(text) => serde_json::to_value(text)?,
            Entity::Update(update) => serde_json::to_value(update)?,
            Entity::Vacuum(vacuum) => serde_json::to_value(vacuum)?,
            Entity::Valve(valve) => serde_json::to_value(valve)?,
            Entity::WaterHeater(water_heater) => serde_json::to_value(water_heater)?,
        };
        Ok(attributes)
    }
}
