use serde::{Deserialize, Serialize};

use crate::Radius;

const TYPE_NAME: &str = "length";

/// A structure representing a length in points.
///
/// # Examples
///
/// ```
/// use typed::Length;
///
/// let length = Length::new(72.0);
/// assert_eq!(length.pt(), 72.0);
/// assert_eq!(length.inches(), 1.0);
/// ```
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(try_from = "LengthCbor", into = "LengthCbor")]
pub struct Length {
    points: f64,
}

impl Length {
    /// Creates a new `Length` instance from a given number of points.
    ///
    /// # Arguments
    ///
    /// * `points` - A floating-point value representing the length in points.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed::Length;
    ///
    /// let length = Length::new(72.0);
    /// assert_eq!(length.pt(), 72.0);
    /// ```
    pub const fn new(points: f64) -> Self {
        Self { points }
    }

    /// Returns the length in points.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed::Length;
    ///
    /// let length = Length::new(72.0);
    /// assert_eq!(length.pt(), 72.0);
    /// ```
    pub const fn pt(&self) -> f64 {
        self.points
    }

    /// Converts the length to millimeters.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed::Length;
    ///
    /// let length = Length::new(72.0);
    /// assert_eq!(length.mm(), 25.4);
    /// ```
    pub const fn mm(&self) -> f64 {
        self.points * (25.4 / 72.0)
    }

    /// Converts the length to centimeters.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed::Length;
    ///
    /// let length = Length::new(720.0);
    /// assert_eq!(length.cm(), 25.4);
    /// ```
    pub const fn cm(&self) -> f64 {
        self.points * (25.4 / 720.0)
    }

    /// Converts the length to inches.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed::Length;
    ///
    /// let length = Length::new(72.0);
    /// assert_eq!(length.inches(), 1.0);
    /// ```
    pub const fn inches(&self) -> f64 {
        self.points / 72.0
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct LengthCbor {
    typed_type: String,
    points: f64,
}

// Serialize into LengthCbor
impl From<Length> for LengthCbor {
    fn from(value: Length) -> Self {
        Self {
            typed_type: TYPE_NAME.to_string(),
            points: value.points,
        }
    }
}

// Deserialize from LengthCbor
impl TryFrom<LengthCbor> for Length {
    type Error = String;

    fn try_from(value: LengthCbor) -> Result<Self, Self::Error> {
        if value.typed_type != TYPE_NAME {
            return Err(format!(
                "Invalid typed-type for Length: {}",
                value.typed_type
            ));
        }

        Ok(Length::new(value.points))
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
/// An enum representing either a `Length` or a `Radius`.
///
/// # Examples
///
/// ```
/// use typed::{Length, LengthRadius};
///
/// let lr = LengthRadius::Length(Length::new(72.0));
/// ```
pub enum LengthRadius {
    Length(Length),
    Radius(Radius),
}
