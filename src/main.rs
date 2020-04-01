extern crate text_io;

use text_io::scan;

use iced::{button, Align, Button, Column, Element, Sandbox, Settings, Text, Container, Row};

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

#[derive(Default)]
struct Display {
    prog_ct: u32,
    button: button::State,
    word: [String; 5],
    instructions: [u32; 5],
    index: usize,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Pressed,
}

impl Sandbox for Display {
    type Message = Message;

    fn new() -> Self {
        Display {
            prog_ct: 0,
            button: button::State::new(),
            // load 1 10, store 1 14, move 3 1, add 4 3 0x5  ...   0000001000000101010000000000000, 
            // 0000001010000101110000000000000, 00000000110010001100001000000000, 00000000001000010000011000000101
            word: ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()],
            instructions: [16949248, 21159936, 13156864, 2164229, 0],
            index: 0,
        }
    }

    fn title(&self) -> String {
        String::from("Simulator")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Pressed => {
                self.word[self.index] = format!("{:b}", self.instructions[self.index]);
                self.prog_ct+=1;
                self.index+=1;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {

        Column::new()
            .push(Text::new(self.prog_ct.to_string()).size(50))
            .push(
                Button::new(&mut self.button, Text::new("Next Instruction"))
                    .on_press(Message::Pressed),
            )
            .push(Text::new(self.word[0].to_string()))
            .push(Text::new(self.word[1].to_string()))
            .push(Text::new(self.word[2].to_string()))
            .push(Text::new(self.word[3].to_string()))
            .into()
    }
}


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

    Display::run(Settings::default());

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
