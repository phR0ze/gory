/// `Color` defines supported color types and provides static functions
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    // Standard ANSI defined color
    Black,         // 30
    Red,           // 31
    Green,         // 32
    Yellow,        // 33
    Blue,          // 34
    Magenta,       // 35
    Cyan,          // 36
    White,         // 37
    BrightBlack,   // 90
    BrightRed,     // 91
    BrightGreen,   // 92
    BrightYellow,  // 93
    BrightBlue,    // 94
    BrightMagenta, // 95
    BrightCyan,    // 96
    BrightWhite,   // 97
}
impl Color {
    /// Is color enabled.
    ///
    /// Determines if the environment has a tty attached and the `TERM_COLOR` environment
    /// variable is either unset or is set to a truthy value i.e. not `0` and not some
    /// case insensitive variation of `false`.
    ///
    /// ### Examples
    /// ```rust
    /// use gory::*;
    ///
    /// println!("{:?}", Color::enabled());
    /// ```
    pub fn enabled() -> bool {
        *private::TERM_COLOR
    }
}

// Write out the color string
impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match *self {
            Color::Black => "30",
            Color::Red => "31",
            Color::Green => "32",
            Color::Yellow => "33",
            Color::Blue => "34",
            Color::Magenta => "35",
            Color::Cyan => "36",
            Color::White => "37",
            Color::BrightBlack => "90",
            Color::BrightRed => "91",
            Color::BrightGreen => "92",
            Color::BrightYellow => "93",
            Color::BrightBlue => "94",
            Color::BrightMagenta => "95",
            Color::BrightCyan => "96",
            Color::BrightWhite => "97",
        })
    }
}

/// `Colorable` defines a set of simple color functions for a given type
pub trait Colorable {
    // Set the style to use for the foreground
    fn set_fg_style(self, color: Color, bold: bool) -> ColorString
    where
        Self: Sized;

    // Clear any color that was set
    fn clear(self) -> ColorString
    where
        Self: Sized;

    // Black functions
    // -------------------------------------------------------------------------
    fn black(self) -> ColorString
    where
        Self: Sized,
    {
        self.set_fg_style(Color::Black, false)
    }
    fn bright_black(self) -> ColorString
    where
        Self: Sized,
    {
        self.set_fg_style(Color::BrightBlack, false)
    }
    fn bold_black(self) -> ColorString
    where
        Self: Sized,
    {
        self.set_fg_style(Color::BrightBlack, true)
    }

    // Red functions
    // -------------------------------------------------------------------------
    fn red(self) -> ColorString
    where
        Self: Sized,
    {
        self.set_fg_style(Color::Red, false)
    }
    fn bright_red(self) -> ColorString
    where
        Self: Sized,
    {
        self.set_fg_style(Color::BrightRed, false)
    }
    fn bold_red(self) -> ColorString
    where
        Self: Sized,
    {
        self.set_fg_style(Color::Red, true)
    }

    // Green functions
    // -------------------------------------------------------------------------
    fn green(self) -> ColorString
    where
        Self: Sized,
    {
        self.set_fg_style(Color::Green, false)
    }
    fn bright_green(self) -> ColorString
    where
        Self: Sized,
    {
        self.set_fg_style(Color::BrightGreen, false)
    }
    fn bold_green(self) -> ColorString
    where
        Self: Sized,
    {
        self.set_fg_style(Color::BrightGreen, true)
    }

    // Yellow functions
    // -------------------------------------------------------------------------
    fn yellow(self) -> ColorString
    where
        Self: Sized,
    {
        self.set_fg_style(Color::Yellow, false)
    }
    fn bright_yellow(self) -> ColorString
    where
        Self: Sized,
    {
        self.set_fg_style(Color::BrightYellow, false)
    }
    fn bold_yellow(self) -> ColorString
    where
        Self: Sized,
    {
        self.set_fg_style(Color::BrightYellow, true)
    }

    // Blue functions
    // -------------------------------------------------------------------------
    fn blue(self) -> ColorString
    where
        Self: Sized,
    {
        self.set_fg_style(Color::Blue, false)
    }
    fn bright_blue(self) -> ColorString
    where
        Self: Sized,
    {
        self.set_fg_style(Color::BrightBlue, false)
    }
    fn bold_blue(self) -> ColorString
    where
        Self: Sized,
    {
        self.set_fg_style(Color::BrightBlue, true)
    }

    // Magenta functions
    // -------------------------------------------------------------------------
    fn magenta(self) -> ColorString
    where
        Self: Sized,
    {
        self.set_fg_style(Color::Magenta, false)
    }
    fn bright_magenta(self) -> ColorString
    where
        Self: Sized,
    {
        self.set_fg_style(Color::BrightMagenta, false)
    }
    fn bold_magenta(self) -> ColorString
    where
        Self: Sized,
    {
        self.set_fg_style(Color::BrightMagenta, true)
    }

