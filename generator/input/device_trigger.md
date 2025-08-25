---
title: "MQTT Device trigger"
description: "Instructions on how to integrate MQTT device triggers within Home Assistant."
ha_category:
  - Device automation
ha_release: 0.106
ha_iot_class: Configurable
ha_domain: mqtt
---

The `mqtt` device trigger platform uses an MQTT message payload to generate device trigger events.

An MQTT device trigger is a better option than a [binary sensor](/integrations/binary_sensor.mqtt/) for buttons, remote controls etc.

## Configuration

MQTT device triggers are only supported through [MQTT discovery](/integrations/mqtt/#mqtt-discovery), manual setup through `configuration.yaml` is not supported.
The discovery topic needs to be: `<discovery_prefix>/device_automation/[<node_id>/]<object_id>/config`. Note that only one trigger may be defined per unique discovery topic. Also note that the combination of `type` and `subtype` should be unique for a device.


### Example

This shows a complete example of defining a remote control type device with two triggers: "left arrow click" and "right arrow click".

Note that it is not necessary to provide the full device information in each message, but the identifying information, `identifier` in the example, must be the same.

#### Left arrow click configuration

- Discovery topic: `homeassistant/device_automation/0x90fd9ffffedf1266/action_arrow_left_click/config`
- Discovery payload:

  ```json
  {
      "automation_type": "trigger",
      "type": "action",
      "subtype": "arrow_left_click",
      "payload": "arrow_left_click",
      "topic": "zigbee2mqtt/0x90fd9ffffedf1266/action",
      "device": {
          "identifiers": [
              "zigbee2mqtt_0x90fd9ffffedf1266"
          ],
          "name": "0x90fd9ffffedf1266",
          "sw_version": "Zigbee2MQTT 1.14.0",
          "model": "TRADFRI remote control (E1524/E1810)",
          "manufacturer": "IKEA"
      }
  }
  ```

- Trigger topic: `zigbee2mqtt/0x90fd9ffffedf1266/action`
- Trigger payload: `arrow_left_click`

#### Right arrow click configuration

- Discovery topic: `homeassistant/device_automation/0x90fd9ffffedf1266/action_arrow_right_click/config`
- Discovery payload:

  ```json
  {
      "automation_type": "trigger",
      "type": "action",
      "subtype": "arrow_right_click",
      "payload": "arrow_right_click",
      "topic": "zigbee2mqtt/0x90fd9ffffedf1266/action",
      "device": {
          "identifiers": [
              "zigbee2mqtt_0x90fd9ffffedf1266"
          ]
      }
  }   
  ```

- Trigger topic: `zigbee2mqtt/0x90fd9ffffedf1266/action`
- Trigger payload: `arrow_right_click`
