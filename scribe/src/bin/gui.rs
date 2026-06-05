/* SPDX-License-Identifier: GPL-3.0-or-later */
use iced::futures::SinkExt;
use iced::stream;
use iced::widget::{button, image::Handle, text_input, Button, Column, Container, Image, Row};
use iced::{window, Alignment, Element, Font, Length, Size, Subscription, Task, Theme};
use scribe::AppState;
use std::io::Read;
use std::net::TcpListener;
use std::thread::spawn;
const WINDOW_WIDTH: f32 = 495.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CommandKind {
    Translate,
    Conjugate,
    Plural,
}

impl CommandKind {
    fn label(self) -> &'static str {
        match self {
            Self::Translate => "Translate",
            Self::Conjugate => "Conjugate",
            Self::Plural => "Plural",
        }
    }

    fn placeholder(self) -> &'static str {
        match self {
            Self::Translate => "Enter text to translate...",
            Self::Conjugate => "Enter verb to conjugate...",
            Self::Plural => "Enter noun to pluralize...",
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    KeyReceived(char),
    TextInputChanged(String),
    ToggleMenu,
    Settings,
    Translate,
    Conjugate,
    Plural,
    ExecuteCommand,
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
    selected_command: Option<CommandKind>,
    show_menu: bool,
    show_settings: bool,
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
            selected_command: None,
            show_menu: false,
            show_settings: false,
            state: AppState {
                is_dark_theme: is_dark,
            },
            theme: detected_theme,
        }
    }
}

