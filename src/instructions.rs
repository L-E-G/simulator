use std::cell::RefCell;
use std::rc::Rc;

use crate::result::SimResult;
use crate::memory::{Memory,Registers};

pub struct Load {
    dest: usize,
    addr: u32,
    value: u32,
}

impl Load {
    pub fn new() -> Load {
        Load{
            dest: 0,
            addr: 0,
            value: 0,
        }
    }
}

/// Defines operations which a single instruction must perform while it is in
/// the pipeline.
pub trait Instruction {
    /// Extracts parameters from instruction bits and stores them in the
    /// implementing struct for use by future stages. It also retrieves register
    /// values if necessary and does the same.
    fn decode_and_fetch(&mut self, instruction: u32, registers: &mut Registers) -> SimResult<(), String>;

    /// Executes the instruction.
    fn execute(&mut self) -> SimResult<(), String>;

    /// Accesses memory.
    fn access_memory(&mut self, memory: Rc<RefCell<dyn Memory<u32, u32>>>) -> SimResult<(), String>;

    /// Write results to registers.
    fn write_back(&mut self, registers: &mut Registers) -> SimResult<(), String>;
}

impl Instruction for Load {
    fn decode_and_fetch(&mut self, instruction: u32, registers: &mut Registers) -> SimResult<(), String> {
        let mut instString: String = instruction.to_string();
        let mut inststr: &str = &instString[..];
        let mut instbin = usize::from_str_radix(inststr, 2).unwrap();

        self.addr = ((instbin << 14) >> 31) as u32;

        self.dest = (instbin << 9) >> 31;
        return SimResult::Wait(0, ());
    }

    fn execute(&mut self) -> SimResult<(), String> {
        return SimResult::Wait(0, ());
    }

    fn access_memory(&mut self, memory: Rc<RefCell<dyn Memory<u32, u32>>>) -> SimResult<(), String> {
        let mut wait = 0;
        match memory.borrow_mut().get(self.addr) {
            SimResult::Err(e) => eprintln!("Failed to get {}: {}", self.addr, e),
            SimResult::Wait(c, v) => {
                self.value = v;
                wait += c;
            }
        };
        return SimResult::Wait(wait, ());
    }

    fn write_back(&mut self, registers: &mut Registers) -> SimResult<(), String> {
        registers[self.dest] = self.value;
        return SimResult::Wait(0, ());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    /// Ensures that the load instruction functions correctly.
    #[test]
    fn test_load_instruction() {
        // Pack instruction bits
        // dest = 10100 = R20
        // addr = 00110 = R6
        const DEST_REG_IDX: usize = 20;
        const ADDR_REG_IDX: usize = 6;
        const instruction: u32 = ((ADDR_REG_IDX as u32) << 14) |
        ((DEST_REG_IDX as u32) << 9);

        // Setup registers
        let mut regs = Registers::new();

        const DEST_VAL: u32 = 444;
        const ADDR_VAL: u32 = 777;
        regs[DEST_REG_IDX] = DEST_VAL;
        regs[ADDR_REG_IDX] = ADDR_VAL;

        // Decode and fetch
        let mut load_instruction = Load::new();
        assert_eq!(load_instruction.decode_and_fetch(instruction, &mut regs),
                   SimResult::Wait(0, ()), "decode_and_fetch result");
        assert_eq!(load_instruction.dest, DEST_VAL as usize, "instruction.dest");
        assert_eq!(load_instruction.addr, ADDR_VAL, "instruction.addr");
    }
}
