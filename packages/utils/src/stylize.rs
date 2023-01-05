use core::fmt;
use std::ops::{Add, Sub};

use yansi;

#[derive(Clone, Copy)]
enum Color {
    Black,
    Red,
    Green,
    Blue,
    Magenta,
    Yellow,
    Cyan,
    White,
}

#[derive(Clone, Copy)]
enum Attribute {
    Bold,
    Dimmed,
    Italic,
    Underline,
    Blink,
    Invert,
    Hidden,
    Strikethrough,
}

const DEFAULT: Style = Style {
    color: None,
    background: None,
    bold: false,
    dim: false,
    italic: false,
    underlined: false,
    blink: false,
    reverse: false,
    hidden: false,
    strikethrough: false,
};

#[derive(Clone, Copy)]
pub struct Style {
    color: Option<Color>,
    background: Option<Color>,
    bold: bool,
    dim: bool,
    italic: bool,
    underlined: bool,
    blink: bool,
    reverse: bool,
    hidden: bool,
    strikethrough: bool,
}

impl Style {
    pub fn detect_color_support() {
        if supports_color::on(supports_color::Stream::Stdout).is_none() {
            yansi::Paint::disable();
        }
    }

    pub const fn new() -> Style {
        Style { ..DEFAULT }
    }

    const fn set_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    const fn set_background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    const fn set_attr(mut self, attribute: Attribute, enabled: bool) -> Self {
        match attribute {
            Attribute::Bold => self.bold = enabled,
            Attribute::Dimmed => self.dim = enabled,
            Attribute::Italic => self.italic = enabled,
            Attribute::Underline => self.underlined = enabled,
            Attribute::Blink => self.blink = enabled,
            Attribute::Invert => self.reverse = enabled,
            Attribute::Hidden => self.hidden = enabled,
            Attribute::Strikethrough => self.strikethrough = enabled,
        }
        self
    }

    pub const fn black(&self) -> Style {
        self.set_color(Color::Black)
    }
    pub const fn red(&self) -> Style {
        self.set_color(Color::Red)
    }
    pub const fn green(&self) -> Style {
        self.set_color(Color::Green)
    }
    pub const fn yellow(&self) -> Style {
        self.set_color(Color::Yellow)
    }
    pub const fn blue(&self) -> Style {
        self.set_color(Color::Blue)
    }
    pub const fn magenta(&self) -> Style {
        self.set_color(Color::Magenta)
    }
    pub const fn cyan(&self) -> Style {
        self.set_color(Color::Cyan)
    }
    pub const fn white(&self) -> Style {
        self.set_color(Color::White)
    }

    pub const fn on_black(&self) -> Style {
        self.set_background(Color::Black)
    }
    pub const fn on_red(&self) -> Style {
        self.set_background(Color::Red)
    }
    pub const fn on_green(&self) -> Style {
        self.set_background(Color::Green)
    }
    pub const fn on_yellow(&self) -> Style {
        self.set_background(Color::Yellow)
    }
    pub const fn on_blue(&self) -> Style {
        self.set_background(Color::Blue)
    }
    pub const fn on_magenta(&self) -> Style {
        self.set_background(Color::Magenta)
    }
    pub const fn on_cyan(&self) -> Style {
        self.set_background(Color::Cyan)
    }
    pub const fn on_white(&self) -> Style {
        self.set_background(Color::White)
    }

    pub const fn bold(&self) -> Style {
        self.set_attr(Attribute::Bold, true)
    }
    pub const fn dim(&self) -> Style {
        self.set_attr(Attribute::Dimmed, true)
    }
    pub const fn italic(&self) -> Style {
        self.set_attr(Attribute::Italic, true)
    }
    pub const fn underlined(&self) -> Style {
        self.set_attr(Attribute::Underline, true)
    }
    pub const fn blink(&self) -> Style {
        self.set_attr(Attribute::Blink, true)
    }
    pub const fn reverse(&self) -> Style {
        self.set_attr(Attribute::Invert, true)
    }
    pub const fn hidden(&self) -> Style {
        self.set_attr(Attribute::Hidden, true)
    }
    pub const fn strikethrough(&self) -> Style {
        self.set_attr(Attribute::Strikethrough, true)
    }

    const fn merge(&self, style: Style) -> Style {
        Style {
            color: match style.color {
                Some(color) => Some(color),
                None => self.color,
            },
            background: match style.background {
                Some(color) => Some(color),
                None => self.background,
            },
            bold: style.bold || self.bold,
            dim: style.dim || self.dim,
            italic: style.italic || self.italic,
            underlined: style.underlined || self.underlined,
            blink: style.blink || self.blink,
            reverse: style.reverse || self.reverse,
            hidden: style.hidden || self.hidden,
            strikethrough: style.strikethrough || self.strikethrough,
        }
    }

    pub fn apply<D: fmt::Display>(&self, val: D) -> impl fmt::Display {
        let style: yansi::Style = self.into();
        style.paint(val)
    }
}

impl From<&Style> for yansi::Style {
    fn from(style: &Style) -> Self {
        let mut converted = yansi::Style::new(yansi::Color::Unset);

        if let Some(fg) = style.color {
            converted = converted.fg(fg.into());
        };

        if let Some(bg) = style.background {
            converted = converted.fg(bg.into());
        };

        if style.bold {
            converted = converted.bold()
        }
        if style.dim {
            converted = converted.dimmed()
        }
        if style.italic {
            converted = converted.italic()
        }
        if style.underlined {
            converted = converted.underline()
        }
        if style.blink {
            converted = converted.blink()
        }
        if style.reverse {
            converted = converted.invert()
        }
        if style.hidden {
            converted = converted.hidden()
        }
        if style.strikethrough {
            converted = converted.strikethrough()
        }

        converted
    }
}

impl From<Color> for Style {
    fn from(color: Color) -> Self {
        Style::new().set_color(color)
    }
}

impl From<Attribute> for Style {
    fn from(attribute: Attribute) -> Self {
        Style::new().set_attr(attribute, true)
    }
}

impl From<Color> for yansi::Color {
    fn from(color: Color) -> Self {
        match color {
            Color::Black => yansi::Color::Black,
            Color::Red => yansi::Color::Red,
            Color::Green => yansi::Color::Green,
            Color::Blue => yansi::Color::Blue,
            Color::Magenta => yansi::Color::Magenta,
            Color::Cyan => yansi::Color::Cyan,
            Color::Yellow => yansi::Color::Yellow,
            Color::White => yansi::Color::White,
        }
    }
}

impl Add<Style> for Style {
    type Output = Style;
    fn add(self, rhs: Style) -> Self::Output {
        self.merge(rhs)
    }
}

impl Add<Color> for Style {
    type Output = Style;
    fn add(self, rhs: Color) -> Self::Output {
        self.set_color(rhs)
    }
}

impl Add<Attribute> for Style {
    type Output = Style;
    fn add(self, attribute: Attribute) -> Self::Output {
        self.set_attr(attribute, true)
    }
}

impl Sub<Attribute> for Style {
    type Output = Style;
    fn sub(self, attribute: Attribute) -> Self::Output {
        self.set_attr(attribute, false)
    }
}
