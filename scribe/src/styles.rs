use iced::{
    widget::{button, text_input},
    Background, Border, Color, Vector,
};

use iced::widget::text_input::Appearance as TextInputAppearance;

pub struct CustomTextInput;

impl text_input::StyleSheet for CustomTextInput {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> TextInputAppearance {
        TextInputAppearance {
            background: Background::Color(Color::from_rgb(0.9, 0.9, 0.9)),
            border: Border {
                color: Color::from_rgb(0.7, 0.7, 0.7),
                width: 1.0,
                radius: 4.0.into(),
            },
            icon_color: Color::from_rgb(0.5, 0.5, 0.5),
        }
    }
    fn focused(&self, _style: &Self::Style) -> TextInputAppearance {
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
    fn value_color(&self, _style: &Self::Style) -> iced::Color {
        Color::from_rgb(0.7, 0.7, 0.7)
    }
    fn disabled_color(&self, _style: &Self::Style) -> iced::Color {
        Color::from_rgb(1.0, 0.7, 0.7)
    }
    fn selection_color(&self, _style: &Self::Style) -> iced::Color {
        Color::from_rgb(0.7, 0.7, 0.7)
    }
    fn placeholder_color(&self, _style: &Self::Style) -> iced::Color {
        Color::from_rgb(0.7, 0.7, 0.7)
    }
}
