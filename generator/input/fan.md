---
title: "MQTT Fan"
description: "Instructions on how to integrate MQTT fans into Home Assistant."
ha_category:
  - Fan
ha_release: 0.27
ha_iot_class: Configurable
ha_domain: mqtt
---

The `mqtt` fan platform lets you control your MQTT enabled fans.

## Configuration

In an ideal scenario, the MQTT device will have a `state_topic` to publish state changes. If these messages are published with a `RETAIN` flag, the MQTT fan will receive an instant state update after subscription and will start with the correct state. Otherwise, the initial state of the fan will be `unknown`. A MQTT device can reset the current state to `unknown` using a `None` payload.

When a `state_topic` is not available, the fan will work in optimistic mode. In this mode, the fan will immediately change state after every command. Otherwise, the fan will wait for state confirmation from the device (message from `state_topic`).  The initial state is set to `False` / `off` in optimistic mode.

Optimistic mode can be forced even if a `state_topic` is available. Try to enable it if you are experiencing incorrect fan operation.

To enable MQTT fans in your installation, add the following to your {% term "`configuration.yaml`" %} file.
{% include integrations/restart_ha_after_config_inclusion.md %}

```yaml
# Example configuration.yaml entry
mqtt:
  - fan:
      command_topic: "bedroom_fan/on/set"
```


⚠ Important\

Make sure that your topics match exactly. `some-topic/` and `some-topic` are different topics.


## Examples

In this section you find some real-life examples of how to use this fan.

### Full configuration

The example below shows a full configuration for a MQTT fan using percentage and preset modes.
There are 10 speeds within the speed range, so  `percentage_step` = 100 / 10 steps = 10.0 %.

```yaml
# Example using percentage based speeds with preset modes configuration.yaml
mqtt:
  - fan:
      name: "Bedroom Fan"
      state_topic: "bedroom_fan/on/state"
      command_topic: "bedroom_fan/on/set"
      direction_state_topic: "bedroom_fan/direction/state"
      direction_command_topic: "bedroom_fan/direction/set"
      oscillation_state_topic: "bedroom_fan/oscillation/state"
      oscillation_command_topic: "bedroom_fan/oscillation/set"
      percentage_state_topic: "bedroom_fan/speed/percentage_state"
      percentage_command_topic: "bedroom_fan/speed/percentage"
      preset_mode_state_topic: "bedroom_fan/preset/preset_mode_state"
      preset_mode_command_topic: "bedroom_fan/preset/preset_mode"
      preset_modes:
        -  "auto"
        -  "smart"
        -  "whoosh"
        -  "eco"
        -  "breeze"
      qos: 0
      payload_on: "true"
      payload_off: "false"
      payload_oscillation_on: "true"
      payload_oscillation_off: "false"
      speed_range_min: 1
      speed_range_max: 10
```

### Configuration using command templates

This example demonstrates how to use command templates with JSON output.


```yaml
# Example configuration.yaml with command templates
mqtt:
  - fan:
      name: "Bedroom Fan"
      command_topic: "bedroom_fan/on/set"
      command_template: "{ state: '{{ value }}'}"
      direction_command_template: "{{ iif(value == 'forward', 'fwd', 'rev') }}"
      direction_value_template: "{{ iif(value == 'fwd', 'forward', 'reverse') }}"
      oscillation_command_topic: "bedroom_fan/oscillation/set"
      oscillation_command_template: "{ oscillation: '{{ value }}'}"
      percentage_command_topic: "bedroom_fan/speed/percentage"
      percentage_command_template: "{ percentage: '{{ value }}'}"
      preset_mode_command_topic: "bedroom_fan/preset/preset_mode"
      preset_mode_command_template: "{ preset_mode: '{{ value }}'}"
      preset_modes:
        -  "auto"
        -  "smart"
        -  "whoosh"
        -  "eco"
        -  "breeze"
```


This example shows how to configure a fan that doesn't use `forward` and `backward` as directions.


```yaml
# Example configuration.yaml with direction templates
mqtt:
  - fan:
      name: "Bedroom Fan"
      direction_command_template: "{{ iif(value == 'forward', 'fwd', 'rev') }}"
      direction_value_template: "{{ iif(value == 'fwd', 'forward', 'reverse') }}"
```

