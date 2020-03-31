use std::cell::RefCell;
use std::rc::Rc;

use crate::result::SimResult;
use crate::memory::{Memory,Registers};

pub struct load {
    dest: usize,
    addr: u32,
    value: u32,
}

impl load {
    pub fn new() -> load {
        load{
            dest: 0,
            addr: 0,
            value: 0,
        }
    }
}

struct store {
    src_reg: usize,
    addr: u32,
    value: u32,
}

impl store {
    pub fn new() -> store {
        store{
            src_reg: 0,
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

impl Instruction for load {
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
            SimResult::Err(e) => return SimResult::Err(e),
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

impl Instruction for store {
    fn decode_and_fetch(&mut self, instruction: u32, registers: &mut Registers) -> SimResult<(), String> {
        let mut instString: String = instruction.to_string();
        let mut inststr: &str = &instString[..];
        let mut instbin = usize::from_str_radix(inststr, 2).unwrap();

        self.addr = ((instbin << 14) >> 31) as u32;

        self.src_reg = (instbin << 9) >> 31;

        self.value = registers[self.src_reg];
        return SimResult::Wait(0, ());
    }

    fn execute(&mut self) -> SimResult<(), String> {
        return SimResult::Wait(0, ());
    }

    fn access_memory(&mut self, memory: Rc<RefCell<dyn Memory<u32, u32>>>) -> SimResult<(), String> {
        let mut wait = 0;
        match memory.borrow_mut().set(self.addr, self.value) {
            SimResult::Err(e) => return SimResult::Err(e),
            SimResult::Wait(c, _v) => {
                wait += c;

            }
        };
        return SimResult::Wait(wait, ());
    }

    fn write_back(&mut self, registers: &mut Registers) -> SimResult<(), String> {
        return SimResult::Wait(0, ());
    }

}
