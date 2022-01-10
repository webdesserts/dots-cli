pub use console::{Attribute, Color};

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
}

#[macro_export]
macro_rules! style {
    /* returns final expression if no more tokens remain */
    (@props () -> ($style:expr)) => { $style };
    /* Adds a color to the style */
    (@props (color: $fg:ident; $($rest:tt)*) -> ($style:expr)) => {
        style!(@props ($($rest)*) -> ($style.color($crate::stylize::Color::$fg)))
    };
    /* Adds a background to the style */
    (@props (background: $bg:ident; $($rest:tt)*) -> ($style:expr)) => {
        style!(@props ($($rest)*) -> ($style.background($crate::stylize::Color::$bg)))
    };
    /* Adds a color and matching background to the style */
    (@props (color: $fg:ident on $bg:ident; $($rest:tt)*) -> ($style:expr)) => {
        style!(@props ($($rest)*) -> ($style.color($crate::stylize::Color::$fg).background($crate::stylize::Color::$bg)))
    };
    /* Adds an attribute to the style */
    (@props ($attr:ident; $($rest:tt)*) -> ($style:expr)) => {
        style!(@props ($($rest)*) -> ($style.attr($crate::stylize::Attribute::$attr)))
    };
    (@props ($($rest:tt)*) -> ($style:expr)) => {
        style!(@props ($($rest)*;) -> ($style))
    };
    /* Creates the style and starts parsing for props */
    ($($props:tt)*) => {
        style!(@props ($($props)*) -> ($crate::stylize::Style::new()))
    };
}

impl Style {
    pub const fn new() -> Style {
        Style { ..DEFAULT }
    }

    pub const fn color(&self, color: Color) -> Style {
        self.merge(&Self {
            color: Some(color),
            ..DEFAULT
        })
    }

    pub const fn background(&self, background: Color) -> Style {
        self.merge(&Self {
            background: Some(background),
            ..DEFAULT
        })
    }

    pub const fn attr(&self, attribute: Attribute) -> Style {
        let mut style = Style::new();

        match attribute {
            Attribute::Bold => style.bold = true,
            Attribute::Dim => style.dim = true,
            Attribute::Italic => style.italic = true,
            Attribute::Underlined => style.underlined = true,
            Attribute::Blink => style.blink = true,
            Attribute::Reverse => style.reverse = true,
            Attribute::Hidden => style.hidden = true,
        }

        self.merge(&style)
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
    pub const fn color256(&self, x: u8) -> Style {
        self.color(Color::Color256(x))
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
    pub const fn on_color256(&self, x: u8) -> Style {
        self.background(Color::Color256(x))
    }

    pub const fn bold(&self) -> Style {
        self.attr(Attribute::Bold)
    }
    pub const fn dim(&self) -> Style {
        self.attr(Attribute::Dim)
    }
    pub const fn italic(&self) -> Style {
        self.attr(Attribute::Italic)
    }
    pub const fn underlined(&self) -> Style {
        self.attr(Attribute::Underlined)
    }
    pub const fn blink(&self) -> Style {
        self.attr(Attribute::Blink)
    }
    pub const fn reverse(&self) -> Style {
        self.attr(Attribute::Reverse)
    }
    pub const fn hidden(&self) -> Style {
        self.attr(Attribute::Hidden)
    }

    pub const fn merge(&self, style: &Style) -> Style {
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
        }
    }

    pub fn apply<D>(&self, val: D) -> console::StyledObject<D> {
        self.to_console_style().apply_to(val)
    }

    pub fn to_console_style(&self) -> console::Style {
        let mut style = console::Style::new();

        if let Some(fg) = self.color {
            style = style.fg(fg);
        };

        if let Some(bg) = self.background {
            style = style.bg(bg);
        };

        if self.bold {
            style = style.bold()
        }
        if self.dim {
            style = style.dim()
        }
        if self.italic {
            style = style.italic()
        }
        if self.underlined {
            style = style.underlined()
        }
        if self.blink {
            style = style.blink()
        }
        if self.reverse {
            style = style.reverse()
        }
        if self.hidden {
            style = style.hidden()
        }

        style
    }
}

pub trait Stylable
where
    Self: Sized,
{
    fn apply_style(self, style: Style) -> console::StyledObject<Self>;
}

impl<'a> Stylable for &'a str {
    fn apply_style(self, style: Style) -> console::StyledObject<Self> {
        style.apply(self)
    }
}

impl Stylable for String {
    fn apply_style(self, style: Style) -> console::StyledObject<Self> {
        style.apply(self)
    }
}

#[cfg(test)]
mod tests {
    // #![feature(trace_macros)]
    // trace_macros!(true);

    use console::Color;

    #[test]
    fn style_macro_should_accept_a_color() {
        let style = style! { color: White };
        assert_eq!(style.color, Some(Color::White))
    }

    #[test]
    fn style_macro_should_allow_a_trailing_comma() {
        let style = style! { color: White; };
        assert_eq!(style.color, Some(Color::White));
        assert_eq!(style.background, None);
    }

    #[test]
    fn style_macro_should_accept_a_background_color() {
        let style = style! { background: Black; };
        assert_eq!(style.color, None);
        assert_eq!(style.background, Some(Color::Black));
    }

    #[test]
    fn style_macro_should_accept_a_color_with_background_with_on_keyword() {
        let style = style! { color: White on Black; };
        assert_eq!(style.color, Some(Color::White));
        assert_eq!(style.background, Some(Color::Black))
    }

    #[test]
    fn style_macro_should_accept_an_attribute() {
        let style = style! { Bold; Underlined; };
        assert_eq!(style.bold, true);
        assert_eq!(style.underlined, true);
    }
}
