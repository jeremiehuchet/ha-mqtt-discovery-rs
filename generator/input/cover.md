---
title: "MQTT Cover"
description: "Instructions on how to integrate MQTT covers into Home Assistant."
ha_category:
  - Cover
ha_iot_class: Configurable
ha_release: 0.18
ha_domain: mqtt
---

The `mqtt` cover platform allows you to control an MQTT cover (such as blinds, a roller shutter or a garage door).

## Configuration

A cover entity can be in states (`open`, `opening`, `closed` or `closing`).

If a `state_topic` is configured, the entity's state will be updated only after an MQTT message is received on `state_topic` matching `state_open`, `state_opening`, `state_closed` or `state_closing`. For covers that only report 3 states (`opening`, `closing`, `stopped`), a `state_stopped` state can be configured to indicate that the device is not moving. When this payload is received on the `state_topic`, and a `position_topic` is not configured, the cover will be set to state `closed` if its state was `closing` and to state `open` otherwise. If a `position_topic` is set, the cover's position will be used to set the state to either `open` or `closed` state.

If the cover reports its position, a `position_topic` can be configured for receiving the position. If no `state_topic` is configured, the cover's state will be set to either `open` or `closed` when a position is received.

If the cover reports its tilt position, a `tilt_status_topic` can be configured for receiving the tilt position.
If position topic and state topic are both defined, the device state (`open`, `opening`, `closed` or `closing`) will be set by the state topic and the cover position will be set by the position topic.

If neither a state topic nor a position topic are defined, the cover will work in optimistic mode. In this mode, the cover will immediately change state (`open` or `closed`) after every command sent by Home Assistant. If a state topic/position topic is defined, the cover will wait for a message on `state_topic` or `position_topic`.

Optimistic mode can be forced, even if a `state_topic` / `position_topic` is defined. Try to enable it if experiencing incorrect cover operation (Google Assistant gauge may need optimistic mode as it often send request to your Home Assistant immediately after send set_cover_position in which case MQTT could be too slow).

The `mqtt` cover platform optionally supports a list of `availability` topics to receive online and offline messages (birth and LWT messages) from the MQTT cover device. During normal operation, if the MQTT cover device goes offline (i.e., publishes a matching `payload_not_available` to any `availability` topic), Home Assistant will display the cover as "unavailable". If these messages are published with the `retain` flag set, the cover will receive an instant update after subscription and Home Assistant will display correct availability state of the cover when Home Assistant starts up. If the `retain` flag is not set, Home Assistant will display the cover as "unavailable" when Home Assistant starts up.

To use your MQTT cover in your installation, add the following to your {% term "`configuration.yaml`" %} file:

```yaml
# Example configuration.yaml entry
mqtt:
  - cover:
      command_topic: "living-room-cover/set"
```



🛈 Note\
MQTT cover expects position and tilt values to be in range of 0 to 100, where 0 indicates closed position and 100 indicates fully open position.
If position `min` or `max` are set to a different range (e.g. 40 to 140), when sending command to the device the range will be adjusted to the device range (position 0 will send a value of 40 to device) and when position payload is received from the device it will be adjusted back to the 0 to 100 range (device value of 40 will report cover position 0).
`min` and `max` can also be used to reverse the direction of the device, if `min` is set to 100 and `max` is set to `0` device operation will be inverted (e.g. when setting position to 40, a value of 60 will be sent to device).

## Examples

In this section you will find some real-life examples of how to use this platform.

### Full configuration state topic without tilt

The example below shows a full configuration for a cover without tilt with state topic only.


```yaml
# Example configuration.yaml entry
mqtt:
  - cover:
      name: "MQTT Cover"
      command_topic: "living-room-cover/set"
      state_topic: "living-room-cover/state"
      availability:
        - topic: "living-room-cover/availability"
      qos: 0
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
      value_template: "{{ value.x }}"
```


### Full configuration position topic without tilt

The example below shows a full configuration for a cover without tilt with position topic.


```yaml
# Example configuration.yaml entry
mqtt:
  - cover:
      name: "MQTT Cover"
      command_topic: "living-room-cover/set"
      position_topic: "living-room-cover/position"
      availability:
        - topic: "living-room-cover/availability"
      set_position_topic: "living-room-cover/set_position"
      qos: 0
      retain: true
      payload_open: "OPEN"
      payload_close: "CLOSE"
      payload_stop: "STOP"
      position_open: 100
      position_closed: 0
      payload_available: "online"
      payload_not_available: "offline"
      optimistic: false
      value_template: "{{ value.x }}"
```


### Full configuration for position, state and tilt

The example below shows a full configuration for a cover with position, state & tilt.


```yaml
# Example configuration.yaml entry
mqtt:
  - cover:
      name: "MQTT Cover"
      command_topic: "living-room-cover/set"
      state_topic: "living-room-cover/state"
      position_topic: "living-room-cover/position"
      availability:
        - topic: "living-room-cover/availability"
      qos: 0
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
      value_template: "{{ value.x }}"
      position_template: "{{ value.y }}"
      tilt_command_topic: "living-room-cover/tilt"
      tilt_status_topic: "living-room-cover/tilt-state"
      tilt_status_template: "{{ value_json["PWM"]["PWM1"] }}"
      tilt_min: 0
      tilt_max: 180
      tilt_closed_value: 70
      tilt_opened_value: 180
```


