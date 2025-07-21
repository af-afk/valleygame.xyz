#![no_std]
#![no_main]

extern crate alloc;

use stylus_sdk::alloy_primitives;

use alloc::{vec, vec::Vec};

use stylus_sdk::prelude::*;

#[entrypoint]
#[storage]
struct Contract {}

#[public]
impl Contract {
    pub fn something(&self) {
    }
}
