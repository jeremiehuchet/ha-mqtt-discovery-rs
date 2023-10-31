# Button

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**availability** | Option<[**Vec<crate::models::AlarmControlPanelAvailabilityInner>**](AlarmControlPanel_availability_inner.md)> | A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`. | [optional]
**availability_mode** | Option<**String**> | When `availability` is configured, this controls the conditions needed to set the entity to `available`. Valid entries are `all`, `any`, and `latest`. If set to `all`, `payload_available` must be received on all configured availability topics before the entity is marked as online. If set to `any`, `payload_available` must be received on at least one configured availability topic before the entity is marked as online. If set to `latest`, the last `payload_available` or `payload_not_available` received on any configured availability topic controls the availability. (Default: `latest)` | [optional]
**availability_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract device's availability from the `availability_topic`. To determine the devices's availability result of this template will be compared to `payload_available` and `payload_not_available`. | [optional]
**availability_topic** | Option<**String**> | The MQTT topic subscribed to receive availability (online/offline) updates. Must not be used together with `availability`. | [optional]
**command_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to generate the payload to send to `command_topic`. | [optional]
**command_topic** | Option<**String**> | The MQTT topic to publish commands to trigger the button. | [optional]
**device** | Option<[**crate::models::ButtonDevice**](Button_device.md)> |  | [optional]
**device_class** | Option<**String**> | The [type/class](/integrations/button/#device-class) of the button to set the icon in the frontend. The `device_class` can be `null`. (Default: `None)` | [optional]
**enabled_by_default** | Option<**bool**> | Flag which defines if the entity should be enabled when first added. (Default: `true)` | [optional]
**encoding** | Option<**String**> | The encoding of the published messages. (Default: `utf-8)` | [optional]
**entity_category** | Option<**String**> | The [category](https://developers.home-assistant.io/docs/core/entity#generic-properties) of the entity. (Default: `None)` | [optional]
**json_attributes_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation. | [optional]
**json_attributes_topic** | Option<**String**> | The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation. | [optional]
**name** | Option<**String**> | The name to use when displaying this button. Can be set to `null` if only the device name is relevant. (Default: `MQTT Button)` | [optional]
**object_id** | Option<**String**> | Used instead of `name` for automatic generation of `entity_id` | [optional]
**payload_available** | Option<**String**> | The payload that represents the available state. (Default: `online)` | [optional]
**payload_not_available** | Option<**String**> | The payload that represents the unavailable state. (Default: `offline)` | [optional]
**payload_press** | Option<**String**> | The payload To send to trigger the button. (Default: `PRESS)` | [optional]
**qos** | Option<**i32**> | The maximum QoS level to be used when receiving and publishing messages. | [optional]
**retain** | Option<**bool**> | If the published message should have the retain flag on or not. | [optional]
**unique_id** | Option<**String**> | An ID that uniquely identifies this button entity. If two buttons have the same unique ID, Home Assistant will raise an exception. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

