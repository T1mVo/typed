// #import "@preview/typed:0.1.0"
#import "../../src-typst/src/lib.typ" as typed

#let custom-plugin = plugin("custom_plugin.wasm")

#let custom = (
    "angle": 90deg,
    "color": red,
    "datetime": datetime.today(),
)

#let encoded = typed.cbor.encode(custom)

#let result = custom-plugin.custom_fn(encoded)

#str(result)
