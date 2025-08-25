---
title: "MQTT Humidifier"
description: "Instructions on how to integrate MQTT humidifiers into Home Assistant."
ha_category:
  - Humidifier
ha_release: 2021.8
ha_iot_class: Configurable
ha_domain: mqtt
---

The `mqtt` humidifier platform lets you control your MQTT enabled humidifiers.

## Configuration

In an ideal scenario, the MQTT device will have a `state_topic` to publish state changes. If these messages are published with a `RETAIN` flag, the MQTT humidifier will receive an instant state update after subscription and will start with the correct state. Otherwise, the initial state of the humidifier will be `unknown`. A MQTT device can reset the current state to `unknown` using a `None` payload.

When a `state_topic` is not available, the humidifier will work in optimistic mode. In this mode, the humidifier will immediately change state after every command. Otherwise, the humidifier will wait for state confirmation from the device (message from `state_topic`). The initial state is set to `False` / `off` in optimistic mode.

Optimistic mode can be forced even if a `state_topic` is available. Try to enable it if you are experiencing incorrect humidifier operation.

To use an MQTT humidifier in your installation, add the following to your `configuration.yaml` file.
{% include integrations/restart_ha_after_config_inclusion.md %}

```yaml
# Example configuration.yaml entry
mqtt:
  - humidifier:
      command_topic: "bedroom_humidifier/on/set"
      target_humidity_command_topic: "bedroom_humidifier/humidity/set"
```

Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).


⚠ Important\
Make sure that your topics match exactly. `some-topic/` and `some-topic` are different topics.

## Examples

In this section you find some real-life examples of how to use this humidifier.

### Full configuration

The example below shows a full configuration for a MQTT humidifier including modes.


```yaml
# Example configuration.yaml
mqtt:
  - humidifier:
      name: "Bedroom humidifier"
      device_class: "humidifier"
      state_topic: "bedroom_humidifier/on/state"
      action_topic: "bedroom_humidifier/action"
      command_topic: "bedroom_humidifier/on/set"
      current_humidity_topic: "bedroom_humidifier/humidity/current"
      target_humidity_command_topic: "bedroom_humidifier/humidity/set"
      target_humidity_state_topic: "bedroom_humidifier/humidity/state"
      mode_state_topic: "bedroom_humidifier/mode/state"
      mode_command_topic: "bedroom_humidifier/preset/preset_mode"
      modes:
        - "normal"
        - "eco"
        - "away"
        - "boost"
        - "comfort"
        - "home"
        - "sleep"
        - "auto"
        - "baby"
      qos: 0
      payload_on: "true"
      payload_off: "false"
      min_humidity: 30
      max_humidity: 80
```

