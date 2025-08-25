use assert_json_diff::assert_json_eq;
use ha_mqtt_discovery::mqtt::units::TemperatureUnit;
use ha_mqtt_discovery::{
    Entity, HomeAssistantMqtt,
    mqtt::{
        binary_sensor::BinarySensor,
        common::{Availability, Device, DeviceConnection, Origin, SensorStateClass},
        device_classes::{BinarySensorDeviceClass, NumberDeviceClass, SensorDeviceClass},
        number::Number,
        sensor::Sensor,
    },
};
use rumqttc::v5::{
    AsyncClient,
    Event::Incoming,
    MqttOptions,
    mqttbytes::{
        QoS::ExactlyOnce,
        v5::{Packet, Publish},
    },
};
use rust_decimal_macros::dec;
use serde_json::{Value, json};
use std::time::Duration;
use testcontainers_modules::{mosquitto, testcontainers::runners::AsyncRunner};
use tokio::task;

fn origin() -> Origin {
    Origin::new("Integration test")
        .with_sw_version("0.0.1")
        .with_support_url("https://www.github.com")
}

fn device() -> Device {
    Device::default()
        .name("Barometer")
        .add_identifier("barometer-09AF")
        .add_connection(DeviceConnection::mac("09:AF:A4:54:F0:9D"))
        .configuration_url("https://barometer.home/admin")
        .manufacturer("Awesome corp")
        .model("Awesome model")
        .suggested_area("kitchen")
        .sw_version("0.1a")
        .hw_version("rev B")
        .via_device("manufacturer cloud")
}

async fn do_with_mosquitto(callback: fn(AsyncClient) -> ()) -> (Publish, Value) {
    // start a mosquitto container
    let mosquitto_container = mosquitto::Mosquitto::default().start().await.unwrap();

    // open a client to communicate with mosquitto container
    let mqtt_options = MqttOptions::new(
        "test",
        "127.0.0.1",
        mosquitto_container.get_host_port_ipv4(1883).await.unwrap(),
    );
    let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10);

    // listen all topics
    client
        .subscribe("#", ExactlyOnce)
        .await
        .expect("successful subscription to all topics");

    // executes callback
    callback(client.clone());

    // disconnect client in 2 seconds to avoid waiting indefinitely
    task::spawn(async move {
        tokio::time::sleep(Duration::from_secs(2)).await;
        client
            .disconnect()
            .await
            .expect("client to disconnect from mosquitto");
    });
    // wait for a text message
    while let Ok(message) = eventloop.poll().await {
        match message {
            Incoming(Packet::Publish(content)) => {
                let payload_string =
                    String::from_utf8(content.payload.to_vec()).expect("a valid UTF-8 string");
                return (
                    content,
                    serde_json::from_str(&payload_string).expect("a valid json"),
                );
            }
            _ => {}
        }
    }
    // or panic
    panic!("timeout waiting for expected message");
}

#[tokio::test]
async fn can_publish_a_binary_sensor_configuration() {
    let (packet, json) = do_with_mosquitto(|client| {
        let registry = HomeAssistantMqtt::new(client, "homeassistant/");
        tokio::spawn(async move {
            registry
                .publish_entity(Entity::BinarySensor(
                    BinarySensor::default()
                        .topic_prefix("temperature_devices/barometer-09AF")
                        .origin(origin())
                        .device(device())
                        .object_id("barometer-09AF")
                        .unique_id("barometer-09AF_state")
                        .availability(
                            Availability::single_topic("~/availability").expire_after(120),
                        )
                        .state_topic("~/state")
                        .value_template("{{ json_value.state }}")
                        .device_class(BinarySensorDeviceClass::Door)
                        .force_update(true)
                        .name("Barometer state")
                        .off_delay(10)
                        .payload_off("Off")
                        .payload_on("On"),
                ))
                .await
                .expect("message to be published");
        });
    })
    .await;

    assert_eq!(
        packet.topic,
        "homeassistant/binary_sensor/barometer-09AF_state/config"
    );
    assert_json_eq!(
        json,
        json!(
            {
                "p": "binary_sensor",
                "~": "temperature_devices/barometer-09AF",
                "o": {
                  "name": "Integration test",
                  "sw": "0.0.1",
                  "support_url": "https://www.github.com"
                },
                "dev": {
                  "name": "Barometer",
                  "ids": [
                    "barometer-09AF"
                  ],
                  "cns": [
                    [
                      "mac",
                      "09:AF:A4:54:F0:9D"
                    ]
                  ],
                  "cu": "https://barometer.home/admin",
                  "mf": "Awesome corp",
                  "mdl": "Awesome model",
                  "sa": "kitchen",
                  "sw": "0.1a",
                  "hw": "rev B",
                  "via_device": "manufacturer cloud"
                },
                "obj_id": "barometer-09AF",
                "uniq_id": "barometer-09AF_state",
                "avty_mode": "all",
                "avty": [
                  {
                    "t": "~/availability"
                  }
                ],
                "stat_t": "~/state",
                "val_tpl": "{{ json_value.state }}",
                "dev_cla": "door",
                "exp_aft": 120,
                "frc_upd": true,
                "name": "Barometer state",
                "off_dly": 10,
                "pl_off": "Off",
                "pl_on": "On"
              }
        )
    );
}

