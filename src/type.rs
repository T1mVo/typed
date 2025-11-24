use serde::{Deserialize, Serialize};

use crate::FromBytes;

const TYPE_NAME: &str = "type";

/// A structure representing a type defined by a string.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[serde(try_from = "TypeCbor", into = "TypeCbor")]
pub struct Type {
    pub ty: String,
}

impl Type {
    /// Creates a new `Type` instance from the given string.
    ///
    /// # Arguments
    ///
    /// * `r#type` - A string representing the type.
    ///
    /// # Examples
    ///
    /// ```
    /// let custom_type = Type::new(String::from("CustomType"));
    /// assert_eq!(custom_type.r#type, "CustomType");
    /// ```
    pub const fn new(ty: String) -> Self {
        Self { ty }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct TypeCbor {
    typed_type: String,
    ty: String,
}

// Serialize into TypeCbor
impl From<Type> for TypeCbor {
    fn from(value: Type) -> Self {
        Self {
            typed_type: TYPE_NAME.to_string(),
            ty: value.ty,
        }
    }
}

// Deserialize from TypeCbor
impl TryFrom<TypeCbor> for Type {
    type Error = String;

    fn try_from(value: TypeCbor) -> Result<Self, Self::Error> {
        if value.typed_type != TYPE_NAME {
            return Err(format!("Invalid typed-type for Type: {}", value.typed_type));
        }

        Ok(Self::new(value.ty))
    }
}

impl FromBytes for Type {
    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        ciborium::from_reader(bytes).map_err(|err| err.to_string())
    }
}
