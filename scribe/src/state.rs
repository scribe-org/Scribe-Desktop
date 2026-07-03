/* SPDX-License-Identifier: GPL-3.0-or-later */
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Language {
    #[default]
    English,
    French,
    German,
    Italian,
    Portuguese,
    Russian,
    Spanish,
    Swedish,
}

impl Language {
    pub const ALL: [Language; 8] = [
        Language::English,
        Language::French,
        Language::German,
        Language::Italian,
        Language::Portuguese,
        Language::Russian,
        Language::Spanish,
        Language::Swedish,
    ];
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Language::English => write!(f, "English"),
            Language::French => write!(f, "French"),
            Language::German => write!(f, "German"),
            Language::Italian => write!(f, "Italian"),
            Language::Portuguese => write!(f, "Portuguese"),
            Language::Russian => write!(f, "Russian"),
            Language::Spanish => write!(f, "Spanish"),
            Language::Swedish => write!(f, "Swedish"),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AppState {
    pub is_dark_theme: bool,
    pub from_language: Language,
    pub to_language: Language,
}

impl AppState {
    pub fn toggle_theme(&mut self) {
        self.is_dark_theme = !self.is_dark_theme;
    }
}