### Full configuration using stopped state

The example below shows a full configuration for a cover using stopped state.


```yaml
# Example configuration.yaml entry
mqtt:
  - cover:
      name: "MQTT Cover"
      command_topic: "living-room-cover/set"
      state_topic: "living-room-cover/state"
      position_topic: "living-room-cover/position"
      availability:
        - topic: "living-room-cover/availability"
      qos: 0
      retain: true
      payload_open: "OPEN"
      payload_close: "CLOSE"
      payload_stop: "STOP"
      state_opening: "opening"
      state_closed: "closed"
      state_stopped: "stopped"
      payload_available: "online"
      payload_not_available: "offline"
      optimistic: false
      value_template: "{{ value.x }}"
      position_template: "{{ value.y }}"
```


### Configuration for disabling cover commands

The example below shows a configuration for a cover that does not have a close command.
Setting `payload_close` empty or to `null` disables the close command and will not show the close button.


```yaml
# Example configuration.yaml entry
mqtt:
  - cover:
      payload_open: "on"
      payload_close: 
      payload_stop: "on"
```

The following commands can be disabled: `open`, `close`, `stop` by overriding their payloads: `payload_open`, `payload_close`, `payload_stop`

For auto discovery message the payload needs to be set to `null`, example for cover without close command:

```json
{
  "cover": [
    {
      "payload_open": "on",
      "payload_close": null,
      "payload_stop": "on"
    }
  ]
}
```


### Full configuration using `entity_id`- variable in the template

The example below shows an example of how to correct the state of the blind depending if it moved up, or down.


```yaml
# Example configuration.yaml entry
mqtt:
  - cover:
      name: "MQTT Cover"
      command_topic: "living-room-cover/set"
      state_topic: "living-room-cover/state"
      position_topic: "living-room-cover/position"
      set_position_topic: "living-room-cover/position/set"
      payload_open:  "open"
      payload_close: "close"
      payload_stop:  "stop"
      state_opening: "open"
      state_closing: "close"
      state_stopped: "stop"
      optimistic: false
      position_template: |-
        {% if not state_attr(entity_id, "current_position") %}
          {{ value }}
        {% elif state_attr(entity_id, "current_position") < (value | int) %}
          {{ (value | int + 1) }}
        {% elif state_attr(entity_id, "current_position") > (value | int) %}
          {{ (value | int - 1) }}
        {% else %}
          {{ value }}
        {% endif %}
```


### Full configuration using advanced templating

The `position_template` can accept JSON, where `position` and `tilt_position` is provided at the same time.

The example below shows a full example of how to set up a venetian blind which has a combined position and tilt topic. The blind in the example has moveable slats which tilt with a position change. In the example, it takes the blind 6% of the movement for a full rotation of the slats.

Following variable might be used in `position_template`, `set_position_template`, `tilt_command_template` and `tilt_status_template`, `json_attributes_template` (only `entity_id`).

- `entity_id` - The ID of the entity itself. It can be used to reference its attributes with the help of the [states](/docs/configuration/templating/#states) template function.
- `position_open`
- `position_closed`
- `tilt_min`
- `tilt_max`


```yaml
# Example configuration.yaml entry
mqtt:
  - cover:
      name: "MQTT Cover"
      command_topic: "living-room-cover/set"
      state_topic: "living-room-cover/state"
      position_topic: "living-room-cover/position"
      set_position_topic: "living-room-cover/position/set"
      tilt_command_topic: "living-room-cover/position/set" # same as `set_position_topic`
      qos: 1
      retain: false
      payload_open:  "open"
      payload_close: "close"
      payload_stop:  "stop"
      state_opening: "open"
      state_closing: "close"
      state_stopped: "stop"
      position_open: 100
      position_closed: 0
      tilt_min: 0
      tilt_max: 6
      tilt_opened_value: 3
      tilt_closed_value: 0
      optimistic: false
      position_template: |-
        {% if not state_attr(entity_id, "current_position") %}
          {
            "position" : {{ value }},
            "tilt_position" : 0
          }
        {% else %}
          {% set old_position = state_attr(entity_id, "current_position") %}
          {% set old_tilt_percent = (state_attr(entity_id, "current_tilt_position")) %}

          {% set movement = value | int - old_position %}
          {% set old_tilt_position = (old_tilt_percent / 100 * (tilt_max - tilt_min)) %}
          {% set new_tilt_position = min(max((old_tilt_position + movement), tilt_min), tilt_max) %}
  
          {
            "position": {{ value }},
            "tilt_position": {{ new_tilt_position }}
          }
        {% endif %}
    tilt_command_template: >-
        {% set position = state_attr(entity_id, "current_position") %}
        {% set tilt = state_attr(entity_id, "current_tilt_position") %}
        {% set movement = (tilt_position - tilt) / 100 * tilt_max %}
        {{ position + movement }}
      payload_open: "on"
      payload_close: 
      payload_stop: "on"
```


### Testing your configuration

To test, you can use the command line tool `mosquitto_pub` shipped with `mosquitto` or the `mosquitto-clients` package to send MQTT messages. This allows you to operate your cover manually:

```bash
mosquitto_pub -h 127.0.0.1 -t living-room-cover/set -m "CLOSE"
```
