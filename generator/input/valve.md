---
title: "MQTT Valve"
description: "Instructions on how to integrate MQTT valves into Home Assistant."
ha_category:
  - Valve
ha_iot_class: Configurable
ha_release: 2024.1
ha_domain: mqtt
---

The `mqtt` valve platform allows you to control an MQTT valve (such a gas or water valve).

## Configuration

A valve entity can be have the following states: `open`, `opening`, `closed` or `closing`.

### Valve controlled by states

If a `state_topic` is configured, the entity's state will be updated only after an MQTT message is received on `state_topic` matching `state_open`, `state_opening`, `state_closed` or `state_closing`. Commands configured through `payload_open`, `payload_closed`, and `payload_stop` will be published to `command_topic` to control the valve.

To use your MQTT valve in your installation, add the following to your {% term "`configuration.yaml`" %} file:

```yaml
# Example configuration.yaml entry for a value that is set by open or close command
mqtt:
  - valve:
      command_topic: "home-assistant/valve/set"
      state_topic: "home-assistant/valve/state"
```

### Valve controlled by position

If the valve supports reporting its position (the `reports_position` config option is set to `true`), a numeric state is expected on `state_topic`, but state updates are still allowed for `state_opening` and `state_closing`. Also, a JSON format is supported. It allows both `state` and `position` to be reported together.

Example of a JSON state update:

```json
{"state": "opening", "position": 10}
```

The wanted position value or `payload_stop` will be published to `command_topic` to control the valve when the actions `valve.open`, `value.close`, or `value.set_position` are called.

To use your MQTT valve in your installation, add the following to your {% term "`configuration.yaml`" %} file:

```yaml
# Example configuration.yaml entry for a valve that reports position
mqtt:
  - valve:
      command_topic: "home-assistant/valve/set"
      state_topic: "home-assistant/valve/state"
      reports_position: true
```

### Optimistic operation

If a `state_topic` is not defined, the valve will work in optimistic mode. In this mode, the valve will immediately change state (`open` or `closed`) after every command sent by Home Assistant. It won't wait for an update from the device. Optimistic mode can be forced by setting `optimistic` to `true`, even if a `state_topic` is defined. Try to enable it if you are experiencing an incorrect valve operation.



🛈 Note\
MQTT valve expects position values to be in the range of 0 to 100, where 0 indicates a closed position and 100 indicates a fully open position.
If `position_open` or `position_closed` are set to a different range (for example, 40 to 140), when sending a command to the device, the range will be adjusted to the device range. For example, position 0 will send a value of 40 to device. When the device receives a position payload, it will be adjusted back to the 0 to 100 range. In our example, the device value of 40 will report valve position 0.
`position_open` and `position_closed` can also be used to reverse the direction of the device: If `position_closed` is set to 100 and `position_open` is set to `0`, the device operation will be inverted. For example, when setting the position to 40, a value of 60 will be sent to the device.

## Examples

This section provides some examples showing how you can use this platform.

### Full configuration for a value that does not report position

The example below shows a full configuration for a valve that does not report position.


```yaml
# Example configuration.yaml entry
mqtt:
  - valve:
      name: "MQTT valve"
      command_template: '{"x": {{ value }} }'
      command_topic: "home-assistant/valve/set"
      state_topic: "home-assistant/valve/state"
      availability:
        - topic: "home-assistant/valve/availability"
      qos: 0
      reports_position: false
      retain: true
      payload_open: "OPEN"
      payload_close: "CLOSE"
      payload_stop: "STOP"
      state_open: "open"
      state_opening: "opening"
      state_closed: "closed"
      state_closing: "closing"
      payload_available: "online"
      payload_not_available: "offline"
      optimistic: false
      value_template: "{{ value_json.x }}"
```


### Sample configuration of a valve that reports the position

The example below shows a sample configuration for a valve that reports the position using JSON messages.


```yaml
# Example configuration.yaml entry
mqtt:
  - valve:
      name: "MQTT valve"
      command_template: '{"x": {{ value }} }'
      command_topic: "home-assistant/valve/set"
      state_topic: "home-assistant/valve/state"
      availability:
        - topic: "home-assistant/valve/availability"
      reports_position: true
      value_template: "{{ value_json.x }}"
```


### Configuration for disabling valve commands

The example below shows a configuration for a valve that does not have a close command.
Setting the `payload_close` to empty or to `null` disables the close command and will not show the close button.


```yaml
# Example configuration.yaml entry
mqtt:
  - valve:
      payload_open: "on"
      payload_close: 
      payload_stop: "on"
```


An MQTT valve will support `open` and `close` commands if a `command_topic` is set. The MQTT valve supports `stop` if `payload_stop` is set.

### Testing your configuration

To test, you can use the command line tool `mosquitto_pub` shipped with `mosquitto` or the `mosquitto-clients` package to send MQTT messages. This allows you to operate your valve manually:

```bash
mosquitto_pub -h 127.0.0.1 -t home-assistant/valve/set -m "CLOSE"
```
