/* SPDX-License-Identifier: GPL-3.0-or-later */
use iced::widget::{image::Handle, text_input, Button, Column, Container, Image, Row};
use iced::{
    executor, window, Alignment, Application, Command, Element, Font, Length, Settings, Size,
    Subscription, Theme,
};
use iced_futures::subscription;
use scribe::styles::ButtonStyle;
use scribe::styles::CustomTextInput;
use scribe::AppState;
use std::io::Read;
use std::net::TcpListener;

#[derive(Debug, Clone)]
pub enum Message {
    KeyReceived(char),
    ToggleListening,
    ToggleTooltips,
    Translate,
    Conjugate,
    Plural,
    ToggleTheme,
    NoOp,
}

struct Scribe {
    keys: String,
    is_listening: bool,
    tool_tips: bool,
    state: AppState,
    theme: Theme,
    manual_override: bool,
}

impl Default for Scribe {
    fn default() -> Self {
        let detected_theme = detect_system_theme();
        let is_dark = matches!(detected_theme, Theme::Dark);
        println!("Initial system theme detected: {:?}", detected_theme);
        Scribe {
            keys: String::new(),
            is_listening: true,
            tool_tips: false,
            state: AppState {
                is_dark_theme: is_dark,
                ..AppState::default()
            },
            theme: detected_theme.clone(),
            manual_override: false,
        }
    }
}

fn detect_system_theme() -> Theme {
    let mode = dark_light::detect();
    match mode {
        dark_light::Mode::Dark => Theme::Dark,
        dark_light::Mode::Light => Theme::Light,
        dark_light::Mode::Default => Theme::Light,
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
        self.theme.clone()
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
            Message::ToggleTooltips => {
                self.tool_tips = !self.tool_tips;
                let height = if self.tool_tips { 92.0 } else { 50.0 };
                return window::resize(
                    window::Id::MAIN,
                    Size {
                        width: 440.0,
                        height,
                    },
                );
            }
            Message::Translate => println!("Translate"),
            Message::Conjugate => println!("Conjugate"),
            Message::Plural => println!("Plural"),
            Message::ToggleTheme => {
                self.manual_override = !self.manual_override;
                self.state.toggle_theme();
                self.theme = if self.state.is_dark_theme {
                    Theme::Dark
                } else {
                    Theme::Light
                };
                println!(
                    "Manual theme toggle - override: {}, theme: {:?}",
                    self.manual_override, self.theme
                );
            }
            Message::NoOp => {}
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
        let is_dark = self.state.is_dark_theme;
        let button_width = Length::Fixed(80.0);

        let logo_data: &[u8] = if is_dark {
            include_bytes!("../../icons/ScribeIconWhite.png")
        } else {
            include_bytes!("../../icons/ScribeIconBlack.png")
        };

        let logo: Image<Handle> = Image::new(Handle::from_memory(logo_data.to_vec())).width(50);

        let text_for_translation = text_input("Enter text for command...", &self.keys)
            .font(Font::DEFAULT)
            .style(iced::theme::TextInput::Custom(Box::new(CustomTextInput {
                state: self.state,
            })));

        let toggle_button = Button::new(logo)
            .on_press(Message::ToggleTooltips)
            .style(iced::theme::Button::Custom(Box::new(ButtonStyle {
                state: self.state,
            })))
            .width(Length::Fixed(60.0))
            .height(Length::Fixed(30.0));

        let mut input_and_buttons = Column::new().spacing(10).width(Length::Fill);
        input_and_buttons = input_and_buttons.push(text_for_translation);

        if self.tool_tips {
            let button_row = Row::new()
                .spacing(10)
                .align_items(Alignment::Center)
                .width(Length::Fill)
                .push(
                    Button::new(
                        Container::new("Translate").width(Length::Fill).center_x(), // or .align_x(iced::Alignment::Center)
                    )
                    .on_press(Message::Translate)
                    .style(iced::theme::Button::Custom(Box::new(ButtonStyle {
                        state: self.state,
                    })))
                    .width(button_width),
                )
                .push(
                    Button::new(
                        Container::new("Conjugate").width(Length::Fill).center_x(), // or .align_x(iced::Alignment::Center)
                    )
                    .on_press(Message::Conjugate)
                    .style(iced::theme::Button::Custom(Box::new(ButtonStyle {
                        state: self.state,
                    })))
                    .width(button_width),
                )
                .push(
                    Button::new(
                        Container::new("Plural").width(Length::Fill).center_x(), // or .align_x(iced::Alignment::Center)
                    )
                    .on_press(Message::Plural)
                    .style(iced::theme::Button::Custom(Box::new(ButtonStyle {
                        state: self.state,
                    })))
                    .width(button_width),
                )
                .push(
                    Button::new(
                        Container::new(if self.manual_override {
                            "Theme (Manual)"
                        } else {
                            "Theme (Auto)"
                        })
                        .width(Length::Fill)
                        .center_x(),
                    )
                    .on_press(Message::ToggleTheme)
                    .style(iced::theme::Button::Custom(Box::new(ButtonStyle {
                        state: self.state,
                    })))
                    .width(button_width),
                );

            input_and_buttons = input_and_buttons.push(button_row);
        }

        let top_column = Column::new()
            .spacing(10)
            .align_items(Alignment::Start)
            .width(Length::Shrink)
            .push(toggle_button);

        let top_row = Row::new()
            .spacing(10)
            .align_items(Alignment::Start)
            .push(top_column)
            .push(input_and_buttons);

        let layout = Column::new()
            .width(Length::Shrink)
            .spacing(10)
            .align_items(Alignment::Center)
            .push(top_row);

        Container::new(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .style(move |_theme: &Theme| {
                let background_color = if is_dark {
                    iced::Color::from_rgb8(0x1E, 0x1E, 0x1E) // Dark mode background
                } else {
                    iced::Color::from_rgb8(0xCE, 0xD2, 0xD9) // Light mode background
                };

                iced::widget::container::Appearance {
                    background: Some(background_color.into()),
                    ..Default::default()
                }
            })
            .into()
    }
}

fn main() -> Result<(), iced::Error> {
    let settings = Settings {
        window: window::Settings {
            min_size: Some(Size {
                width: 400.0,
                height: 55.0,
            }),
            size: Size {
                width: 400.0,
                height: 0.0,
            },
            position: window::Position::Centered,
            resizable: false,
            decorations: true,
            ..window::Settings::default()
        },
        ..Settings::default()
    };

    Scribe::run(settings)
}
