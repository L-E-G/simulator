extern crate clap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::convert::{IntoWasmAbi,FromWasmAbi,WasmSlice};
use wasm_bindgen::JsValue;
use console_error_panic_hook;
use js_sys;
use web_sys::console;

#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::io::BufReader;
use std::fmt::Debug;

mod result;
mod memory;
mod instructions;
mod control_unit;
mod assembler;
use crate::control_unit::ControlUnit;
use crate::result::SimResult;
use crate::memory::{Memory,InspectableMemory};
use crate::assembler::assembler;

/// Represents the state of stages in the pipeline.
/// Values are names of the instruction in each stage.
#[wasm_bindgen]
#[derive(Serialize,Deserialize,Debug)]
pub struct PipelineStatus {
    fetch: Option<String>,
    decode: Option<String>,
    execute: Option<String>,
    access_memory: Option<String>,
    write_back: Option<String>,
}

impl PipelineStatus {
    fn new(cu: &ControlUnit) -> PipelineStatus {
        let mut status = PipelineStatus{
            fetch: None,
            decode: None,
            execute: None,
            access_memory: None,
            write_back: None,
        };

        if let Some(i) = &cu.fetch_instruction {
            status.fetch = Some(format!("{}", i));
        }

        if let Some(i) = &cu.decode_instruction {
            status.decode = Some(format!("{}", i));
        }

        if let Some(i) = &cu.execute_instruction {
            status.execute = Some(format!("{}", i));
        }

        if let Some(i) = &cu.access_mem_instruction {
            status.access_memory = Some(format!("{}", i));
        }

        if let Some(i) = &cu.write_back_instruction {
            status.write_back = Some(format!("{}", i));
        }

        status
    }
}

/// Interface between JavaScript and all simulator functionality.
#[wasm_bindgen]
pub struct Simulator {
    control_unit: ControlUnit,
    pipeline_statuses: Vec<PipelineStatus>,
}

#[wasm_bindgen]
impl Simulator {
    /// Initializes simulator
    #[wasm_bindgen(constructor)]
    pub fn new() -> Simulator {
        // Setup panic logging hook
        console_error_panic_hook::set_once();

        Simulator{
            control_unit: ControlUnit::new(),
            pipeline_statuses: vec![],
        }
    }

    fn use_assembler(file: &str) -> JsValue {
        JsValue::from_serde(&assembler(file)).unwrap()
    }

    /// Returns addresses and values in DRAM. First returned value is a list of
    /// addresses. Second returned value is a list of values corresponding to
    /// the addresses.
    pub fn get_dram(&self) -> JsValue {
        JsValue::from_serde(&self.control_unit.memory.inspect()).unwrap()
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

    /// Returns the status of all pipeline stages.
    pub fn get_pipelines(&mut self) -> JsValue {
        JsValue::from_serde(&self.pipeline_statuses).unwrap()
    }

    /// Returns the control unit cycle count.
    pub fn get_cycle_count(&self) -> u32 {
        self.control_unit.cycle_count
    }

    /// Step through one cycle of processor.
    /// See return value of ControlUnit::step() for details on this methods
    /// return value.
    pub fn step(&mut self) -> Result<JsValue, JsValue> {
        match self.control_unit.step() {
            Err(e) => Err(JsValue::from_serde(&e).unwrap()),
            Ok(done) => {
                self.pipeline_statuses.insert(0, PipelineStatus::new(
                    &self.control_unit));

                Ok(JsValue::from_serde(&done).unwrap())
            }
        }
    }

    /// Steps through processor cycles until the program completes.
    pub fn finish_program(&mut self) -> Result<(), JsValue> {
        let mut program_running = self.control_unit.program_is_running();

        while (program_running) {
            match self.control_unit.step() {
                Err(e) => return Err(JsValue::from_serde(&e).unwrap()),
                Ok(done) => {
                    self.pipeline_statuses.insert(0, PipelineStatus::new(
                        &self.control_unit));

                    program_running = done;
                }
            }
        }

        Ok(())
    }
}

fn main() {}