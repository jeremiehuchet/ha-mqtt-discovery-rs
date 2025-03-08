---
title: "MQTT Switch"
description: "Instructions on how to integrate MQTT switches into Home Assistant."
ha_category:
  - Switch
ha_release: 0.7
ha_iot_class: Configurable
ha_domain: mqtt
---

The `mqtt` switch platform lets you control your MQTT enabled switches.

## Configuration

In an ideal scenario, the MQTT device will have a `state_topic` to publish state changes. If these messages are published with a `RETAIN` flag, the MQTT switch will receive an instant state update after subscription, and will start with the correct state. Otherwise, the initial state of the switch will be `unknown`. A MQTT device can reset the current state to `unknown` using a `None` payload.

When a `state_topic` is not available, the switch will work in optimistic mode. In this mode, the switch will immediately change state after every command. Otherwise, the switch will wait for state confirmation from the device (message from `state_topic`). The initial state is set to `False` / `off` in optimistic mode.

Optimistic mode can be forced, even if the `state_topic` is available. Try to enable it, if experiencing incorrect switch operation.

To enable this switch in your installation, add the following to your {% term "`configuration.yaml`" %} file:

```yaml
# Example configuration.yaml entry
mqtt:
  - switch:
      command_topic: "home/bedroom/switch1/set"
```


⚠ Important\
Make sure that your topic matches exactly. `some-topic/` and `some-topic` are different topics.

## Examples

In this section, you will find some real-life examples of how to use this sensor.

### Full configuration

The example below shows a full configuration for a switch.

```yaml
# Example configuration.yaml entry
mqtt:
  - switch:
      unique_id: bedroom_switch
      name: "Bedroom Switch"
      state_topic: "home/bedroom/switch1"
      command_topic: "home/bedroom/switch1/set"
      availability:
        - topic: "home/bedroom/switch1/available"
      payload_on: "ON"
      payload_off: "OFF"
      state_on: "ON"
      state_off: "OFF"
      optimistic: false
      qos: 0
      retain: true
```

For a check, you can use the command line tools `mosquitto_pub` shipped with `mosquitto` to send MQTT messages. This allows you to operate your switch manually. First, we can simulate the availability message sent for the switch:

```bash
mosquitto_pub -h 127.0.0.1 -t home/bedroom/switch1/available -m "online"
```

We can simulate the switch being turned on by publishing the `ON` command message:

```bash
mosquitto_pub -h 127.0.0.1 -t home/bedroom/switch1/set -m "ON"
```

Finally, we can simulate the switch reporting back the changed state to Home Assistant:

```bash
mosquitto_pub -h 127.0.0.1 -t home/bedroom/switch1 -m "ON"
```

### Set the state of a device with ESPEasy

Assuming that you have flashed your ESP8266 unit with [ESPEasy](https://github.com/letscontrolit/ESPEasy). Under "Config" is a name ("Unit Name:") set for your device (here it's "bathroom"). A configuration for a "Controller" for MQTT with the protocol "OpenHAB MQTT" is present and the entries ("Controller Subscribe:" and "Controller Publish:") are adjusted to match your needs. In this example, the topics are prefixed with "home". There is no further configuration needed as the [GPIOs](https://espeasy.readthedocs.io/en/latest/Controller/C005.html) can be controlled with MQTT directly.

Manually you can set pin 13 to high with `mosquitto_pub` or another MQTT tool:

```bash
mosquitto_pub -h 127.0.0.1 -t home/bathroom/gpio/13 -m "1"
```

The configuration will look like the example below:

```yaml
# Example configuration.yaml entry
mqtt:
  - switch:
      name: bathroom
      state_topic: "home/bathroom/gpio/13"
      command_topic: "home/bathroom/gpio/13"
      payload_on: "1"
      payload_off: "0"
```
