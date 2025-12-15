// #import "@preview/typwire:0.1.0"
#import "../../src-typst/src/lib.typ" as typwire

#let custom-plugin = plugin("custom_plugin.wasm")

#let custom = (
    "angle": 90deg,
    "color": red,
    "datetime": datetime.today(),
)

#let encoded = typwire.cbor.encode(custom)

#let result = custom-plugin.custom_fn(encoded)

#str(result)
