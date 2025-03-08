---
title: "MQTT Vacuum"
description: "Instructions on how to integrate your MQTT enabled Vacuum within Home Assistant."
ha_category:
  - Vacuum
ha_release: 0.54
ha_domain: mqtt
---

The `mqtt` vacuum {% term integration %} allows you to control your MQTT-enabled vacuum.
The initial state of the MQTT vacuum {% term entity %} will set to `unknown` and can be reset by a device by sending a `null` payload as state.

## Configuration

MQTT vacuum configuration section.


## Configuration example

```yaml
# Example configuration.yaml entry
mqtt:
  - vacuum:
      name: "MQTT Vacuum"
      supported_features:
        - start
        - pause
        - stop
        - return_home
        - battery
        - status
        - locate
        - clean_spot
        - fan_speed
        - send_command
      command_topic: "vacuum/command"
      set_fan_speed_topic: "vacuum/set_fan_speed"
      fan_speed_list:
        - min
        - medium
        - high
        - max
      send_command_topic: "vacuum/send_command"
```

## MQTT Protocol

The  configuration for this integration expects an MQTT protocol like the following.

### Basic Commands

MQTT topic: `vacuum/command`

Possible MQTT payloads:

- `start` - Start cleaning
- `pause` - Pause cleaning
- `return_to_base` - Return to base/dock
- `stop` - Stop the vacuum.
- `clean_spot` - Initialize a spot cleaning cycle
- `locate` - Locate the vacuum (typically by playing a song)

### Send custom command

Vacuum send_command allows three parameters:

- entity_id
- command
- params - optional

If params are not provided it sends command as payload to MQTT send_command topic.
If params are provided service sends JSON as payload with such structure:

```json
{
  'command': 'command',
  'param1-key': 'param1-value'
}
```

Action trigger example:

```yaml
- alias: "Push command based on sensor"
    triggers:
      - trigger: state
        entity_id: sensor.sensor
    actions:
      - action: vacuum.send_command
        target:
          entity_id: vacuum.vacuum_entity
        data:
          command: "custom_command"
          params:
            - key: value
```

MQTT topic: `vacuum/send_command`

### Status/Sensor Updates

MQTT topic: `vacuum/state`

MQTT payload:

```json
{
    "battery_level": 61,
    "state": "docked",
    "fan_speed": "off"
}
```

State has to be one of vacuum states supported by Home Assistant:

- cleaning,
- docked,
- paused,
- idle,
- returning,
- error.

### Set Fan Speed

MQTT topic: `vacuum/set_fan_speed`

Possible MQTT payloads:

- `min` - Minimum fan speed
- `medium` - Medium fan speed
- `high` - High fan speed
- `max` - Max fan speed

## Usage examples

### Usage with cloudless Xiaomi vacuums

This integration is supported by the cloud-free Xiaomi Vacuum Webinterface [Valetudo](https://github.com/Hypfer/Valetudo).

### Retrofitting non-wifi vacuums

- Retrofitting your old Roomba with an ESP8266. [This repository](https://github.com/johnboiles/esp-roomba-mqtt) provides MQTT client firmware.
- If you own a non-wifi Neato, you can refer to [this repository](https://github.com/jeroenterheerdt/neato-serial) that uses a Raspberry Pi to retrofit an old Neato.
