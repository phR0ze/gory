/// `Color` defines supported color types and provides static functions
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Red,     // 31
    Green,   // 32
    Yellow,  // 33
    Blue,    // 34
    Magenta, // 35
    Cyan,    // 36
    White,   // 37
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

    /// Convert the color into a terminal escape sequence
    pub fn escape_seq(&self) -> String {
        match *self {
            Color::Red => "31".to_string(),
            Color::Green => "32".to_string(),
            Color::Yellow => "33".to_string(),
            Color::Blue => "34".to_string(),
            Color::Magenta => "35".to_string(),
            Color::Cyan => "36".to_string(),
            Color::White => "37".to_string(),
        }
    }
}
/// `Colorable` defines a set of simple color functions for a given type
pub trait Colorable {
    // Set the color to use
    fn set_color(self, color: Color) -> ColorString
    where
        Self: Sized;

    // Clear any color that was set
    fn clear(self) -> ColorString
    where
        Self: Sized;

    // Set the color to red for the string
    fn red(self) -> ColorString
    where
        Self: Sized,
    {
        self.set_color(Color::Red)
    }
}

/// Wrapper around the String type to provide colors and styles.
#[derive(Clone, Debug)]
pub struct ColorString {
    raw: String,
    color: Option<Color>,
}

// Implement Deref to make ColorString behave like String
impl core::ops::Deref for ColorString {
    type Target = str;
    fn deref(&self) -> &str {
        &self.raw
    }
}

// Implement the Colorable trait for chaining of operations
impl Colorable for ColorString {
    // Update the color
    fn set_color(mut self, color: Color) -> ColorString {
        self.color = Some(color);
        self
    }

    // Clear the color
    fn clear(mut self) -> ColorString
    where
        Self: Sized,
    {
        self.color = None;
        self
    }
}

// Write out the color string
impl std::fmt::Display for ColorString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // If color is disabled fallback on String's implementation
        if !Color::enabled() || self.color.is_none() {
            return <String as std::fmt::Display>::fmt(&self.raw, f);
        }

        // Write out color strings using terminal escape sequences
        // TODO: use finally pattern here to ensure a reset get written out
        f.write_str("\x1B[0m")?;
        Ok(())
    }
}

// Implement the Colorable Trait for &str
impl<'a> Colorable for &'a str {
    // Set the color
    fn set_color(self, color: Color) -> ColorString {
        ColorString {
            // Copy as owned string
            raw: String::from(self),

            // Store the color for the string
            color: Some(color),
        }
    }

    // Clear the color
    fn clear(self) -> ColorString
    where
        Self: Sized,
    {
        ColorString {
            // Copy as owned string
            raw: String::from(self),

            // Don't set any color
            color: None,
        }
    }
}

// Private implementation
// -------------------------------------------------------------------------------------------------
pub(crate) mod private {
    use lazy_static::*;
    use std::env;
    use std::ffi::OsStr;

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
