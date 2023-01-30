use iced::{
    alignment::{Horizontal, Vertical},
    widget::{column, row, text},
    Alignment, Element, Error, Length, Sandbox, Settings,
};

struct DemoFrame {}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    NULL,
}

impl Sandbox for DemoFrame {
    type Message = Message;

    fn new() -> Self {
        Self {}
    }

    fn title(&self) -> String {
        String::from("Counter Demo")
    }

    fn view(&self) -> Element<Message> {
        //------------------------------------------
        // Display Element Row
        //------------------------------------------
        column![
            text::Text::new("Data Display")
                .size(40)
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Center),
            row![
                text::Text::new("Display\nElement")
                    .size(30)
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Center),
                text::Text::new("Display\nElement")
                    .size(30)
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Center),
                text::Text::new("Display\nElement")
                    .size(30)
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Center),
            ]
            .align_items(Alignment::Center)
            .padding(20)
            .spacing(10)
            .width(Length::Fill),
            //------------------------------------------
            // Input Element Row
            //------------------------------------------
            text::Text::new("Inputs")
                .size(40)
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Center),
            row![
                text::Text::new("Input\nElement")
                    .size(30)
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Center),
                text::Text::new("Input\nElement")
                    .size(30)
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Center),
                text::Text::new("Input\nElement")
                    .size(30)
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Center),
            ]
            .align_items(Alignment::Center)
            .padding(20)
            .spacing(10)
            .width(Length::Fill),
            //------------------------------------------
            // Plot Row
            //------------------------------------------
            text::Text::new("Plot")
                .size(40)
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Center),
            text::Text::new("Plot")
                .size(120)
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center)
        ]
        .padding(20)
        .spacing(10)
        .height(Length::Fill)
        .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::NULL => {
                println!("Hello")
            }
        };
    }
}

fn main() -> Result<(), Error> {
    DemoFrame::run(Settings::default())
}