impl Scribe {
    fn handle_key_received(&mut self, char: char) {
        if self.is_executing_command && self.selected_command.is_some() {
            if char == '\x08' {
                self.keys.pop();
            } else if char != '\t' {
                self.keys.push(char);
            }
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::KeyReceived(char) => {
                self.handle_key_received(char);
            }
            Message::ToggleMenu => {
                self.show_menu = !self.show_menu;
                // closing the menu should also close the settings pane
                if !self.show_menu {
                    self.show_settings = false;
                }
                self.is_executing_command = self.show_menu;
                if !self.show_menu {
                    self.selected_command = None;
                }
                let new_height = if self.show_menu { 94.0 } else { 52.0 };
                return window::get_latest()
                    .and_then(move |id| window::resize(id, Size::new(WINDOW_WIDTH, new_height)));
            }
            Message::Settings => {
                // Toggle the settings pane on/off and ensure the menu is visible
                self.show_settings = !self.show_settings;
                if self.show_settings {
                    self.show_menu = true;
                    self.is_executing_command = true;
                    self.selected_command = None;
                }
                println!("Settings clicked: show_settings={}", self.show_settings);
            }
            Message::Translate => {
                self.select_command(CommandKind::Translate);
                println!("Translate");
            }
            Message::Conjugate => {
                self.select_command(CommandKind::Conjugate);
                println!("Conjugate");
            }
            Message::Plural => {
                self.select_command(CommandKind::Plural);
                println!("Plural");
            }
            Message::ExecuteCommand => {
                self.execute_selected_command();
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

    fn select_command(&mut self, command: CommandKind) {
        self.selected_command = Some(command);
        self.is_executing_command = true;
        self.show_settings = false;
    }

    fn execute_selected_command(&self) {
        if let Some(command) = self.selected_command {
            println!("Execute {}: {}", command.label(), self.keys);
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::run(|| {
            stream::channel(100, |mut output| async move {
                spawn(move || {
                    if let Ok(listener) = TcpListener::bind("127.0.0.1:7878") {
                        for mut stream in listener.incoming().flatten() {
                            let mut buffer = [0; 1];
                            if stream.read_exact(&mut buffer).is_ok() {
                                if let Some(received_char) = char::from_u32(buffer[0].into()) {
                                    iced::futures::executor::block_on(async {
                                        let _ =
                                            output.send(Message::KeyReceived(received_char)).await;
                                    });
                                }
                            }
                        }
                    }
                });
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

        let input_placeholder = self
            .selected_command
            .map(CommandKind::placeholder)
            .unwrap_or("Enter text for command...");

        let text_input = text_input(input_placeholder, &self.keys)
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
                self.selected_command == Some(CommandKind::Translate),
            ))
            .push(self.create_command_button(
                conjugate_icon,
                "Conjugate",
                Message::Conjugate,
                button_width,
                self.selected_command == Some(CommandKind::Conjugate),
            ))
            .push(self.create_command_button(
                plural_icon,
                "Plural",
                Message::Plural,
                button_width,
                self.selected_command == Some(CommandKind::Plural),
            ))
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

        // If the settings pane is active, replace command buttons with settings UI
        let settings_row = Row::new()
            .spacing(10)
            .align_y(Alignment::Center)
            .push(
                Container::new(if self.state.is_dark_theme {
                    "Interface Theme: Dark"
                } else {
                    "Interface Theme: Light"
                })
                .width(Length::Fixed(260.0)),
            )
            .push(
                Button::new(Container::new(if self.state.is_dark_theme {
                    "Switch to Light"
                } else {
                    "Switch to Dark"
                }))
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
        let input_row = Row::new()
            .spacing(10)
            .align_y(Alignment::Center)
            .push(text_input)
            .push_maybe(
                self.selected_command
                    .map(|_| self.create_enter_button(Length::Fixed(70.0))),
            );

        let mut right_column = Column::new()
            .spacing(10)
            .width(Length::Fill)
            .push(input_row);

        // Only show buttons when menu is open.
        if self.show_menu {
            if self.show_settings {
                right_column = right_column.push(settings_row);
            } else {
                right_column = right_column.push(button_row);
            }
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
        is_selected: bool,
    ) -> Button<'a, Message> {
        let is_dark = self.state.is_dark_theme;
        let content = Row::new().spacing(5).push(icon).push(label);

        Button::new(Container::new(content))
            .on_press(message)
            .style(move |_theme: &Theme, _status| {
                let background_color = if is_selected {
                    if is_dark {
                        iced::Color::from_rgb8(0x12, 0x66, 0x4F)
                    } else {
                        iced::Color::from_rgb8(0x8A, 0xD6, 0xAA)
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
            .width(width)
    }

    fn create_enter_button<'a>(&self, width: Length) -> Button<'a, Message> {
        let is_dark = self.state.is_dark_theme;

        Button::new(Container::new("Enter").center_x(Length::Fill))
            .on_press(Message::ExecuteCommand)
            .style(move |_theme: &Theme, _status| button::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb8(
                    0x12, 0x66, 0x4F,
                ))),
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
            })
            .width(width)
    }
}

fn main() -> iced::Result {
    iced::application("Scribe", Scribe::update, Scribe::view)
        .subscription(Scribe::subscription)
        .theme(Scribe::theme)
        .window(window::Settings {
            min_size: Some(Size::new(WINDOW_WIDTH, 52.0)),
            size: Size::new(WINDOW_WIDTH, 52.0),
            position: window::Position::Centered,
            resizable: false,
            decorations: true,
            ..window::Settings::default()
        })
        .run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn characters_captured_when_listening() {
        let mut s = Scribe {
            is_executing_command: true,
            selected_command: Some(CommandKind::Translate),
            ..Scribe::default()
        };

        s.handle_key_received('h');
        s.handle_key_received('i');

        assert_eq!(s.keys, "hi");
    }

    #[test]
    fn characters_ignored_without_selected_command() {
        let mut s = Scribe {
            is_executing_command: true,
            selected_command: None,
            ..Scribe::default()
        };

        s.handle_key_received('h');

        assert_eq!(s.keys, "");
    }

    #[test]
    fn command_selection_tracks_active_command() {
        let mut s = Scribe::default();

        s.select_command(CommandKind::Plural);

        assert_eq!(s.selected_command, Some(CommandKind::Plural));
        assert!(s.is_executing_command);
        assert!(!s.show_settings);
    }
}
