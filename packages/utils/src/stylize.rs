pub use owo_colors::{AnsiColors, Effect, Styled};

const DEFAULT: Style = Style {
    color: None,
    background: None,
    bold: false,
    dim: false,
    italic: false,
    underlined: false,
    blink: false,
    blink_fast: false,
    reverse: false,
    hidden: false,
    strikethrough: false,
};

#[derive(Clone, Copy)]
pub struct Style {
    color: Option<AnsiColors>,
    background: Option<AnsiColors>,
    bold: bool,
    dim: bool,
    italic: bool,
    underlined: bool,
    blink: bool,
    reverse: bool,
    hidden: bool,
    blink_fast: bool,
    strikethrough: bool,
}

#[macro_export]
macro_rules! style {
    /* returns final expression if no more tokens remain */
    (@props () -> ($style:expr)) => { $style };
    /* Adds a color to the style */
    (@props (color: $fg:ident; $($rest:tt)*) -> ($style:expr)) => {
        style!(@props ($($rest)*) -> ($style.color($crate::stylize::AnsiColors::$fg)))
    };
    /* Adds a background to the style */
    (@props (background: $bg:ident; $($rest:tt)*) -> ($style:expr)) => {
        style!(@props ($($rest)*) -> ($style.background($crate::stylize::AnsiColors::$bg)))
    };
    /* Adds a color and matching background to the style */
    (@props (color: $fg:ident on $bg:ident; $($rest:tt)*) -> ($style:expr)) => {
        style!(@props ($($rest)*) -> ($style.color($crate::stylize::AnsiColors::$fg).background($crate::stylize::AnsiColors::$bg)))
    };
    /* Adds an attribute to the style */
    (@props ($attr:ident; $($rest:tt)*) -> ($style:expr)) => {
        style!(@props ($($rest)*) -> ($style.attr($crate::stylize::Effect::$attr)))
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

    #[must_use]
    pub const fn color(&self, color: AnsiColors) -> Style {
        self.merge(&Self {
            color: Some(color),
            ..DEFAULT
        })
    }

    #[must_use]
    pub const fn background(&self, background: AnsiColors) -> Style {
        self.merge(&Self {
            background: Some(background),
            ..DEFAULT
        })
    }

    #[must_use]
    pub const fn attr(&self, attribute: Effect) -> Style {
        let mut style = Style::new();

        match attribute {
            Effect::Bold => style.bold = true,
            Effect::Dimmed => style.dim = true,
            Effect::Italic => style.italic = true,
            Effect::Underline => style.underlined = true,
            Effect::Blink => style.blink = true,
            Effect::Reversed => style.reverse = true,
            Effect::Hidden => style.hidden = true,
            Effect::BlinkFast => style.blink_fast = true,
            Effect::Strikethrough => style.strikethrough = true,
        }

        self.merge(&style)
    }

    #[must_use]
    pub const fn black(&self) -> Style {
        self.color(AnsiColors::Black)
    }
    #[must_use]
    pub const fn red(&self) -> Style {
        self.color(AnsiColors::Red)
    }
    #[must_use]
    pub const fn green(&self) -> Style {
        self.color(AnsiColors::Green)
    }
    #[must_use]
    pub const fn yellow(&self) -> Style {
        self.color(AnsiColors::Yellow)
    }
    #[must_use]
    pub const fn blue(&self) -> Style {
        self.color(AnsiColors::Blue)
    }
    #[must_use]
    pub const fn magenta(&self) -> Style {
        self.color(AnsiColors::Magenta)
    }
    #[must_use]
    pub const fn cyan(&self) -> Style {
        self.color(AnsiColors::Cyan)
    }
    #[must_use]
    pub const fn white(&self) -> Style {
        self.color(AnsiColors::White)
    }

    #[must_use]
    pub const fn on_black(&self) -> Style {
        self.background(AnsiColors::Black)
    }
    #[must_use]
    pub const fn on_red(&self) -> Style {
        self.background(AnsiColors::Red)
    }
    #[must_use]
    pub const fn on_green(&self) -> Style {
        self.background(AnsiColors::Green)
    }
    #[must_use]
    pub const fn on_yellow(&self) -> Style {
        self.background(AnsiColors::Yellow)
    }
    #[must_use]
    pub const fn on_blue(&self) -> Style {
        self.background(AnsiColors::Blue)
    }
    #[must_use]
    pub const fn on_magenta(&self) -> Style {
        self.background(AnsiColors::Magenta)
    }
    #[must_use]
    pub const fn on_cyan(&self) -> Style {
        self.background(AnsiColors::Cyan)
    }
    #[must_use]
    pub const fn on_white(&self) -> Style {
        self.background(AnsiColors::White)
    }

    #[must_use]
    pub const fn bold(&self) -> Style {
        self.attr(Effect::Bold)
    }
    #[must_use]
    pub const fn dim(&self) -> Style {
        self.attr(Effect::Dimmed)
    }
    #[must_use]
    pub const fn italic(&self) -> Style {
        self.attr(Effect::Italic)
    }
    #[must_use]
    pub const fn underlined(&self) -> Style {
        self.attr(Effect::Underline)
    }
    #[must_use]
    pub const fn blink(&self) -> Style {
        self.attr(Effect::Blink)
    }
    #[must_use]
    pub const fn reverse(&self) -> Style {
        self.attr(Effect::Reversed)
    }
    #[must_use]
    pub const fn hidden(&self) -> Style {
        self.attr(Effect::Hidden)
    }

    #[must_use]
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
            blink_fast: style.blink_fast || self.blink_fast,
            strikethrough: style.strikethrough || self.strikethrough,
        }
    }

    #[must_use]
    pub fn apply<D>(&self, val: D) -> Styled<D> {
        self.to_owo_style().style(val)
    }

    #[must_use]
    fn to_owo_style(&self) -> owo_colors::Style {
        let mut style = owo_colors::Style::new();

        if let Some(fg) = self.color {
            style = style.color(fg);
        };

        if let Some(bg) = self.background {
            style = style.on_color(bg);
        };

        if self.bold {
            style = style.bold()
        }
        if self.dim {
            style = style.dimmed()
        }
        if self.italic {
            style = style.italic()
        }
        if self.underlined {
            style = style.underline()
        }
        if self.blink {
            style = style.blink()
        }
        if self.reverse {
            style = style.reversed()
        }
        if self.hidden {
            style = style.hidden()
        }
        if self.blink_fast {
            style = style.blink_fast()
        }
        if self.strikethrough {
            style = style.strikethrough()
        }

        style
    }
}

