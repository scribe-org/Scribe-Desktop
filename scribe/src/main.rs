use iced::alignment::Horizontal;
use iced::executor;
use iced::{
    widget::{Column, Container, Text},
    Application, Command, Element, Length, Settings, Subscription, Theme,
};
use iced_native::keyboard::KeyCode;
use rdev::{EventType, Key};
mod key_utils;
mod keys;

#[derive(Default)]
struct Scribe {
    keys: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Keys(keys::Event),
    PrintKeyPressed(KeyCode),
    InputChanged(String),
}

impl Application for Scribe {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Scribe")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::Keys(event) => match event {
                keys::Event::KeyReceived(key_event) => match key_event.event_type {
                    EventType::KeyRelease(key) => {
                        if key == Key::Backspace {
                            self.keys.pop();
                        } else if let Some(allowed_char) = key_utils::is_allowed_key(&key) {
                            self.keys.push(allowed_char);
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        keys::bind().map(|x| Message::Keys(x.expect("Expected input")))
    }

    fn view(&self) -> Element<'_, Message> {
        Container::new(
            Column::new()
                .push(Text::new("Scribe"))
                .push(Text::new(&self.keys).horizontal_alignment(Horizontal::Center))
                .spacing(20),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .into()
    }
}

fn main() -> Result<(), iced::Error> {
    Scribe::run(Settings::default())
}
