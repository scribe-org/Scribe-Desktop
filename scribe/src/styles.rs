/* SPDX-License-Identifier: GPL-3.0-or-later */
use iced::widget::button::Appearance as ButtonAppearance;
use iced::widget::text_input::Appearance as TextInputAppearance;
use iced::{widget::button, widget::text_input, Background, Border, Color, Shadow, Vector};

use crate::state::AppState;

pub struct CustomTextInput {
    pub state: AppState,
}
pub struct ButtonStyle {
    pub state: AppState,
}

impl button::StyleSheet for ButtonStyle {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> ButtonAppearance {
        let is_dark = matches!(style, iced::Theme::Dark);

        ButtonAppearance {
            background: Some(Background::Color(if is_dark {
                Color::from_rgb8(0x3E, 0x95, 0xCC)
            } else {
                Color::from_rgb8(0x4C, 0xAD, 0xE6)
            })),
            text_color: if is_dark {
                Color::from_rgb8(0xFF, 0xFF, 0xFF) // rgb(189, 189, 189) - White text for dark mode
            } else {
                Color::from_rgb8(0x00, 0x00, 0x00) // #000000 - Black text for light mode
            },
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 4.0.into(),
            },
            shadow: Shadow {
                color: Color::TRANSPARENT,
                offset: Vector::new(0.0, 0.0),
                blur_radius: 0.0,
            },
            shadow_offset: Vector::new(0.0, 0.0),
        }
    }
    fn hovered(&self, style: &Self::Style) -> ButtonAppearance {
        self.active(style) // same as active - no hover effects
    }

    fn pressed(&self, style: &Self::Style) -> ButtonAppearance {
        self.active(style) // same as active - no press effects
    }

    fn disabled(&self, _style: &Self::Style) -> ButtonAppearance {
        ButtonAppearance {
            background: Some(Background::Color(Color::from_rgb8(0xCC, 0xCC, 0xCC))), // #CCCCCC - gray for disabled
            text_color: Color::from_rgb8(0x66, 0x66, 0x66), // #666666 - darker gray text
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 4.0.into(),
            },
            shadow: Shadow {
                color: Color::TRANSPARENT,
                offset: Vector::new(0.0, 0.0),
                blur_radius: 0.0,
            },
            shadow_offset: Vector::new(0.0, 0.0),
        }
    }
}

impl text_input::StyleSheet for CustomTextInput {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> TextInputAppearance {
        let is_dark = matches!(style, iced::Theme::Dark);
        TextInputAppearance {
            background: Background::Color(if is_dark {
                Color::from_rgb8(0x30, 0x30, 0x30)
            } else {
                Color::from_rgb8(0xFF, 0xFF, 0xFF)
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
