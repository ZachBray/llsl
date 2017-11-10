# Reactive Protocol

Version: 0.0.1



## Table of contents

- [Codecs](#codecs)
  - [Frame](#frame)
  - [Frame Header](#frame-header)
  - [Setup](#setup)
- [Enumerations](#enumerations)
  - [Frame Type](#frame-type)

## Codecs

The protocol uses the following codecs to represent messages or message parts.

### Frame

When using a transport protocol that does not provide compatible framing, the Frame Length MUST be prepended to the RSocket Frame.

### Frame Header

RSocket frames begin with a RSocket Frame Header.

### Setup

The SETUP frame is sent by the client to inform the server of the parameters under which it desires to operate.

## Enumerations

The protocol uses the following values to represent enumeration values.

### Frame Type



| Type | Value | Description |
| :--- | :---- | :---------- |
| _Reserved_ | `0x0` | Reserved |
| _Setup_ | `0x1` | Sent by client to initiate protocol processing. |
| _Lease_ | `0x2` | Sent by Responder to grant the ability to send requests. |

