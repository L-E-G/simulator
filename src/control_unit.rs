use bit_field::BitField;

use std::boxed::Box;
use std::fmt;

use crate::result::SimResult;
use crate::memory::{Memory,DRAM,Registers,PC};
use crate::instructions::{Instruction,InstructionT,MemoryOp,AddrMode,Load,Store};

/// Responsible for running instructions.
pub struct ControlUnit<'a> {
    /// Processor cycle counter.
    cycle_count: u32,
    
    /// Holds computation registers.
    registers: &'a mut Registers,

    /// Memory system.
    memory: &'a mut DRAM,

    /// Instruction currently in pipeline.
    instruction: Option<Box<dyn Instruction>>,
}

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

impl <'a> fmt::Display for ControlUnit<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\
Cycle Count: {}
Registers  :
{}
Memory     :
{}
Instruction: {:?}",
               self.cycle_count,
               indent(format!("{}", self.registers)),
               indent(format!("{}", self.memory)), self.instruction)
    }
}

impl <'a> ControlUnit<'a> {
    /// Creates a new ControlUnit.
    pub fn new(registers: &'a mut Registers, memory: &'a mut DRAM)
               -> ControlUnit<'a> {
        ControlUnit{
            cycle_count: 0,
            registers: registers,
            memory: memory,
            instruction: None,
        }
    }

    /// Step through one cycle of the processor. Stores resulting state in self.
    /// If Result::Ok is returned the value embedded indicates if the program
    /// should keep running. False indicates it should not.
    pub fn step(&mut self) -> Result<bool, String> {
        // Fetch instruction from memory based on current program counter
        match self.memory.get(self.registers[PC]) {
            SimResult::Err(e) => return Err(
                format!("Failed to retrieve instruction from address {}: {}",
                        self.registers[PC], e)),
            SimResult::Wait(wait, ibits) => {
                // End program execution if instruction is 0
                if ibits == 0 {
                    return Ok(false);
                }

                // Figure out which instruction the bits represent by looking at the
                // type and operation code.
                let itype = ibits.get_bits(5..=6) as u32;
                
                let icreate: Result<Box<dyn Instruction>, String> = match InstructionT::match_val(itype) {
                    Some(InstructionT::Memory) => {
                        let iop = ibits.get_bits(7..=9) as u32;

                        match MemoryOp::match_val(iop) {
                            Some(MemoryOp::LoadRD) => Ok(Box::new(
                                Load::new(AddrMode::RegisterDirect))),
                            Some(MemoryOp::LoadI) => Ok(Box::new(
                                Load::new(AddrMode::Immediate))),
                            // TODO: Make Store instruction take AddrMode parameter
                            // TODO: Make seperate branch for StoreRD & StoreI
                            Some(MemoryOp::StoreRD) => Ok(Box::new(Store::new())),
                            _ => Err(format!("Invalid operation code {} for \
                                              mememory type instruction", iop)),
                        }
                    },
                    _ => Err(format!("Invalid type value {} for instruction",
                                     itype)),
                };

                match icreate {
                    Err(e) => return Err(
                        format!("Failed to determine type of instruction for \
                                 bits {}: {}", ibits, e)),
                    Ok(inst) => {
                        self.instruction = Some(inst);
                    },
                };

                // Set state
                self.cycle_count += wait as u32;
                self.registers[PC] += 1;
                return Ok(true);
            },
        };
    }
}
