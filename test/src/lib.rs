#![allow(unused)]

use serde::Deserialize;
use typwire::{
    Angle, Center, Color, ColorGradient, Content, DateTime, Duration, FromBytes as _, Gradient, Length,
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
    content: Content,
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

#[wasm_func]
fn test(arg: &[u8]) -> Result<Vec<u8>, String> {
    let test = Test::from_bytes(arg)?;
    
    Ok(format!("{:?}", test).as_bytes().to_vec())
}
