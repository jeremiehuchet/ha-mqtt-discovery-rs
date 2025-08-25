---
title: "MQTT button"
description: "Instructions on how to integrate MQTT buttons into Home Assistant."
ha_category:
  - Button
ha_release: 2021.12
ha_iot_class: Configurable
ha_domain: mqtt
---

The `mqtt` button platform lets you send an MQTT message when the button is pressed in the frontend or the button press action is called. This can be used to expose some service of a remote device, for example reboot.

To use an MQTT button in your installation, [add a MQTT device as a subentry](/integrations/mqtt/#configuration), or add the following to your `configuration.yaml` file.
{% include integrations/restart_ha_after_config_inclusion.md %}

## Configuration

```yaml
# Example configuration.yaml entry
mqtt:
  - button:
      command_topic: "home/bedroom/switch1/reboot"
```

Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).


⚠ Important\
Make sure that your topic matches exactly. `some-topic/` and `some-topic` are different topics.

## Examples

In this section, you will find some real-life examples of how to use this feature.

### Full configuration

The example below shows a full configuration for a button.

```yaml
# Example configuration.yaml entry
mqtt:
  - button:
      unique_id: bedroom_switch_reboot_btn
      name: "Restart Bedroom Switch"
      command_topic: "home/bedroom/switch1/commands"
      payload_press: "restart"
      availability:
        - topic: "home/bedroom/switch1/available"
      qos: 0
      retain: false
      entity_category: "config"
      device_class: "restart"
```
