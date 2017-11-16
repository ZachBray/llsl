# Glue Protocol

Version: 0.0.1



## Table of contents

- [Codecs](#codecs)
  - [Common Header](#common-header)
  - [Acknowledge](#acknowledge)
  - [Create Header](#create-header)
  - [Send Header](#send-header)
  - [Dispose Header](#dispose-header)
  - [Error Header](#error-header)
  - [Instance Reference](#instance-reference)
  - [Method Reference](#method-reference)
- [Enumerations](#enumerations)
  - [Frame Type](#frame-type)
  - [Error Code](#error-code)

## Codecs

The protocol uses the following codecs to represent messages or message parts.

### Common Header



```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|     Type      |                  Session Id                   |
+---------------+-----------------------------------------------+
```

| Field | Type | Offset (bits) | Size (bits) | Description |
| :---- | :--- | ------------: | ----------: | :---------- |
| <ins>**Type**</ins> | [Frame Type](#frame-type) | `0` | `8` |  |
| <ins>**Session Id**</ins> | | `8` | `24` |  |


### Acknowledge



```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|0|                      Keep alive period                      |
+-+-------------------------------------------------------------+
```

| Field | Type | Offset (bits) | Size (bits) | Description |
| :---- | :--- | ------------: | ----------: | :---------- |
| <ins>**Keep alive period**</ins> | | `1` | `31` | Time between keep alive frames in milliseconds |


### Create Header



```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|         Service Type          | Major Version | Minor Version |
+-------------------------------+---------------+---------------+
|                        Initial Message                        |
+---------------------------------------------------------------+
```

| Field | Type | Offset (bits) | Size (bits) | Description |
| :---- | :--- | ------------: | ----------: | :---------- |
| <ins>**Service Type**</ins> | | `0` | `16` |  |
| <ins>**Major Version**</ins> | | `16` | `8` |  |
| <ins>**Minor Version**</ins> | | `24` | `8` |  |
| <ins>**Initial Message**</ins> | [Send Header](#send-header) | `32` | `32` | Instance Id describes the one that will be created by the service. |


### Send Header



```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                   Instance                    |    Method     |
+-----------------------------------------------+---------------+
```

| Field | Type | Offset (bits) | Size (bits) | Description |
| :---- | :--- | ------------: | ----------: | :---------- |
| <ins>**Instance**</ins> | [Instance Reference](#instance-reference) | `0` | `24` |  |
| <ins>**Method**</ins> | [Method Reference](#method-reference) | `24` | `8` |  |


### Dispose Header



```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                   Instance                    |
+-----------------------------------------------+
```

| Field | Type | Offset (bits) | Size (bits) | Description |
| :---- | :--- | ------------: | ----------: | :---------- |
| <ins>**Instance**</ins> | [Instance Reference](#instance-reference) | `0` | `24` |  |


### Error Header



```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|     Code      |
+---------------+
```

| Field | Type | Offset (bits) | Size (bits) | Description |
| :---- | :--- | ------------: | ----------: | :---------- |
| <ins>**Code**</ins> | [Error Code](#error-code) | `0` | `8` |  |


### Instance Reference



```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                      Id                       |
+-----------------------------------------------+
```

| Field | Type | Offset (bits) | Size (bits) | Description |
| :---- | :--- | ------------: | ----------: | :---------- |
| <ins>**Id**</ins> | | `0` | `24` |  |


### Method Reference



```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|      Id       |
+---------------+
```

| Field | Type | Offset (bits) | Size (bits) | Description |
| :---- | :--- | ------------: | ----------: | :---------- |
| <ins>**Id**</ins> | | `0` | `8` |  |


## Enumerations

The protocol uses the following values to represent enumeration values.

### Frame Type



| Type | Value | Description |
| :--- | ----: | :---------- |
| _Establish Session_ | `0x1` |  |
| _Acknowledge Session_ | `0x2` |  |
| _Keep Alive_ | `0x3` |  |
| _Error_ | `0x4` |  |
| _Create_ | `0x5` |  |
| _Send_ | `0x6` |  |
| _Dispose_ | `0x7` |  |


### Error Code



| Type | Value | Description |
| :--- | ----: | :---------- |
| _Invalid Request_ | `0x1` |  |
| _Application Error_ | `0x2` |  |
| _Session Does Not Exist_ | `0x3` |  |
| _Instance Does Not Exist_ | `0x4` |  |
| _Method Does Not Exist_ | `0x5` |  |
| _Version Mismatch_ | `0x6` |  |



