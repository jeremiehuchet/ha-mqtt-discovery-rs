---
title: "MQTT binary sensor"
description: "Instructions on how to integrate MQTT binary sensors within Home Assistant."
ha_category:
  - Binary sensor
ha_release: 0.9
ha_iot_class: Configurable
ha_domain: mqtt
---

The `mqtt` binary sensor platform uses an MQTT message received to set the binary sensor's state to `on`, `off` or `unknown`.

The state will be updated only after a new message is published on `state_topic` matching `payload_on`, `payload_off` or `None`. If these messages are published with the `retain` flag set,
the binary sensor will receive an instant state update after subscription and Home Assistant will display the correct state on startup.
Otherwise, the initial state displayed in Home Assistant will be `unknown`.

Stateless devices such as buttons, remote controls etc are better represented by [MQTT device triggers](/integrations/device_trigger.mqtt/) than by binary sensors.

## Configuration

The `mqtt` binary sensor platform optionally supports a list of  `availability` topics to receive online and offline messages (birth and LWT messages) from the MQTT device. During normal operation, if the MQTT sensor device goes offline (i.e., publishes `payload_not_available` to an `availability` topic), Home Assistant will display the binary sensor as `unavailable`. If these messages are published with the `retain` flag set, the binary sensor will receive an instant update after subscription and Home Assistant will display the correct availability state of the binary sensor when Home Assistant starts up. If the `retain` flag is not set, Home Assistant will display the binary sensor as `unavailable` when Home Assistant starts up. If no `availability` topic is defined, Home Assistant will consider the MQTT device to be `available` and will display its state.

To use an MQTT binary sensor in your installation, [add a MQTT device as a subentry](/integrations/mqtt/#configuration), or add the following to your `configuration.yaml` file.
{% include integrations/restart_ha_after_config_inclusion.md %}

```yaml
# Example configuration.yaml entry
mqtt:
  - binary_sensor:
      state_topic: "basement/window/contact"
```

Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).


## Examples

In this section, you will find some real-life examples of how to use this sensor.

### Full configuration with JSON data

This is an example of a configuration where the state is extracted from a JSON formatted MQTT message.
To test, you can use the command line tool `mosquitto_pub` shipped with `mosquitto` or the `mosquitto-clients` package to send MQTT messages.

To set the state of the binary sensor manually:

```bash
mosquitto_pub -h 127.0.0.1 -t home-assistant/window/availability -m "online"
mosquitto_pub -h 127.0.0.1 -t home-assistant/window/contact -m '{"state":"ON"}'
mosquitto_pub -h 127.0.0.1 -t home-assistant/window/contact -m '{"state":"OFF"}'
```

The example below shows a full configuration for a binary sensor:


```yaml
# Example configuration.yaml entry
mqtt:
  - binary_sensor:
      name: "Window Contact Sensor"
      state_topic: "bedroom/window/contact"
      payload_on: "ON"
      availability:
        - topic: "bedroom/window/availability"
          payload_available: "online"
          payload_not_available: "offline"
      qos: 0
      device_class: opening
      value_template: "{{ value_json.state }}"
```


### Toggle the binary sensor each time a message is received on state_topic


```yaml
# Example configuration.yaml entry
mqtt:
  - binary_sensor:
      state_topic: "lab_button/cmnd/POWER"
      value_template: "{%if is_state(entity_id,\"on\")-%}OFF{%-else-%}ON{%-endif%}"
```


### Get the state of a device with ESPEasy

Assuming that you have flashed your ESP8266 unit with [ESPEasy](https://github.com/letscontrolit/ESPEasy). Under "Config" is a name ("Unit Name:") set for your device (here it's "bathroom"). A configuration for a "Controller" for MQTT with the protocol "OpenHAB MQTT" is present and the entries ("Controller Subscribe:" and "Controller Publish:") are adjusted to match your needs. In this example, the topics are prefixed with "home". Also, add a "Switch Input" in the "Devices" tap with the name "switch" and "button" as value.

As soon as the unit is online, you will get the state of the attached button.

```txt
home/bathroom/status Connected
...
home/bathroom/switch/button 1
```

The configuration will look like the example below:

```yaml
# Example configuration.yaml entry
mqtt:
  - binary_sensor:
      name: Bathroom
      state_topic: "home/bathroom/switch/button"
      payload_on: "1"
      payload_off: "0"
```
