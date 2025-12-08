# typed

An extended CBOR encoder for the [Typst](https://typst.app/) plugin system.

This library provides a CBOR encoder with extended [support](#supported-types) for Typst types, along with corresponding Rust types and deserialization, enabling seamless data exchange between Typst and Rust WASM plugins.

## Usage

**Rust:**

```rs
use serde::Deserialize;
use typed::{Angle, Color, DateTime, FromBytes as _};
use wasm_minimal_protocol::*;

initiate_protocol!();

#[derive(Deserialize)]
struct Custom {
    angle: Angle,
    color: Color,
    datetime: DateTime,
}

#[wasm_func]
fn custom_fn(arg: &[u8]) -> Result<(), String> {
    let custom = Custom::from_bytes(arg)?;

    // ...

    Ok(())
}
```

**Typst:**

```typ
#import "@preview/typed:0.1.0"

#let custom-plugin = plugin("custom_plugin.wasm")

#let custom = (
    "angle": 90deg,
    "color": red,
    "datetime": datetime.today(),
)

#let encoded = typed.cbor.encode(custom)

#custom-plugin.custom_fn(encoded)
```

## Supported types

 - [x] int
 - [x] float
 - [x] bytes
 - [x] str
 - [x] bool
 - [x] content
 - [x] none
 - [x] array
 - [x] dictionary
 - [x] angle
 - [x] length
 - [x] ratio
 - [x] color
 - [x] gradient
 - [x] datetime
 - [x] duration
 - [x] version
 - [x] type
 - [ ] fraction
 - [ ] direction
 - [ ] relative
 - [ ] decimal

## TODO

- name enum variants "gradient-linear"?
- use ecostring?
- add color consts
- add color conversion
- add gradient consts/statics
- add value type
