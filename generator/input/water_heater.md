---
title: "MQTT water heater"
description: "Instructions on how to integrate MQTT water heater into Home Assistant."
ha_category:
  - Water heater
ha_release: 2023.7
ha_iot_class: Local Polling
ha_domain: mqtt
---

The `mqtt` water heater platform lets you control your MQTT enabled water heater devices.

## Configuration

To use an MQTT water heater in your installation, add the following to your `configuration.yaml` file.
{% include integrations/restart_ha_after_config_inclusion.md %}

```yaml
# Example configuration.yaml entry
mqtt:
  - water_heater:
      name: Boiler
      mode_command_topic: "basement/boiler/mode/set"
```

Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).


## Optimistic mode

If a property works in *optimistic mode* (when the corresponding state topic is not set), Home Assistant will assume that any state changes published to the command topics did work and change the internal state of the `entity %} immediately after publishing to the command topic. If it does not work in optimistic mode, the internal state of the {% term entity` is only updated when the requested update is confirmed by the device through the state topic. You can enforce optimistic mode by setting the `optimistic` option to `true`. When set, the internal state will always be updated, even when a state topic is defined.

## Using templates

For all `*_state_topic`s, a template can be specified that will be used to render the incoming payloads on these topics. Also, a default template that applies to all state topics can be specified as `value_template`. This can be useful if you received payloads are e.g., in JSON format. Since in JSON, a quoted string (e.g., `"foo"`) is just a string, this can also be used for unquoting.

Say you receive the operation mode `"off"` via your `mode_state_topic`, but the mode is actually called just `off`, here's what you could do:


```yaml
mqtt:
  - water_heater:
      name: Boiler
      modes:
        - "off"
        - "eco"
        - "performance"
      mode_command_topic: "basement/boiler/mode/set"
      mode_state_topic: "basement/boiler/mode/state"
      mode_state_template: "{{ value_json }}"
```


This will parse the incoming `"off"` as JSON, resulting in `off`. Obviously, in this case you could also just set `value_template: {% raw %}"{{ value_json }}"{% endraw %}`.

Similarly for `*_command_topic`s, a template can be specified to render the outgoing payloads on these topics.

## Example

A full configuration example looks like the one below.


```yaml
# Full example configuration.yaml entry
mqtt:
  - water_heater:
      name: Boiler
      modes:
        - "off"
        - "eco"
        - "performance"
      mode_state_topic: "basement/boiler/mode"
      mode_command_topic: "basement/boiler/mode/set"
      mode_command_template: "{{ value if value=="off" else "on" }}"
      temperature_state_topic: "basement/boiler/temperature"
      temperature_command_topic: "basement/boiler/temperature/set"
      current_temperature_topic: "basement/boiler/current_temperature"
      precision: 1.0
```

