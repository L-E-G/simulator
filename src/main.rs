extern crate ux;
extern crate text_io;
extern crate bincode;
extern crate serde;

use std::path::Path;
use std::io;
use std::fs::{File, OpenOptions, remove_file};
use std::io::{Read, Write, BufReader, BufRead, LineWriter};
use std::process::exit;
use std::str::FromStr;

use ux::{u22};
use text_io::scan;

mod memory;


/// Indicates the status of a simulator operation with either a value, error, or
/// result which will be available after a delay. D is the data type, E is the
/// error type.
enum SimResult<D, E> {
    /// Value if operation was successful.
    Ok(D),

    /// Error if operation failed.
    Err(E),

    /// Indicates result is not available yet. First field is the number of
    /// simulator cycles before the value will be ready. The result will be
    /// available during the simulator cycle in which this field reaches 0. The
    /// second field is the result. This result can be OK, Err, or even Wait.
    Wait(u16, D),
}

impl<D, E> SimResult<D, E> {
    /// Decrements the cycle count in a Wait. If this field equals 1 after
    /// decrementing then the contained result is returned. The contained result is
    /// returned when the count equals 1, not 0, because this method is expected to
    /// be called once every cycle. The value which is returned should be processed
    /// in the next cycle, when the count would equal 0.
    ///
    /// Otherwise a Wait with a decremented cycle count is returned. If the
    /// SimResult is Ok or Err just returns, method should not be used on these.
    fn process(self) -> SimResult<D, E> {
        match self {
            SimResult::Ok(v) => SimResult::Ok(v),
            SimResult::Err(e) => SimResult::Err(e),
            SimResult::Wait(i, res) => {
                if i <= 2 {
                    return SimResult::Ok(res);
                }

                SimResult::Wait(i-1, res)
            },
        }
    }
}

fn help() {
    println!("Commands:

- exit(): Exit program
- help(): Show this help text
- get(address): Load address from memory
- set(address, data): Write data to address in memory
- show(level, address): Show an address's cache line, level can be: L1, L2, L3, DRAM");
}

fn main() {
    // memory::cache::newDram(100);
    // memory::cache::newCache(4);
    memory::cache::newMemory(100, 4);

    
    
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
                match memory::cache::get(address) {
                    memory::cache::SimResult::Ok(v) => {
                        println!("Completed in 0 cycles");
                        println!("{}: {}", address, v);
                    },
                    memory::cache::SimResult::Err(e) => eprintln!("Failed to get {}: {}", address, e),
                    memory::cache::SimResult::Wait(c, v) => {
                        println!("Completed in {} cycles", c);
                        println!("{}: {}", address, v);
                    }
                };
            },
            "set" => {
                // Parse operands
                let address: u32;
                let data: u32;
                scan!(operands.bytes() => "{}, {}", address, data);

                // Perform operation
                match memory::cache::set(address, data) {
                    memory::cache::SimResult::Ok(_v) => {
                        println!("Completed in 0 cycles");
                    },
                    memory::cache::SimResult::Err(e) => eprintln!("Failed to set {}: {}", address, e),
                    memory::cache::SimResult::Wait(c, _v) => {
                        println!("Completed in {} cycles", c);
                    }
                };
            },
            "show" => {
                // Parse operands
                let level: String;
                let address: u32;
                scan!(operands.bytes() => "{}, {}", level, address);

                // TODO: Check level
                // TODO: Display
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