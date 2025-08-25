---
title: "MQTT Select"
description: "Instructions on how to interact with a device exposing a Select through MQTT from within Home Assistant."
ha_category:
  - Select
ha_release: 2021.7
ha_iot_class: Configurable
ha_domain: mqtt
---

The `mqtt` Select platform allows you to integrate devices that might expose configuration options through MQTT into Home Assistant as a Select. Every time a message under the `topic` in the configuration is received, the select entity will be updated in Home Assistant and vice-versa, keeping the device and Home Assistant in sync.

## Configuration

To use an MQTT select entity in your installation, add the following to your `configuration.yaml` file.
{% include integrations/restart_ha_after_config_inclusion.md %}

```yaml
# Example configuration.yaml entry
mqtt:
  - select:
      command_topic: topic
      name: "Test Select"
      options:
        - "Option 1"
        - "Option 2"
```

Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).


⚠ Important\
Make sure that your topic matches exactly. `some-topic/` and `some-topic` are different topics.
