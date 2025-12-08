#![allow(unused)]

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
fn custom_fn(arg: &[u8]) -> Result<Vec<u8>, String> {
    let custom = Custom::from_bytes(arg)?;

    // ...

    Ok(vec![])
}
