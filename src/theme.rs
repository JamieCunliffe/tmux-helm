use serde::Deserialize;
use tui::style::Color;

#[derive(Deserialize, Debug)]
pub struct Theme {
    selected_session_foreground: u32,
    session_foreground: u32,
    session_list_border: u32,
    help_binding_foreground: u32,
    prompt_foreground: u32,
    prompt_input_foreground: u32,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            selected_session_foreground: 0xFFFF00,
            session_foreground: 0xFFFFFF,
            session_list_border: 0xAAAAAA,
            help_binding_foreground: 0xFFFFFF,
            prompt_foreground: 0xFFFFFF,
            prompt_input_foreground: 0xAAAAAA,
        }
    }
}

impl Theme {
    pub fn get_highlight_foreground(&self) -> Color {
        Theme::get_color(self.selected_session_foreground)
    }

    pub fn get_session_foreground(&self) -> Color {
        Theme::get_color(self.session_foreground)
    }

    pub fn get_session_list_border(&self) -> Color {
        Theme::get_color(self.session_list_border)
    }

    pub fn get_help_binding_foreground(&self) -> Color {
        Theme::get_color(self.help_binding_foreground)
    }

    pub fn get_prompt_foreground(&self) -> Color {
        Theme::get_color(self.prompt_foreground)
    }

    pub fn get_prompt_input_foreground(&self) -> Color {
        Theme::get_color(self.prompt_input_foreground)
    }

    fn get_color(rgb: u32) -> Color {
        let r: u8 = ((0xFF0000 & rgb) >> 16) as u8;
        let g: u8 = ((0xFF00 & rgb) >> 8) as u8;
        let b: u8 = (0xFF & rgb) as u8;

        Color::Rgb(r, g, b)
    }
}
