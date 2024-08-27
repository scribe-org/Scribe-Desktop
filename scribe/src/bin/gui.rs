use iced::widget::image::Handle;
use iced::widget::{Button, Column, Image, Text};
use iced::window;
use iced::{executor, Alignment, Application, Command, Element, Settings, Subscription, Theme};
use iced_futures::subscription;
use std::io::Read;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub enum Message {
    KeyReceived(char),
    NoOp,
}

struct Scribe {
    keys: Arc<Mutex<String>>,
}

impl Default for Scribe {
    fn default() -> Self {
        Scribe {
            keys: Arc::new(Mutex::new(String::new())),
        }
    }
}

impl Application for Scribe {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Scribe")
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::KeyReceived(char) => {
                let mut keys = self.keys.lock().unwrap();
                if char == '\x08' {
                    keys.pop();
                } else {
                    keys.push(char);
                }
            }
            Message::NoOp => todo!(),
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::unfold((), self.keys.clone(), |keys_arc| async move {
            let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
            let mut incoming = listener.incoming();

            while let Some(stream) = incoming.next() {
                if let Ok(mut stream) = stream {
                    let mut buffer = [0; 1];
                    if let Ok(_) = stream.read_exact(&mut buffer) {
                        if let Some(received_char) = char::from_u32(buffer[0] as u32) {
                            return (Message::KeyReceived(received_char), keys_arc);
                        } else {
                            println!("Received an invalid character");
                            return (Message::NoOp, keys_arc);
                        }
                    }
                }
            }

            (Message::NoOp, keys_arc)
        })
    }

    fn view(&self) -> Element<Message> {
        let logo_handle = Handle::from_path("../../ScribeBtnPadBlack.png");
        let logo: Image<Handle> = Image::new(logo_handle);
        let header = Text::new("Welcome to Scribe").size(30);

        let keys = self.keys.lock().unwrap().clone();
        let text_element = if keys.is_empty() {
            Text::new("Your translation here ...")
        } else {
            Text::new(keys)
        };
        let translate_button = Button::new("Translate");

        Column::new()
            .push(logo)
            .push(header)
            .push(text_element)
            .push(translate_button)
            .align_items(Alignment::Center)
            .into()
    }
}

fn main() -> Result<(), iced::Error> {
    let settings = Settings {
        window: window::Settings {
            size: iced::Size::new(400.0, 400.0),
            position: window::Position::Centered,
            resizable: true,
            decorations: true,
            ..window::Settings::default()
        },
        ..Settings::default()
    };

    Scribe::run(settings)
}
