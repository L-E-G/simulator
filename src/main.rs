use std::env;

mod result;
mod memory;
mod instructions;
mod gui;
mod control_unit;
pub use crate::result::SimResult;
pub use crate::memory::{Memory,InspectableMemory,DRAM,DMCache};
pub use crate::instructions::Instruction;
pub use crate::gui::Display;
pub use crate::control_unit::ControlUnit;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 2 {
        panic!("Usage: {} [gui]", args[0]);
    }

    if args.len() == 2 && args[1] == "gui" {
        Display::start();
    }
}
