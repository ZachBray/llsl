# Reactive Protocol

Version: 0.0.1



## Table of contents

- [Codecs](#codecs)
- [Enumerations](#enumerations)

    - [Frame Type](#frame-type)


## Codecs

The protocol uses the following codecs to represent messages or message parts.

## Enumerations

The protocol uses the following values to represent enumeration values.


### Frame Type



| Type | Value | Description |
| :--- | :---- | :---------- |
| _Reserved_ | `0x0` | Reserved |
| _Setup_ | `0x1` | Sent by client to initiate protocol processing. |
| _Lease_ | `0x2` | Sent by Responder to grant the ability to send requests. |

