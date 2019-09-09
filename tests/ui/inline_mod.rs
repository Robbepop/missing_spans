#![feature(proc_macro_hygiene)]

use missing_spans::my_attribute;

#[my_attribute]
mod foo {}

fn main() {}
