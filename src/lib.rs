use wasm_bindgen::prelude::*;
use wasm_bindgen::convert::{IntoWasmAbi,FromWasmAbi,WasmSlice};
use wasm_bindgen::JsValue;
use console_error_panic_hook;

#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;

mod result;
mod memory;
mod instructions;
mod control_unit;
use crate::control_unit::ControlUnit;
use crate::result::SimResult;
use crate::memory::{Memory,InspectableMemory};

/// Interface between JavaScript and all simulator functionality.
#[wasm_bindgen]
pub struct Simulator {
    control_unit: ControlUnit,
}

#[wasm_bindgen]
impl Simulator {
    /// Initializes simulator.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Simulator {
        // Setup panic logging hook
        console_error_panic_hook::set_once();

        Simulator{
            control_unit: ControlUnit::new(),
        }
    }

    /// Returns addresses and values in DRAM. First returned value is a list of
    /// addresses. Second returned value is a list of values corresponding to
    /// the addresses.
    pub fn get_addresses(self) -> Result<JsValue, JsValue> {
        match self.control_unit.memory.inspect() {
            Err(e) => {
                Err(JsValue::from_serde(&format!("failed to inspect DRAM: {}", e)).unwrap())
            },
            Ok(addresses) => {
                Ok(JsValue::from_serde(&addresses).unwrap())
            },
        }
    }

    /// Returns valid addresses in DRAM.
    pub fn get_dram_addresses(self) -> Option<Vec<u32>> {
        match self.control_unit.memory.inspect() {
            Err(e) => {
                println!("Simulator::get_dram_addresses, error: {}", e);
                None
            },
            Ok(v) => {
                let mut addrs: Vec<u32> = Vec::new();
                
                for (key, _value) in v.into_iter() {
                    addrs.push(key);
                }

                Some(addrs)
            },
        }
    }

    /// Return value of address in DRAM.
    pub fn get_dram_address(mut self, address: u32) -> Option<u32> {
        match (&mut (self.control_unit.memory)).get(address) {
            SimResult::Err(e) => {
                println!("Simulator::get_dram_address, error: {}", e);
                None
            },
            SimResult::Wait(_wait, v) => {
                Some(v)
            },
        }
    }
}
