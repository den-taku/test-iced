use iced::{
    button, executor, Align, Application, Button, Clipboard, Column, Command, Element, Settings,
    Text,
};

struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
}

impl Default for Counter {
    fn default() -> Self {
        Self {
            value: 0,
            increment_button: button::State::new(),
            decrement_button: button::State::new(),
        }
    }
}

impl Application for Counter {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("A Simple Counter")
    }

    fn view(&mut self) -> Element<Self::Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.increment_button, Text::new("１増やす"))
                    .on_press(Message::IncrementPressed),
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.decrement_button, Text::new("１減らす"))
                    .on_press(Message::DecrementPressed),
            )
            .into()
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Self::Message> {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
        Command::none()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

fn main() -> iced::Result {
    Counter::run(Settings {
        default_font: Some(include_bytes!(
            "../font/YonagaOldMincho_Version200/YonagaOldMincho-Regular.ttf"
        )),
        ..Settings::default()
    })
}
