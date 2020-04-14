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

fn main() {

    print!("Would you like to run Display or Console? (Y/N)");
    io::stdout().flush().expect("failed to flush stdout");
    let answer: String;
    scan!("{}\n", answer);
    
    if(answer == "Y" || answer == "y"){
        Display::run(Settings::default());
    }
}
