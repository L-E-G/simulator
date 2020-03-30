use std::cell::RefCell;
use std::rc::Rc;

use crate::result::SimResult;
use crate::memory::{Memory,Registers};

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
