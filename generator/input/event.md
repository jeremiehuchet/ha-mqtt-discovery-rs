---
title: "MQTT Event"
description: "Instructions on how to integrate MQTT events into Home Assistant."
ha_category:
  - Event
ha_release: 2023.8
ha_iot_class: Configurable
ha_domain: mqtt
---

The `mqtt` event platform allows you to process event info from an MQTT message. Events are signals that are emitted when something happens, for example, when a user presses a physical button like a doorbell or when a button on a remote control is pressed. With the event some event attributes can be sent to become available as an attribute on the entity. MQTT events are stateless. For example, a doorbell does not have a state like being "on" or "off" but instead is momentarily pressed.

## Configuration

```yaml
# Example configuration.yaml entry
mqtt:
  - event:
      state_topic: "home/doorbell/state"
      event_types:
        - press
```


⚠ Important\
Make sure that your topic matches exactly. `some-topic/` and `some-topic` are different topics.

### Full configuration with JSON data

The example below shows a full configuration for an event MQTT entity.

```yaml
# Example configuration.yaml entry
mqtt:
  - event:
      state_topic: "home/doorbell/state"
      event_types:
        - "press"
        - "hold"
      availability:
        - topic: "home/doorbell/available"
      qos: 0
      device_class: "doorbell"
```

The event information is extracted from a JSON formatted MQTT message.
To test, you can use the command line tool `mosquitto_pub` shipped with `mosquitto` or the `mosquitto-clients` package to send MQTT messages.

To set trigger the `mqtt` event entity manually:

```bash
mosquitto_pub -h 127.0.0.1 -t home/doorbell/available -m "online"
mosquitto_pub -h 127.0.0.1 -t home/doorbell/state -m '{"event_type": "hold"}'
```

Besides the required `event_type` element, the payload can contain additional event attributes.
These additional attribute updates will be exposed as attributes on the `mqtt` event entity.

The example below demonstrates how event attributes can be added to the event data.

```bash
mosquitto_pub -h 127.0.0.1 -t home/doorbell/state -m '{"event_type": "press", "duration": 0.1}'
```

### Example: processing event data using a value template

In many cases, translation of an existing published payload is needed.
The example config below translates the payload `{"Button1": {"Action": "SINGLE"}}` of
the device `Button1` with event type `single` to the required format.
An extra attribute `button` will be set to `Button1` and be added to the entity,
but only if the `Action` property is set. Empty dictionaries will be ignored.


```yaml
mqtt:
  - event:
      name: "Desk button"
      state_topic: "desk/button/state"
      event_types:
        - single
        - double
      device_class: "button"
      value_template: |
        { {% for key in value_json %}
        {% if value_json[key].get("Action") %}
        "button": "{{ key }}",
        "event_type": "{{ value_json[key].Action | lower }}"
        {% endif %}
        {% endfor %} }
```

