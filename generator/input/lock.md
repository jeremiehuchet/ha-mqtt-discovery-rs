---
title: "MQTT Lock"
description: "Instructions on how to integrate MQTT locks into Home Assistant."
ha_category:
  - Lock
ha_release: 0.15
ha_iot_class: Configurable
ha_domain: mqtt
---

The `mqtt` lock platform lets you control your MQTT enabled locks.

## Configuration

In an ideal scenario, the MQTT device will have a `state_topic` to publish state changes. If these messages are published with a `RETAIN` flag, the MQTT lock will receive an instant state update after subscription and will start with correct state. Otherwise, the initial state of the lock will be `false` / unlocked.

When a `state_topic` is not available, the lock will work in optimistic mode. In this mode, the lock will immediately change state after every command. Otherwise, the lock will wait for state confirmation from the device (message from `state_topic`).

Optimistic mode can be forced, even if state topic is available. Try to enable it, if experiencing incorrect lock operation.

It's mandatory for locks to support `lock` and `unlock`. A lock may optionally support `open`, (e.g. to open the bolt in addition to the latch), in this case, `payload_open` is required in the configuration. If the lock is in optimistic mode, it will change states to `unlocked` when handling the `open` command.

An MQTT lock can also report the intermediate states `unlocking`, `locking` or `jammed` if the motor reports a jammed state.
To enable MQTT locks in your installation, add the following to your {% term "`configuration.yaml`" %} file:

```yaml
# Example configuration.yaml entry
mqtt:
  - lock:
      command_topic: "home/frontdoor/set"
```


⚠ Important\
Make sure that your topics match exactly. `some-topic/` and `some-topic` are different topics.

## Examples

In this section you will find some real-life examples of how to use this lock.

### Full configuration

The example below shows a full configuration for a MQTT lock.


```yaml
# Example configuration.yaml entry
mqtt:
  - lock:
      name: Frontdoor
      state_topic: "home-assistant/frontdoor/state"
      code_format: "^\\d{4}$"
      command_topic: "home-assistant/frontdoor/set"
      command_template: '{ "action": "{{ value }}", "code":"{{ code }}" }'
      payload_lock: "LOCK"
      payload_unlock: "UNLOCK"
      state_locked: "LOCK"
      state_unlocked: "UNLOCK"
      state_locking: "LOCKING"
      state_unlocking: "UNLOCKING"
      state_jammed: "MOTOR_JAMMED"
      state_ok: "MOTOR_OK"
      optimistic: false
      qos: 1
      retain: true
      value_template: "{{ value.x }}"
```


Keep an eye on retaining messages to keep the state as you don't want to unlock your door by accident when you restart something.

For a check you can use the command line tools `mosquitto_pub` shipped with `mosquitto` to send MQTT messages. This allows you to operate your lock manually:

```bash
mosquitto_pub -h 127.0.0.1 -t home-assistant/frontdoor/set -m "LOCK"
```
