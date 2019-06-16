#[macro_use]
extern crate rustler;

#[macro_use]
extern crate lazy_static;
extern crate rustler_codegen;

extern crate serde;
extern crate serde_json;

mod atoms;
mod decoder;

rustler_export_nifs! {
    "Elixir.JRex",
    [("decode", 1, decoder::decode)],
    None
}
