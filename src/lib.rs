use wasm_bindgen::prelude::*;
use wasm_bindgen::convert::{IntoWasmAbi,FromWasmAbi,WasmSlice};
use wasm_bindgen::JsValue;
use console_error_panic_hook;

#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::io::BufReader;

mod result;
mod memory;
mod instructions;
mod control_unit;
use crate::control_unit::ControlUnit;
use crate::result::SimResult;
use crate::memory::{Memory,InspectableMemory};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

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
    pub fn get_dram(self) -> Result<JsValue, JsValue> {
        match self.control_unit.memory.inspect() {
            Err(e) => {
                Err(JsValue::from_serde(
                    &format!("failed to inspect DRAM: {}", e)).unwrap())
            },
            Ok(addresses) => {
                Ok(JsValue::from_serde(&addresses).unwrap())
            },
        }
    }

    /// Sets the contents of DRAM based on binary input.
    /// See DRAM::load_from_reader() for details on the required format of
    /// the input.
    pub fn set_dram(&mut self, input: &[u8]) -> Result<(), JsValue> {
        match self.control_unit.memory.load_from_reader(input) {
            Err(e) => Err(JsValue::from_serde(
                &format!("failed to load input into DRAM: {}", e)).unwrap()),
            Ok(_v) => Ok(()),
        }
    }
}
