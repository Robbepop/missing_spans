#![feature(proc_macro_hygiene)]

use missing_spans::my_attribute;

#[my_attribute]
struct foo {}

fn main() {}