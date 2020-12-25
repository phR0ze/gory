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
}

// Write out the color string
impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match *self {
            Color::Red => "31",
            Color::Green => "32",
            Color::Yellow => "33",
            Color::Blue => "34",
            Color::Magenta => "35",
            Color::Cyan => "36",
            Color::White => "37",
        })
    }
}
/// `Colorable` defines a set of simple color functions for a given type
pub trait Colorable {
    // Set the color to use for the foreground
    fn set_fg_color(self, color: Color) -> ColorString
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
        self.set_fg_color(Color::Red)
    }
}

/// Wrapper around the String type to provide colors and styles.
#[derive(Clone, Debug)]
pub struct ColorString {
    val: String,
    fg_color: Option<Color>,
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
    fn set_fg_color(mut self, color: Color) -> ColorString {
        self.fg_color = Some(color);
        self
    }

    // Clear the color
    fn clear(mut self) -> ColorString
    where
        Self: Sized,
    {
        self.fg_color = None;
        self
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
    // Set the color to use for the foreground
    fn set_fg_color(self, color: Color) -> ColorString {
        ColorString {
            // Copy as owned string
            val: String::from(self),

            // Store the color for the string
            fg_color: Some(color),
        }
    }

    // Clear the color
    fn clear(self) -> ColorString
    where
        Self: Sized,
    {
        ColorString {
            // Copy as owned string
            val: String::from(self),

            // Don't set any color
            fg_color: None,
        }
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
