/* SPDX-License-Identifier: GPL-3.0-or-later */
use iced::futures::SinkExt;
use iced::stream;
use iced::widget::{button, image::Handle, text_input, Button, Column, Container, Image, Row};
use iced::{window, Alignment, Element, Font, Length, Size, Subscription, Task, Theme};
use scribe::AppState;
use std::io::Read;
use std::net::TcpListener;

#[derive(Debug, Clone)]
pub enum Message {
    KeyReceived(char),
    TextInputChanged(String),
    ToggleMenu,
    Settings,
    Translate,
    Conjugate,
    Plural,
    ToggleTheme,
    NoOp,
}

fn detect_system_theme() -> Theme {
    match dark_light::detect() {
        dark_light::Mode::Dark => Theme::Dark,
        _ => Theme::Light,
    }
}

struct Scribe {
    keys: String,
    is_executing_command: bool,
    show_menu: bool,
    state: AppState,
    theme: Theme,
}

impl Default for Scribe {
    fn default() -> Self {
        let detected_theme = detect_system_theme();
        let is_dark = matches!(detected_theme, Theme::Dark);
        Scribe {
            keys: String::new(),
            is_executing_command: false,
            show_menu: false,
            state: AppState {
                is_dark_theme: is_dark,
                ..AppState::default()
            },
            theme: detected_theme,
        }
    }
}

