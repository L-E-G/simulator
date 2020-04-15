mod result;
mod memory;
mod instructions;
mod control_unit;
pub use crate::result::SimResult;
pub use crate::memory::{Registers,Memory,InspectableMemory,DRAM,DMCache};
pub use crate::instructions::Instruction;
pub use crate::control_unit::ControlUnit;

fn main() {
    // Run pipeline
    let mut cu = ControlUnit::new("test-data/example-prog.bin");
    let mut program_running = true;

    while program_running {
        println!("====================");
        match cu.step() {
            Err(e) => panic!("Failed to run processor cycle: {}", e),
            Ok(keep_running) => program_running = keep_running,
        };

        println!("{}", cu);
        if !program_running {
            println!("Program ended");
        }
    }
}
