---
title: "MQTT Siren"
description: "Instructions on how to integrate MQTT sirens into Home Assistant."
ha_category:
  - Siren
ha_release: 2022.3
ha_iot_class: Configurable
ha_domain: mqtt
---

The `mqtt` siren platform lets you control your MQTT enabled sirens and text based notification devices.

## Configuration

In an ideal scenario, the MQTT device will have a `state_topic` to publish state changes. If these messages are published with a `RETAIN` flag, the MQTT siren will receive an instant state update after subscription, and will start with the correct state. Otherwise, the initial state of the siren will be `false` / `off`.

When a `state_topic` is not available, the siren will work in optimistic mode. In this mode, the siren will immediately change state after every command. Otherwise, the siren will wait for state confirmation from the device (message from `state_topic`).

Optimistic mode can be forced, even if the `state_topic` is available. Try to enable it, if experiencing incorrect operation.

To use an MQTT siren in your installation, add the following to your `configuration.yaml` file.
{% include integrations/restart_ha_after_config_inclusion.md %}

```yaml
# Example configuration.yaml entry
mqtt:
  - siren:
      command_topic: "home/bedroom/siren/set"
```

Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).


⚠ Important\
Make sure that your topic matches exactly. `some-topic/` and `some-topic` are different topics.

## Examples

In this section, you will find an example of how to use this siren platform.

### Full configuration

The example below shows a full configuration for a siren.


```yaml
# Example configuration.yaml entry
mqtt:
  - siren:
      unique_id: custom_siren
      name: "Intrusion siren"
      state_topic: "home/alarm/siren1"
      command_topic: "home/alarm/siren1/set"
      available_tones:
        - ping
        - siren
      availability:
        - topic: "home/alarm/siren1/available"
      payload_on: "ON"
      payload_off: "OFF"
      state_on: "ON"
      state_off: "OFF"
      optimistic: false
      qos: 0
      retain: true
```


### On/Off only siren controlling a Tasmota relay

The example below shows a configuration for an On/Off type siren, which does not accept JSON commands.


```yaml
# Example configuration.yaml entry
mqtt:
  - siren:
      unique_id: tasmota_siren
      name: "garage"
      state_topic: "stat/SIREN/RESULT"
      command_topic: "cmnd/SIREN/POWER"
      availability_topic: "tele/SIREN/LWT"
      command_template: "{{ value }}"
      state_value_template: "{{ value_json.POWER }}"
      payload_on: "ON"
      payload_off: "OFF"
      payload_available: "Online"
      payload_not_available: "Offline"
```


For a check, you can use the command line tools `mosquitto_pub` shipped with `mosquitto` to send MQTT messages. This allows you to operate your siren manually:

```bash
mosquitto_pub -h 127.0.0.1 -t home/alarm/siren1 -m "ON"
```
