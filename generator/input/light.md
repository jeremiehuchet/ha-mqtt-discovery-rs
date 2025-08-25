---
title: "MQTT Light"
description: "Instructions on how to setup MQTT lights using default schema within Home Assistant."
ha_category:
  - Light
ha_iot_class: Configurable
ha_release: 0.8
ha_domain: mqtt
---

The `mqtt` light platform lets you control your MQTT enabled lights through one of the supported message schemas, `default`, `json` or `template`.

## Comparison of light MQTT schemas

| Function          | [`default`](#default-schema) | [`json`](#json-schema) | [`template`](#template-schema) |
| ----------------- | ---------------------------- | ---------------------- | ------------------------------ |
| Brightness        | ✔                            | ✔                      | ✔                              |
| Color mode        | ✔                            | ✔                      | ✘                              |
| Color temperature | ✔                            | ✔                      | ✔                              |
| Effects           | ✔                            | ✔                      | ✔                              |
| Flashing          | ✘                            | ✔                      | ✔                              |
| HS Color          | ✔                            | ✔                      | ✔                              |
| RGB Color         | ✔                            | ✔                      | ✔                              |
| RGBW Color        | ✔                            | ✔                      | ✘                              |
| RGBWW Color       | ✔                            | ✔                      | ✘                              |
| Transitions       | ✘                            | ✔                      | ✔                              |
| White             | ✔                            | ✔                      | ✘                              |
| XY Color          | ✔                            | ✔                      | ✘                              |

## Default schema

The `mqtt` light platform with default schema lets you control your MQTT enabled lights. It supports setting brightness, color temperature, effects, on/off, RGB colors, XY colors and white.

## Default schema - Configuration

In an ideal scenario, the MQTT device will have a state topic to publish state changes. If these messages are published with a `RETAIN` flag, the MQTT light will receive an instant state update after subscription and will start with the correct state. Otherwise, the initial state of the switch will be `unknown`. A MQTT device can reset the current state to `unknown` using a `None` payload.

When a state topic is not available, the light will work in optimistic mode. In this mode, the light will immediately change state after every command. Otherwise, the light will wait for state confirmation from the device (message from `state_topic`). The initial state is set to `False` / `off` in optimistic mode.

Optimistic mode can be forced, even if the `state_topic` is available. Try to enable it, if experiencing incorrect light operation.

Home Assistant internally assumes that a light's state corresponds to a defined `color_mode`.
The state of MQTT lights with default schema and support for both color and color temperature will set the `color_mode` according to the last received valid color or color temperature. Optionally, a `color_mode_state_topic` can be configured for explicit control of the `color_mode`.

To use an MQTT basic light in your installation, [add a MQTT device as a subentry](/integrations/mqtt/#configuration), or add the following to your `configuration.yaml` file.
{% include integrations/restart_ha_after_config_inclusion.md %}

```yaml
# Example configuration.yaml entry
mqtt:
  - light:
      command_topic: "office/rgb1/light/switch"
```

Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).


⚠ Important\
Make sure that your topics match exactly. `some-topic/` and `some-topic` are different topics.

🛈 Note\
XY and RGB can not be used at the same time. If both are provided, XY overrides RGB.

## Default schema - Examples

In this section you will find some real-life examples of how to use this sensor.

### Brightness and RGB support

To enable a light with brightness and RGB support in your installation, add the following to your `configuration.yaml` file:


```yaml
# Example configuration.yaml entry
mqtt:
  - light:
      name: "Office Light RGB"
      state_topic: "office/rgb1/light/status"
      command_topic: "office/rgb1/light/switch"
      brightness_state_topic: "office/rgb1/brightness/status"
      brightness_command_topic: "office/rgb1/brightness/set"
      rgb_state_topic: "office/rgb1/rgb/status"
      rgb_command_topic: "office/rgb1/rgb/set"
      state_value_template: "{{ value_json.state }}"
      brightness_value_template: "{{ value_json.brightness }}"
      rgb_value_template: "{{ value_json.rgb | join(',') }}"
      qos: 0
      payload_on: "ON"
      payload_off: "OFF"
      optimistic: false
```


### Brightness and no RGB support

To enable a light with brightness (no RGB version) in your installation, add the following to your `configuration.yaml` file:

```yaml
# Example configuration.yaml entry
mqtt:
  - light:
      name: "Office light"
      state_topic: "office/light/status"
      command_topic: "office/light/switch"
      brightness_state_topic: 'office/light/brightness'
      brightness_command_topic: 'office/light/brightness/set'
      qos: 0
      payload_on: "ON"
      payload_off: "OFF"
      optimistic: false
```

### Brightness without on commands

To enable a light that sends only brightness topics to turn it on, add the following to your `configuration.yaml` file. The `command_topic` is only used to send an off command in this case:

```yaml
# Example configuration.yaml entry
mqtt:
  - light:
      name: "Brightness light"
      state_topic: "office/light/status"
      command_topic: "office/light/switch"
      payload_off: "OFF"
      brightness_state_topic: 'office/light/brightness'
      brightness_command_topic: 'office/light/brightness/set'
      on_command_type: 'brightness'
```

## Default schema - Implementations

- A [basic example](https://github.com/mertenats/open-home-automation/tree/master/ha_mqtt_light) using a NodeMCU board (ESP8266) to control its built-in LED (on/off).
- Another [example](https://github.com/mertenats/open-home-automation/tree/master/ha_mqtt_rgb_light) to control a RGB LED (on/off, brightness, and colors).
- [Integration guide](https://github.com/xoseperez/espurna/wiki/HomeAssistant) for the ESPUrna firmware (ESP8285/ESP8266).

## JSON schema

The `mqtt` light platform with JSON schema lets you control a MQTT-enabled light that can receive [JSON](https://en.wikipedia.org/wiki/JSON) messages.

This schema supports on/off, brightness, RGB colors, XY colors, color temperature, transitions and short/long flashing. The messages sent to/from the lights look similar to this, omitting fields when they aren't needed. The `color_mode` will not be present in messages sent to the light. It is optional in messages received from the light, but can be used to disambiguate the current mode in the light. In the example below, `color_mode` is set to `rgb` and `color_temp`, `color.c`, `color.w`, `color.x`, `color.y`, `color.h`, `color.s` will all be ignored by Home Assistant:

```json
{
  "brightness": 255,
  "color_mode": "rgb",
  "color_temp": 155,
  "color": {
    "r": 255,
    "g": 180,
    "b": 200,
    "c": 100,
    "w": 50,
    "x": 0.406,
    "y": 0.301,
    "h": 344.0,
    "s": 29.412
  },
  "effect": "colorloop",
  "state": "ON",
  "transition": 2,
}
```

## JSON schema - Configuration

In an ideal scenario, the MQTT device will have a state topic to publish state changes. If these messages are published with the RETAIN flag, the MQTT light will receive an instant state update after subscription and will start with the correct state. Otherwise, the initial state of the light will be off.

When a state topic is not available, the light will work in optimistic mode. In this mode, the light will immediately change state after every command. Otherwise, the light will wait for state confirmation from the device (message from `state_topic`).

Optimistic mode can be forced, even if state topic is available. Try enabling it if the light is operating incorrectly.

To use an MQTT JSON light in your installation, [add a MQTT device as a subentry](/integrations/mqtt/#configuration), or add the following to your `configuration.yaml` file.
{% include integrations/restart_ha_after_config_inclusion.md %}

```yaml
# Example configuration.yaml entry
mqtt:
  - light:
      schema: json
      command_topic: "home/rgb1/set"
```

Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).


*The `color` attribute dict in the JSON state payload should contain the following keys based on the `color_mode`:

- `hs`:
  - `h`: The hue value
  - `s`: The saturation value
- `xy`:
  - `x`: X color value
  - `y`: Y color value
- `rgb`:
  - `r`: Red color value
  - `g`: Green color value
  - `b`: Blue color value
- `rgbw`:
  - `r`: Red color value
  - `g`: Green color value
  - `b`: Blue color value
  - `w`: White value
- `rgbww`:
  - `r`: Red color value
  - `g`: Green color value
  - `b`: Blue color value
  - `c`: Cool white value
  - `w`: Warm white value

More details about the different colors, color modes and range values [can be found here](https://www.home-assistant.io/integrations/light/).

⚠ Important\
Make sure that your topics match exact. `some-topic/` and `some-topic` are different topics.

🛈 Note\
RGB, XY and HSV can not be used at the same time in `state_topic` messages. Make sure that only one of the color models is in the "color" section of the state MQTT payload.

## JSON schema - Examples

In this section you find some real-life examples of how to use this sensor.

### Brightness and RGB support

To enable a light with brightness and RGB support in your installation, add the following to your `configuration.yaml` file:

```yaml
# Example configuration.yaml entry
mqtt:
  - light:
      schema: json
      name: mqtt_json_light_1
      state_topic: "home/rgb1"
      command_topic: "home/rgb1/set"
      brightness: true
      supported_color_modes: ["rgb"]
```

### Brightness and no RGB support

To enable a light with brightness (but no color support) in your installation, add the following to your `configuration.yaml` file:

```yaml
# Example configuration.yaml entry
mqtt:
  - light:
      schema: json
      name: mqtt_json_light_1
      state_topic: "home/rgb1"
      command_topic: "home/rgb1/set"
      brightness: true
      supported_color_modes: ["brightness"]
```

### Brightness scaled

To enable a light using a brightness scale other than 8bit the `brightness_scale` option may be added to denote the "fully on" value:

```yaml
# Example configuration.yaml entry
mqtt:
  - light:
      schema: json
      name: mqtt_json_light_1
      state_topic: "home/light"
      command_topic: "home/light/set"
      brightness: true
      brightness_scale: 4095
      supported_color_modes: ["brightness"]
```

Home Assistant will then convert its 8bit value in the message to and from the device:

```json
{
  "brightness": 4095,
  "state": "ON"
}
```

### HS color

To use a light with hue+saturation as the color model, set `supported_color_modes` to `["hs"]` in the platform configuration:

```yaml
mqtt:
  - light:
      schema: json
      name: mqtt_json_hs_light
      state_topic: "home/light"
      command_topic: "home/light/set"
      supported_color_modes: ["hs"]
```

Home Assistant expects the hue values to be in the range 0 to 360 and the saturation values to be scaled from 0 to 100. For example, the following is a blue color shade:

```json
{
  "state": "ON",
  "color_mode": "hs",
  "color": {
    "h": 24.0,
    "s": 100.0
  }
}
```

### Brightness and RGBW support

To enable a light with brightness, RGB support and a separate white channel (RGBW) in your installation, add the following to your `configuration.yaml` file:

```yaml
# Example configuration.yaml entry
mqtt:
  - light:
      schema: json
      name: mqtt_json_light_1
      state_topic: "home/rgbw1"
      command_topic: "home/rgbw1/set"
      brightness: true
      supported_color_modes: ["rgbw"]
```

## Implementations

- A full example of custom lighting using this platform and an ESP8266 microcontroller can be found [here](https://github.com/corbanmailloux/esp-mqtt-rgb-led). It supports on/off, brightness, transitions, RGB colors, and flashing.

- There is also another implementation forked from the above repository, it supports all the same features but is made for addressable LED strips using FastLED on a NodeMCU V3 it can be found [here](https://github.com/JammyDodger231/nodemcu-mqtt-rgb-led).

- [McLighting](https://github.com/toblum/McLighting) is another ESP8266 firmware for WS2812 addressable LEDs.

- [MQTT JSON Light](https://github.com/mertenats/Open-Home-Automation/tree/master/ha_mqtt_rgbw_light_with_discovery) is another implementation for ESP8266 including [MQTT discovery](/integrations/mqtt/#mqtt-discovery).

- [ESPHome](https://esphome.io) implements the JSON schema for MQTT based installs and supports [MQTT discovery](/integrations/mqtt/#mqtt-discovery).

- [AiLight](https://github.com/stelgenhof/AiLight) is a custom firmware for the Ai-Thinker (and equivalent) RGBW WiFi light bulbs that has an ESP8266 onboard and controlled by the MY9291 LED driver. It implements the [MQTT JSON light](/integrations/light.mqtt) platform and supports ON/OFF, RGBW colours, brightness, color temperature, flashing and transitions. Also it includes [MQTT Auto Discovery](/integrations/mqtt/#mqtt-discovery)) and the MQTT Last Will and Testament is enabled as well.

- [h801-mqtt-json](https://github.com/starkillerOG/h801-mqtt-json) is a custom firmware for the H801 LED dimmer, a 5 channel (RGBWWCW)  WiFi LED strip controller for 12V LED strips. The firmware is meant to control the 5 channels of the H801 to simultaneously control an RGB and a Warm-white/Cold-white LED strip such as a 5050 RGB LED strip and a 5025 Dual White strip. It implements the [MQTT JSON light](/integrations/light.mqtt) platform and supports ON/OFF, RGBW colours (RGB strip), brightness, color temperature (CW/WW strip) and transitions.

## Template schema

The `mqtt` light platform with template schema lets you control a MQTT-enabled light that receive commands on a command topic and optionally sends status update on a state topic.
It is format-agnostic so you can use any data format you want (i.e., string, JSON), just configure it with templating.

This schema supports on/off, brightness, RGB colors, XY colors, HS Color, color temperature, transitions, short/long flashing and effects.

## Template schema - Configuration

In an ideal scenario, the MQTT device will have a state topic to publish state changes. If these messages are published with the RETAIN flag, the MQTT light will receive an instant state update after subscription and will start with the correct state. Otherwise, the initial state of the light will be off.

When a state topic is not available, the light will work in optimistic mode. In this mode, the light will immediately change state after every command. Otherwise, the light will wait for state confirmation from the device (message from `state_topic`).

Optimistic mode can be forced, even if state topic is available. Try enabling it if the light is operating incorrectly.

To use an MQTT template light in your installation, [add a MQTT device as a subentry](/integrations/mqtt/#configuration), or add the following to your `configuration.yaml` file.
{% include integrations/restart_ha_after_config_inclusion.md %}

```yaml
# Example configuration.yaml entry
mqtt:
  - light:
      schema: template
      command_topic: "home/rgb1/set"
      command_on_template: "on"
      command_off_template: "off"
```

Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).


⚠ Important\
Make sure that your topics match exact. `some-topic/` and `some-topic` are different topics.

## Template schema - Examples

In this section you find some real-life examples of how to use this light.

### Simple string payload

For a simple string payload with the format `state,brightness,r-g-b,h-s` (e.g., `on,255,255-255-255,360-100`), add the following to your `configuration.yaml` file:


```yaml
# Example configuration.yaml entry
mqtt:
  - light:
      schema: template
      command_topic: "home/rgb1/set"
      state_topic: "home/rgb1/status"
      command_on_template: "on,{{ brightness|d }},{{ red|d }}-{{ green|d }}-{{ blue|d }},{{ hue|d }}-{{ sat|d }}"
      command_off_template: "off"
      state_template: "{{ value.split(',')[0] }}"  # must return `on` or `off`
      brightness_template: "{{ value.split(',')[1] }}"
      red_template: "{{ value.split(',')[2].split('-')[0] }}"
      green_template: "{{ value.split(',')[2].split('-')[1] }}"
      blue_template: "{{ value.split(',')[2].split('-')[2] }}"
```


### JSON payload

For a JSON payload with the format `{"state": "on", "brightness": 255, "color": [255, 255, 255], "effect": "rainbow"}`, add the following to your `configuration.yaml` file:


```yaml
# Example configuration.yaml entry
mqtt:
  - light:
      schema: template
      effect_list:
        - rainbow
        - colorloop
      command_topic: "home/rgb1/set"
      state_topic: "home/rgb1/status"
      command_on_template: >
        {"state": "on"
        {%- if brightness is defined -%}
        , "brightness": {{ brightness }}
        {%- endif -%}
        {%- if red is defined and green is defined and blue is defined -%}
        , "color": [{{ red }}, {{ green }}, {{ blue }}]
        {%- endif -%}
        {%- if hue is defined and sat is defined -%}
        , "huesat": [{{ hue }}, {{ sat }}]
        {%- endif -%}
        {%- if effect is defined -%}
        , "effect": "{{ effect }}"
        {%- endif -%}
        }
      command_off_template: '{"state": "off"}'
      state_template: '{{ value_json.state }}'
      brightness_template: '{{ value_json.brightness }}'
      red_template: '{{ value_json.color[0] }}'
      green_template: '{{ value_json.color[1] }}'
      blue_template: '{{ value_json.color[2] }}'
      effect_template: '{{ value_json.effect }}'
```


### CCT light (brightness and temperature)

This example comes from a configuration of Shelly RGBW Bulb working in White mode.
`max_mireds` and `min_mireds` set color temperature boundaries to 3000K - 6500K. Notice the same limits are applied in `command_on_template`, but in kelvin units this time. It's due to conversion from mired to kelvin which causes exceeding boundary values accepted by the device.
The code also ensures bi-directional conversion of brightness scale between 0-100 (required by the device) and 0-255 (required by Home Assistant).
Add the following to your `configuration.yaml` file:


```yaml
# Example configuration.yaml entry
mqtt:
  - light:
      schema: template
      name: "Bulb-white"
      command_topic: "shellies/bulb/color/0/set"
      state_topic: "shellies/bulb/color/0/status"
      availability_topic: "shellies/bulb/online"
      command_on_template: >
        {"turn": "on", "mode": "white"
        {%- if brightness is defined -%}
        , "brightness": {{brightness | float | multiply(0.39215686) | round(0)}}
        {%- endif -%}
        {%- if color_temp is defined -%}
        , "temp": {{ [[(1000000 / color_temp | float) | round(0), 3000] | max, 6500] | min }}
        {%- endif -%}
        }
      command_off_template: '{"turn":"off", "mode": "white"}'
      state_template: "{% if value_json.ison and value_json.mode == 'white' %}on{% else %}off{% endif %}"
      brightness_template: "{{ value_json.brightness | float | multiply(2.55) | round(0) }}"
      color_temp_template: "{{ (1000000 / value_json.temp | float) | round(0) }}"
      payload_available: "true"
      payload_not_available: "false"
      max_mireds: 334
      min_mireds: 153
      qos: 1
      retain: false
      optimistic: false  
```


### Template schema - No brightness or color support

If you don't want brightness, color or effect support, just omit the corresponding configuration sections.
