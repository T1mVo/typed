#![allow(unused)]

use serde::Deserialize;
use typed::{
    Angle, Center, Color, ColorGradient, DateTime, Duration, FromBytes, Gradient, Length,
    LengthRadius, Radius, Ratio, Stop, Type, Version,
};
use wasm_minimal_protocol::*;

initiate_protocol!();

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct Test {
    angle: Angle,
    color: Color,
    color_gradient: ColorGradient,
    center: Center,
    datetime: DateTime,
    duration: Duration,
    linear_gradient: Gradient,
    radial_gradient: Gradient,
    conic_gradient: Gradient,
    length: Length,
    length_radius: LengthRadius,
    radius: Radius,
    ratio: Ratio,
    r#type: Type,
    version: Version,
}

impl FromBytes for Test {
    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        ciborium::from_reader(bytes).map_err(|err| err.to_string())
    }
}

#[wasm_func]
fn test(arg: &[u8]) -> Result<Vec<u8>, String> {
    let test = Test::from_bytes(arg)?;
    //let test: ciborium::Value = ciborium::from_reader(arg).unwrap();

    Ok(format!("{:?}", test).as_bytes().to_vec())
}
