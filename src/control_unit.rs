use bit_field::BitField;

use std::boxed::Box;
use std::fmt;

use crate::result::SimResult;
use crate::memory::{Memory,DRAM,Registers,PC};
use crate::instructions::{Instruction,InstructionT,MemoryOp,AddrMode,Load,Store};

/// Responsible for running instructions.
pub struct ControlUnit {
    /// Processor cycle counter.
    cycle_count: u32,
    
    /// Holds computation registers.
    registers: Registers,

    /// Memory system.
    memory: DRAM,

    /// Instruction which resulted from the fetch stage of the pipeline.
    fetch_instruction: Option<u32>,

    /// Instruction currently in the decode stage of the pipeline.
    decode_instruction: Option<Box<dyn Instruction>>,

    /// Instruction currently in the execute stage of the pipeline.
    execute_instruction: Option<Box<dyn Instruction>>,

    /// Instruction currently in the access memory stage of the pipeline.
    access_mem_instruction: Option<Box<dyn Instruction>>,

    /// Instruction currently in the write back stage of the pipeline.
    write_back_instruction: Option<Box<dyn Instruction>>,
}

/// Prepends 4 spaces to every line.
fn indent(src: String) -> String {
    let mut out = String::new();

    let mut i = 0;
    for line in src.lines() {
        out.push_str("    ");
        out.push_str(line);

        if i + 1 != src.lines().count() {
            out.push_str("\n");
        }
        
        i += 1;
    }

    out
}

impl fmt::Display for ControlUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\
Cycle Count: {}
Registers  :
{}
Memory     :
{}
Instructions:
    Fetch        : {:?}
    Decode       : {:?}
    Execute      : {:?}
    Access Memory: {:?}
    Write Back   : {:?}",
               self.cycle_count,
               indent(format!("{}", self.registers)),
               indent(format!("{}", self.memory)),
               self.fetch_instruction, self.decode_instruction,
               self.execute_instruction, self.access_mem_instruction,
               self.write_back_instruction)
    }
}

impl ControlUnit {
    /// Creates a new ControlUnit.
    pub fn new(dram_f: &str) -> ControlUnit {
        ControlUnit{
            cycle_count: 0,
            registers: Registers::new(),
            memory: DRAM::new(100, dram_f),
            fetch_instruction: None,            
            decode_instruction: None,
            execute_instruction: None,
            access_mem_instruction: None,
            write_back_instruction: None,
        }
    }

    /// Step through one cycle of the processor. Stores resulting state in self.
    /// If Result::Ok is returned the value embedded indicates if the program
    /// should keep running. False indicates it should not.
    pub fn step(&mut self) -> Result<bool, String> {
        //  Write back stage
        match &mut self.access_mem_instruction {
            None => self.write_back_instruction = None,
            Some(access_mem_inst) => {
                match access_mem_inst.write_back(&mut self.registers) {
                    SimResult::Err(e) => return Err(
                        format!("Failed to write back for instruction: {}",
                                e)),
                    SimResult::Wait(wait, _v) => {
                        // Update state
                        self.cycle_count += wait as u32;
                    },
                };

                self.write_back_instruction = self.access_mem_instruction.take();
            },
        }
        
        // Access memory stage
        match &mut self.execute_instruction {
            None => self.access_mem_instruction = None,
            Some(exec_inst) => {
                match exec_inst.access_memory(&mut self.memory) {
                    SimResult::Err(e) => return Err(
                        format!("Failed to access memory for instruction: {}",
                                e)),
                    SimResult::Wait(wait, _v) => {
                        // Update state
                        self.cycle_count += wait as u32;
                    },
                };

                self.access_mem_instruction = self.execute_instruction.take();
            },
        };
        
        // Execute stage
        match &mut self.decode_instruction {
            None => self.execute_instruction = None,
            Some(decode_inst) => {
                match decode_inst.execute() {
                    SimResult::Err(e) => return Err(format!("Failed to execute instruction: {}", e)),
                    SimResult::Wait(wait, _v) => {
                        // Update state
                        self.cycle_count += wait as u32;
                    },
                };

                self.execute_instruction = self.decode_instruction.take();
            },
        };

        // Decode stage
        match self.fetch_instruction {
            None => self.decode_instruction = None,
            Some(fetch_inst) => {
                // Figure out which instruction the bits represent by
                // looking at the type and operation code.
                let itype = fetch_inst.get_bits(5..=6) as u32;
                
                let icreate: Result<Box<dyn Instruction>, String> = 
                    match InstructionT::match_val(itype) {
                    Some(InstructionT::Memory) => {
                        let iop = fetch_inst.get_bits(7..=9) as u32;

                        match MemoryOp::match_val(iop) {
                            Some(MemoryOp::LoadRD) => Ok(Box::new(
                                Load::new(AddrMode::RegisterDirect))),
                            Some(MemoryOp::LoadI) => Ok(Box::new(
                                Load::new(AddrMode::Immediate))),
                            // TODO: Make Store instruction take AddrMode parameter
                            // TODO: Make seperate branch for StoreRD & StoreI
                            Some(MemoryOp::StoreRD) => Ok(Box::new(
                                Store::new())),
                            _ => Err(format!("Invalid operation code {} for \
                                              mememory type instruction", iop)),
                        }
                    },
                        _ => Err(format!("Invalid type value {} for instruction",
                                         itype)),
                };

                // Run instruction specific decode
                self.decode_instruction = match icreate {
                    Err(e) => return Err(format!("Failed to determine type of \
                                                  instruction for bits {}: {}",
                                                 fetch_inst, e)),
                    Ok(mut inst_box) => match (*inst_box).decode(fetch_inst,
                                                                 &self.registers) {
                        SimResult::Err(e) => return Err(
                            format!("Failed to decode instruction {}: {}",
                                    fetch_inst, e)),
                        SimResult::Wait(wait, _v) => {
                            // Update state
                            self.cycle_count += wait as u32;

                            Some(inst_box)
                        },
                    },
                };
            },
        };
        
        // Fetch stage
        match self.memory.get(self.registers[PC]) {
            SimResult::Err(e) => return Err(
                format!("Failed to retrieve instruction from address {}: {}",
                        self.registers[PC], e)),
            SimResult::Wait(wait, ibits) => {
                // End program execution if instruction is 0
                if ibits == 0 {
                    self.fetch_instruction = None;
                } else {
                    self.fetch_instruction = Some(ibits);
                }

                // Set state
                self.cycle_count += wait as u32;
            },
        };

        // Update state after all stages
        self.registers[PC] += 1;

        // Determine if program should continue running
            Ok(self.decode_instruction.is_some() ||
               self.fetch_instruction.is_some() ||
               self.execute_instruction.is_some() ||
               self.access_mem_instruction.is_some())
    }
}
