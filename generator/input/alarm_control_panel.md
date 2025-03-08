---
title: "MQTT Alarm control panel"
description: "Instructions on how to integrate MQTT capable alarm panels into Home Assistant."
ha_category:
  - Alarm
ha_release: 0.7.4
ha_iot_class: Configurable
ha_domain: mqtt
related:
  - docs: /docs/configuration/
    title: Configuration file
---

The `mqtt` alarm panel {% term integration %} enables the possibility to control MQTT capable alarm panels. The Alarm icon will change state after receiving a new state from `state_topic`. If these messages are published with *RETAIN* flag, the MQTT alarm panel will receive an instant state update after subscription and will start with the correct state. Otherwise, the initial state will be `unknown`.

The {% term integration %} will accept the following states from your Alarm Panel (in lower case):

- `disarmed`
- `armed_home`
- `armed_away`
- `armed_night`
- `armed_vacation`
- `armed_custom_bypass`
- `pending`
- `triggered`
- `arming`
- `disarming`

The {% term integration %} can control your Alarm Panel by publishing to the `command_topic` when a user interacts with the Home Assistant frontend.

## Configuration

To enable this {% term integration %}, add the following lines to your {% term "`configuration.yaml`" %} file.
{% include integrations/restart_ha_after_config_inclusion.md %}

```yaml
# Example configuration.yaml entry
mqtt:
  - alarm_control_panel:
      state_topic: "home/alarm"
      command_topic: "home/alarm/set"
```


## Examples

In this section you find some real-life examples of how to use this alarm control panel.

### Configuration with partial feature support

The example below shows a full configuration with an alarm panel that only supports the `arm_home` and `arm_away` features.


```yaml
# Example with partial feature support
mqtt:
  - alarm_control_panel:
      name: "Alarm Panel"
      supported_features:
        - arm_home
        - arm_away
      state_topic: "alarmdecoder/panel"
      command_topic: "alarmdecoder/panel/set"
```


### Configuration with local code validation

The example below shows a full configuration with local code validation.


```yaml
# Example using text based code with local validation configuration.yaml
mqtt:
  - alarm_control_panel:
      name: "Alarm Panel With Numeric Keypad"
      state_topic: "alarmdecoder/panel"
      value_template: "{{value_json.state}}"
      command_topic: "alarmdecoder/panel/set"
      code: mys3cretc0de
```


### Configurations with remote code validation

The example below shows a full configuration with remote code validation and `command_template`.


```yaml
# Example using text code with remote validation configuration.yaml
mqtt:
  - alarm_control_panel:
      name: "Alarm Panel With Text Code Dialog"
      state_topic: "alarmdecoder/panel"
      value_template: "{{ value_json.state }}"
      command_topic: "alarmdecoder/panel/set"
      code: REMOTE_CODE_TEXT
      command_template: >
        { "action": "{{ action }}", "code": "{{ code }}" }
```

```yaml
# Example using numeric code with remote validation configuration.yaml
mqtt:
  - alarm_control_panel:
      name: "Alarm Panel With Numeric Keypad"
      state_topic: "alarmdecoder/panel"
      value_template: "{{ value_json.state }}"
      command_topic: "alarmdecoder/panel/set"
      code: REMOTE_CODE
      command_template: >
        { "action": "{{ action }}", "code": "{{ code }}" }
```


🚨 Caution\
When your MQTT connection is not secured, this will send your secret code over the network unprotected!
 
