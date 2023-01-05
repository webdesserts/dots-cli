use core::fmt;
use std::ops::Add;

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

    const fn color(&self, color: Color) -> Style {
        self.merge(Self {
            color: Some(color),
            ..DEFAULT
        })
    }

    const fn background(&self, background: Color) -> Style {
        self.merge(Self {
            background: Some(background),
            ..DEFAULT
        })
    }

    const fn attr(&self, attribute: Attribute) -> Style {
        let mut style = Style::new();

        match attribute {
            Attribute::Bold => style.bold = true,
            Attribute::Dimmed => style.dim = true,
            Attribute::Italic => style.italic = true,
            Attribute::Underline => style.underlined = true,
            Attribute::Blink => style.blink = true,
            Attribute::Invert => style.reverse = true,
            Attribute::Hidden => style.hidden = true,
            Attribute::Strikethrough => style.strikethrough = true,
        }

        self.merge(style)
    }

    pub const fn black(&self) -> Style {
        self.color(Color::Black)
    }
    pub const fn red(&self) -> Style {
        self.color(Color::Red)
    }
    pub const fn green(&self) -> Style {
        self.color(Color::Green)
    }
    pub const fn yellow(&self) -> Style {
        self.color(Color::Yellow)
    }
    pub const fn blue(&self) -> Style {
        self.color(Color::Blue)
    }
    pub const fn magenta(&self) -> Style {
        self.color(Color::Magenta)
    }
    pub const fn cyan(&self) -> Style {
        self.color(Color::Cyan)
    }
    pub const fn white(&self) -> Style {
        self.color(Color::White)
    }

    pub const fn on_black(&self) -> Style {
        self.background(Color::Black)
    }
    pub const fn on_red(&self) -> Style {
        self.background(Color::Red)
    }
    pub const fn on_green(&self) -> Style {
        self.background(Color::Green)
    }
    pub const fn on_yellow(&self) -> Style {
        self.background(Color::Yellow)
    }
    pub const fn on_blue(&self) -> Style {
        self.background(Color::Blue)
    }
    pub const fn on_magenta(&self) -> Style {
        self.background(Color::Magenta)
    }
    pub const fn on_cyan(&self) -> Style {
        self.background(Color::Cyan)
    }
    pub const fn on_white(&self) -> Style {
        self.background(Color::White)
    }

    pub const fn bold(&self) -> Style {
        self.attr(Attribute::Bold)
    }
    pub const fn dim(&self) -> Style {
        self.attr(Attribute::Dimmed)
    }
    pub const fn italic(&self) -> Style {
        self.attr(Attribute::Italic)
    }
    pub const fn underlined(&self) -> Style {
        self.attr(Attribute::Underline)
    }
    pub const fn blink(&self) -> Style {
        self.attr(Attribute::Blink)
    }
    pub const fn reverse(&self) -> Style {
        self.attr(Attribute::Invert)
    }
    pub const fn hidden(&self) -> Style {
        self.attr(Attribute::Hidden)
    }
    pub const fn strikethrough(&self) -> Style {
        self.attr(Attribute::Strikethrough)
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
        Style::new().color(color)
    }
}

impl From<Attribute> for Style {
    fn from(attribute: Attribute) -> Self {
        Style::new().attr(attribute)
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
        self.color(rhs)
    }
}

impl Add<Attribute> for Style {
    type Output = Style;
    fn add(self, rhs: Attribute) -> Self::Output {
        self.attr(rhs)
    }
}
