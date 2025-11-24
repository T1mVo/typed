/// Create a new center dictionary used in gradients.
///
/// - x (ratio): A ratio value between 0% and 100%.
/// - y (ratio): A ratio value between 0% and 100%.
/// -> dictionary
#let new(x, y) = {
  assert(type(x) == ratio, message: "center.new: x must be of type ratio")
  assert(type(y) == ratio, message: "center.new: y must be of type ratio")

  (
    "x": x,
    "y": y,
  )
}

/// Encode a center point into a CBOR-compatible dictionary.
///
/// - x (ratio): A ratio value between 0% and 100%.
/// - y (ratio): A ratio value between 0% and 100%.
/// -> dictionary
#let encode(x, y) = {
  assert(type(x) == ratio, message: "center.encode: x must be of type ratio")
  assert(type(y) == ratio, message: "center.encode: y must be of type ratio")

  (
    "x": x,
    "y": y,
  )
}
