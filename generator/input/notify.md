---
title: "MQTT notify"
description: "Instructions on how to integrate MQTT notify entities into Home Assistant."
ha_category:
  - Notifications
ha_release: 2024.5
ha_iot_class: Configurable
ha_domain: mqtt
---

The **MQTT notify** platform lets you send an MQTT message when the `send_message` action is called. This can be used to expose a action of a remote device that allows processing a message, such as showing it on a screen.

## Configuration

To use an MQTT notify entity in your installation, [add a MQTT device as a subentry](/integrations/mqtt/#configuration), or add the following to your `configuration.yaml` file.
{% include integrations/restart_ha_after_config_inclusion.md %}

To use an MQTT notify entity in your installation, add the following to your `configuration.yaml` file.
{% include integrations/restart_ha_after_config_inclusion.md %}

```yaml
# Example configuration.yaml entry
mqtt:
  - notify:
      command_topic: "home/living_room/status_screen/notifications"
```

Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).


⚠ Important\
Make sure that your topic matches exactly. `some-topic/` and `some-topic` are different topics.

## Examples

In this section, you will find some real-life examples of how to use this feature.

### Full configuration

The example below shows a full configuration for a notify entity.

```yaml
# Example configuration.yaml entry
mqtt:
  - notify:
      unique_id: living_room_stat_scr01
      name: "Living room status screen"
      command_topic: "home/living_room/status_screen/notifications"
      availability:
        - topic: "home/living_room/status_screen/available"
      qos: 0
      retain: false
```
