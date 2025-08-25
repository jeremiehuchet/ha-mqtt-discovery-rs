use super::common::Qos;
use super::common::{Availability, Device, EntityCategory, Origin};
use crate::Entity;
use serde_derive::Serialize;

/// ---
/// title: "MQTT Light"
/// description: "Instructions on how to setup MQTT lights using default schema within Home Assistant."
/// ha_category:
///   - Light
/// ha_iot_class: Configurable
/// ha_release: 0.8
/// ha_domain: mqtt
/// ---
///
/// The `mqtt` light platform lets you control your MQTT enabled lights through one of the supported message schemas, `default`, `json` or `template`.
///
/// ## Comparison of light MQTT schemas
///
/// | Function          | [`default`](#default-schema) | [`json`](#json-schema) | [`template`](#template-schema) |
/// | ----------------- | ---------------------------- | ---------------------- | ------------------------------ |
/// | Brightness        | ✔                            | ✔                      | ✔                              |
/// | Color mode        | ✔                            | ✔                      | ✘                              |
/// | Color temperature | ✔                            | ✔                      | ✔                              |
/// | Effects           | ✔                            | ✔                      | ✔                              |
/// | Flashing          | ✘                            | ✔                      | ✔                              |
/// | HS Color          | ✔                            | ✔                      | ✔                              |
/// | RGB Color         | ✔                            | ✔                      | ✔                              |
/// | RGBW Color        | ✔                            | ✔                      | ✘                              |
/// | RGBWW Color       | ✔                            | ✔                      | ✘                              |
/// | Transitions       | ✘                            | ✔                      | ✔                              |
/// | White             | ✔                            | ✔                      | ✘                              |
/// | XY Color          | ✔                            | ✔                      | ✘                              |
///
/// ## Default schema
///
/// The `mqtt` light platform with default schema lets you control your MQTT enabled lights. It supports setting brightness, color temperature, effects, on/off, RGB colors, XY colors and white.
///
/// ## Default schema - Configuration
///
/// In an ideal scenario, the MQTT device will have a state topic to publish state changes. If these messages are published with a `RETAIN` flag, the MQTT light will receive an instant state update after subscription and will start with the correct state. Otherwise, the initial state of the switch will be `unknown`. A MQTT device can reset the current state to `unknown` using a `None` payload.
///
/// When a state topic is not available, the light will work in optimistic mode. In this mode, the light will immediately change state after every command. Otherwise, the light will wait for state confirmation from the device (message from `state_topic`). The initial state is set to `False` / `off` in optimistic mode.
///
/// Optimistic mode can be forced, even if the `state_topic` is available. Try to enable it, if experiencing incorrect light operation.
///
/// Home Assistant internally assumes that a light's state corresponds to a defined `color_mode`.
/// The state of MQTT lights with default schema and support for both color and color temperature will set the `color_mode` according to the last received valid color or color temperature. Optionally, a `color_mode_state_topic` can be configured for explicit control of the `color_mode`.
///
/// To use an MQTT basic light in your installation, [add a MQTT device as a subentry](/integrations/mqtt/#configuration), or add the following to your `configuration.yaml` file.
/// {% include integrations/restart_ha_after_config_inclusion.md %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - light:
///       command_topic: "office/rgb1/light/switch"
/// ```
///
/// Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
///
///
/// ⚠ Important\
/// Make sure that your topics match exactly. `some-topic/` and `some-topic` are different topics.
///
/// 🛈 Note\
/// XY and RGB can not be used at the same time. If both are provided, XY overrides RGB.
///
/// ## Default schema - Examples
///
/// In this section you will find some real-life examples of how to use this sensor.
///
/// ### Brightness and RGB support
///
/// To enable a light with brightness and RGB support in your installation, add the following to your `configuration.yaml` file:
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - light:
///       name: "Office Light RGB"
///       state_topic: "office/rgb1/light/status"
///       command_topic: "office/rgb1/light/switch"
///       brightness_state_topic: "office/rgb1/brightness/status"
///       brightness_command_topic: "office/rgb1/brightness/set"
///       rgb_state_topic: "office/rgb1/rgb/status"
///       rgb_command_topic: "office/rgb1/rgb/set"
///       state_value_template: "{{ value_json.state }}"
///       brightness_value_template: "{{ value_json.brightness }}"
///       rgb_value_template: "{{ value_json.rgb | join(',') }}"
///       qos: 0
///       payload_on: "ON"
///       payload_off: "OFF"
///       optimistic: false
/// ```
///
///
/// ### Brightness and no RGB support
///
/// To enable a light with brightness (no RGB version) in your installation, add the following to your `configuration.yaml` file:
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - light:
///       name: "Office light"
///       state_topic: "office/light/status"
///       command_topic: "office/light/switch"
///       brightness_state_topic: 'office/light/brightness'
///       brightness_command_topic: 'office/light/brightness/set'
///       qos: 0
///       payload_on: "ON"
///       payload_off: "OFF"
///       optimistic: false
/// ```
///
/// ### Brightness without on commands
///
/// To enable a light that sends only brightness topics to turn it on, add the following to your `configuration.yaml` file. The `command_topic` is only used to send an off command in this case:
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - light:
///       name: "Brightness light"
///       state_topic: "office/light/status"
///       command_topic: "office/light/switch"
///       payload_off: "OFF"
///       brightness_state_topic: 'office/light/brightness'
///       brightness_command_topic: 'office/light/brightness/set'
///       on_command_type: 'brightness'
/// ```
///
/// ## Default schema - Implementations
///
/// - A [basic example](https://github.com/mertenats/open-home-automation/tree/master/ha_mqtt_light) using a NodeMCU board (ESP8266) to control its built-in LED (on/off).
/// - Another [example](https://github.com/mertenats/open-home-automation/tree/master/ha_mqtt_rgb_light) to control a RGB LED (on/off, brightness, and colors).
/// - [Integration guide](https://github.com/xoseperez/espurna/wiki/HomeAssistant) for the ESPUrna firmware (ESP8285/ESP8266).
///
/// ## JSON schema
///
/// The `mqtt` light platform with JSON schema lets you control a MQTT-enabled light that can receive [JSON](https://en.wikipedia.org/wiki/JSON) messages.
///
/// This schema supports on/off, brightness, RGB colors, XY colors, color temperature, transitions and short/long flashing. The messages sent to/from the lights look similar to this, omitting fields when they aren't needed. The `color_mode` will not be present in messages sent to the light. It is optional in messages received from the light, but can be used to disambiguate the current mode in the light. In the example below, `color_mode` is set to `rgb` and `color_temp`, `color.c`, `color.w`, `color.x`, `color.y`, `color.h`, `color.s` will all be ignored by Home Assistant:
///
/// ```json
/// {
///   "brightness": 255,
///   "color_mode": "rgb",
///   "color_temp": 155,
///   "color": {
///     "r": 255,
///     "g": 180,
///     "b": 200,
///     "c": 100,
///     "w": 50,
///     "x": 0.406,
///     "y": 0.301,
///     "h": 344.0,
///     "s": 29.412
///   },
///   "effect": "colorloop",
///   "state": "ON",
///   "transition": 2,
/// }
/// ```
///
/// ## JSON schema - Configuration
///
/// In an ideal scenario, the MQTT device will have a state topic to publish state changes. If these messages are published with the RETAIN flag, the MQTT light will receive an instant state update after subscription and will start with the correct state. Otherwise, the initial state of the light will be off.
///
/// When a state topic is not available, the light will work in optimistic mode. In this mode, the light will immediately change state after every command. Otherwise, the light will wait for state confirmation from the device (message from `state_topic`).
///
/// Optimistic mode can be forced, even if state topic is available. Try enabling it if the light is operating incorrectly.
///
/// To use an MQTT JSON light in your installation, [add a MQTT device as a subentry](/integrations/mqtt/#configuration), or add the following to your `configuration.yaml` file.
/// {% include integrations/restart_ha_after_config_inclusion.md %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - light:
///       schema: json
///       command_topic: "home/rgb1/set"
/// ```
///
/// Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
///
///
/// *The `color` attribute dict in the JSON state payload should contain the following keys based on the `color_mode`:
///
/// - `hs`:
///   - `h`: The hue value
///   - `s`: The saturation value
/// - `xy`:
///   - `x`: X color value
///   - `y`: Y color value
/// - `rgb`:
///   - `r`: Red color value
///   - `g`: Green color value
///   - `b`: Blue color value
/// - `rgbw`:
///   - `r`: Red color value
///   - `g`: Green color value
///   - `b`: Blue color value
///   - `w`: White value
/// - `rgbww`:
///   - `r`: Red color value
///   - `g`: Green color value
///   - `b`: Blue color value
///   - `c`: Cool white value
///   - `w`: Warm white value
///
/// More details about the different colors, color modes and range values [can be found here](https://www.home-assistant.io/integrations/light/).
///
/// ⚠ Important\
/// Make sure that your topics match exact. `some-topic/` and `some-topic` are different topics.
///
/// 🛈 Note\
/// RGB, XY and HSV can not be used at the same time in `state_topic` messages. Make sure that only one of the color models is in the "color" section of the state MQTT payload.
///
/// ## JSON schema - Examples
///
/// In this section you find some real-life examples of how to use this sensor.
///
/// ### Brightness and RGB support
///
/// To enable a light with brightness and RGB support in your installation, add the following to your `configuration.yaml` file:
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - light:
///       schema: json
///       name: mqtt_json_light_1
///       state_topic: "home/rgb1"
///       command_topic: "home/rgb1/set"
///       brightness: true
///       supported_color_modes: ["rgb"]
/// ```
///
/// ### Brightness and no RGB support
///
/// To enable a light with brightness (but no color support) in your installation, add the following to your `configuration.yaml` file:
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - light:
///       schema: json
///       name: mqtt_json_light_1
///       state_topic: "home/rgb1"
///       command_topic: "home/rgb1/set"
///       brightness: true
///       supported_color_modes: ["brightness"]
/// ```
///
/// ### Brightness scaled
///
/// To enable a light using a brightness scale other than 8bit the `brightness_scale` option may be added to denote the "fully on" value:
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - light:
///       schema: json
///       name: mqtt_json_light_1
///       state_topic: "home/light"
///       command_topic: "home/light/set"
///       brightness: true
///       brightness_scale: 4095
///       supported_color_modes: ["brightness"]
/// ```
///
/// Home Assistant will then convert its 8bit value in the message to and from the device:
///
/// ```json
/// {
///   "brightness": 4095,
///   "state": "ON"
/// }
/// ```
///
/// ### HS color
///
/// To use a light with hue+saturation as the color model, set `supported_color_modes` to `["hs"]` in the platform configuration:
///
/// ```yaml
/// mqtt:
///   - light:
///       schema: json
///       name: mqtt_json_hs_light
///       state_topic: "home/light"
///       command_topic: "home/light/set"
///       supported_color_modes: ["hs"]
/// ```
///
/// Home Assistant expects the hue values to be in the range 0 to 360 and the saturation values to be scaled from 0 to 100. For example, the following is a blue color shade:
///
/// ```json
/// {
///   "state": "ON",
///   "color_mode": "hs",
///   "color": {
///     "h": 24.0,
///     "s": 100.0
///   }
/// }
/// ```
///
/// ### Brightness and RGBW support
///
/// To enable a light with brightness, RGB support and a separate white channel (RGBW) in your installation, add the following to your `configuration.yaml` file:
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - light:
///       schema: json
///       name: mqtt_json_light_1
///       state_topic: "home/rgbw1"
///       command_topic: "home/rgbw1/set"
///       brightness: true
///       supported_color_modes: ["rgbw"]
/// ```
///
/// ## Implementations
///
/// - A full example of custom lighting using this platform and an ESP8266 microcontroller can be found [here](https://github.com/corbanmailloux/esp-mqtt-rgb-led). It supports on/off, brightness, transitions, RGB colors, and flashing.
///
/// - There is also another implementation forked from the above repository, it supports all the same features but is made for addressable LED strips using FastLED on a NodeMCU V3 it can be found [here](https://github.com/JammyDodger231/nodemcu-mqtt-rgb-led).
///
/// - [McLighting](https://github.com/toblum/McLighting) is another ESP8266 firmware for WS2812 addressable LEDs.
///
/// - [MQTT JSON Light](https://github.com/mertenats/Open-Home-Automation/tree/master/ha_mqtt_rgbw_light_with_discovery) is another implementation for ESP8266 including [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
///
/// - [ESPHome](https://esphome.io) implements the JSON schema for MQTT based installs and supports [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
///
/// - [AiLight](https://github.com/stelgenhof/AiLight) is a custom firmware for the Ai-Thinker (and equivalent) RGBW WiFi light bulbs that has an ESP8266 onboard and controlled by the MY9291 LED driver. It implements the [MQTT JSON light](/integrations/light.mqtt) platform and supports ON/OFF, RGBW colours, brightness, color temperature, flashing and transitions. Also it includes [MQTT Auto Discovery](/integrations/mqtt/#mqtt-discovery)) and the MQTT Last Will and Testament is enabled as well.
///
/// - [h801-mqtt-json](https://github.com/starkillerOG/h801-mqtt-json) is a custom firmware for the H801 LED dimmer, a 5 channel (RGBWWCW)  WiFi LED strip controller for 12V LED strips. The firmware is meant to control the 5 channels of the H801 to simultaneously control an RGB and a Warm-white/Cold-white LED strip such as a 5050 RGB LED strip and a 5025 Dual White strip. It implements the [MQTT JSON light](/integrations/light.mqtt) platform and supports ON/OFF, RGBW colours (RGB strip), brightness, color temperature (CW/WW strip) and transitions.
///
/// ## Template schema
///
/// The `mqtt` light platform with template schema lets you control a MQTT-enabled light that receive commands on a command topic and optionally sends status update on a state topic.
/// It is format-agnostic so you can use any data format you want (i.e., string, JSON), just configure it with templating.
///
/// This schema supports on/off, brightness, RGB colors, XY colors, HS Color, color temperature, transitions, short/long flashing and effects.
///
/// ## Template schema - Configuration
///
/// In an ideal scenario, the MQTT device will have a state topic to publish state changes. If these messages are published with the RETAIN flag, the MQTT light will receive an instant state update after subscription and will start with the correct state. Otherwise, the initial state of the light will be off.
///
/// When a state topic is not available, the light will work in optimistic mode. In this mode, the light will immediately change state after every command. Otherwise, the light will wait for state confirmation from the device (message from `state_topic`).
///
/// Optimistic mode can be forced, even if state topic is available. Try enabling it if the light is operating incorrectly.
///
/// To use an MQTT template light in your installation, [add a MQTT device as a subentry](/integrations/mqtt/#configuration), or add the following to your `configuration.yaml` file.
/// {% include integrations/restart_ha_after_config_inclusion.md %}
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - light:
///       schema: template
///       command_topic: "home/rgb1/set"
///       command_on_template: "on"
///       command_off_template: "off"
/// ```
///
/// Alternatively, a more advanced approach is to set it up via [MQTT discovery](/integrations/mqtt/#mqtt-discovery).
///
///
/// ⚠ Important\
/// Make sure that your topics match exact. `some-topic/` and `some-topic` are different topics.
///
/// ## Template schema - Examples
///
/// In this section you find some real-life examples of how to use this light.
///
/// ### Simple string payload
///
/// For a simple string payload with the format `state,brightness,r-g-b,h-s` (e.g., `on,255,255-255-255,360-100`), add the following to your `configuration.yaml` file:
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - light:
///       schema: template
///       command_topic: "home/rgb1/set"
///       state_topic: "home/rgb1/status"
///       command_on_template: "on,{{ brightness|d }},{{ red|d }}-{{ green|d }}-{{ blue|d }},{{ hue|d }}-{{ sat|d }}"
///       command_off_template: "off"
///       state_template: "{{ value.split(',')[0] }}"  # must return `on` or `off`
///       brightness_template: "{{ value.split(',')[1] }}"
///       red_template: "{{ value.split(',')[2].split('-')[0] }}"
///       green_template: "{{ value.split(',')[2].split('-')[1] }}"
///       blue_template: "{{ value.split(',')[2].split('-')[2] }}"
/// ```
///
///
/// ### JSON payload
///
/// For a JSON payload with the format `{"state": "on", "brightness": 255, "color": [255, 255, 255], "effect": "rainbow"}`, add the following to your `configuration.yaml` file:
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - light:
///       schema: template
///       effect_list:
///         - rainbow
///         - colorloop
///       command_topic: "home/rgb1/set"
///       state_topic: "home/rgb1/status"
///       command_on_template: >
///         {"state": "on"
///         {%- if brightness is defined -%}
///         , "brightness": {{ brightness }}
///         {%- endif -%}
///         {%- if red is defined and green is defined and blue is defined -%}
///         , "color": [{{ red }}, {{ green }}, {{ blue }}]
///         {%- endif -%}
///         {%- if hue is defined and sat is defined -%}
///         , "huesat": [{{ hue }}, {{ sat }}]
///         {%- endif -%}
///         {%- if effect is defined -%}
///         , "effect": "{{ effect }}"
///         {%- endif -%}
///         }
///       command_off_template: '{"state": "off"}'
///       state_template: '{{ value_json.state }}'
///       brightness_template: '{{ value_json.brightness }}'
///       red_template: '{{ value_json.color[0] }}'
///       green_template: '{{ value_json.color[1] }}'
///       blue_template: '{{ value_json.color[2] }}'
///       effect_template: '{{ value_json.effect }}'
/// ```
///
///
/// ### CCT light (brightness and temperature)
///
/// This example comes from a configuration of Shelly RGBW Bulb working in White mode.
/// `max_mireds` and `min_mireds` set color temperature boundaries to 3000K - 6500K. Notice the same limits are applied in `command_on_template`, but in kelvin units this time. It's due to conversion from mired to kelvin which causes exceeding boundary values accepted by the device.
/// The code also ensures bi-directional conversion of brightness scale between 0-100 (required by the device) and 0-255 (required by Home Assistant).
/// Add the following to your `configuration.yaml` file:
///
///
/// ```yaml
/// # Example configuration.yaml entry
/// mqtt:
///   - light:
///       schema: template
///       name: "Bulb-white"
///       command_topic: "shellies/bulb/color/0/set"
///       state_topic: "shellies/bulb/color/0/status"
///       availability_topic: "shellies/bulb/online"
///       command_on_template: >
///         {"turn": "on", "mode": "white"
///         {%- if brightness is defined -%}
///         , "brightness": {{brightness | float | multiply(0.39215686) | round(0)}}
///         {%- endif -%}
///         {%- if color_temp is defined -%}
///         , "temp": {{ [[(1000000 / color_temp | float) | round(0), 3000] | max, 6500] | min }}
///         {%- endif -%}
///         }
///       command_off_template: '{"turn":"off", "mode": "white"}'
///       state_template: "{% if value_json.ison and value_json.mode == 'white' %}on{% else %}off{% endif %}"
///       brightness_template: "{{ value_json.brightness | float | multiply(2.55) | round(0) }}"
///       color_temp_template: "{{ (1000000 / value_json.temp | float) | round(0) }}"
///       payload_available: "true"
///       payload_not_available: "false"
///       max_mireds: 334
///       min_mireds: 153
///       qos: 1
///       retain: false
///       optimistic: false  
/// ```
///
///
/// ### Template schema - No brightness or color support
///
/// If you don't want brightness, color or effect support, just omit the corresponding configuration sections.
///
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Light {
    /// Replaces `~` with this value in any MQTT topic attribute.
    /// [See Home Assistant documentation](https://www.home-assistant.io/integrations/mqtt/#using-abbreviations-and-base-topic)
    #[serde(rename = "~", skip_serializing_if = "Option::is_none")]
    pub topic_prefix: Option<String>,

    /// It is encouraged to add additional information about the origin that supplies MQTT entities via MQTT discovery by adding the origin option (can be abbreviated to o) to the discovery payload. Note that these options also support abbreviations. Information of the origin will be logged to the core event log when an item is discovered or updated.
    #[serde(rename = "o")]
    pub origin: Origin,

    /// Information about the device this button is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/en/device_registry_index.html). Only works when [`unique_id`](#unique_id) is set. At least one of identifiers or connections must be present to identify the device.
    #[serde(rename = "dev")]
    pub device: Device,

    /// A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`.
    #[serde(flatten)]
    pub availability: Availability,

    /// The category of the entity. (optional, default: None)
    #[serde(rename = "ent_cat", skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<EntityCategory>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to compose message which will be sent to `brightness_command_topic`. Available variables: `value`.
    #[serde(rename = "bri_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub brightness_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the light’s brightness.
    #[serde(rename = "bri_cmd_t", skip_serializing_if = "Option::is_none")]
    pub brightness_command_topic: Option<String>,

    /// Defines the maximum brightness value (i.e., 100%) of the MQTT device.
    #[serde(rename = "bri_scl", skip_serializing_if = "Option::is_none")]
    pub brightness_scale: Option<i32>,

    /// The MQTT topic subscribed to receive brightness state updates.
    #[serde(rename = "bri_stat_t", skip_serializing_if = "Option::is_none")]
    pub brightness_state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the brightness value.
    #[serde(rename = "bri_val_tpl", skip_serializing_if = "Option::is_none")]
    pub brightness_value_template: Option<String>,

    /// The MQTT topic subscribed to receive color mode updates. If this is not configured, `color_mode` will be automatically set according to the last received valid color or color temperature. The unit used is mireds, or if `color_temp_kelvin` is set to `true`, in Kelvin.
    #[serde(rename = "clrm_stat_t", skip_serializing_if = "Option::is_none")]
    pub color_mode_state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the color mode.
    #[serde(rename = "clrm_val_tpl", skip_serializing_if = "Option::is_none")]
    pub color_mode_value_template: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to compose message which will be sent to `color_temp_command_topic`. Available variables: `value`.
    #[serde(rename = "clr_temp_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub color_temp_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the light’s color temperature state. By default the color temperature command slider has a range of 153 to 500 mireds (micro reciprocal degrees) or a range of 2000 to 6535 Kelvin if `color_temp_kelvin` is set to `true`.
    #[serde(rename = "clr_temp_cmd_t", skip_serializing_if = "Option::is_none")]
    pub color_temp_command_topic: Option<String>,

    /// When set to `true`, `color_temp_command_topic` will publish color mode updates in Kelvin and process `color_temp_state_topic` will process state updates in Kelvin. When not set the `color_temp` values are converted to mireds.
    #[serde(rename = "color_temp_kelvin", skip_serializing_if = "Option::is_none")]
    pub color_temp_kelvin: Option<bool>,

    /// The MQTT topic subscribed to receive color temperature state updates.
    #[serde(rename = "clr_temp_stat_t", skip_serializing_if = "Option::is_none")]
    pub color_temp_state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the color temperature value.
    #[serde(rename = "clr_temp_val_tpl", skip_serializing_if = "Option::is_none")]
    pub color_temp_value_template: Option<String>,

    /// The MQTT topic to publish commands to change the switch state.
    #[serde(rename = "cmd_t")]
    pub command_topic: String,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to compose message which will be sent to `effect_command_topic`. Available variables: `value`.
    #[serde(rename = "fx_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub effect_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the light's effect state.
    #[serde(rename = "fx_cmd_t", skip_serializing_if = "Option::is_none")]
    pub effect_command_topic: Option<String>,

    /// The list of effects the light supports.
    #[serde(rename = "fx_list", skip_serializing_if = "Option::is_none")]
    pub effect_list: Option<Vec<String>>,

    /// The MQTT topic subscribed to receive effect state updates.
    #[serde(rename = "fx_stat_t", skip_serializing_if = "Option::is_none")]
    pub effect_state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the effect value.
    #[serde(rename = "fx_val_tpl", skip_serializing_if = "Option::is_none")]
    pub effect_value_template: Option<String>,

    /// Flag which defines if the entity should be enabled when first added.
    #[serde(rename = "en", skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,

    /// The encoding of the payloads received and published messages. Set to `""` to disable decoding of incoming payload.
    #[serde(rename = "e", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,

    /// Picture URL for the entity.
    #[serde(rename = "ent_pic", skip_serializing_if = "Option::is_none")]
    pub entity_picture: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to compose message which will be sent to `hs_command_topic`. Available variables: `hue` and `sat`.
    #[serde(rename = "hs_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub hs_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the light's color state in HS format (Hue Saturation). Range for Hue: 0° .. 360°, Range of Saturation: 0..100. Note: Brightness is sent separately in the `brightness_command_topic`.
    #[serde(rename = "hs_cmd_t", skip_serializing_if = "Option::is_none")]
    pub hs_command_topic: Option<String>,

    /// The MQTT topic subscribed to receive color state updates in HS format. The expected payload is the hue and saturation values separated by commas, for example, `359.5,100.0`. Note: Brightness is received separately in the `brightness_state_topic`.
    #[serde(rename = "hs_stat_t", skip_serializing_if = "Option::is_none")]
    pub hs_state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the HS value.
    #[serde(rename = "hs_val_tpl", skip_serializing_if = "Option::is_none")]
    pub hs_value_template: Option<String>,

    /// [Icon](/docs/configuration/customizing-devices/#icon) for the entity.
    #[serde(rename = "ic", skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation.
    #[serde(rename = "json_attr_tpl", skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation.
    #[serde(rename = "json_attr_t", skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,

    /// The maximum color temperature in Kelvin.
    #[serde(rename = "max_kelvin", skip_serializing_if = "Option::is_none")]
    pub max_kelvin: Option<i32>,

    /// The maximum color temperature in mireds.
    #[serde(rename = "max_mirs", skip_serializing_if = "Option::is_none")]
    pub max_mireds: Option<i32>,

    /// The minimum color temperature in Kelvin.
    #[serde(rename = "min_kelvin", skip_serializing_if = "Option::is_none")]
    pub min_kelvin: Option<i32>,

    /// The minimum color temperature in mireds.
    #[serde(rename = "min_mirs", skip_serializing_if = "Option::is_none")]
    pub min_mireds: Option<i32>,

    /// The name of the light. Can be set to `null` if only the device name is relevant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    #[serde(rename = "obj_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,

    /// Defines when on the payload_on is sent. Using `last` (the default) will send any style (brightness, color, etc) topics first and then a `payload_on` to the `command_topic`. Using `first` will send the `payload_on` and then any style topics. Using `brightness` will only send brightness commands instead of the `payload_on` to turn the light on.
    #[serde(rename = "on_cmd_type", skip_serializing_if = "Option::is_none")]
    pub on_command_type: Option<String>,

    /// Flag that defines if switch works in optimistic mode.
    #[serde(rename = "opt", skip_serializing_if = "Option::is_none")]
    pub optimistic: Option<bool>,

    /// The payload that represents the off state.
    #[serde(rename = "pl_off", skip_serializing_if = "Option::is_none")]
    pub payload_off: Option<String>,

    /// The payload that represents the on state.
    #[serde(rename = "pl_on", skip_serializing_if = "Option::is_none")]
    pub payload_on: Option<String>,

    /// Must be `light`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    #[serde(rename = "platform")]
    pub platform: String,

    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,

    /// If the published message should have the retain flag on or not.
    #[serde(rename = "ret", skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to compose message which will be sent to `rgb_command_topic`. Available variables: `red`, `green` and `blue`.
    #[serde(rename = "rgb_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub rgb_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the light's RGB state.
    #[serde(rename = "rgb_cmd_t", skip_serializing_if = "Option::is_none")]
    pub rgb_command_topic: Option<String>,

    /// The MQTT topic subscribed to receive RGB state updates. The expected payload is the RGB values separated by commas, for example, `255,0,127`.
    #[serde(rename = "rgb_stat_t", skip_serializing_if = "Option::is_none")]
    pub rgb_state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the RGB value.
    #[serde(rename = "rgb_val_tpl", skip_serializing_if = "Option::is_none")]
    pub rgb_value_template: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to compose message which will be sent to `rgbw_command_topic`. Available variables: `red`, `green`, `blue` and `white`.
    #[serde(rename = "rgbw_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub rgbw_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the light's RGBW state.
    #[serde(rename = "rgbw_cmd_t", skip_serializing_if = "Option::is_none")]
    pub rgbw_command_topic: Option<String>,

    /// The MQTT topic subscribed to receive RGBW state updates. The expected payload is the RGBW values separated by commas, for example, `255,0,127,64`.
    #[serde(rename = "rgbw_stat_t", skip_serializing_if = "Option::is_none")]
    pub rgbw_state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the RGBW value.
    #[serde(rename = "rgbw_val_tpl", skip_serializing_if = "Option::is_none")]
    pub rgbw_value_template: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to compose message which will be sent to `rgbww_command_topic`. Available variables: `red`, `green`, `blue`, `cold_white` and `warm_white`.
    #[serde(rename = "rgbww_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub rgbww_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the light's RGBWW state.
    #[serde(rename = "rgbww_cmd_t", skip_serializing_if = "Option::is_none")]
    pub rgbww_command_topic: Option<String>,

    /// The MQTT topic subscribed to receive RGBWW state updates. The expected payload is the RGBWW values separated by commas, for example, `255,0,127,64,32`.
    #[serde(rename = "rgbww_stat_t", skip_serializing_if = "Option::is_none")]
    pub rgbww_state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the RGBWW value.
    #[serde(rename = "rgbww_val_tpl", skip_serializing_if = "Option::is_none")]
    pub rgbww_value_template: Option<String>,

    /// The schema to use. Must be `basic` or omitted to select the default schema.
    #[serde(rename = "schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,

    /// The MQTT topic subscribed to receive state updates. A "None" payload resets to an `unknown` state. An empty payload is ignored. By default, valid state payloads are `OFF` and `ON`. The accepted payloads can be overridden with the `payload_off` and `payload_on` config options.
    #[serde(rename = "stat_t", skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the state value. The template should return the values defined by `payload_on` (defaults to "ON") and `payload_off` (defaults to "OFF") settings, or "None".
    #[serde(rename = "stat_val_tpl", skip_serializing_if = "Option::is_none")]
    pub state_value_template: Option<String>,

    /// An ID that uniquely identifies this light. If two lights have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    #[serde(rename = "uniq_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,

    /// The MQTT topic to publish commands to change the light to white mode with a given brightness.
    #[serde(rename = "whit_cmd_t", skip_serializing_if = "Option::is_none")]
    pub white_command_topic: Option<String>,

    /// Defines the maximum white level (i.e., 100%) of the MQTT device.
    #[serde(rename = "whit_scl", skip_serializing_if = "Option::is_none")]
    pub white_scale: Option<i32>,

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to compose message which will be sent to `xy_command_topic`. Available variables: `x` and `y`.
    #[serde(rename = "xy_cmd_tpl", skip_serializing_if = "Option::is_none")]
    pub xy_command_template: Option<String>,

    /// The MQTT topic to publish commands to change the light's XY state.
    #[serde(rename = "xy_cmd_t", skip_serializing_if = "Option::is_none")]
    pub xy_command_topic: Option<String>,

    /// The MQTT topic subscribed to receive XY state updates. The expected payload is the X and Y color values separated by commas, for example, `0.675,0.322`.
    #[serde(rename = "xy_stat_t", skip_serializing_if = "Option::is_none")]
    pub xy_state_topic: Option<String>,

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the XY value.
    #[serde(rename = "xy_val_tpl", skip_serializing_if = "Option::is_none")]
    pub xy_value_template: Option<String>,
}

