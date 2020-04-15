mod result;
mod memory;
mod instructions;
mod control_unit;
pub use crate::result::SimResult;
pub use crate::memory::{Registers,Memory,InspectableMemory,DRAM,DMCache};
pub use crate::instructions::Instruction;
pub use crate::control_unit::ControlUnit;

fn main() {
    /*
    let dram = Rc::new(RefCell::new(DRAM::new(100)));
    let l3_cache = Rc::new(RefCell::new(DMCache::new(40, dram.clone())));
    let l2_cache = Rc::new(RefCell::new(DMCache::new(10, l3_cache.clone())));
    let l1_cache = Rc::new(RefCell::new(DMCache::new(1, l2_cache.clone())));
     */
    // Run pipeline
    let mut cu = ControlUnit::new("test-data/example-prog.bin");
    let mut program_running = true;

    while program_running {
        println!("====================");
        match cu.step() {
            Err(e) => panic!("Failed to run processor cycle: {}", e),
            Ok(keep_running) => program_running = keep_running,
        };

        if program_running {
            println!("{}", cu);
        } else {
            println!("Program ended");
        }
    }
}
