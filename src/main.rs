extern crate text_io;

use text_io::scan;

use std::process::exit;
use std::cell::RefCell;
use std::rc::Rc;
use std::io;
use std::io::Write;

mod result;
mod memory;
mod instructions;
pub use crate::result::SimResult;
pub use crate::memory::{Memory,InspectableMemory,DRAM,DMCache};
pub use crate::instructions::Instruction;

fn help() {
    println!("Commands:

- exit(): Exit program
- help(): Show this help text
- get(address): Load address from memory
- set(address, data): Write data to address in memory
- show(level, address): Show an address's cache line, level can be: L1, L2, L3, DRAM. If address is \"_\" the entire level will be shown");
}

fn main() {
    let dram = Rc::new(RefCell::new(DRAM::new(100)));
    let l3_cache = Rc::new(RefCell::new(DMCache::new(40, dram.clone())));
    let l2_cache = Rc::new(RefCell::new(DMCache::new(10, l3_cache.clone())));
    let l1_cache = Rc::new(RefCell::new(DMCache::new(1, l2_cache.clone())));

    let memory = &l1_cache;

    help();

    loop {
        print!("> ");
        io::stdout().flush().expect("failed to flush stdout");
        
        let cmd: String;
        let operands: String;
        scan!("{}({})\n", cmd, operands);

        match cmd.as_str() {
            "get" => {
                // Parse operands
                let address: u32;
                scan!(operands.bytes() => "{}", address);

                // Perform operation
                match memory.borrow_mut().get(address) {
                    SimResult::Err(e) => eprintln!("Failed to get {}: {}",
                                                   address, e),
                    SimResult::Wait(c, v) => {
                        println!("Completed in {} cycles", c);
                        println!("{}: {:?}", address, v);
                    }
                };
            },
            "set" => {
                // Parse operands
                let address: u32;
                let data: u32;
                scan!(operands.bytes() => "{}, {}", address, data);

                // Perform operation
                match memory.borrow_mut().set(address, data) {
                    SimResult::Err(e) => eprintln!("Failed to set {}: {}",
                                                   address, e),
                    SimResult::Wait(c, _v) => {
                        println!("Completed in {} cycles", c);
                    }
                };
            },
            "show" => {
                // Parse operands
                let level: String;
                let address_str: String;
                scan!(operands.bytes() => "{}, {}", level, address_str);

                let inspect_res = match address_str.as_str() {
                    "_" => {
                        match level.as_str() {
                            "L1" => l1_cache.borrow().inspect_txt(),
                            "L2" => l2_cache.borrow().inspect_txt(),
                            "L3" => l3_cache.borrow().inspect_txt(),
                            "DRAM" => dram.borrow().inspect_txt(),
                            _ => Err(format!("Cache level name \"{}\" not \
                                recognized", level)),
                        }
                    },
                    _ => {
                        match address_str.parse() {
                            Err(e) => Err(format!("Failed to parse address argument {} as u32: {}", address_str, e)),
                            Ok(address) => {
                                match level.as_str() {
                                    "L1" => l1_cache.borrow()
                                        .inspect_address_txt(address),
                                    "L2" => l2_cache.borrow()
                                        .inspect_address_txt(address),
                                    "L3" => l3_cache.borrow()
                                        .inspect_address_txt(address),
                                    "DRAM" => dram.borrow()
                                        .inspect_address_txt(address),
                                    _ => Err(format!("Cache level name \"{}\" not \
                                                      recognized", level)),
                                }
                            }
                        }
                    },
                };

                match inspect_res {
                    Ok(txt) => {
                        println!("{} at {}", level, address_str);
                        println!("{}", txt);
                    },
                    Err(e) => {
                        eprintln!("Failed to inspect {} at {}: {}", level,
                                  address_str, e);
                    }
                };
            },
            "help" => help(),
            "exit" => {
                exit(0);
            },
            _ => {
                eprintln!("Invalid command: {}", cmd);
                eprintln!("Use help() command to see valid commands");
            }
        }
    }
}
