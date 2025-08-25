---
title: "MQTT climate (HVAC)"
description: "Instructions on how to integrate MQTT climate into Home Assistant."
ha_category:
  - Climate
ha_release: 0.55
ha_iot_class: Local Polling
ha_domain: mqtt
---

The `mqtt` climate platform lets you control your MQTT enabled HVAC devices.

## Configuration

To use an MQTT climate in your installation, [add an MQTT device as a subentry](/integrations/mqtt/#configuration), or add the following to your `configuration.yaml` file.
{% include integrations/restart_ha_after_config_inclusion.md %}

```yaml
# Example configuration.yaml entry
mqtt:
  - climate:
      name: Study
      mode_command_topic: "study/ac/mode/set"
```

Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).


## Optimistic mode

If a property works in *optimistic mode* (when the corresponding state topic is not set), Home Assistant will assume that any state changes published to the command topics did work and change the internal state of the entity immediately after publishing to the command topic. If it does not work in optimistic mode, the internal state of the entity is only updated when the requested update is confirmed by the device through the state topic. You can enforce optimistic mode by setting the `optimistic` option to `true`. When set, the internal state will always be updated, even when a state topic is defined.

## Using templates

For all `*_state_topic`s, a template can be specified that will be used to render the incoming payloads on these topics. Also, a default template that applies to all state topics can be specified as `value_template`. This can be useful if you received payloads are e.g., in JSON format. Since in JSON, a quoted string (e.g., `"foo"`) is just a string, this can also be used for unquoting.

Say you receive the operation mode `"auto"` via your `mode_state_topic`, but the mode is actually called just `auto`, here's what you could do:


```yaml
mqtt:
  - climate:
      name: Study
      modes:
        - "off"
        - "heat"
        - "auto"
      mode_command_topic: "study/ac/mode/set"
      mode_state_topic: "study/ac/mode/state"
      mode_state_template: "{{ value_json }}"
```


This will parse the incoming `"auto"` as JSON, resulting in `auto`. Obviously, in this case you could also just set `value_template: {% raw %}"{{ value_json }}"{% endraw %}`.

Similarly for `*_command_topic`s, a template can be specified to render the outgoing payloads on these topics.

## Example

A full configuration example looks like the one below.


```yaml
# Full example configuration.yaml entry
mqtt:
  - climate:
      name: Study
      modes:
        - "off"
        - "cool"
        - "fan_only"
      swing_horizontal_modes:
        - "on"
        - "off"
      swing_modes:
        - "on"
        - "off"
      fan_modes:
        - "high"
        - "medium"
        - "low"
      preset_modes:
        - "eco"
        - "sleep"
        - "activity"
      power_command_topic: "study/ac/power/set"
      preset_mode_command_topic: "study/ac/preset_mode/set"
      mode_command_topic: "study/ac/mode/set"
      mode_command_template: "{{ value if value=="off" else "on" }}"
      temperature_command_topic: "study/ac/temperature/set"
      fan_mode_command_topic: "study/ac/fan/set"
      swing_horizontal_mode_command_topic: "study/ac/swingH/set"
      swing_mode_command_topic: "study/ac/swing/set"
      precision: 1.0
```

