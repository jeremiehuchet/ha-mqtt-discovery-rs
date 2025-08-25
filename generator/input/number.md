---
title: "MQTT Number"
description: "Instructions on how to interact with a device exposing a Number through MQTT from within Home Assistant."
ha_category:
  - Number
ha_release: 2021.2
ha_iot_class: Configurable
ha_domain: mqtt
---

The `mqtt` Number platform allows you to integrate devices that might expose configuration options through MQTT into Home Assistant as a Number. Every time a message under the `topic` in the configuration is received, the number entity will be updated in Home Assistant and vice-versa, keeping the device and Home Assistant in-sync.

## Configuration

To use an MQTT number entity in your installation, add the following to your `configuration.yaml` file.
{% include integrations/restart_ha_after_config_inclusion.md %}

```yaml
# Example configuration.yaml entry
mqtt:
  - number:
      command_topic: my-device/threshold
```

Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).


⚠ Important\
Make sure that your topic matches exactly. `some-topic/` and `some-topic` are different topics.
