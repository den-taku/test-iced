use iced::{button, Button, Column, Text};

struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
}

impl Counter {
    pub fn view(&mut self) -> Column<Message> {
        Column::new()
            .push(
                Button::new(&mut self.increment_button, Text::new("+"))
                    .on_press(Message::IncrementPressed),
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.decrement_button, Text::new("-"))
                    .on_press(Message::DecrementPressed),
            )
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

fn main() {
    println!("Hello, world!");
}
