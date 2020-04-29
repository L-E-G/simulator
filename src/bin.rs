use std::env;

mod result;
mod memory;
mod instructions;
mod control_unit;
pub use crate::result::SimResult;
pub use crate::memory::{Registers,Memory,InspectableMemory,DRAM,DMCache};
pub use crate::instructions::Instruction;
pub use crate::control_unit::ControlUnit;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 2 {
        panic!("Usage: {} [gui]", args[0]);
    }

    // Run GUI
    if args.len() == 2 && args[1] == "gui" {
        panic!("No GUI implemented at the moment");
    } else {
        // Run text interface
        let mut cu = ControlUnit::new();

        match cu.load_memory_from_file("test-data/example-prog.bin") {
            Err(e) => panic!("Failed to load initial control unit memory \
                              from file: {}", e),
            Ok(_v) => (),
        }
      
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
}
