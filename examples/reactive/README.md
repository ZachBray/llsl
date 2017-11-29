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

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                    length                     |     header   ...
+-----------------------------------------------+---------------+
```

| Field | Type | Offset (bits) | Size (bits) | Description |
| :---- | :--- | ------------: | ----------: | :---------- |
| <ins>**length**</ins> | | `0` | `24` | Unsigned 24-bit integer representing the length of Frame in bytes. Excluding the Frame Length field. |
| <ins>**header**</ins> | [Frame Header](#frame-header) | `24` | `` |  |


### Frame Header

RSocket frames begin with a RSocket Frame Header.

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|0|                          Stream Id                          |
+-+---------+-+-+-----------------------------------------------+
|Frame Type |I|M|                     Blobby                   ...
+-----------+-+-+-----------------------------------------------+
```

| Field | Type | Offset (bits) | Size (bits) | Description |
| :---- | :--- | ------------: | ----------: | :---------- |
| <ins>**Stream Id**</ins> | | `1` | `31` | Unsigned 31-bit integer representing the stream Identifier for this frame or 0 to indicate the entire connection. |
| <ins>**Frame Type**</ins> | [Frame Type](#frame-type) | `32` | `6` | Type of Frame. |
| <ins>**I**</ins>gnore | | `38` | `1` | Ignore frame if not understood |
| <ins>**M**</ins>etadata | | `39` | `1` | Metadata present |
| <ins>**Blobby**</ins> | | `40` | `` |  |


### Setup

The SETUP frame is sent by the client to inform the server of the parameters under which it desires to operate.

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|L|/ / / / / / / / / / / / / / /|
+-+-----------------------------+-------------------------------+
|         Major Version         |         Minor Version         |
+-+-----------------------------+-------------------------------+
|0|                Time Between KEEPALIVE Frames                |
+-+-------------------------------------------------------------+
|0|                        Max Lifetime                         |
+-+-------------------------------------------------------------+
```

| Field | Type | Offset (bits) | Size (bits) | Description |
| :---- | :--- | ------------: | ----------: | :---------- |
| <ins>**L**</ins>ease | | `0` | `1` | Will honor LEASE (or not). |
| <ins>**Major Version**</ins> | | `16` | `16` | Unsigned 16-bit integer of Major version number of the protocol. |
| <ins>**Minor Version**</ins> | | `32` | `16` | Unsigned 16-bit integer of Minor version number of the protocol. |
| <ins>**Time Between KEEPALIVE Frames**</ins> | | `49` | `31` | Unsigned 31-bit integer of Time (in milliseconds) between KEEPALIVE frames that the client will send. Value MUST be > 0. |
| <ins>**Max Lifetime**</ins> | | `81` | `31` | Unsigned 31-bit integer of Time (in milliseconds) that a client will allow a server to not respond to a KEEPALIVE before it is assumed to be dead. Value MUST be > 0. |


## Enumerations

The protocol uses the following values to represent enumeration values.

### Frame Type



| Type | Value | Description |
| :--- | ----: | :---------- |
| _Reserved_ | `0x0` | Reserved |
| _Setup_ | `0x1` | Sent by client to initiate protocol processing. |
| _Lease_ | `0x2` | Sent by Responder to grant the ability to send requests. |



