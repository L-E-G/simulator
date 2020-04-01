use iced::{button, Align, Button, Column, Element, Sandbox, Settings, Text, Container};

#[derive(Default)]
pub struct Counter {
    value: u64,
    increment_button: button::State,
    decrement_button: button::State,
    word: [String; 4],
    index: usize,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Counter {
            value: 0,
            increment_button: button::State::new(),
            decrement_button: button::State::new(),
            word: ["".to_string(), "".to_string(), "".to_string(), "".to_string()],
            index: 0,
        }
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.word[self.index] = "Rob".to_string();
                self.index+=1;
            }
            Message::DecrementPressed => {
                self.value = 76;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {

        Column::new()
            .push(
                Button::new(&mut self.increment_button, Text::new("Increment"))
                    .on_press(Message::IncrementPressed),
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                    .on_press(Message::DecrementPressed),
            )
            .push(Text::new(self.word[0].to_string()))
            .push(Text::new(self.word[1].to_string()))
            .push(Text::new(self.word[2].to_string()))
            .push(Text::new(self.word[3].to_string()))
            .into()
    }
}