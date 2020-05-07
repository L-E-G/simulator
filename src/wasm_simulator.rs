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
use std::io::{Cursor,BufReader};
use std::fmt::Debug;
use std::str::from_utf8;
use std::cell::RefCell;
use std::rc::Rc;

mod result;
mod memory;
mod instructions;
mod control_unit;
mod assembler;
use crate::control_unit::ControlUnit;
use crate::result::SimResult;
use crate::memory::{DMCache,DRAM,Memory,InspectableMemory};
use crate::assembler::Assembler;

/// Run configuration which determines how programs run in the simulator.
#[derive(Serialize,Deserialize)]
pub struct RunConfig {
    /// Indicates if a pipeline should be used.
    pipeline_enabled: bool,

    /// Indicates if the cache should be used.
    cache_enabled: bool,
}

/// All caches information.
#[derive(Serialize,Deserialize)]
pub struct CachesInfo {
    l1: CacheInfo,
    l2: CacheInfo,
    l3: CacheInfo,
}

/// A single cache's values and aliases.
#[derive(Serialize,Deserialize)]
pub struct CacheInfo {
    data: HashMap<u32, u32>,
    aliases: HashMap<u32, String>,
}

/// Interface between JavaScript and all simulator functionality.
/// The run configuration should be set before step() is ever called.
#[wasm_bindgen]
pub struct Simulator {
    l1_cache: Rc<RefCell<DMCache>>,
    l2_cache: Rc<RefCell<DMCache>>,
    l3_cache: Rc<RefCell<DMCache>>,
    dram: Rc<RefCell<DRAM>>,
    control_unit: ControlUnit,
    assembler: Assembler,
    
    /// Status of pipeline during each step. New steps added to end of vector.
    /// If the pipeline is enabled inner vector holds a representation of each
    /// pipeline stage starting with fetch at index 0 and ending with write back
    /// at index 4. If the pipeline is not enabled the inner vector holds one
    /// element which represents the last instruction run.
    pipeline_statuses: Vec<Vec<Option<String>>>,
}

#[wasm_bindgen]
impl Simulator {
    /// Initializes simulator
    #[wasm_bindgen(constructor)]
    pub fn new() -> Simulator {
        // Setup panic logging hook
        console_error_panic_hook::set_once();

        let dram = Rc::new(RefCell::new(DRAM::new(100)));
        let l3_cache = Rc::new(RefCell::new(
            DMCache::new(40, 512, dram.clone())
        ));
        let l2_cache = Rc::new(RefCell::new(
            DMCache::new(10, 32, l3_cache.clone())
        ));
        let l1_cache = Rc::new(RefCell::new(
            DMCache::new(1, 16, l2_cache.clone())
        ));

        Simulator{
            l1_cache: l1_cache.clone(),
            l2_cache: l2_cache.clone(),
            l3_cache: l3_cache.clone(),
            dram: dram.clone(),
            control_unit: ControlUnit::new(dram.clone(), l1_cache.clone()),
            assembler: Assembler::new(),
            pipeline_statuses: vec![],
        }
    }

    /// Returns a pipeline
    fn mk_pipeline_statuses(&self) -> Vec<Option<String>> {
        if self.control_unit.pipeline_enabled {
            let mut status = vec![];

            if let Some(i) = &self.control_unit.fetch_instruction {
                status.push(Some(format!("{}", i)));
            } else {
                status.push(None);
            }

            if let Some(i) = &self.control_unit.decode_instruction {
                status.push(Some(format!("{}", i)));
            } else {
                status.push(None);
            }

            if let Some(i) = &self.control_unit.execute_instruction {
                status.push(Some(format!("{}", i)));
            } else {
                status.push(None);
            }

            if let Some(i) = &self.control_unit.access_mem_instruction {
                status.push(Some(format!("{}", i)));
            } else {
                status.push(None);
            }

            if let Some(i) = &self.control_unit.write_back_instruction {
                status.push(Some(format!("{}", i)));
            } else {
                status.push(None);
            }

            status
        } else {
            if let Some(i) = &self.control_unit.no_pipeline_instruction {
                vec![Some(format!("{}", i))]
            } else {
                vec![None]
            }
        }
    }

        /// Assemble input and set DRAM to the resulting binary.
    pub fn set_dram_assembled(&mut self, input: &str) -> Result<(), JsValue> {
        let bin = match self.assembler.assemble(input.as_bytes()) {
            Err(e) => return Err(JsValue::from_serde(
                &format!("failed to assemble input: {}", e)).unwrap()),
            Ok(v) => v,
        };

        match self.dram.borrow_mut().load_from_reader(&mut Cursor::new(bin)) {
            Err(e) => Err(JsValue::from_serde(
                &format!("failed to load input into DRAM: {}", e)).unwrap()),
            Ok(_v) => Ok(()),
        }
    }

    /// Returns the control unit's run configuration.
    pub fn get_run_config(&self) -> JsValue {
        let run_config = RunConfig {
            pipeline_enabled: self.control_unit.pipeline_enabled,
            cache_enabled: self.control_unit.cache_enabled,
        };
        
        JsValue::from_serde(&run_config).unwrap()
    }

    /// Sets the control unit's run configuration.
    pub fn set_run_config(&mut self, raw_run_config: JsValue) {
        let run_config: RunConfig = raw_run_config.into_serde().unwrap();

        self.control_unit.pipeline_enabled = run_config.pipeline_enabled;
        self.control_unit.cache_enabled = run_config.cache_enabled;
    }

    /// Returns the address and values in the cache.
    pub fn get_cache(&self) -> JsValue {
        JsValue::from_serde(&CachesInfo{
            l1: CacheInfo{
                data: self.l1_cache.borrow().inspect_valid(),
                aliases: self.l1_cache.borrow().inspect_valid_aliases(),
            },
            l2: CacheInfo{
                data: self.l2_cache.borrow().inspect_valid(),
                aliases: self.l2_cache.borrow().inspect_valid_aliases(),
            },
            l3: CacheInfo{
                data: self.l3_cache.borrow().inspect_valid(),
                aliases: self.l3_cache.borrow().inspect_valid_aliases(),
            },
        }).unwrap()
    }

    /// Returns addresses and values in DRAM. 
    pub fn get_dram(&self) -> JsValue {
        JsValue::from_serde(&self.dram.borrow().inspect()).unwrap()
    }

    /// Sets the contents of DRAM based on binary input.
    /// See DRAM::load_from_reader() for details on the required format of
    /// the input.
    pub fn set_dram(&mut self, input: &[u8]) -> Result<(), JsValue> {
        match self.dram.borrow_mut().load_from_reader(input) {
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
                self.pipeline_statuses.insert(0, self.mk_pipeline_statuses());

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
                    self.pipeline_statuses.insert(0, self.mk_pipeline_statuses());

                    program_running = done;
                }
            }
        }

        Ok(())
    }
}
