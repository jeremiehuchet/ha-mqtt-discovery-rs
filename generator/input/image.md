---
title: "MQTT Image"
description: "Instructions on how to use an MQTT image message as an Image within Home Assistant."
ha_category:
  - Image
ha_release: 2023.7
ha_iot_class: Configurable
ha_domain: mqtt
---

The `mqtt` image platform allows you to integrate the content of an image file sent through MQTT into Home Assistant as an image.
The `image` platform is a simplified version of the `camera` platform that only accepts images.
Every time a message under the `image_topic` in the configuration is received, the image displayed in Home Assistant will also be updated. Messages received on `image_topic` should contain the full contents of an image file, for example, a JPEG image, without any additional encoding or metadata.

This can be used with an application or a service capable of sending images through MQTT.

An alternative setup is to use the `url_topic` option to receive an image URL for a new picture to show.

## Configuration

To use an MQTT image entity in your installation, add the following to your `configuration.yaml` file.
{% include integrations/restart_ha_after_config_inclusion.md %}

```yaml
# Example configuration.yaml entry
mqtt:
  - image:
      url_topic: mynas/status/url
```

Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).


### Example receiving images from a URL

Add the configuration below to your `configuration.yaml`.

To test it publish an image URL to the topic from the console:

```shell
mosquitto_pub -h <mqtt_broker> -t mynas/status/url -m "https://design.home-assistant.io/images/logo.png"
```


```yaml
# Example configuration.yaml entry
mqtt:
  - image:
      url_topic: mynas/status/url
```


### Example receiving images from a file

Add the configuration below to your `configuration.yaml`.

To test it, publish an image URL to the topic from the console:

```shell
mosquitto_pub -h <mqtt_broker> -t mynas/status/file -f <logo.png>
```


```yaml
# Example configuration.yaml entry
mqtt:
  - image:
      image_topic: mynas/status/file
      content_type: image/png
```

