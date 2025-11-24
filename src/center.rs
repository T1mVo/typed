use serde::{Deserialize, Serialize};

use crate::{FromBytes, Ratio};

/// A structure representing the center of a gradient, defined by x and y ratios.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Center {
    pub x: Ratio,
    pub y: Ratio,
}

impl Center {
    /// Creates a new `Center` instance with the specified x and y ratios.
    ///
    /// # Arguments
    ///
    /// * `x` - The x ratio of the center.
    /// * `y` - The y ratio of the center.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed::{Center, Ratio};
    ///
    /// let center = Center::new(Ratio::new(0.5), Ratio::new(0.5));
    /// assert_eq!(center.x, Ratio::new(0.5));
    /// assert_eq!(center.y, Ratio::new(0.5));
    /// ```
    pub const fn new(x: Ratio, y: Ratio) -> Self {
        Self { x, y }
    }
}

impl FromBytes for Center {
    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        ciborium::from_reader(bytes).map_err(|err| err.to_string())
    }
}
