#import "../../src-typst/src/lib.typ" as typed

#let typed-test = plugin("typed_test.wasm")

#let test = (
  "angle": 90deg,
  "color": color.linear-rgb(color.red),
  "color-gradient": color.green,
  "center": ("x": 75%, "y": 25%),
  "datetime": datetime.today(),
  "duration": duration(days: 3, hours: 12),
  "linear-gradient": gradient.linear(red, blue),
  "radial-gradient": gradient.radial(green, red),
  "conic-gradient": gradient.conic(black, white),
  "length": 12pt,
  "length-radius": 6pt,
  "radius": ("top-left": 6pt, "top-right": 0pt, "bottom-left": 0pt, "bottom-right": 12pt),
  "ratio": 2%,
  "type": int,
  "version": version(0, 1, 1),
)

#let encoded = typed.encode(test)
//#cbor(encoded)
#let string = typed-test.test(encoded)

#str(string)
