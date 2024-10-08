use iced::{widget::text_input, Background, Border, Color};
use rdev::Key;

pub fn allowed_keys(key: &Key) -> Option<char> {
    match key {
        Key::KeyA => Some('a'),
        Key::KeyB => Some('b'),
        Key::KeyC => Some('c'),
        Key::KeyD => Some('d'),
        Key::KeyE => Some('e'),
        Key::KeyF => Some('f'),
        Key::KeyG => Some('g'),
        Key::KeyH => Some('h'),
        Key::KeyI => Some('i'),
        Key::KeyJ => Some('j'),
        Key::KeyK => Some('k'),
        Key::KeyL => Some('l'),
        Key::KeyM => Some('m'),
        Key::KeyN => Some('n'),
        Key::KeyO => Some('o'),
        Key::KeyP => Some('p'),
        Key::KeyQ => Some('q'),
        Key::KeyR => Some('r'),
        Key::KeyS => Some('s'),
        Key::KeyT => Some('t'),
        Key::KeyU => Some('u'),
        Key::KeyV => Some('v'),
        Key::KeyW => Some('w'),
        Key::KeyX => Some('x'),
        Key::KeyY => Some('y'),
        Key::KeyZ => Some('z'),
        Key::Num0 => Some('0'),
        Key::Num1 => Some('1'),
        Key::Num2 => Some('2'),
        Key::Num3 => Some('3'),
        Key::Num4 => Some('4'),
        Key::Num5 => Some('5'),
        Key::Num6 => Some('6'),
        Key::Num7 => Some('7'),
        Key::Num8 => Some('8'),
        Key::Num9 => Some('9'),
        Key::Space => Some(' '),
        Key::Backspace => Some('\x08'),
        Key::Comma => Some(','),
        Key::SemiColon => Some(';'),
        Key::Period => Some('.'),
        Key::Slash => Some('/'),
        Key::Backslash => Some('\\'),
        Key::Apostrophe => Some('\''),
        Key::Equal => Some('='),
        Key::Minus => Some('-'),
        Key::LBracket => Some('['),
        Key::RBracket => Some(']'),
        Key::Grave => Some('`'),
        Key::Tab => Some('\t'),
        Key::Colon => Some(':'),   // In case of ':' on certain layouts
        Key::Exclamation => Some('!'),
        Key::Question => Some('?'),
        Key::At => Some('@'),
        Key::Hash => Some('#'),
        Key::Dollar => Some('$'),
        Key::Percent => Some('%'),
        Key::Caret => Some('^'),
        Key::Ampersand => Some('&'),
        Key::Asterisk => Some('*'),
        Key::LParen => Some('('),
        Key::RParen => Some(')'),
        Key::Underscore => Some('_'),
        Key::Plus => Some('+'),
        Key::Quote => Some('"'),
        Key::LessThan => Some('<'),
        Key::GreaterThan => Some('>'),
        _ => None,
    }
}

pub struct CustomTextInput;

impl text_input::StyleSheet for CustomTextInput {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(Color::from_rgb(0.9, 0.9, 0.9)),
            border: Border {
                color: Color::from_rgb(0.7, 0.7, 0.7),
                width: 1.0,
                radius: 4.0.into(),
            },
            icon_color: Color::from_rgb(0.5, 0.5, 0.5),
        }
    }
    fn focused(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(Color::from_rgb(0.8, 0.8, 0.8)),
            border: Border {
                color: Color::from_rgb(0.7, 0.7, 0.7),
                width: 1.0,
                radius: 4.0.into(),
            },
            icon_color: Color::from_rgb(0.5, 0.5, 0.5),
        }
    }
    fn hovered(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(Color::from_rgb(0.8, 0.8, 0.8)),
            border: Border {
                color: Color::from_rgb(0.7, 0.7, 0.7),
                width: 1.0,
                radius: 4.0.into(),
            },
            icon_color: Color::from_rgb(0.5, 0.5, 0.5),
        }
    }
    fn disabled(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
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
        Color::from_rgb(0.7, 0.7, 0.7)
    }
    fn selection_color(&self, _style: &Self::Style) -> iced::Color {
        Color::from_rgb(0.7, 0.7, 0.7)
    }
    fn placeholder_color(&self, _style: &Self::Style) -> iced::Color {
        Color::from_rgb(0.7, 0.7, 0.7)
    }
}
