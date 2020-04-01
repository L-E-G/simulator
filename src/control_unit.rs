use std::cell::RefCell;
use std::rc::Rc;
use std::boxed::Box;

use crate::result::SimResult;
use crate::instructions::{Instruction,Noop,Load,Store};
use crate::memory::{Memory,DRAM,DMCache,Registers,PC};

/// Responsible for running instructions.
pub struct ControlUnit {
    /// Holds computation registers
    registers: Registers,

    dram: Rc<RefCell<DRAM>>,
}

impl ControlUnit {
    /// Creates a new ControlUnit.
    pub fn new() -> ControlUnit {
        let dram = Rc::new(RefCell::new(DRAM::new(100)));
                
        /*
        let l3_cache = Rc::new(RefCell::new(DMCache::new(40, dram.clone())));
        let l2_cache = Rc::new(RefCell::new(DMCache::new(10, l3_cache.clone())));
        let l1_cache = Rc::new(RefCell::new(DMCache::new(1, l2_cache.clone())));
         */
        
        let registers = Registers::new();
        
        ControlUnit{
            registers: registers,
            dram: dram,
        }
    }

    /// Run pipeline by loading in DRAM file and starting with the program counter
    /// at 0. Executes instructions via the pipeline until a memory address with
    /// 0 is encountered.
    pub fn run(self, dram_f: &str) -> Result<(), String> {
        // Load DRAM
        match self.dram.borrow_mut().load_from_file(dram_f) {
            Ok(_v) => (),
            Err(e) => {
                return Err(format!("Failed to load DRAM from file: {}", e));
            },
        };

        // Execute instructions
        let mut last_fetch = false;
        let mut fetch_res: SimResult<u32, String> = SimResult::Wait(0, 0);

        let mut last_decode = false;
        let mut decode_inst: Rc<RefCell<Box<dyn Instruction>>> = Rc::new(
            RefCell::new(Box::new(Noop::new())));
        let mut decode_res: SimResult<(), String> = SimResult::Wait(0, ());

        let mut last_execute = false;
            let mut execute_inst: Rc<RefCell<Box<dyn Instruction>>> = Rc::new(
                RefCell::new(Box::new(Noop::new())));
        let mut execute_res: SimResult<(), String> = SimResult::Wait(0, ());

        let mut last_mem_access = false;
            let mut mem_access_inst: Rc<RefCell<Box<dyn Instruction>>> = Rc::new(
                RefCell::new(Box::new(Noop::new())));
        let mut mem_access_res: SimResult<(), String> = SimResult::Wait(0, ());

            let mut write_inst: Rc<RefCell<Box<dyn Instruction>>> = Rc::new(
                RefCell::new(Box::new(Noop::new())));
        let mut write_res: SimResult<(), String> = SimResult::Wait(0, ());
        
        loop {
            // Write back stage
            match write_res {
                SimResult::Err(e) => {
                    return Err(format!("Failed to run write back stage: {}", e));
                },
                SimResult::Wait(wait, _v) => {
                    // If still in progress
                    if wait > 0 {
                        write_res = SimResult::Wait(wait-1, ()); // Keep waiting
                    } else if last_mem_access {
                        // If last instruction done in the memory access stage
                        return Ok(()); // Finish run
                    } else if mem_access_res.ready() {
                        // Run next instruction if prev. stage is ready
                        write_inst = mem_access_inst;
                        write_res = write_inst.borrow_mut()
                            .write_back(&mut self.registers);
                    } else {
                        // Blocked by prev. stage, wait
                        write_res = SimResult::Wait(0, ());
                    }
                },
            };

            // Memory access stage
            match mem_access_res {
                SimResult::Err(e) => {
                    return Err(format!("Failed to run memory access \
                                        stage: {}", e));
                },
                SimResult::Wait(wait, _v) => {
                    // If still in progress
                    if wait > 0 {
                        // Keep waiting
                        mem_access_res = SimResult::Wait(wait-1, ());
                    } else if last_execute {
                        // If last instruction done in the execute stage
                        last_mem_access = true;
                        mem_access_res = SimResult::Wait(0, ());
                    } else if execute_res.ready() {
                        // Run next instruction if prev. stage is ready
                        mem_access_inst = execute_inst;
                        mem_access_res = mem_access_inst.borrow_mut()
                            .access_memory(self.dram);
                    } else {
                        // Blocked by prev. stage, wait
                        mem_access_res = SimResult::Wait(0, ());
                    }
                },
            };

            // Execute stage
            match execute_res {
                SimResult::Err(e) => {
                    return Err(format!("Failed to run execute stage: {}", e));
                },
                SimResult::Wait(wait, _v) => {
                    // If still in progress
                    if wait > 0 {
                        execute_res = SimResult::Wait(wait-1, ()); // Keep waiting
                    } else if last_decode {
                        // If last instruction done in decode stage
                        last_execute = true;
                        execute_res = SimResult::Wait(0, ());
                    } else if decode_res.ready() {
                        // Run next instruction if prev. stage is ready
                        execute_inst = decode_inst;
                        execute_res = execute_inst.borrow_mut().execute();
                    } else {
                        // Blocked by prev. stage, wait
                        execute_res = SimResult::Wait(0, ());
                    }
                },
            };

            // Decode stage
            match decode_res {
                SimResult::Err(e) => {
                    return Err(format!("Failed to run decode stage: {}", e));
                },
                SimResult::Wait(wait, _v) => {
                    // If still in progress
                    if wait > 0 {
                        decode_res = SimResult::Wait(wait-1, ()); // Keep waiting
                    } else if last_fetch {
                        // If last instruction done in fetch stage
                        last_decode = true;
                        decode_res = SimResult::Wait(0, ());
                    } else if fetch_res.ready() {
                        // Run next instruction if prev. stage is ready
                        
                        // This unwrap should never panic b/c fetch_res.ready()
                        // will not return true unless fetch_res
                        // is SimResult::Wait.
                        let (_fetch_wait, instruction_bits) = fetch_res
                            .unwrap("Failed to unwrap instruction bits,\
                                     this should not panic! See comment in code");
                        
                        match self.build_instruction(instruction_bits) {
                            Err(e) => {
                                return Err(format!("Failed to build \
                                                    instruction: {}", e));
                            },
                            Ok(inst) => {
                                decode_inst = Rc::new(RefCell::new(inst));
                                decode_res = decode_inst.borrow_mut()
                                    .decode_and_fetch(instruction_bits,
                                                      &self.registers);
                            },
                        };
                    } else {
                        // Blocked by prev. stage, wait
                        decode_res = SimResult::Wait(0, ());
                    }
                },
            };
            
            // Fetch stage
            match fetch_res {
                SimResult::Err(e) => {
                    return Err(format!("Failed to run fetch stage: {}", e));
                },
                SimResult::Wait(wait, instruction_bits) => {
                    // If still in progress
                    if wait > 0 {
                        fetch_res = SimResult::Wait(wait-1, instruction_bits);
                    } else if instruction_bits == 0 {
                        // Check if last instruction
                        last_fetch = true;
                        fetch_res = SimResult::Wait(0, 0);
                    } else {
                        // If done, get new instruction's bits
                        fetch_res = self.dram.borrow_mut().get(self.registers[PC]);
                    }
                },
            };
        }
    }

    /// Builds an empty instruction based on the type and operation code fields
    /// of instruction bits.
    fn build_instruction(self, instruction_bits: u32) -> Result<Box<dyn Instruction>, String> {
        let itype = (instruction_bits << 25) >> 30;

        match itype {
            1 => {
                let iop = (instruction_bits << 23) >> 30;
                match iop {
                    0 => Ok(Box::new(Load::new())),
                    1 => Ok(Box::new(Store::new())),
                    _ => {
                        Err(format!("Unknown memory instruction operation \
                                     code: {}", iop))
                    },
                }
            },
            _ => {
                Err(format!("Unknown instruction type field {}", itype))
            },
        }
    }
}
