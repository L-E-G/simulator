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

// #[derive(Default)]
struct Display {
    dram: Rc<RefCell<DRAM>>,
    prog_ct: u32,
    button: button::State,
    word: [String; 9],
    instructions: [&'static str; 9],
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
            dram: Rc::new(RefCell::new(DRAM::new(100))),
            prog_ct: 0,
            button: button::State::new(),
            // load 1 10: 0000001000000101010000000000000, decimal: 16949248
            // store 1 14: 0000001010000101110000000000000, decimal: 21159936
            // move 3 1: 00000000110010001100001000000000, decimal: 13156864
            // add 4 3 0x5: 00000000001000010000011000000101, decimal: 2164229
            word: ["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()],
            instructions: [
                "ldr 1 10 0000001000000101010000000000000\nStage: fetch_and_decode\nDram addr# 10: 43 Reg #1: 0\n", 
                "ldr 1 10 0000001000000101010000000000000\nStage: execute\nstr 1 14 0000001010000101110000000000000\nStage: fetch_and_decode\nDram addr# 14: 0, Dram addr# 10: 43 Reg #1: 0\n",
                "ldr 1 10 0000001000000101010000000000000\nStage: memory_access\nstr 1 14 0000001010000101110000000000000\nStage: Blocked\nmv 3 1 00000000110010001100001000000000\nStage: Blocked\nDram addr# 14: 0, Dram addr# 10: 43 Reg #1: 0\n", 
                "ldr 1 10 0000001000000101010000000000000\nStage: write_back\nstr 1 14 0000001010000101110000000000000\nStage: execute\nmv 3 1 00000000110010001100001000000000\nStage: fetch_and_decode\nDram addr# 14: 0, Dram addr# 10: 43 Reg #1: 43\n", 
                "str 1 14 0000001010000101110000000000000\nStage: memory_access\nmv 3 1 00000000110010001100001000000000\nStage: execute\naddImm 4 3 0x5 00000000001000010000011000000101\nStage: fetch_and_decode\nDram addr# 14: 43, Dram addr# 10: 43 Reg #1: 43, #3: 0\n",
                "str 1 14 0000001010000101110000000000000\nStage: write_back\nmv 3 1 00000000110010001100001000000000\nStage: memory_access\naddImm 4 3 0x5 00000000001000010000011000000101\nStage: Blocked\nDram addr# 14: 43, Dram addr# 10: 43 Reg #1: 43, #3: 0\n",
                "mv 3 1 00000000110010001100001000000000\nStage: write_back\naddImm 4 3 0x5 00000000001000010000011000000101\nStage: execute\nDram addr# 14: 43, Dram addr# 10: 43 Reg #1: 43, #3: 43\n",
                "addImm 4 3 0x5 00000000001000010000011000000101\nStage: memory_access\nDram addr# 14: 43, Dram addr# 10: 43 Reg #1: 43, #3: 43\n",
                "addImm 4 3 0x5 00000000001000010000011000000101\nStage: write_back\nDram addr# 14: 43, Dram addr# 10: 43 Reg #1: 43, #3: 43, #4: 48\n",
            ],
            index: 0,
        }
    }