impl Scribe {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::KeyReceived(char) => {
                if self.is_executing_command {
                    if char == '\x08' {
                        self.keys.pop();
                    } else {
                        self.keys.push(char);
                    }
                }
            }
            Message::ToggleMenu => {
                self.show_menu = !self.show_menu;
                self.is_executing_command = false;
                let new_height = if self.show_menu { 94.0 } else { 52.0 };
                return window::get_latest()
                    .and_then(move |id| window::resize(id, Size::new(626.0, new_height)));
            }
            Message::Settings => {
                println!("Settings clicked");
            }
            Message::Translate => {
                self.is_executing_command = true;
                println!("Translate");
            }
            Message::Conjugate => {
                self.is_executing_command = true;
                println!("Conjugate");
            }
            Message::Plural => {
                self.is_executing_command = true;
                println!("Plural");
            }
            Message::TextInputChanged(new_text) => {
                self.keys = new_text;
            }
            Message::ToggleTheme => {
                self.state.toggle_theme();
                println!("Theme toggled! is_dark_theme: {}", self.state.is_dark_theme);

                // Update self.theme to the custom theme
                self.theme = if self.state.is_dark_theme {
                    Theme::custom(
                        "Dark Custom".to_string(),
                        iced::theme::Palette {
                            background: iced::Color::from_rgb8(0x1E, 0x1E, 0x1E),
                            text: iced::Color::WHITE,
                            primary: iced::Color::from_rgb8(0x3E, 0x95, 0xCC),
                            success: iced::Color::from_rgb8(0x12, 0x66, 0x4f),
                            danger: iced::Color::from_rgb8(0xc3, 0x42, 0x3f),
                        },
                    )
                } else {
                    Theme::custom(
                        "Light Custom".to_string(),
                        iced::theme::Palette {
                            background: iced::Color::from_rgb8(0xCE, 0xD2, 0xD9),
                            text: iced::Color::BLACK,
                            primary: iced::Color::from_rgb8(0x4C, 0xAD, 0xE6),
                            success: iced::Color::from_rgb8(0x12, 0x66, 0x4f),
                            danger: iced::Color::from_rgb8(0xc3, 0x42, 0x3f),
                        },
                    )
                };
            }
            Message::NoOp => {}
        }
        Task::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::run(|| {
            stream::channel(100, |mut output| async move {
                loop {
                    match TcpListener::bind("127.0.0.1:7878") {
                        Ok(listener) => {
                            for stream in listener.incoming() {
                                if let Ok(mut stream) = stream {
                                    let mut buffer = [0; 1];
                                    if stream.read_exact(&mut buffer).is_ok() {
                                        if let Some(received_char) =
                                            char::from_u32(buffer[0].into())
                                        {
                                            let _ = output
                                                .send(Message::KeyReceived(received_char))
                                                .await;
                                        }
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                        }
                    }
                }
            })
        })
    }

    fn view(&self) -> Element<'_, Message> {
        let is_dark = self.state.is_dark_theme;
        const ICON_SIZE: u16 = 25;
        const SCRIBE_ICON_SIZE: u16 = 20;
        const BUTTON_ICON_SIZE: u16 = 22;

        // MARK: Icons

        let (
            scribe_logo_data,
            close_icon_data,
            settings_icon_data,
            translate_icon_data,
            conjugate_icon_data,
            plural_icon_data,
        ) = if is_dark {
            (
                include_bytes!("../../icons/ScribeIconWhite.png").as_slice(),
                include_bytes!("../../icons/CloseIconWhite.png").as_slice(),
                include_bytes!("../../icons/SettingsIconWhite.png").as_slice(),
                include_bytes!("../../icons/TranslateIconWhite.png").as_slice(),
                include_bytes!("../../icons/ConjugateIconWhite.png").as_slice(),
                include_bytes!("../../icons/PluralIconWhite.png").as_slice(),
            )
        } else {
            (
                include_bytes!("../../icons/ScribeIconBlack.png").as_slice(),
                include_bytes!("../../icons/CloseIconBlack.png").as_slice(),
                include_bytes!("../../icons/SettingsIconBlack.png").as_slice(),
                include_bytes!("../../icons/TranslateIconBlack.png").as_slice(),
                include_bytes!("../../icons/ConjugateIconBlack.png").as_slice(),
                include_bytes!("../../icons/PluralIconBlack.png").as_slice(),
            )
        };

        let create_icon =
            |data: &[u8], width: u16| Image::new(Handle::from_bytes(data.to_vec())).width(width);

        let scribe_logo = create_icon(scribe_logo_data, ICON_SIZE);
        let close_icon = create_icon(close_icon_data, ICON_SIZE);
        let settings_icon = create_icon(settings_icon_data, ICON_SIZE);
        let translate_icon = create_icon(translate_icon_data, BUTTON_ICON_SIZE);
        let conjugate_icon = create_icon(conjugate_icon_data, BUTTON_ICON_SIZE);
        let plural_icon = create_icon(plural_icon_data, BUTTON_ICON_SIZE);

        let text_input = text_input("Enter text for command...", &self.keys)
            .font(Font::DEFAULT)
            .on_input(Message::TextInputChanged)
            .style(move |_theme: &Theme, _status| {
                text_input::Style {
                    background: iced::Background::Color(if is_dark {
                        iced::Color::from_rgb8(0x00, 0x00, 0x00) // Black background
                    } else {
                        iced::Color::from_rgb8(0xFF, 0xFF, 0xFF) // White background
                    }),
                    border: iced::Border {
                        color: iced::Color::from_rgb(0.7, 0.7, 0.7),
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    icon: if is_dark {
                        iced::Color::WHITE
                    } else {
                        iced::Color::BLACK
                    },
                    placeholder: if is_dark {
                        iced::Color::from_rgb(0.6, 0.6, 0.6)
                    } else {
                        iced::Color::from_rgb(0.7, 0.7, 0.7)
                    },
                    value: if is_dark {
                        iced::Color::WHITE // WHITE TEXT for dark mode!
                    } else {
                        iced::Color::BLACK
                    },
                    selection: if is_dark {
                        iced::Color::from_rgb(0.3, 0.4, 0.6)
                    } else {
                        iced::Color::from_rgb(0.7, 0.8, 1.0)
                    },
                }
            });

        // Left column with logo/menu buttons.
        let left_column = if self.show_menu {
            Column::new()
                .spacing(10)
                .align_x(Alignment::Center)
                .push(self.create_icon_button(close_icon, Message::ToggleMenu, ICON_SIZE, false))
                .push(self.create_icon_button(settings_icon, Message::Settings, ICON_SIZE, true))
        } else {
            Column::new()
                .spacing(10)
                .align_x(Alignment::Center)
                .push(self.create_icon_button(
                    scribe_logo,
                    Message::ToggleMenu,
                    SCRIBE_ICON_SIZE,
                    false,
                ))
        };

        // MARK: Command Buttons

        let button_width = Length::Fixed(130.0);
        let button_row = Row::new()
            .spacing(10)
            .align_y(Alignment::Start)
            .push(self.create_command_button(
                translate_icon,
                "Translate",
                Message::Translate,
                button_width,
            ))
            .push(self.create_command_button(
                conjugate_icon,
                "Conjugate",
                Message::Conjugate,
                button_width,
            ))
            .push(self.create_command_button(plural_icon, "Plural", Message::Plural, button_width))
            .push(
                Button::new(Container::new("Theme"))
                    .on_press(Message::ToggleTheme)
                    .style(move |_theme: &Theme, _status| {
                        let background_color = iced::Color::from_rgb8(0x4C, 0xAD, 0xE6);

                        button::Style {
                            background: Some(iced::Background::Color(background_color)),
                            text_color: if is_dark {
                                iced::Color::WHITE
                            } else {
                                iced::Color::BLACK
                            },
                            border: iced::Border {
                                color: iced::Color::TRANSPARENT,
                                width: 0.0,
                                radius: 4.0.into(),
                            },
                            shadow: iced::Shadow::default(),
                        }
                    })
                    .width(button_width),
            );

        // Right column with input and buttons.
        let mut right_column = Column::new()
            .spacing(10)
            .width(Length::Fill)
            .push(text_input);

        // Only show buttons when menu is open.
        if self.show_menu {
            right_column = right_column.push(button_row);
        }

        // MARK: Main Layout

        let layout = Column::new()
            .width(Length::Shrink)
            .spacing(10)
            .align_x(Alignment::Start)
            .push(
                Row::new()
                    .spacing(10)
                    .align_y(Alignment::Start)
                    .push(left_column)
                    .push(right_column),
            );

        let background_color = if is_dark {
            iced::Color::from_rgb8(0x1E, 0x1E, 0x1E)
        } else {
            iced::Color::from_rgb8(0xCE, 0xD2, 0xD9)
        };

        Container::new(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .style(move |_theme: &Theme| iced::widget::container::Style {
                background: Some(background_color.into()),
                ..Default::default()
            })
            .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn create_icon_button<'a>(
        &self,
        icon: Image<Handle>,
        message: Message,
        icon_size: u16,
        is_settings: bool,
    ) -> Button<'a, Message> {
        let is_dark = self.state.is_dark_theme;

        Button::new(icon)
            .on_press(message)
            .style(move |_theme: &Theme, _status| {
                let background_color = if is_settings {
                    if is_dark {
                        iced::Color::from_rgb8(0xD1, 0x7B, 0x0F)
                    } else {
                        iced::Color::from_rgb8(0xFD, 0xAD, 0x0D)
                    }
                } else {
                    iced::Color::from_rgb8(0x4C, 0xAD, 0xE6)
                };

                button::Style {
                    background: Some(iced::Background::Color(background_color)),
                    text_color: if is_dark {
                        iced::Color::WHITE
                    } else {
                        iced::Color::BLACK
                    },
                    border: iced::Border {
                        color: iced::Color::TRANSPARENT,
                        width: 0.0,
                        radius: 4.0.into(),
                    },
                    shadow: iced::Shadow::default(),
                }
            })
            .width(Length::Fixed(
                icon_size as f32 + if icon_size == 20 { 36.0 } else { 30.0 },
            ))
            .height(Length::Fixed(
                icon_size as f32 + if icon_size == 20 { 10.0 } else { 6.0 },
            ))
    }

    fn create_command_button<'a>(
        &self,
        icon: Image<Handle>,
        label: &'a str,
        message: Message,
        width: Length,
    ) -> Button<'a, Message> {
        let is_dark = self.state.is_dark_theme;
        let content = Row::new().spacing(5).push(icon).push(label);

        Button::new(Container::new(content))
            .on_press(message)
            .style(move |_theme: &Theme, _status| {
                let background_color = iced::Color::from_rgb8(0x4C, 0xAD, 0xE6);

                button::Style {
                    background: Some(iced::Background::Color(background_color)),
                    text_color: if is_dark {
                        iced::Color::WHITE
                    } else {
                        iced::Color::BLACK
                    },
                    border: iced::Border {
                        color: iced::Color::TRANSPARENT,
                        width: 0.0,
                        radius: 4.0.into(),
                    },
                    shadow: iced::Shadow::default(),
                }
            })
            .width(width)
    }
}

fn main() -> iced::Result {
    iced::application("Scribe", Scribe::update, Scribe::view)
        .subscription(Scribe::subscription)
        .theme(Scribe::theme)
        .window(window::Settings {
            min_size: Some(Size::new(626.0, 52.0)),
            size: Size::new(626.0, 52.0),
            position: window::Position::Centered,
            resizable: false,
            decorations: true,
            ..window::Settings::default()
        })
        .run()
}
