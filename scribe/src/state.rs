#[derive(Debug, Clone, Copy)]
pub struct AppState {
    pub is_dark_theme: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            is_dark_theme: false,
        }
    }
}

impl AppState {
    pub fn toggle_theme(&mut self) {
        self.is_dark_theme = !self.is_dark_theme;
    }
}