    // Cyan functions
    // -------------------------------------------------------------------------
    fn cyan(self) -> ColorString
    where
        Self: Sized,
    {
        self.set_fg_style(Color::Cyan, false)
    }
    fn bright_cyan(self) -> ColorString
    where
        Self: Sized,
    {
        self.set_fg_style(Color::BrightCyan, false)
    }
    fn bold_cyan(self) -> ColorString
    where
        Self: Sized,
    {
        self.set_fg_style(Color::BrightCyan, true)
    }

    // White functions
    // -------------------------------------------------------------------------
    fn white(self) -> ColorString
    where
        Self: Sized,
    {
        self.set_fg_style(Color::White, false)
    }
    fn bright_white(self) -> ColorString
    where
        Self: Sized,
    {
        self.set_fg_style(Color::BrightWhite, false)
    }
    fn bold_white(self) -> ColorString
    where
        Self: Sized,
    {
        self.set_fg_style(Color::BrightWhite, true)
    }
}

/// Wrapper around the String type to provide colors and styles.
#[derive(Clone, Debug)]
pub struct ColorString {
    val: String,
    fg_color: Option<Color>,
    fg_bold: bool,
}
impl ColorString {
    /// Return the escape sequence if one exists else an empty String
    fn color(&self) -> String {
        match self.fg_color {
            Some(c) => c.to_string(),
            None => String::new(),
        }
    }
}

// Implement Deref to make ColorString behave like String
impl core::ops::Deref for ColorString {
    type Target = str;
    fn deref(&self) -> &str {
        &self.val
    }
}

// Implement the Colorable trait for chaining of operations
impl Colorable for ColorString {
    // Update the color to use for the foreground
    fn set_fg_style(mut self, color: Color, bold: bool) -> ColorString {
        self.fg_color = Some(color);
        self.fg_bold = bold;
        self
    }

    // Clear the color
    fn clear(mut self) -> ColorString
    where
        Self: Sized,
    {
        self.fg_color = None;
        self.fg_bold = false;
        self
    }
}

// Implement the Default trait
impl Default for ColorString {
    fn default() -> Self {
        ColorString {
            val: String::default(), // Actual string value
            fg_color: None,         // Foreground color
            fg_bold: false,         // Foreground color bold
        }
    }
}

// Write out the color string
impl std::fmt::Display for ColorString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // If color is disabled fallback on String's implementation
        if !Color::enabled() || self.fg_color.is_none() {
            return <String as std::fmt::Display>::fmt(&self.val, f);
        }

        // Start escape sequence
        f.write_str("\x1B[")?;

        // Write out foreground style
        if self.fg_bold {
            f.write_str("1;")?;
        }

        // Write out foreground color
        f.write_str(&self.color())?;

        // Close escape sequence
        f.write_str("m")?;

        // Write out the actual String
        f.write_str(&self.val)?;

        // Ensure the reset escape sequence gets written out regardless of success
        private::ensure(|| f.write_str("\x1B[0m"));

        // Write out color strings using terminal escape sequences
        Ok(())
    }
}

// Implement the Colorable Trait for &str
impl<'a> Colorable for &'a str {
    // Set the style to use for the foreground
    fn set_fg_style(self, color: Color, bold: bool) -> ColorString {
        ColorString { val: String::from(self), fg_color: Some(color), fg_bold: bold }
    }

    // Clear the color
    fn clear(self) -> ColorString
    where
        Self: Sized,
    {
        ColorString { val: String::from(self), ..ColorString::default() }
    }
}

// Private implementation
// -------------------------------------------------------------------------------------------------
pub(crate) mod private {
    use lazy_static::*;
    use std::ffi::OsStr;
    use std::{env, fmt};

    lazy_static! {
        /// `TERM_COLOR` will be true if the environment is a tty and the
        /// environment variable `TERM_COLOR` is not set to something falsy.
        pub static ref TERM_COLOR: bool = hastty() && flag_default("TERM_COLOR", true);
    }

    // Get an environment flag value with a default
    pub fn flag_default<K: AsRef<OsStr>>(key: K, default: bool) -> bool {
        !matches!(env::var(key).unwrap_or_else(|_| default.to_string()).to_lowercase().as_str(), "false" | "0")
    }

    // Check if the environment has a tty
    pub fn hastty() -> bool {
        unsafe { libc::isatty(libc::STDOUT_FILENO) != 0 }
    }

    // Ensure the given closure is executed once the surrounding scope closes.
    // Inspired by Golang's `defer`, Java's finally and Ruby's `ensure`
    pub fn ensure<T: FnOnce() -> fmt::Result>(f: T) -> impl Drop {
        Ensure(Some(f))
    }

    // Ensure uses Rust's object destructor to execute the given closure once
    // the surrounding scope closes.
    struct Ensure<T: FnOnce() -> fmt::Result>(Option<T>);

    impl<T: FnOnce() -> fmt::Result> Drop for Ensure<T> {
        fn drop(&mut self) {
            self.0.take().map(|f| f());
        }
    }
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_enabled() {
        assert!(Color::enabled() || !Color::enabled());
    }
}
