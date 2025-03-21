---
title: "MQTT Sensor"
description: "Instructions on how to integrate MQTT sensors within Home Assistant."
ha_category:
  - Sensor
ha_release: 0.7
ha_iot_class: Configurable
ha_domain: mqtt
---

This `mqtt` sensor platform uses the MQTT message payload as the sensor value. If messages in this `state_topic` are published with *RETAIN* flag, the sensor will receive an instant update with last known value. Otherwise, the initial state will be undefined.

## Configuration

To use your MQTT sensor in your installation, add the following to your {% term "`configuration.yaml`" %} file:

```yaml
# Example configuration.yaml entry
mqtt:
  sensor:
    - name: "Bedroom Temperature"
      state_topic: "home/bedroom/temperature"
```


## Examples

In this section, you find some real-life examples showing how to use this sensor.

### Processing Unix EPOCH timestamps

The example below shows how an MQTT sensor can process a Unix EPOCH payload.


Set up via YAML:

```yaml
# Example configuration.yaml entry
mqtt:
  sensor:
    - name: "turned on"
      state_topic: "pump/timestamp_on"
      device_class: "timestamp"
      value_template: "{{ as_datetime(value) }}"
      unique_id: "hp_1231232_ts_on"
      device:
        name: "Heat pump"
        identifiers:
          - "hp_1231232"
```


Or set up via MQTT discovery:

Discovery topic: `homeassistant/sensor/hp_1231232/config`


```json
{
  "name": "turned on",
  "state_topic": "pump/timestamp_on",
  "device_class": "timestamp",
  "value_template": "{{ as_datetime(value) }}",
  "unique_id": "hp_1231232_ts_on",
  "device": {
    "name": "Heat pump",
    "identifiers": [
      "hp_1231232"
    ]
  }
}
```


To test, you can use the command line tool `mosquitto_pub` shipped with `mosquitto` or the `mosquitto-clients` package to send MQTT messages.

Payload topic: `pump/timestamp_on`
Payload: `1707294116`

To set the state of the sensor manually:

```bash
mosquitto_pub -h 127.0.0.1 -p 1883 -u username -P some_password -t pump/timestamp_on -m '1707294116'
```

Make sure the IP address of your MQTT broker is used and that user credentials have been set up correctly.

The `value_template` will render the Unix EPOCH timestamp to correct format: `2024-02-07 08:21:56+00:00`.

### JSON attributes topic configuration

The example sensor below shows a configuration example which uses the following separate topic and JSON structure to add extra attributes.

Topic: `home/sensor1/attributes`
 ```json
 {
    "ClientName": <string>,
    "IP": <string>,
    "MAC": <string>,
    "RSSI": <string>,
    "HostName": <string>,
    "ConnectedSSID": <string>
}
 ```
 It also makes use of the `availability` topic.

