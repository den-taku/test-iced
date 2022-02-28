// for WindowsOS
#![windows_subsystem = "windows"]

use file_rotate::{
    compression::Compression, suffix::AppendTimestamp, suffix::FileLimit, ContentLimit, FileRotate,
};
use iced::{
    button, executor, window, Align, Application, Button, Clipboard, Color, Column, Command,
    Element, Settings, Text,
};
use simplelog::*;

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
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.increment_button, Text::new("１増やす"))
                    .on_press(Message::IncrementPressed),
            )
            .push(Text::new(fizz_buzz(self.value)).size(250))
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

    fn mode(&self) -> window::Mode {
        window::Mode::Windowed
    }

    fn background_color(&self) -> Color {
        Color::from_rgb8(30, 43, 120)
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::time::every(std::time::Duration::from_millis(500)).map(|_| Message::IncrementPressed)
    }
}

fn fizz_buzz(number: i32) -> String {
    if number % 15 == 0 {
        "FizzBuzz".to_string()
    } else if number % 5 == 0 {
        "Buzz".to_string()
    } else if number % 3 == 0 {
        "Fizz".to_string()
    } else {
        number.to_string()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

fn main() -> iced::Result {
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Debug,
        Config::default(),
        FileRotate::new(
            "target/trace_log",
            AppendTimestamp::default(FileLimit::MaxFiles(10)),
            ContentLimit::Lines(100_000),
            Compression::None,
        ),
    )])
    .unwrap();
    Counter::run(Settings {
        default_font: Some(include_bytes!(
            // Install from here https://booth.pm/ja/items/3489185
            // and unzip in test-iced/font
            "../font/YonagaOldMincho_Version200/YonagaOldMincho-Regular.ttf"
        )),
        ..Settings::default()
    })
}
