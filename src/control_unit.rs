use bit_field::BitField;

use std::boxed::Box;
use std::fmt;

use crate::result::SimResult;
use crate::memory::{Memory,DRAM,Registers,PC};
use crate::instructions::{Instruction,InstructionT,
    MemoryOp,AddrMode,Load,Store,
    ArithMode,ALUOp,Move,ArithI,Comp,
    AS,LS,LogicType,ThreeOpLogic,Not,
};

/// Responsible for running instructions.
pub struct ControlUnit {
    /// Processor cycle counter.
    pub cycle_count: u32,
    
    /// Holds computation registers.
    pub registers: Registers,

    /// Memory system.
    pub memory: DRAM,

    /// Instruction which resulted from the fetch stage of the pipeline.
    pub fetch_instruction: Option<u32>,

    /// Instruction currently in the decode stage of the pipeline.
    pub decode_instruction: Option<Box<dyn Instruction>>,

    /// Instruction currently in the execute stage of the pipeline.
    pub execute_instruction: Option<Box<dyn Instruction>>,

    /// Instruction currently in the access memory stage of the pipeline.
    pub access_mem_instruction: Option<Box<dyn Instruction>>,

    /// Instruction currently in the write back stage of the pipeline.
    pub write_back_instruction: Option<Box<dyn Instruction>>,
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
    pub fn new() -> ControlUnit {
        ControlUnit{
            cycle_count: 0,
            registers: Registers::new(),
            memory: DRAM::new(100),
            fetch_instruction: None,            
            decode_instruction: None,
            execute_instruction: None,
            access_mem_instruction: None,
            write_back_instruction: None,
        }
    }

