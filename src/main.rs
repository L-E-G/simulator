extern crate text_io;

use text_io::scan;

use iced::{button, Align, Button, Column, Element, Sandbox, Settings, 
    Text, Container, Row, Scrollable, scrollable, PaneGrid, pane_grid, 
    Length, keyboard, HorizontalAlignment,
};

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
    scroll: scrollable::State,
    panes: pane_grid::State<Content>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Pressed,
    Split(pane_grid::Axis, pane_grid::Pane),
}

impl Sandbox for Display {
    type Message = Message;

    fn new() -> Self {
        let (panes, _) = pane_grid::State::new(Content::new(32));

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
            scroll: scrollable::State::new(),
            panes,
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
            Message::Split(axis, pane) => {
                let _ = self.panes.split(
                    axis,
                    &pane,
                    Content::new(12),
                );
            }
        }

        
    }

    fn view(&mut self) -> Element<Message> {

        let pane_grid =
            PaneGrid::new(&mut self.panes, |pane, content, focus| {
                content.view(pane, focus)
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(10);

        Container::new(pane_grid)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .into()
    }
}

struct Content{
    prog_ct: u32,
}

impl Content{
    fn new(pc: u32) -> Self{
        Content{
            prog_ct: pc,
        }
    }
    fn view(&mut self, pane: pane_grid::Pane, focus: Option<pane_grid::Focus>) -> Element<Message>{

        Message::Split(pane_grid::Axis::Vertical, pane);

        let column = Column::new()
            .push(Text::new(self.prog_ct.to_string()).size(20))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Align::Center)
            .padding(5);
        
        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .center_y()
            .style(style::Pane {
                is_focused: focus.is_some(),
            })
            .into()
    }
}

mod style {
    use iced::{button, container, Background, Color, Vector};

    const SURFACE: Color = Color::from_rgb(
        0xF2 as f32 / 255.0,
        0xF3 as f32 / 255.0,
        0xF5 as f32 / 255.0,
    );

    const ACTIVE: Color = Color::from_rgb(
        0x72 as f32 / 255.0,
        0x89 as f32 / 255.0,
        0xDA as f32 / 255.0,
    );

    const HOVERED: Color = Color::from_rgb(
        0x67 as f32 / 255.0,
        0x7B as f32 / 255.0,
        0xC4 as f32 / 255.0,
    );

    pub struct Pane {
        pub is_focused: bool,
    }

    impl container::StyleSheet for Pane {
        fn style(&self) -> container::Style {
            container::Style {
                background: Some(Background::Color(SURFACE)),
                border_width: 2,
                border_color: Color {
                    a: if self.is_focused { 1.0 } else { 0.3 },
                    ..Color::BLACK
                },
                ..Default::default()
            }
        }
    }

    pub enum Button {
        Primary,
        Destructive,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            let (background, text_color) = match self {
                Button::Primary => (Some(ACTIVE), Color::WHITE),
                Button::Destructive => {
                    (None, Color::from_rgb8(0xFF, 0x47, 0x47))
                }
            };

            button::Style {
                text_color,
                background: background.map(Background::Color),
                border_radius: 5,
                shadow_offset: Vector::new(0.0, 0.0),
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            let active = self.active();

            let background = match self {
                Button::Primary => Some(HOVERED),
                Button::Destructive => Some(Color {
                    a: 0.2,
                    ..active.text_color
                }),
            };

            button::Style {
                background: background.map(Background::Color),
                ..active
            }
        }
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
