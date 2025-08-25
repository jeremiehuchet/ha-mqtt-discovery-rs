---
title: "MQTT device tracker"
description: "Instructions on how to use MQTT to track devices in Home Assistant."
ha_category:
  - Presence detection
ha_iot_class: Configurable
ha_release: 0.7.3
ha_domain: mqtt
related:
  - docs: /docs/configuration/
    title: Configuration file
---


The `mqtt` device tracker `integration` allows you to define new device_trackers through [manual YAML configuration](#yaml-configuration) in `configuration.yaml` and also to automatically discover device_trackers [using the MQTT Discovery protocol](#using-the-discovery-protocol).

## Configuration

To use an MQTT device tracker in your installation, add the following to your `configuration.yaml` file.
{% include integrations/restart_ha_after_config_inclusion.md %}

```yaml
# Example configuration.yaml entry
mqtt:
  - device_tracker:
      name: "annetherese_n4"
      state_topic: "location/annetherese"
  - device_tracker:
      name: "paulus_oneplus"
      state_topic: "location/paulus"
```

Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).


## Examples

### Using the discovery protocol

The device_tracker can be created via publishing to a discovery topic that follows the following [MQTT Discovery](/integrations/mqtt/#mqtt-discovery#discovery-topic) topic name format: `<discovery_prefix>/device_tracker/[<node_id>/]<object_id>/config`.

You can use the command line tool `mosquitto_pub` shipped with `mosquitto` or the `mosquitto-clients` package to send MQTT messages.

To create the device_tracker:

```bash
mosquitto_pub -h 127.0.0.1 -t homeassistant/device_tracker/a4567d663eaf/config -m '{"state_topic": "homeassistant/device_tracker/a4567d663eaf/state", "name": "My Tracker", "payload_home": "home", "payload_not_home": "not_home"}'
```

To set the state of the device tracker to "home":

```bash
mosquitto_pub -h 127.0.0.1 -t homeassistant/device_tracker/a4567d663eaf/state -m 'home'
```

To set the state of the device tracker to a named location:

```bash
mosquitto_pub -h 127.0.0.1 -t homeassistant/device_tracker/a4567d663eaf/state -m 'location_name'
```

If the device supports GPS coordinates then they can be sent to Home Assistant by specifying an attributes topic (i.e. "json_attributes_topic") in the configuration payload:

- Attributes topic: `homeassistant/device_tracker/a4567d663eaf/attributes`
- Example attributes payload:

Example message to be received at topic `homeassistant/device_tracker/a4567d663eaf/attributes`:

```json
{
  "latitude": 32.87336,
  "longitude": -117.22743,
  "gps_accuracy": 1.2
 }
```

To create the device_tracker with GPS coordinates support:

```bash
mosquitto_pub -h 127.0.0.1 -t homeassistant/device_tracker/a4567d663eaf/config -m '{"json_attributes_topic": "homeassistant/device_tracker/a4567d663eaf/attributes", "name": "My Tracker"}'
```

🛈 Note\

Using `state_topic` is optional when using `json_attributes_topic` to determine the state of the device tracker.


To set the state of the device tracker to specific coordinates:

```bash
mosquitto_pub -h 127.0.0.1 -t homeassistant/device_tracker/a4567d663eaf/attributes -m '{"latitude": 32.87336, "longitude": -117.22743, "gps_accuracy": 1.2}'
```


### YAML configuration

The following example shows how to configure the same device tracker through configuration.yaml


```yaml
# Example configuration.yaml entry
mqtt:
  - device_tracker:
      name: "My Tracker"
      state_topic: "a4567d663eaf/state"
      payload_home: "home"
      payload_not_home: "not_home"
```