impl Light {
    /// Replaces `~` with this value in any MQTT topic attribute.
    /// [See Home Assistant documentation](https://www.home-assistant.io/integrations/mqtt/#using-abbreviations-and-base-topic)
    pub fn topic_prefix<S: Into<String>>(mut self, topic_prefix: S) -> Self {
        self.topic_prefix = Some(topic_prefix.into());
        self
    }

    /// It is encouraged to add additional information about the origin that supplies MQTT entities via MQTT discovery by adding the origin option (can be abbreviated to o) to the discovery payload. Note that these options also support abbreviations. Information of the origin will be logged to the core event log when an item is discovered or updated.
    pub fn origin(mut self, origin: Origin) -> Self {
        self.origin = origin;
        self
    }

    /// Information about the device this sensor is a part of to tie it into the [device registry](https://developers.home-assistant.io/docs/device_registry_index/). Only works when `unique_id` is set. At least one of identifiers or connections must be present to identify the device.
    pub fn device(mut self, device: Device) -> Self {
        self.device = device;
        self
    }

    /// The category of the entity. (optional, default: None)
    pub fn entity_category(mut self, entity_category: EntityCategory) -> Self {
        self.entity_category = Some(entity_category);
        self
    }

    /// Defines how HA will check for entity availability.
    pub fn availability(mut self, availability: Availability) -> Self {
        self.availability = availability;
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to compose message which will be sent to `brightness_command_topic`. Available variables: `value`.
    pub fn brightness_command_template<T: Into<String>>(
        mut self,
        brightness_command_template: T,
    ) -> Self {
        self.brightness_command_template = Some(brightness_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the light’s brightness.
    pub fn brightness_command_topic<T: Into<String>>(
        mut self,
        brightness_command_topic: T,
    ) -> Self {
        self.brightness_command_topic = Some(brightness_command_topic.into());
        self
    }

    /// Defines the maximum brightness value (i.e., 100%) of the MQTT device.
    pub fn brightness_scale(mut self, brightness_scale: i32) -> Self {
        self.brightness_scale = Some(brightness_scale);
        self
    }

    /// The MQTT topic subscribed to receive brightness state updates.
    pub fn brightness_state_topic<T: Into<String>>(mut self, brightness_state_topic: T) -> Self {
        self.brightness_state_topic = Some(brightness_state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the brightness value.
    pub fn brightness_value_template<T: Into<String>>(
        mut self,
        brightness_value_template: T,
    ) -> Self {
        self.brightness_value_template = Some(brightness_value_template.into());
        self
    }

    /// The MQTT topic subscribed to receive color mode updates. If this is not configured, `color_mode` will be automatically set according to the last received valid color or color temperature. The unit used is mireds, or if `color_temp_kelvin` is set to `true`, in Kelvin.
    pub fn color_mode_state_topic<T: Into<String>>(mut self, color_mode_state_topic: T) -> Self {
        self.color_mode_state_topic = Some(color_mode_state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the color mode.
    pub fn color_mode_value_template<T: Into<String>>(
        mut self,
        color_mode_value_template: T,
    ) -> Self {
        self.color_mode_value_template = Some(color_mode_value_template.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to compose message which will be sent to `color_temp_command_topic`. Available variables: `value`.
    pub fn color_temp_command_template<T: Into<String>>(
        mut self,
        color_temp_command_template: T,
    ) -> Self {
        self.color_temp_command_template = Some(color_temp_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the light’s color temperature state. By default the color temperature command slider has a range of 153 to 500 mireds (micro reciprocal degrees) or a range of 2000 to 6535 Kelvin if `color_temp_kelvin` is set to `true`.
    pub fn color_temp_command_topic<T: Into<String>>(
        mut self,
        color_temp_command_topic: T,
    ) -> Self {
        self.color_temp_command_topic = Some(color_temp_command_topic.into());
        self
    }

    /// When set to `true`, `color_temp_command_topic` will publish color mode updates in Kelvin and process `color_temp_state_topic` will process state updates in Kelvin. When not set the `color_temp` values are converted to mireds.
    pub fn color_temp_kelvin(mut self, color_temp_kelvin: bool) -> Self {
        self.color_temp_kelvin = Some(color_temp_kelvin);
        self
    }

    /// The MQTT topic subscribed to receive color temperature state updates.
    pub fn color_temp_state_topic<T: Into<String>>(mut self, color_temp_state_topic: T) -> Self {
        self.color_temp_state_topic = Some(color_temp_state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the color temperature value.
    pub fn color_temp_value_template<T: Into<String>>(
        mut self,
        color_temp_value_template: T,
    ) -> Self {
        self.color_temp_value_template = Some(color_temp_value_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the switch state.
    pub fn command_topic<T: Into<String>>(mut self, command_topic: T) -> Self {
        self.command_topic = command_topic.into();
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to compose message which will be sent to `effect_command_topic`. Available variables: `value`.
    pub fn effect_command_template<T: Into<String>>(mut self, effect_command_template: T) -> Self {
        self.effect_command_template = Some(effect_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the light's effect state.
    pub fn effect_command_topic<T: Into<String>>(mut self, effect_command_topic: T) -> Self {
        self.effect_command_topic = Some(effect_command_topic.into());
        self
    }

    /// The list of effects the light supports.
    pub fn effect_list<T: Into<String>>(mut self, effect_list: Vec<T>) -> Self {
        self.effect_list = Some(effect_list.into_iter().map(|v| v.into()).collect());
        self
    }

    /// The MQTT topic subscribed to receive effect state updates.
    pub fn effect_state_topic<T: Into<String>>(mut self, effect_state_topic: T) -> Self {
        self.effect_state_topic = Some(effect_state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the effect value.
    pub fn effect_value_template<T: Into<String>>(mut self, effect_value_template: T) -> Self {
        self.effect_value_template = Some(effect_value_template.into());
        self
    }

    /// Flag which defines if the entity should be enabled when first added.
    pub fn enabled_by_default(mut self, enabled_by_default: bool) -> Self {
        self.enabled_by_default = Some(enabled_by_default);
        self
    }

    /// The encoding of the payloads received and published messages. Set to `""` to disable decoding of incoming payload.
    pub fn encoding<T: Into<String>>(mut self, encoding: T) -> Self {
        self.encoding = Some(encoding.into());
        self
    }

    /// Picture URL for the entity.
    pub fn entity_picture<T: Into<String>>(mut self, entity_picture: T) -> Self {
        self.entity_picture = Some(entity_picture.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to compose message which will be sent to `hs_command_topic`. Available variables: `hue` and `sat`.
    pub fn hs_command_template<T: Into<String>>(mut self, hs_command_template: T) -> Self {
        self.hs_command_template = Some(hs_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the light's color state in HS format (Hue Saturation). Range for Hue: 0° .. 360°, Range of Saturation: 0..100. Note: Brightness is sent separately in the `brightness_command_topic`.
    pub fn hs_command_topic<T: Into<String>>(mut self, hs_command_topic: T) -> Self {
        self.hs_command_topic = Some(hs_command_topic.into());
        self
    }

    /// The MQTT topic subscribed to receive color state updates in HS format. The expected payload is the hue and saturation values separated by commas, for example, `359.5,100.0`. Note: Brightness is received separately in the `brightness_state_topic`.
    pub fn hs_state_topic<T: Into<String>>(mut self, hs_state_topic: T) -> Self {
        self.hs_state_topic = Some(hs_state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the HS value.
    pub fn hs_value_template<T: Into<String>>(mut self, hs_value_template: T) -> Self {
        self.hs_value_template = Some(hs_value_template.into());
        self
    }

    /// [Icon](/docs/configuration/customizing-devices/#icon) for the entity.
    pub fn icon<T: Into<String>>(mut self, icon: T) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation.
    pub fn json_attributes_template<T: Into<String>>(
        mut self,
        json_attributes_template: T,
    ) -> Self {
        self.json_attributes_template = Some(json_attributes_template.into());
        self
    }

    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation.
    pub fn json_attributes_topic<T: Into<String>>(mut self, json_attributes_topic: T) -> Self {
        self.json_attributes_topic = Some(json_attributes_topic.into());
        self
    }

    /// The maximum color temperature in Kelvin.
    pub fn max_kelvin(mut self, max_kelvin: i32) -> Self {
        self.max_kelvin = Some(max_kelvin);
        self
    }

    /// The maximum color temperature in mireds.
    pub fn max_mireds(mut self, max_mireds: i32) -> Self {
        self.max_mireds = Some(max_mireds);
        self
    }

    /// The minimum color temperature in Kelvin.
    pub fn min_kelvin(mut self, min_kelvin: i32) -> Self {
        self.min_kelvin = Some(min_kelvin);
        self
    }

    /// The minimum color temperature in mireds.
    pub fn min_mireds(mut self, min_mireds: i32) -> Self {
        self.min_mireds = Some(min_mireds);
        self
    }

    /// The name of the light. Can be set to `null` if only the device name is relevant.
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Used `object_id` instead of `name` for automatic generation of `entity_id`. This only works when the entity is added for the first time. When set, this overrides a user-customized Entity ID in case the entity was deleted and added again.
    pub fn object_id<T: Into<String>>(mut self, object_id: T) -> Self {
        self.object_id = Some(object_id.into());
        self
    }

    /// Defines when on the payload_on is sent. Using `last` (the default) will send any style (brightness, color, etc) topics first and then a `payload_on` to the `command_topic`. Using `first` will send the `payload_on` and then any style topics. Using `brightness` will only send brightness commands instead of the `payload_on` to turn the light on.
    pub fn on_command_type<T: Into<String>>(mut self, on_command_type: T) -> Self {
        self.on_command_type = Some(on_command_type.into());
        self
    }

    /// Flag that defines if switch works in optimistic mode.
    pub fn optimistic(mut self, optimistic: bool) -> Self {
        self.optimistic = Some(optimistic);
        self
    }

    /// The payload that represents the off state.
    pub fn payload_off<T: Into<String>>(mut self, payload_off: T) -> Self {
        self.payload_off = Some(payload_off.into());
        self
    }

    /// The payload that represents the on state.
    pub fn payload_on<T: Into<String>>(mut self, payload_on: T) -> Self {
        self.payload_on = Some(payload_on.into());
        self
    }

    /// Must be `light`. Only allowed and required in [MQTT auto discovery device messages](/integrations/mqtt/#device-discovery-payload).
    pub fn platform<T: Into<String>>(mut self, platform: T) -> Self {
        self.platform = platform.into();
        self
    }

    /// The maximum QoS level to be used when receiving and publishing messages.
    pub fn qos(mut self, qos: Qos) -> Self {
        self.qos = Some(qos);
        self
    }

    /// If the published message should have the retain flag on or not.
    pub fn retain(mut self, retain: bool) -> Self {
        self.retain = Some(retain);
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to compose message which will be sent to `rgb_command_topic`. Available variables: `red`, `green` and `blue`.
    pub fn rgb_command_template<T: Into<String>>(mut self, rgb_command_template: T) -> Self {
        self.rgb_command_template = Some(rgb_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the light's RGB state.
    pub fn rgb_command_topic<T: Into<String>>(mut self, rgb_command_topic: T) -> Self {
        self.rgb_command_topic = Some(rgb_command_topic.into());
        self
    }

    /// The MQTT topic subscribed to receive RGB state updates. The expected payload is the RGB values separated by commas, for example, `255,0,127`.
    pub fn rgb_state_topic<T: Into<String>>(mut self, rgb_state_topic: T) -> Self {
        self.rgb_state_topic = Some(rgb_state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the RGB value.
    pub fn rgb_value_template<T: Into<String>>(mut self, rgb_value_template: T) -> Self {
        self.rgb_value_template = Some(rgb_value_template.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to compose message which will be sent to `rgbw_command_topic`. Available variables: `red`, `green`, `blue` and `white`.
    pub fn rgbw_command_template<T: Into<String>>(mut self, rgbw_command_template: T) -> Self {
        self.rgbw_command_template = Some(rgbw_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the light's RGBW state.
    pub fn rgbw_command_topic<T: Into<String>>(mut self, rgbw_command_topic: T) -> Self {
        self.rgbw_command_topic = Some(rgbw_command_topic.into());
        self
    }

    /// The MQTT topic subscribed to receive RGBW state updates. The expected payload is the RGBW values separated by commas, for example, `255,0,127,64`.
    pub fn rgbw_state_topic<T: Into<String>>(mut self, rgbw_state_topic: T) -> Self {
        self.rgbw_state_topic = Some(rgbw_state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the RGBW value.
    pub fn rgbw_value_template<T: Into<String>>(mut self, rgbw_value_template: T) -> Self {
        self.rgbw_value_template = Some(rgbw_value_template.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to compose message which will be sent to `rgbww_command_topic`. Available variables: `red`, `green`, `blue`, `cold_white` and `warm_white`.
    pub fn rgbww_command_template<T: Into<String>>(mut self, rgbww_command_template: T) -> Self {
        self.rgbww_command_template = Some(rgbww_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the light's RGBWW state.
    pub fn rgbww_command_topic<T: Into<String>>(mut self, rgbww_command_topic: T) -> Self {
        self.rgbww_command_topic = Some(rgbww_command_topic.into());
        self
    }

    /// The MQTT topic subscribed to receive RGBWW state updates. The expected payload is the RGBWW values separated by commas, for example, `255,0,127,64,32`.
    pub fn rgbww_state_topic<T: Into<String>>(mut self, rgbww_state_topic: T) -> Self {
        self.rgbww_state_topic = Some(rgbww_state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the RGBWW value.
    pub fn rgbww_value_template<T: Into<String>>(mut self, rgbww_value_template: T) -> Self {
        self.rgbww_value_template = Some(rgbww_value_template.into());
        self
    }

    /// The schema to use. Must be `basic` or omitted to select the default schema.
    pub fn schema<T: Into<String>>(mut self, schema: T) -> Self {
        self.schema = Some(schema.into());
        self
    }

    /// The MQTT topic subscribed to receive state updates. A "None" payload resets to an `unknown` state. An empty payload is ignored. By default, valid state payloads are `OFF` and `ON`. The accepted payloads can be overridden with the `payload_off` and `payload_on` config options.
    pub fn state_topic<T: Into<String>>(mut self, state_topic: T) -> Self {
        self.state_topic = Some(state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the state value. The template should return the values defined by `payload_on` (defaults to "ON") and `payload_off` (defaults to "OFF") settings, or "None".
    pub fn state_value_template<T: Into<String>>(mut self, state_value_template: T) -> Self {
        self.state_value_template = Some(state_value_template.into());
        self
    }

    /// An ID that uniquely identifies this light. If two lights have the same unique ID, Home Assistant will raise an exception. Required when used with device-based discovery.
    pub fn unique_id<T: Into<String>>(mut self, unique_id: T) -> Self {
        self.unique_id = Some(unique_id.into());
        self
    }

    /// The MQTT topic to publish commands to change the light to white mode with a given brightness.
    pub fn white_command_topic<T: Into<String>>(mut self, white_command_topic: T) -> Self {
        self.white_command_topic = Some(white_command_topic.into());
        self
    }

    /// Defines the maximum white level (i.e., 100%) of the MQTT device.
    pub fn white_scale(mut self, white_scale: i32) -> Self {
        self.white_scale = Some(white_scale);
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-command-templates-with-mqtt) to compose message which will be sent to `xy_command_topic`. Available variables: `x` and `y`.
    pub fn xy_command_template<T: Into<String>>(mut self, xy_command_template: T) -> Self {
        self.xy_command_template = Some(xy_command_template.into());
        self
    }

    /// The MQTT topic to publish commands to change the light's XY state.
    pub fn xy_command_topic<T: Into<String>>(mut self, xy_command_topic: T) -> Self {
        self.xy_command_topic = Some(xy_command_topic.into());
        self
    }

    /// The MQTT topic subscribed to receive XY state updates. The expected payload is the X and Y color values separated by commas, for example, `0.675,0.322`.
    pub fn xy_state_topic<T: Into<String>>(mut self, xy_state_topic: T) -> Self {
        self.xy_state_topic = Some(xy_state_topic.into());
        self
    }

    /// Defines a [template](/docs/configuration/templating/#using-value-templates-with-mqtt) to extract the XY value.
    pub fn xy_value_template<T: Into<String>>(mut self, xy_value_template: T) -> Self {
        self.xy_value_template = Some(xy_value_template.into());
        self
    }
}

impl Default for Light {
    fn default() -> Self {
        Self {
            topic_prefix: Default::default(),
            origin: Default::default(),
            device: Default::default(),
            entity_category: Default::default(),
            availability: Default::default(),
            brightness_command_template: Default::default(),
            brightness_command_topic: Default::default(),
            brightness_scale: Default::default(),
            brightness_state_topic: Default::default(),
            brightness_value_template: Default::default(),
            color_mode_state_topic: Default::default(),
            color_mode_value_template: Default::default(),
            color_temp_command_template: Default::default(),
            color_temp_command_topic: Default::default(),
            color_temp_kelvin: Default::default(),
            color_temp_state_topic: Default::default(),
            color_temp_value_template: Default::default(),
            command_topic: Default::default(),
            effect_command_template: Default::default(),
            effect_command_topic: Default::default(),
            effect_list: Default::default(),
            effect_state_topic: Default::default(),
            effect_value_template: Default::default(),
            enabled_by_default: Default::default(),
            encoding: Default::default(),
            entity_picture: Default::default(),
            hs_command_template: Default::default(),
            hs_command_topic: Default::default(),
            hs_state_topic: Default::default(),
            hs_value_template: Default::default(),
            icon: Default::default(),
            json_attributes_template: Default::default(),
            json_attributes_topic: Default::default(),
            max_kelvin: Default::default(),
            max_mireds: Default::default(),
            min_kelvin: Default::default(),
            min_mireds: Default::default(),
            name: Default::default(),
            object_id: Default::default(),
            on_command_type: Default::default(),
            optimistic: Default::default(),
            payload_off: Default::default(),
            payload_on: Default::default(),
            platform: "light".to_string(),
            qos: Default::default(),
            retain: Default::default(),
            rgb_command_template: Default::default(),
            rgb_command_topic: Default::default(),
            rgb_state_topic: Default::default(),
            rgb_value_template: Default::default(),
            rgbw_command_template: Default::default(),
            rgbw_command_topic: Default::default(),
            rgbw_state_topic: Default::default(),
            rgbw_value_template: Default::default(),
            rgbww_command_template: Default::default(),
            rgbww_command_topic: Default::default(),
            rgbww_state_topic: Default::default(),
            rgbww_value_template: Default::default(),
            schema: Default::default(),
            state_topic: Default::default(),
            state_value_template: Default::default(),
            unique_id: Default::default(),
            white_command_topic: Default::default(),
            white_scale: Default::default(),
            xy_command_template: Default::default(),
            xy_command_topic: Default::default(),
            xy_state_topic: Default::default(),
            xy_value_template: Default::default(),
        }
    }
}

impl From<Light> for Entity {
    fn from(value: Light) -> Self {
        Entity::Light(value)
    }
}
