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

//------------------- second shot at it -----------------------


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
    Split(pane_grid::Axis, pane_grid::Pane, u32),
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
                self.word[self.index] = self.instructions[self.index].to_string();
                // pipeline(var);   Can use this for pipeline call
                
                self.prog_ct+=1;
                self.index+=1;
            }

            Message::Split(axis, pane, data) => {
                let _ = self.panes.split(
                    axis,
                    &pane,
                    Content::new(data),
                );
            }
        }
    }

    fn view(&mut self) -> Element<Message> {

        let row = Row::new()
            .push(Text::new("Program counter: 32"))
            .spacing(745)
            .push(
                Button::new(&mut self.button, Text::new("Increment")),
            )
            .width(Length::Fill)
            .max_height(40);

        let column = Column::new()
            .push(Text::new("Rob is cool"));
        
        let scroll_column = Scrollable::new(&mut self.scroll)
            .push(column);
        
        let window = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(row)
            .spacing(30)
            .push(scroll_column);

        let pane_grid =
            PaneGrid::new(&mut self.panes, |pane, content, focus| {
                content.view(pane, focus)
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(10);

        Container::new(window) // Change this to pane_grid for a look at what I made with the panes
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .into()
    }
}

struct Content{
    prog_ct: u32,
    button: button::State,
}

impl Content{
    fn new(pc: u32) -> Self{
        Content{
            prog_ct: pc,
            button: button::State::new(),
        }
    }
    fn view(&mut self, pane: pane_grid::Pane, focus: Option<pane_grid::Focus>) -> Element<Message>{

        let button = Button::new(
                    &mut self.button,
                    Text::new("Split")
                        .width(Length::Fill)
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .size(16),
                )
                .width(Length::Fill)
                .padding(8)
                .on_press(Message::Split(pane_grid::Axis::Horizontal, pane, 12))
                .style(style::Button::Primary);

        let column = Column::new()
            .width(Length::Fill)
            .spacing(10)
            .align_items(Align::Center)
            .push(Text::new(self.prog_ct.to_string()))
            .push(button);
        
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