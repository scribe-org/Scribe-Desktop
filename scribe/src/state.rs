/* SPDX-License-Identifier: GPL-3.0-or-later */
#[derive(Debug, Clone, Copy, Default)]
pub struct AppState {
    pub is_dark_theme: bool,
}

impl AppState {
    pub fn toggle_theme(&mut self) {
        self.is_dark_theme = !self.is_dark_theme;
    }
}
