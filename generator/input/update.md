---
title: "MQTT Update"
description: "Instructions on how to interact with a device exposing an Update entity through MQTT from within Home Assistant."
ha_category:
  - Update
ha_release: "2021.11"
ha_iot_class: Configurable
ha_domain: mqtt
---

The `mqtt` Update platform allows you to integrate devices that might expose firmware/software installed and the latest versions through MQTT into Home Assistant as an Update entity. Every time a message under the `topic` in the configuration is received, the entity will be updated in Home Assistant.

## Configuration

To use an MQTT update entity in your installation, add the following to your `configuration.yaml` file.
{% include integrations/restart_ha_after_config_inclusion.md %}

```yaml
# Example configuration.yaml entry
mqtt:
  - update:
      state_topic: topic-installed
      latest_version_topic: topic-latest
```

Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).


⚠ Important\
Make sure that your topic matches exactly. `some-topic/` and `some-topic` are different topics.

## Examples

This is an example of Update entity configuration for Shelly Gen1 device.


```yaml
# Example configuration.yaml entry
mqtt:
  - update:
      name: "Shelly Plug S Firmware Update"
      title: "Shelly Plug S Firmware"
      release_url: "https://shelly-api-docs.shelly.cloud/gen1/#changelog"
      entity_picture: "https://brands.home-assistant.io/_/shelly/icon.png"
      state_topic: "shellies/shellyplug-s-112233/info"
      value_template: "{{ value_json['update'].old_version }}"
      latest_version_topic: "shellies/shellyplug-s-112233/info"
      latest_version_template: "{% if value_json['update'].new_version %}{{ value_json['update'].new_version }}{% else %}{{ value_json['update'].old_version }}{% endif %}"
      device_class: "firmware"
      command_topic: "shellies/shellyplug-s-112233/command"
      payload_install: "update_fw"
```


JSON can also be used as `state_topic` payload. Note that this feature also allows to process and show live progress information.


```json
{
  "installed_version": "1.21.0",
  "latest_version": "1.22.0",
  "title": "Device Firmware",
  "release_url": "https://example.com/release",
  "release_summary": "A new version of our amazing firmware",
  "entity_picture": "https://example.com/icon.png"
}
```


Simple progress state update example:


```json
{
  "installed_version": "1.21.0",
  "latest_version": "1.22.0",
  "title": "Device Firmware",
  "release_url": "https://example.com/release",
  "release_summary": "A new version of our amazing firmware",
  "entity_picture": "https://example.com/icon.png",
  "in_progress": true
}
```


Update percentage state update example:


```json
{
  "installed_version": "1.21.0",
  "latest_version": "1.22.0",
  "title": "Device Firmware",
  "release_url": "https://example.com/release",
  "release_summary": "A new version of our amazing firmware",
  "entity_picture": "https://example.com/icon.png",
  "update_percentage": 78
}
```


Publish `null` to reset the update percentage state update's:


```json
{
  "installed_version": "1.22.0",
  "latest_version": "1.22.0",
  "title": "Device Firmware",
  "release_url": "https://example.com/release",
  "release_summary": "A new version of our amazing firmware",
  "entity_picture": "https://example.com/icon.png",
  "update_percentage": null
}
```


The values in the JSON are optional but must be valid within the following schema:


For the above JSON payload examples, the `update` entity configuration should look like this:


```yaml
# Example configuration.yaml entry
mqtt:
  - update:
      name: "Amazing Device Update"
      title: "Device Firmware"
      state_topic: "amazing-device/state-topic"
      device_class: "firmware"
      command_topic: "amazing-device/command"
      payload_install: "install"
```


If the device/service sends data as JSON but the schema differs, `value_template` can be use to reformat the JSON.


```json
{
  "installed_ver": "2022.11",
  "new_ver": "2022.12"
}
```


For the above JSON payload, the `update` entity configuration should look like this:


```yaml
# Example configuration.yaml entry
mqtt:
   update:
      name: "Amazing Device Update"
      title: "Device Firmware"
      state_topic: "amazing-device/state-topic"
      value_template: "{{ {'installed_version': value_json.installed_ver, 'latest_version': value_json.new_ver } | to_json }}"
      device_class: "firmware"
      command_topic: "amazing-device/command"
      payload_install: "install"
```

