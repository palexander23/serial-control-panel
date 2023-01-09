use iced::{
    widget::{button, column, row, text},
    Alignment, Element, Error, Sandbox, Settings,
};

struct Counter {
    // The counter value
    value: i32,
    numbers: Vec<i32>,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self {
            value: 0,
            numbers: vec![],
        }
    }

    fn title(&self) -> String {
        String::from("Counter Demo")
    }

    fn view(&self) -> Element<Message> {
        // Control panel for the counter
        let counter_controls: Element<_> = row![
            // The decrement button. We tell it to produce a
            // `DecrementPressed` message when pressed
            button("-").on_press(Message::DecrementPressed),
            // We show the value of the counter here
            text(self.value).size(50),
            // The increment button. We tell it to produce an
            // `IncrementPressed` message when pressed
            button("+").on_press(Message::IncrementPressed),
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into();

        // A dynamic chunk showing how the UI can be changed on the fly
        let dynamic_numbers: Element<_> = column(
            self.numbers
                .iter()
                .map(|i| text(i).size(50).into())
                .collect(),
        )
        .spacing(20)
        .into();

        // A final layout for the UI
        // This is returned as the view
        column![counter_controls, dynamic_numbers]
            .spacing(20)
            .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
                self.numbers.push(self.value);
            }
            Message::DecrementPressed => {
                self.value -= 1;
                self.numbers.pop();
            }
        }
    }
}

fn main() -> Result<(), Error> {
    Counter::run(Settings::default())
}