    fn title(&self) -> String {
        String::from("Simulator")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Pressed => {
                if self.index <= 9{
                    // self.word[self.index] = format!("{}: {}",self.assembly[self.index], self.instructions[self.index].to_string());
                    self.word[self.index] = self.instructions[self.index].to_string();
                    // let mut inst: u32 = 0;
                    // match self.instructions[self.index].parse::<u32>() {
                    //     Result::Err(e) => {},       // I know I know, this is not how we fail gracefully...
                    //     Result::Ok(f) => inst = f,
                    // }
                    // pipeline(inst);   Can use this for pipeline call

                    // self.index+=1;
                    // match self.dram.borrow().inspect_txt() {
                    //     Err(e) => println!("Failed to get stuff from DRAM per error {}", e),
                    //     Ok(d) => self.word[self.index] = d,
                    // }
                    if self.index == 2 || self.index == 4{
                        self.prog_ct += 100;
                    }else{
                        self.prog_ct+=1;
                    }
                      //This can eventually reference he program counter from the pipeline file
                    self.index+=1;
                }
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
            .push(Text::new(self.word[4].to_string()))
            .push(Text::new(self.word[5].to_string()))
            .push(Text::new(self.word[6].to_string()))
            .push(Text::new(self.word[7].to_string()))
            .push(Text::new(self.word[8].to_string()))
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

    // help();

    // loop {
    //     print!("> ");
    //     io::stdout().flush().expect("failed to flush stdout");
        
    //     let cmd: String;
    //     let operands: String;
    //     scan!("{}({})\n", cmd, operands);

    //     match cmd.as_str() {
    //         "get" => {
    //             // Parse operands
    //             let address: u32;
    //             scan!(operands.bytes() => "{}", address);

    //             // Perform operation
    //             match memory.borrow_mut().get(address) {
    //                 SimResult::Err(e) => eprintln!("Failed to get {}: {}",
    //                                                address, e),
    //                 SimResult::Wait(c, v) => {
    //                     println!("Completed in {} cycles", c);
    //                     println!("{}: {:?}", address, v);
    //                 }
    //             };
    //         },
    //         "set" => {
    //             // Parse operands
    //             let address: u32;
    //             let data: u32;
    //             scan!(operands.bytes() => "{}, {}", address, data);

    //             // Perform operation
    //             match memory.borrow_mut().set(address, data) {
    //                 SimResult::Err(e) => eprintln!("Failed to set {}: {}",
    //                                                address, e),
    //                 SimResult::Wait(c, _v) => {
    //                     println!("Completed in {} cycles", c);
    //                 }
    //             };
    //         },
    //         "show" => {
    //             // Parse operands
    //             let level: String;
    //             let address_str: String;
    //             scan!(operands.bytes() => "{}, {}", level, address_str);

    //             let inspect_res = match address_str.as_str() {
    //                 "_" => {
    //                     match level.as_str() {
    //                         "L1" => l1_cache.borrow().inspect_txt(),
    //                         "L2" => l2_cache.borrow().inspect_txt(),
    //                         "L3" => l3_cache.borrow().inspect_txt(),
    //                         "DRAM" => dram.borrow().inspect_txt(),
    //                         _ => Err(format!("Cache level name \"{}\" not \
    //                             recognized", level)),
    //                     }
    //                 },
    //                 _ => {
    //                     match address_str.parse() {
    //                         Err(e) => Err(format!("Failed to parse address argument {} as u32: {}", address_str, e)),
    //                         Ok(address) => {
    //                             match level.as_str() {
    //                                 "L1" => l1_cache.borrow()
    //                                     .inspect_address_txt(address),
    //                                 "L2" => l2_cache.borrow()
    //                                     .inspect_address_txt(address),
    //                                 "L3" => l3_cache.borrow()
    //                                     .inspect_address_txt(address),
    //                                 "DRAM" => dram.borrow()
    //                                     .inspect_address_txt(address),
    //                                 _ => Err(format!("Cache level name \"{}\" not \
    //                                                   recognized", level)),
    //                             }
    //                         }
    //                     }
    //                 },
    //             };

    //             match inspect_res {
    //                 Ok(txt) => {
    //                     println!("{} at {}", level, address_str);
    //                     println!("{}", txt);
    //                 },
    //                 Err(e) => {
    //                     eprintln!("Failed to inspect {} at {}: {}", level,
    //                               address_str, e);
    //                 }
    //             };
    //         },
    //         "help" => help(),
    //         "exit" => {
    //             exit(0);
    //         },
    //         _ => {
    //             eprintln!("Invalid command: {}", cmd);
    //             eprintln!("Use help() command to see valid commands");
    //         }
    //     }
    // }
}
