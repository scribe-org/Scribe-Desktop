use iced::widget::{container, image::Handle, row, text, text_input, Button, Column, Image};
use iced::{
    alignment::Horizontal, executor, window, Alignment, Application, Command, Element, Font,
    Length, Settings, Size, Subscription, Theme,
};
use iced_futures::subscription;
use scribe::CustomTextInput;
use std::io::Read;
use std::net::TcpListener;

#[derive(Debug, Clone)]
pub enum Message {
    KeyReceived(char),
    ToggleListening,
    NoOp,
}

struct Scribe {
    keys: String,
    is_listening: bool,
}

impl Default for Scribe {
    fn default() -> Self {
        Scribe {
            keys: String::new(),
            is_listening: true,
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

    fn theme(&self) -> Self::Theme {
        Theme::Light
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::KeyReceived(char) => {
                if self.is_listening {
                    let keys = &mut self.keys;
                    if char == '\x08' {
                        keys.pop();
                    } else {
                        keys.push(char);
                    }
                }
            }
            Message::ToggleListening => self.is_listening = !self.is_listening,
            Message::NoOp => todo!(),
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::unfold((), self.keys.clone(), |keys| async move {
            let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
            let mut incoming = listener.incoming();

            while let Some(stream) = incoming.next() {
                if let Ok(mut stream) = stream {
                    let mut buffer = [0; 1];
                    if let Ok(_) = stream.read_exact(&mut buffer) {
                        if let Some(received_char) = char::from_u32(buffer[0].into()) {
                            return (Message::KeyReceived(received_char), keys);
                        } else {
                            println!("Received an invalid character");
                            return (Message::NoOp, keys);
                        }
                    }
                }
            }

            (Message::NoOp, keys)
        })
    }

    fn view(&self) -> Element<Message> {
        let logo_data: &[u8] = include_bytes!("../../ScribeBtnPadBlack.png");
        let logo_handle = Handle::from_memory(logo_data.to_vec());
        let logo_button: Image<Handle> = Image::new(logo_handle.clone()).width(50);

        let listening_button = Button::new(logo_button)
            .style(iced::theme::Button::Text)
            .on_press(Message::ToggleListening);

        let text_for_translation = text_input("Your translation here ...", &self.keys.clone())
            .font(Font::DEFAULT)
            .style(iced::theme::TextInput::Custom(Box::new(CustomTextInput {})));

        let title_row = container(row!(text("Welcome to Scribe").size(30)))
            .width(Length::Fill)
            .align_x(Horizontal::Center);
        let content_row =
            row!(listening_button, text_for_translation).align_items(Alignment::Center);

        Column::new()
            .width(Length::Fill)
            .padding(10)
            .spacing(10)
            .push(title_row)
            .push(content_row)
            .into()
    }
}

fn main() -> Result<(), iced::Error> {
    let settings = Settings {
        window: window::Settings {
            size: Size {
                width: 400.0,
                height: 100.0,
            },
            position: window::Position::Centered,
            resizable: true,
            decorations: true,
            ..window::Settings::default()
        },
        ..Settings::default()
    };

    Scribe::run(settings)
}