pub trait Stylable
where
    Self: Sized,
{
    #[must_use]
    fn apply_style(self, style: Style) -> Styled<Self>;
}

impl<'a> Stylable for &'a str {
    #[must_use]
    fn apply_style(self, style: Style) -> Styled<Self> {
        style.apply(self)
    }
}

impl Stylable for String {
    #[must_use]
    fn apply_style(self, style: Style) -> Styled<Self> {
        style.apply(self)
    }
}

#[cfg(test)]
mod tests {
    // #![feature(trace_macros)]
    // trace_macros!(true);

    use owo_colors::AnsiColors;

    #[test]
    fn style_macro_should_accept_a_color() {
        let style = style! { color: White };
        assert_eq!(style.color, Some(AnsiColors::White))
    }

    #[test]
    fn style_macro_should_allow_a_trailing_comma() {
        let style = style! { color: White; };
        assert_eq!(style.color, Some(AnsiColors::White));
        assert_eq!(style.background, None);
    }

    #[test]
    fn style_macro_should_accept_a_background_color() {
        let style = style! { background: Black; };
        assert_eq!(style.color, None);
        assert_eq!(style.background, Some(AnsiColors::Black));
    }

    #[test]
    fn style_macro_should_accept_a_color_with_background_with_on_keyword() {
        let style = style! { color: White on Black; };
        assert_eq!(style.color, Some(AnsiColors::White));
        assert_eq!(style.background, Some(AnsiColors::Black))
    }

    #[test]
    fn style_macro_should_accept_an_attribute() {
        let style = style! { Bold; Underline; };
        assert!(style.bold);
        assert!(style.underlined);
    }
}
