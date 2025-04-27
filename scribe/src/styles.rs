/* SPDX-License-Identifier: GPL-3.0-or-later */
use iced::{
    widget::{button, text_input},
    Background, Border, Color, Vector,
};

use iced::widget::text_input::Appearance as TextInputAppearance;

use crate::state::AppState;

pub struct CustomTextInput {
    pub state: AppState,
}

impl text_input::StyleSheet for CustomTextInput {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> TextInputAppearance {
        let is_dark = self.state.is_dark_theme;
        TextInputAppearance {
            background: Background::Color(if is_dark {
                Color::from_rgb8(0xFF, 0xFF, 0xFF)
            } else {
                Color::from_rgb8(0x00, 0x00, 0x00)
            }),
            border: Border {
                color: if is_dark {
                    Color::from_rgb(0.7, 0.7, 0.7)
                } else {
                    Color::from_rgb(0.7, 0.7, 0.7)
                },
                width: 1.0,
                radius: 4.0.into(),
            },
            icon_color: if is_dark {
                Color::from_rgb8(0xFF, 0xFF, 0xFF)
            } else {
                Color::from_rgb8(0x00, 0x00, 0x00)
            },
        }
    }

    fn focused(&self, style: &Self::Style) -> TextInputAppearance {
        let is_dark = matches!(style, iced::Theme::Dark);

        TextInputAppearance {
            background: Background::Color(if is_dark {
                Color::from_rgb(0.25, 0.25, 0.25)
            } else {
                Color::from_rgb(0.8, 0.8, 0.8)
            }),
            border: Border {
                color: if is_dark {
                    Color::from_rgb(0.5, 0.5, 0.5)
                } else {
                    Color::from_rgb(0.7, 0.7, 0.7)
                },
                width: 1.0,
                radius: 4.0.into(),
            },
            icon_color: if is_dark {
                Color::from_rgb8(0xFF, 0xFF, 0xFF)
            } else {
                Color::from_rgb8(0x00, 0x00, 0x00)
            },
        }
    }
    fn hovered(&self, _style: &Self::Style) -> TextInputAppearance {
        TextInputAppearance {
            background: Background::Color(Color::from_rgb(0.8, 0.8, 0.8)),
            border: Border {
                color: Color::from_rgb(0.7, 0.7, 0.7),
                width: 1.0,
                radius: 4.0.into(),
            },
            icon_color: Color::from_rgb(0.5, 0.5, 0.5),
        }
    }

    fn disabled(&self, _style: &Self::Style) -> TextInputAppearance {
        TextInputAppearance {
            background: Background::Color(Color::from_rgb(1.0, 1.0, 1.0)),
            border: Border {
                color: Color::from_rgb(0.7, 0.7, 0.7),
                width: 1.0,
                radius: 4.0.into(),
            },
            icon_color: Color::from_rgb(0.5, 0.5, 0.5),
        }
    }
    fn value_color(&self, style: &Self::Style) -> iced::Color {
        if matches!(style, iced::Theme::Dark) {
            Color::from_rgb(0.9, 0.9, 0.9)
        } else {
            Color::from_rgb(0.2, 0.2, 0.2)
        }
    }
    fn disabled_color(&self, style: &Self::Style) -> iced::Color {
        if matches!(style, iced::Theme::Dark) {
            Color::from_rgb(0.5, 0.5, 0.5)
        } else {
            Color::from_rgb(0.7, 0.7, 0.7)
        }
    }
    fn selection_color(&self, style: &Self::Style) -> iced::Color {
        if matches!(style, iced::Theme::Dark) {
            Color::from_rgb(0.3, 0.4, 0.6)
        } else {
            Color::from_rgb(0.7, 0.8, 1.0)
        }
    }
    fn placeholder_color(&self, style: &Self::Style) -> iced::Color {
        if matches!(style, iced::Theme::Dark) {
            Color::from_rgb(0.6, 0.6, 0.6)
        } else {
            Color::from_rgb(0.7, 0.7, 0.7)
        }
    }
}
