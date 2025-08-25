---
title: "MQTT Camera"
description: "Instructions on how to use an MQTT image message as a Camera within Home Assistant."
ha_category:
  - Camera
ha_release: 0.43
ha_iot_class: Configurable
ha_domain: mqtt
---

The `mqtt` camera platform allows you to integrate the content of an image file sent through MQTT into Home Assistant as a camera. Every time a message under the `topic` in the configuration is received, the image displayed in Home Assistant will also be updated. Messages received on `topic` should contain the full contents of an image file, for example, a JPEG image, without any additional encoding or metadata.

This can be used with an application or a service capable of sending images through MQTT.

## Configuration

To use an MQTT camera in your installation, add the following to your `configuration.yaml` file.
{% include integrations/restart_ha_after_config_inclusion.md %}

```yaml
# Example configuration.yaml entry
mqtt:
  - camera:
      topic: zanzito/shared_locations/my-device
```

Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).

The sample configuration above can be tested by publishing an image to the topic from the console:

```shell
mosquitto_pub -h <mqtt_broker> -t zanzito/shared_locations/my-device -f <camera_imaga.jpg>
```

