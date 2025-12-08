pub use angle::Angle;
pub use center::Center;
pub use color::{Cmyk, Color, ColorGradient, Hsl, Hsv, LinearRgb, Luma, Oklab, Oklch, Rgb};
pub use datetime::{DateTime, DateTimeBuilder};
pub use duration::{Duration, DurationBuilder};
pub use gradient::{
    ColorSpace, ConicGradientBuilder, Gradient, LinearGradientBuilder, RadialGradientBuilder,
};
pub use length::{Length, LengthRadius};
pub use radius::Radius;
pub use ratio::Ratio;
use serde::de::DeserializeOwned;
pub use stop::Stop;
pub use r#type::Type;
pub use version::Version;

mod angle;
mod center;
pub mod color;
mod datetime;
mod duration;
mod gradient;
mod length;
mod radius;
mod ratio;
mod stop;
mod r#type;
mod version;

/// A trait for types that can be deserialized from a byte slice.
///
/// This trait is used by data structures such as `Gradient`, `Stop`, and `Center`
/// to provide a way to reconstruct instances from a series of bytes, often serialized
/// data.
/// ```
pub trait FromBytes: Sized + DeserializeOwned {
    /// Deserializes an instance of a type implementing this trait from a byte slice.
    ///
    /// # Arguments
    ///
    /// * `bytes` - A byte slice containing serialized data.
    ///
    /// # Returns
    ///
    /// Returns an instance of the type on success or a string error message on failure.
    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        ciborium::from_reader(bytes).map_err(|err| err.to_string())
    }
}

impl<T: DeserializeOwned> FromBytes for T {}
