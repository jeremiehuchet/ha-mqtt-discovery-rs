---
title: "MQTT lawn mower"
description: "Instructions on how to integrate MQTT lawn mowers into Home Assistant."
ha_category:
  - Lawn mower
ha_release: 2023.9
ha_iot_class: Configurable
ha_domain: mqtt
---

The `mqtt` `lawn_mower` platform allows controlling a lawn mower over MQTT.

## Configuration

To use an MQTT lawn mower in your installation, add the following to your `configuration.yaml` file.
{% include integrations/restart_ha_after_config_inclusion.md %}

```yaml
# Example configuration.yaml entry
mqtt:
  - lawn_mower:
      command_topic: topic
      name: "Test Lawn Mower"
```

Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).


⚠ Important\
Make sure that your topic matches exactly. `some-topic/` and `some-topic` are different topics.

## Example

The example below shows how to use a single command topic with a command template.


```yaml
# Example configuration.yaml entry
mqtt:
  - lawn_mower:
      name: "Lawn Mower Plus"
      activity_state_topic: "lawn_mower_plus/state"
      activity_value_template: "{{ value_json.activity }}" 
      pause_command_topic: "lawn_mower_plus/set"
      pause_command_template: '{"activity": "{{ value }}"}' 
      dock_command_topic: "lawn_mower_plus/set"
      dock_command_template: '{"activity": "{{ value }}"}' 
      start_mowing_command_topic: "lawn_mower_plus/set"
      start_mowing_command_template: '{"activity": "{{ value }}"}' 
```

