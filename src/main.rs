extern crate text_io;

use text_io::scan;

use iced::{
    executor, Application, Column, Command, Container,
    Element, Length, Settings, Subscription, Text, Row, button, Button,
    scrollable, Scrollable,
};

use std::time::{Duration, Instant};

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

#[derive(Debug, Default)]
struct Display {
    last: Vec<String>,
    enabled: bool,
    button: button::State,
    scroll: scrollable::State,
    prog_ct: u32
}

#[derive(Debug, Clone)]
enum Message {
    Get(Instant),
    Toggled(bool),
    AddEvent(String)
}

impl Application for Display {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Display, Command<Message>) {
        (
            Display {
                last: Vec::new(),
                enabled: true,
                button: button::State::new(),
                scroll: scrollable::State::new(),
                prog_ct: 0,
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Events - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Get(event) => {
                for i in 0..10 {
                    Message::AddEvent(i.to_string());
                }
            }
            Message::Toggled(enabled) => {
                self.enabled = enabled;
            }
            Message::AddEvent(event) => {
                self.last.push(event);
                self.prog_ct+=1;
            }
        };

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        // time::every(Duration::from_millis(10)).map(Message::Get)
        Subscription::none()
    }

    fn view(&mut self) -> Element<Message> {

        let row = Row::new()
            .push(Text::new(format!("Program Counter: {}", self.prog_ct)))
            .spacing(745)
            .push(
                Button::new(&mut self.button, Text::new("Step"))
                    .on_press(Message::AddEvent("Rob was here".to_string())),
            )
            .width(Length::Fill)
            .max_height(40);

        let events = self.last.iter().fold(
            Column::new().spacing(10),
            |column, event| {
                column.push(Text::new(format!("{}", event)).size(20))
            },
        );

        // let toggle = Checkbox::new(
        //     self.enabled,
        //     "Listen to runtime events",
        //     Message::Toggled,
        // );
        let scroll_column = Scrollable::new(&mut self.scroll)
            .push(events)
            .width(Length::Fill)
            .height(Length::Fill);

        let content = Column::new()
            .spacing(10)
            .push(row)
            .spacing(20)
            .push(scroll_column)
            .spacing(10);

        Container::new(content)
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .into()
    }
}

// mod time {
//     use iced::futures;

//     pub fn every(duration: std::time::Duration) -> iced::Subscription<std::time::Instant> {
//         iced::Subscription::from_recipe(Every(duration))
//     }

//     struct Every(std::time::Duration);

//     impl<H, I> iced_native::subscription::Recipe<H, I> for Every
//     where
//         H: std::hash::Hasher,
//     {
//         type Output = std::time::Instant;

//         fn hash(&self, state: &mut H) {
//             use std::hash::Hash;

//             std::any::TypeId::of::<Self>().hash(state);
//             self.0.hash(state);
//         }

//         fn stream(
//             self: Box<Self>,
//             _input: futures::stream::BoxStream<'static, I>,
//         ) -> futures::stream::BoxStream<'static, Self::Output> {
//             // use futures::stream::StreamExt;

//             async_std::stream::interval(self.0)
//                 .map(|_| std::time::Instant::now())
//                 .boxed()
//         }
//     }
// }

fn help() {
    println!("Commands:

- exit(): Exit program
- help(): Show this help text
- get(address): Load address from memory
- set(address, data): Write data to address in memory
- show(level, address): Show an address's cache line, level can be: L1, L2, L3, DRAM. If address is \"_\" the entire level will be shown");
}

fn main() {
    // let dram = Rc::new(RefCell::new(DRAM::new(100)));
    // let l3_cache = Rc::new(RefCell::new(DMCache::new(40, dram.clone())));
    // let l2_cache = Rc::new(RefCell::new(DMCache::new(10, l3_cache.clone())));
    // let l1_cache = Rc::new(RefCell::new(DMCache::new(1, l2_cache.clone())));

    // let memory = &l1_cache;

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
