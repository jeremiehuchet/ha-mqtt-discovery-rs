---
title: "MQTT tag scanner"
description: "Instructions on how to integrate MQTT scanner within Home Assistant."
ha_category:
  - Tag scanner
ha_release: 0.116
ha_iot_class: Configurable
ha_domain: mqtt
---

The `mqtt` tag scanner platform uses an MQTT message payload to generate tag scanned events.

## Configuration

MQTT scanners are only supported through [MQTT discovery](/integrations/mqtt/#mqtt-discovery), manual setup through `configuration.yaml` is not supported.
The discovery topic needs to be: `<discovery_prefix>/tag/[<node_id>/]<object_id>/config`.


## Examples

In this section, you will find some real-life examples of how to use this sensor.

### Full configuration with tag ID extracted from JSON data

This is an example of a configuration where the tag ID is extracted from a JSON formatted MQTT message.
To test, you can use the command line tool `mosquitto_pub` shipped with `mosquitto` or the `mosquitto-clients` package to send MQTT messages.

Discover the tag scanner:


```bash
mosquitto_pub -h 127.0.0.1 -t homeassistant/tag/0AFFD2/config -m '{"topic": "0AFFD2/tag_scanned", "value_template": "{{ value_json.PN532.UID }}"}'
```


Generate tag scanned event:


```bash
mosquitto_pub -h 127.0.0.1 -t 0AFFD2/tag_scanned -m '{"Time":"2020-09-28T17:02:10","PN532":{"UID":"E9F35959", "DATA":"ILOVETASMOTA"}}'
```

