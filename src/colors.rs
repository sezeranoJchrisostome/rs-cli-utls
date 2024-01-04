//! Colorized output utilities for the terminal using ANSI escape codes.
//! # Examples:
//! ```
//! use cli_utils::colors::*;
//! println!("{}{}{}", red("Red"), green("Green"), blue("Blue"));
//! ```

/// Returns a string with the ANSI escape code for red.
/// # Examples:
/// ```
/// use cli_utils::colors::*;
/// println!("{}", red("Red"));
/// ```
pub fn red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}

pub fn green(s: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", s)
}

pub fn blue(s: &str) -> String {
    format!("\x1b[34m{}\x1b[0m", s)
}

pub fn bold(s: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", s)
}

pub fn reset(s: &str) -> String {
    format!("\x1b[0m{}\x1b[0m", s)
}

/// Returns a string with the ANSI escape code for green.
/// # Examples:
/// ```
/// use cli_utils::colors::*;
/// println!("{}", green("Green"));
/// ```
pub enum Color{
    Red,
    Green,
    Blue,
    Bold,
}

/// A struct that contains a string and a color.
/// # Examples:
/// ```
/// use cli_utils::colors::*;
/// let mut color_string = ColorString {
///    color: Color::Red,
///   string: "Red".to_string(),
///   colorized: "".to_string()
/// };
/// color_string.paint();
/// println!("{}", color_string.colorized);
/// ```
/// # Notes:
/// This struct is used to colorize a string.
pub struct ColorString {
    pub color: Color,
    pub string: String,
    pub colorized: String
}

impl ColorString {
    // create a method that will use the string and color fields to create a colorized string and assign it to the colorized field
    pub fn paint(&mut self) {
        match self.color {
            Color::Red => self.colorized = red(&self.string),
            Color::Green => self.colorized = green(&self.string),
            Color::Blue => self.colorized = blue(&self.string),
            Color::Bold => self.colorized = bold(&self.string),
        };
    }

    pub fn reset(&mut self) {
        self.colorized = reset(&self.string);
    }

}