#[tokio::test]
async fn can_publish_a_number_configuration() {
    let (packet, json) = do_with_mosquitto(|client| {
        let registry = HomeAssistantMqtt::new(client, "homeassistant/");
        tokio::spawn(async move {
            registry
                .publish_entity(Entity::Number(
                    Number::default()
                        .topic_prefix("temperature_devices/barometer-09AF")
                        .origin(origin())
                        .device(device())
                        .object_id("barometer-09AF")
                        .unique_id("barometer-09AF_temperature_drift")
                        .availability(
                            Availability::single_topic("~/availability").expire_after(120),
                        )
                        .state_topic("~/state")
                        .value_template("{{ json_value.temperature }}")
                        .command_topic("~/command".to_string())
                        .command_template("{{ json_value.command }}".to_string())
                        .optimistic(false)
                        .retain(true)
                        .device_class(NumberDeviceClass::Temperature)
                        .name("Temperature drift")
                        .min(dec!(-10.0))
                        .max(dec!(10.0))
                        .mode("slider")
                        .payload_reset("NaN")
                        .step(dec!(0.1))
                        .unit_of_measurement(TemperatureUnit::Celsius),
                ))
                .await
                .expect("message to be published");
        });
    })
    .await;

    assert_eq!(
        packet.topic,
        "homeassistant/number/barometer-09AF_temperature_drift/config"
    );
    assert_json_eq!(
        json,
        json!(
            {
                "p": "number",
                "~": "temperature_devices/barometer-09AF",
                "o": {
                  "name": "Integration test",
                  "sw": "0.0.1",
                  "support_url": "https://www.github.com"
                },
                "dev": {
                  "name": "Barometer",
                  "ids": [
                    "barometer-09AF"
                  ],
                  "cns": [
                    [
                      "mac",
                      "09:AF:A4:54:F0:9D"
                    ]
                  ],
                  "cu": "https://barometer.home/admin",
                  "mf": "Awesome corp",
                  "mdl": "Awesome model",
                  "sa": "kitchen",
                  "sw": "0.1a",
                  "hw": "rev B",
                  "via_device": "manufacturer cloud"
                },
                "obj_id": "barometer-09AF",
                "uniq_id": "barometer-09AF_temperature_drift",
                "avty_mode": "all",
                "avty": [
                  {
                    "t": "~/availability"
                  }
                ],
                "stat_t": "~/state",
                "val_tpl": "{{ json_value.temperature }}",
                "cmd_t": "~/command",
                "cmd_tpl": "{{ json_value.command }}",
                "dev_cla": "temperature",
                "exp_aft": 120,
                "name": "Temperature drift",
                "min": -10.0,
                "max": 10.0,
                "mode": "slider",
                "pl_rst": "NaN",
                "step": 0.1,
                "opt": false,
                "ret": true,
                "unit_of_meas": "°C",
              }
        )
    );
}

#[tokio::test]
async fn can_publish_a_sensor_configuration() {
    let (packet, json) = do_with_mosquitto(|client| {
        let registry = HomeAssistantMqtt::new(client, "homeassistant/");
        tokio::spawn(async move {
            registry
                .publish_entity(Entity::Sensor(
                    Sensor::default()
                        .topic_prefix("temperature_devices/barometer-09AF")
                        .origin(origin())
                        .device(device())
                        .object_id("barometer-09AF")
                        .unique_id("barometer-09AF_temperature")
                        .availability(
                            Availability::single_topic("~/availability").expire_after(120),
                        )
                        .state_topic("~/state")
                        .value_template("{{ json_value.temperature }}")
                        .device_class(SensorDeviceClass::Temperature)
                        .force_update(true)
                        .name("Temperature")
                        .suggested_display_precision(1)
                        .state_class(SensorStateClass::Measurement)
                        .unit_of_measurement(TemperatureUnit::Celsius),
                ))
                .await
                .expect("message to be published");
        });
    })
    .await;

    assert_eq!(
        packet.topic,
        "homeassistant/sensor/barometer-09AF_temperature/config"
    );
    assert_json_eq!(
        json,
        json!(
            {
                "p": "sensor",
                "~": "temperature_devices/barometer-09AF",
                "o": {
                  "name": "Integration test",
                  "sw": "0.0.1",
                  "support_url": "https://www.github.com"
                },
                "dev": {
                  "name": "Barometer",
                  "ids": [
                    "barometer-09AF"
                  ],
                  "cns": [
                    [
                      "mac",
                      "09:AF:A4:54:F0:9D"
                    ]
                  ],
                  "cu": "https://barometer.home/admin",
                  "mf": "Awesome corp",
                  "mdl": "Awesome model",
                  "sa": "kitchen",
                  "sw": "0.1a",
                  "hw": "rev B",
                  "via_device": "manufacturer cloud"
                },
                "obj_id": "barometer-09AF",
                "uniq_id": "barometer-09AF_temperature",
                "avty_mode": "all",
                "avty": [
                  {
                    "t": "~/availability"
                  }
                ],
                "stat_t": "~/state",
                "val_tpl": "{{ json_value.temperature }}",
                "dev_cla": "temperature",
                "exp_aft": 120,
                "frc_upd": true,
                "name": "Temperature",
                "sug_dsp_prc": 1,
                "stat_cla": "measurement",
                "unit_of_meas": "°C"
              }
        )
    );
}
