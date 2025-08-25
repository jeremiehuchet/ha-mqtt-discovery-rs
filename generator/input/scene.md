---
title: "MQTT Scene"
description: "Instructions on how to integrate MQTT scenes into Home Assistant."
ha_category:
  - Scene
ha_release: 2020.12
ha_iot_class: Configurable
ha_domain: mqtt
---

The `mqtt` scene platform lets you control your MQTT enabled scenes.

## Configuration

To use an MQTT scene entity in your installation, add the following to your `configuration.yaml` file.
{% include integrations/restart_ha_after_config_inclusion.md %}

```yaml
# Example configuration.yaml entry
mqtt:
  - scene:
      command_topic: zigbee2mqtt/living_room_group/set
```

Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).


⚠ Important\
Make sure that your topic matches exactly. `some-topic/` and `some-topic` are different topics.

## Examples

In this section, you will find some real-life examples of how to use the MQTT Scene.

### Full configuration

The example below shows a full configuration for a scene.

```yaml
# Example configuration.yaml entry
mqtt:
  - scene:
      unique_id: living_room_party_scene
      name: "Party Scene"
      command_topic: "home/living_room/party_scene/set"
      availability:
        - topic: "home/living_room/party_scene/available"
      payload_on: "ON"
      qos: 0
      retain: true
      device:
        name: "Living Room"
        identifiers: "livingroom_lights" 
```

### Use with a JSON Payload

The example below shows a configuration using a JSON payload.

```yaml
# Example configuration.yaml entry
mqtt:
  - scene:
      name: Living Room Blue Scene
      unique_id: living_room_blue_scene
      command_topic: "home/living_room/set"
      payload_on: '{"activate_scene": "Blue Scene"}'
```
