#[macro_use]
extern crate rustler;
extern crate lazy_static;
extern crate rustler_codegen;

extern crate json;

use rustler::schedule::SchedulerFlags::*;

mod atoms;
mod decoder;

rustler_export_nifs! {
    "Elixir.JsonRex",
    [("decode", 1, decoder::decode),
     ("decode_dirty", 1, decoder::decode, DirtyCpu)],
    None
}
