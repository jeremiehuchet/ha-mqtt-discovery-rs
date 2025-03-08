---
title: "MQTT Text"
description: "Instructions on how to interact with a device exposing text capability through MQTT from within Home Assistant."
ha_category:
  - Text
ha_release: "2022.12"
ha_iot_class: Configurable
ha_domain: mqtt
---

The `mqtt` Text platform allows you to integrate devices that show text that can be set remotely. Optionally the text state can be monitored too using MQTT.

## Configuration

To enable MQTT text platform in your installation, add the following to your {% term "`configuration.yaml`" %} file:

```yaml
# Example configuration.yaml entry
mqtt:
  - text:
      command_topic: command-topic
```


⚠ Important\
Make sure that your topic matches exactly. `some-topic/` and `some-topic` are different topics.

## Examples

This is an example of a manual configured MQTT `text` item.


```yaml
# Example configuration.yaml entry
mqtt:
  - text:
      name: "Remote LCD screen"
      icon: mdi:ab-testing
      mode: "text"
      command_topic: "txt/cmd"
      state_topic: "txt/state"
      min: 2
      max: 20
```

