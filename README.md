# kdl-test

This repository implements an implementation-agnostic test suite for [KDL](https://kdl.dev).

The [official test cases](https://github.com/kdl-org/kdl/tree/main/tests) are underspecified and requires implementations have a rendering implementation ([ref](https://github.com/kdl-org/kdl/issues/252)). This test suite is minimal and only verifies that you've parsed the document correctly.

## Installation

Download binaries on the release page.

## Usage

```shell
kdl-test --decoder my-kdl-decoder
```

You should implement a `my-kdl-decoder` executable for your implementation with the following interface:

* Accept KDL input on `stdin`
* If the KDL data is invalid, return a non-zero exit code
    * You may output error information to `stderr`, for debugging
* If the KDL data is valid, output the [JSON encoding](#json-encoding) of that data on `stdout` and return with a zero exit code

## JSON encoding

Node lists (and therefore documents) are represented as a JSON array.

Nodes are represented as a JSON object with the following schema. All keys are required.

```json5
{
  "type": "type_of_node", // `null` if no annotation
  "name": "name_of_node",
  "entries": [
    // Argument
    {
      "name": null,
      "type": "type_of_value", // `null` if no annotation
      "value": { /* See Value encoding */ },
    },
    // Property
    {
      "name": "name_of_prop",
      "type": "type_of_value", // `null` if no annotation
      "value": { /* See Value encoding */ },
    }
  ],
  "children": [ /* nodes */ ]
}
```

Values are represented as a JSON object with the following schema.

```json5
// String
//   Multi-line + raw strings are converted to single-line strings
{
  "type": "string",
  "value": "hello world"
}

// Number
//   Ints should always include ".0"
//   Hex/octal/binary should be decoded
//   Keywords should be verbatim, e.g. "#-inf"
{
  "type": "number",
  "value": "123.0"
}

// Boolean
{
  "type": "boolean",
  "value": "true"
}

// Null
{
  "type": "null"
}
```