    /// Loads a memory file into the control unit's memory. See
    /// DRAM::load_from_file() for details on the expected file structure.
    pub fn load_memory_from_file(&mut self, f: &str) -> Result<(), String> {
        self.memory.load_from_file(f)
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
                            Some(MemoryOp::StoreRD) => Ok(Box::new(
                                Store::new(AddrMode::RegisterDirect))),
                            Some(MemoryOp::StoreI) => Ok(Box::new(
                                Store::new(AddrMode::Immediate))),
                            _ => Err(format!("Invalid operation code {} for \
                                              mememory type instruction", iop)),
                        }
                    },
                    // Immediates:
                    // Unsigned = false
                    // Signed = true
                    Some(InstructionT::ALU) => {
                        let iop = fetch_inst.get_bits(7..=11) as u32;

                        match ALUOp::match_val(iop) {    // Don't quite know how to add sign/unsign
                            Some(ALUOp::Move) => Ok(Box::new(
                                Move::new())),
                            // ---- Add ----
                            Some(ALUOp::AddUIRD) => Ok(Box::new(
                                ArithI::new(AddrMode::RegisterDirect, ArithMode::Add))),
                            Some(ALUOp::AddUII) => Ok(Box::new(
                                ArithI::new(AddrMode::Immediate, ArithMode::Add))),
                            Some(ALUOp::AddSIRD) => Ok(Box::new(
                                ArithI::new(AddrMode::RegisterDirect, ArithMode::Add))),
                            Some(ALUOp::AddSII) => Ok(Box::new(
                                ArithI::new(AddrMode::Immediate, ArithMode::Add))),
                            // ---- Sub ----
                            Some(ALUOp::SubUIRD) => Ok(Box::new(
                                ArithI::new(AddrMode::RegisterDirect, ArithMode::Sub))),
                            Some(ALUOp::SubUII) => Ok(Box::new(
                                ArithI::new(AddrMode::Immediate, ArithMode::Sub))),
                            Some(ALUOp::SubSIRD) => Ok(Box::new(
                                ArithI::new(AddrMode::RegisterDirect, ArithMode::Sub))),
                            Some(ALUOp::SubSII) => Ok(Box::new(
                                ArithI::new(AddrMode::Immediate, ArithMode::Sub))),
                            // ---- Mul ----
                            Some(ALUOp::MulUIRD) => Ok(Box::new(
                                ArithI::new(AddrMode::RegisterDirect, ArithMode::Mul))),
                            Some(ALUOp::MulUII) => Ok(Box::new(
                                ArithI::new(AddrMode::Immediate, ArithMode::Mul))),
                            Some(ALUOp::MulSIRD) => Ok(Box::new(
                                ArithI::new(AddrMode::RegisterDirect, ArithMode::Mul))),
                            Some(ALUOp::MulSII) => Ok(Box::new(
                                ArithI::new(AddrMode::Immediate, ArithMode::Mul))),
                            // ---- Div ----
                            Some(ALUOp::DivUIRD) => Ok(Box::new(
                                ArithI::new(AddrMode::RegisterDirect, ArithMode::Div))),
                            Some(ALUOp::DivUII) => Ok(Box::new(
                                ArithI::new(AddrMode::Immediate, ArithMode::Div))),
                            Some(ALUOp::DivSIRD) => Ok(Box::new(
                                ArithI::new(AddrMode::RegisterDirect, ArithMode::Div))),
                            Some(ALUOp::DivSII) => Ok(Box::new(
                                ArithI::new(AddrMode::Immediate, ArithMode::Div))),
                            // ---- Comp ----
                            Some(ALUOp::CompUI) => Ok(Box::new(
                                Comp::new(false))),
                            Some(ALUOp::CompSI) => Ok(Box::new(
                                Comp::new(true))),
                            // ---- Arithmetic Shift ----
                            Some(ALUOp::ASLRD) => Ok(Box::new(
                                AS::new(AddrMode::RegisterDirect, false))),
                            Some(ALUOp::ASLI) => Ok(Box::new(
                                AS::new(AddrMode::Immediate, false))),
                            Some(ALUOp::ASRRD) => Ok(Box::new(
                                AS::new(AddrMode::RegisterDirect, true))),
                            Some(ALUOp::ASRI) => Ok(Box::new(
                                AS::new(AddrMode::Immediate, true))),
                            // ---- Logical Shift ----
                            Some(ALUOp::LSLRD) => Ok(Box::new(
                                LS::new(AddrMode::RegisterDirect, false))),
                            Some(ALUOp::LSLI) => Ok(Box::new(
                                LS::new(AddrMode::Immediate, false))),
                            Some(ALUOp::LSRRD) => Ok(Box::new(
                                LS::new(AddrMode::RegisterDirect, true))),
                            Some(ALUOp::LSRI) => Ok(Box::new(
                                LS::new(AddrMode::Immediate, true))),
                            // ---- 3 Operation Logic ----
                            Some(ALUOp::AndRD) => Ok(Box::new(
                                ThreeOpLogic::new(AddrMode::RegisterDirect, LogicType::And))),
                            Some(ALUOp::AndI) => Ok(Box::new(
                                ThreeOpLogic::new(AddrMode::Immediate, LogicType::And))),
                            Some(ALUOp::OrRD) => Ok(Box::new(
                                ThreeOpLogic::new(AddrMode::RegisterDirect, LogicType::Or))),
                            Some(ALUOp::OrI) => Ok(Box::new(
                                ThreeOpLogic::new(AddrMode::Immediate, LogicType::Or))),
                            Some(ALUOp::XorRD) => Ok(Box::new(
                                ThreeOpLogic::new(AddrMode::RegisterDirect, LogicType::Xor))),
                            Some(ALUOp::XorI) => Ok(Box::new(
                                ThreeOpLogic::new(AddrMode::Immediate, LogicType::Xor))),
                            // ---- Not ----
                            Some(ALUOp::Not) => Ok(Box::new(
                                Not::new())),
                            
                            _ => Err(format!("Invalid operation code {} for \
                                ALU type instruction", iop)),
                        }
                    }
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
