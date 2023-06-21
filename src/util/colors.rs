use termion::color;

pub fn color_fg(string: String, color: impl color::Color) -> String {
    format!("{}{}{}", color::Fg(color), string, color::Fg(color::Reset))
}

pub fn color_bg(string: String, color: impl color::Color) -> String {
    format!("{}{}{}", color::Bg(color), string, color::Bg(color::Reset))
}
