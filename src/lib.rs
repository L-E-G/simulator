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

/// Represents the state of stages in the pipeline.
/// Values are names of the instruction in each stage.
#[wasm_bindgen]
#[derive(Serialize,Deserialize)]
pub struct PipelineStatus {
    cycle_count: u32,
    fetch: Option<String>,
    decode: Option<String>,
    execute: Option<String>,
    access_memory: Option<String>,
    write_back: Option<String>,
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
    pub fn get_dram(&self) -> Result<JsValue, JsValue> {
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

    /// Returns contents of registers.
    pub fn get_registers(&self) -> JsValue {
        JsValue::from_serde(&self.control_unit.registers.file).unwrap()
    }
    

    /// Returns the status of pipeline stages.
    pub fn get_pipeline(&self) -> JsValue {
        let mut status = PipelineStatus{
            cycle_count: self.control_unit.cycle_count,
            fetch: None,
            decode: None,
            execute: None,
            access_memory: None,
            write_back: None,
        };

        if let Some(i) = &self.control_unit.fetch_instruction {
            status.fetch = Some(format!("{}", i));
        }

        if let Some(i) = &self.control_unit.decode_instruction {
            status.decode = Some(format!("{}", i));
        }

        if let Some(i) = &self.control_unit.execute_instruction {
            status.execute = Some(format!("{}", i));
        }

        if let Some(i) = &self.control_unit.access_mem_instruction {
            status.access_memory = Some(format!("{}", i));
        }

        if let Some(i) = &self.control_unit.write_back_instruction {
            status.write_back = Some(format!("{}", i));
        }

        JsValue::from_serde(&status).unwrap()
    }

    /// Step through one cycle of processor.
    /// See return value of ControlUnit::step() for details on this methods
    /// return value.
    pub fn step(&mut self) -> Result<JsValue, JsValue> {
        match self.control_unit.step() {
            Err(e) => Err(JsValue::from_serde(&e).unwrap()),
            Ok(v) => Ok(JsValue::from_serde(&v).unwrap()),
        }
    }
}