Extra attributes will be displayed in the frontend and can also be extracted in [Templates](/docs/configuration/templating/#attributes). For example, to extract the `ClientName` attribute from the sensor below, use a template similar to: {% raw %}`{{ state_attr('sensor.bs_rssi', 'ClientName') }}`{% endraw %}.


```yaml
# Example configuration.yaml entry
mqtt:
  sensor:
    - name: "RSSI"
      state_topic: "home/sensor1/infojson"
      unit_of_measurement: "dBm"
      value_template: "{{ value_json.RSSI }}"
      availability:
        - topic: "home/sensor1/status"
      payload_available: "online"
      payload_not_available: "offline"
      json_attributes_topic: "home/sensor1/attributes"
```


### JSON attributes template configuration

The example sensor below shows a configuration example which uses the following topic and JSON structure with a template to add `Timer1.Arm` and `Timer1.Time` as extra attributes.

Topic: `tele/sonoff/sensor`
```json
{
    "Timer1": {
        "Arm": <status>,
        "Time": <time>
    },
    "Timer2": {
        "Arm": <status>,
        "Time": <time>
    }
}
``` 
To instead only add `Timer1.Arm`as an extra attribute, change `json_attributes_template` to: {% raw %}`"{{ {'Arm': value_json.Timer1} | tojson }}"`{% endraw %}.

Extra attributes will be displayed in the frontend and can also be extracted in [Templates](/docs/configuration/templating/#attributes). For example, to extract the `Arm` attribute from the sensor below, use a template similar to: {% raw %}`{{ state_attr('sensor.timer1', 'Arm') }}`{% endraw %}.


```yaml
# Example configuration.yaml entry
mqtt:
  sensor:
    - name: "Timer 1"
      state_topic: "tele/sonoff/sensor"
      value_template: "{{ value_json.Timer1.Arm }}"
      json_attributes_topic: "tele/sonoff/sensor"
      json_attributes_template: "{{ value_json.Timer1 | tojson }}"

    - name: "Timer 2"
      state_topic: "tele/sonoff/sensor"
      value_template: "{{ value_json.Timer2.Arm }}"
      json_attributes_topic: "tele/sonoff/sensor"
      json_attributes_template: "{{ value_json.Timer2 | tojson }}"
```


{% warning %}
If `json_attributes_topic` and `state_topic` share the same topic, a state update will happen only once, unless the state update did not change the state or `force_update` was set to `true`.

Setting up MQTT sensor's with extra state attributes that contain values that change at every update, like timestamps, or enabling the `force_update` option, is discouraged, as this will trigger state writes for every update. This can have a serious impact on the total system performance. A better option is creating separate sensors instead.
{% endwarning %}

### Usage of `entity_id` in the template

The example below shows how a simple filter, that calculates the value by adding 90% of the new value and 10% of the previous value, can be implemented in a template.


```yaml
# Example configuration.yaml entry
mqtt:
  sensor:
    - name: "Temp 1"
      state_topic: "sensor/temperature"
      value_template: |-
        {% if states(entity_id) == None %}
          {{ value | round(2) }}
        {% else %}
          {{ value | round(2) * 0.9 + states(entity_id) * 0.1 }}
        {% endif %}
```


### Owntracks battery level sensor

If you are using the [OwnTracks](/integrations/owntracks) and enable the reporting of the battery level then you can use an MQTT sensor to keep track of your battery. A regular MQTT message from OwnTracks looks like this:

Topic: `owntracks/tablet/tablet`
```json
{
    "_type": "location",
    "lon": 7.21,
    "t": "u",
    "batt": 92,
    "tst": 144995643,
    "tid": "ta",
    "acc": 27,
    "lat": 46.12
} 
```

Thus the trick is extracting the battery level from the payload.


```yaml
# Example configuration.yaml entry
mqtt:
  sensor:
    - name: "Battery Tablet"
      state_topic: "owntracks/tablet/tablet"
      unit_of_measurement: "%"
      value_template: "{{ value_json.batt }}"
```


### Temperature and humidity sensors

If you are using a DHT sensor and a NodeMCU board (esp8266), you can retrieve temperature and humidity with a MQTT sensor. A code example can be found [here](https://github.com/mertenats/open-home-automation/tree/master/ha_mqtt_sensor_dht22). A regular MQTT message from this example looks like this:

Topic: `office/sensor1`
```json
  {
    "temperature": 23.20,
    "humidity": 43.70
  }
```

Then use this configuration example to extract the data from the payload:


```yaml
# Example configuration.yaml entry
mqtt:
  sensor:
    - name: "Temperature"
      state_topic: "office/sensor1"
      suggested_display_precision: 1
      unit_of_measurement: "°C"
      value_template: "{{ value_json.temperature }}"
    - name: "Humidity"
      state_topic: "office/sensor1"
      unit_of_measurement: "%"
      value_template: "{{ value_json.humidity }}"
```


### Get sensor value from a device with ESPEasy

Assuming that you have flashed your ESP8266 unit with [ESPEasy](https://github.com/letscontrolit/ESPEasy). Under "Config" set a name ("Unit Name:") for your device (here it's "bathroom"). A "Controller" for MQTT with the protocol "OpenHAB MQTT" is present and the entries ("Controller Subscribe:" and "Controller Publish:") are adjusted to match your needs. In this example the topics are prefixed with "home". Please keep in mind that the ESPEasy default topics start with a `/` and only contain the name when writing your entry for the {% term "`configuration.yaml`" %} file.

- **Controller Subscribe**: `home/%sysname%/#` (instead of `/%sysname%/#`)
- **Controller Publish**: `home/%sysname%/%tskname%/%valname%` (instead of `/%sysname%/%tskname%/%valname%`)

Also, add a sensor in the "Devices" tap with the name "analog" and "brightness" as value.

As soon as the unit is online, you will get the state of the sensor.

```bash
home/bathroom/status Connected
...
home/bathroom/analog/brightness 290.00
```

The configuration will look like the example below:

```yaml
# Example configuration.yaml entry
mqtt:
  sensor:
    - name: "Brightness"
      state_topic: "home/bathroom/analog/brightness"
```
