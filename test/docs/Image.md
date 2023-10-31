# Image

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**availability** | Option<[**Vec<crate::models::AlarmControlPanelAvailabilityInner>**](AlarmControlPanel_availability_inner.md)> | A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`. | [optional]
**availability_mode** | Option<**String**> | When `availability` is configured, this controls the conditions needed to set the entity to `available`. Valid entries are `all`, `any`, and `latest`. If set to `all`, `payload_available` must be received on all configured availability topics before the entity is marked as online. If set to `any`, `payload_available` must be received on at least one configured availability topic before the entity is marked as online. If set to `latest`, the last `payload_available` or `payload_not_available` received on any configured availability topic controls the availability. (Default: `latest)` | [optional]
**availability_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract device's availability from the `availability_topic`. To determine the devices's availability result of this template will be compared to `payload_available` and `payload_not_available`. | [optional]
**availability_topic** | Option<**String**> | The MQTT topic subscribed to receive availability (online/offline) updates. Must not be used together with `availability`. | [optional]
**content_type** | Option<**String**> | The content type of and image data message received on `image_topic`. This option cannot be used with the `url_topic` because the content type is derived when downloading the image. (Default: `image/jpeg)` | [optional]
**device** | Option<[**crate::models::CameraDevice**](Camera_device.md)> |  | [optional]
**enabled_by_default** | Option<**bool**> | Flag which defines if the entity should be enabled when first added. (Default: `true)` | [optional]
**encoding** | Option<**String**> | The encoding of the payloads received. Set to `\"\"` to disable decoding of incoming payload. Use `image_encoding` to enable `Base64` decoding on `image_topic`. (Default: `utf-8)` | [optional]
**entity_category** | Option<**String**> | The [category](https://developers.home-assistant.io/docs/core/entity#generic-properties) of the entity. (Default: `None)` | [optional]
**image_encoding** | Option<**String**> | The encoding of the image payloads received. Set to `\"b64\"` to enable base64 decoding of image payload. If not set, the image payload must be raw binary data. (Default: `None)` | [optional]
**image_topic** | Option<**String**> | The MQTT topic to subscribe to receive the image payload of the image to be downloaded. Ensure the `content_type` type option is set to the corresponding content type. This option cannot be used together with the `url_topic` option. But at least one of these option is required. | [optional]
**json_attributes_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`. | [optional]
**json_attributes_topic** | Option<**String**> | The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Implies `force_update` of the current sensor state when a message is received on this topic. | [optional]
**name** | Option<**String**> | The name of the image. Can be set to `null` if only the device name is relevant. | [optional]
**object_id** | Option<**String**> | Used instead of `name` for automatic generation of `entity_id` | [optional]
**unique_id** | Option<**String**> | An ID that uniquely identifies this image. If two images have the same unique ID Home Assistant will raise an exception. | [optional]
**url_template** | Option<**String**> | Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the image URL from a message received at `url_topic`. | [optional]
**url_topic** | Option<**String**> | The MQTT topic to subscribe to receive an image URL. A `url_template` option can extract the URL from the message. The `content_type` will be derived from the image when downloaded. This option cannot be used together with the `image_topic` option, but at least one of these options is required. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

