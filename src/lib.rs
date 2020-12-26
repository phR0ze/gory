/// `Color` defines supported color types and provides static functions
///
/// ### Examples
/// ```rust
/// use gory::*;
///
/// print!("{}  ", format!("\\e[1;{}m", Color::Red).red());
/// print!("{}  ", "red".red());
/// println!();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    // Standard ANSI defined color
    Black,   // 90
    Red,     // 91
    Green,   // 92
    Yellow,  // 93
    Blue,    // 94
    Magenta, // 95
    Cyan,    // 96
    White,   // 97
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

    /// Force color to be enable or disabled regardless of attached tty or value of
    /// `TERM_COLOR` based on the `bool` given.
    ///
    /// To return to automatic color control simply call with a value of `None`.
    ///
    /// ### Examples
    /// ```rust,ignore
    /// use gory::*;
    ///
    /// Color::force(Some(true));
    /// Color::force(Some(false));
    /// Color::force(None);
    /// ```
    pub fn force(val: Option<bool>) {
        *private::FORCE_COLOR.lock().unwrap() = val;
    }

    // Internal functions to check the status of the force value
    pub(crate) fn force_on() -> bool {
        match *private::FORCE_COLOR.lock().unwrap() {
            Some(val) => val,
            None => false,
        }
    }
    pub(crate) fn force_off() -> bool {
        match *private::FORCE_COLOR.lock().unwrap() {
            Some(val) => !val,
            None => false,
        }
    }
}

// Write out the color string
impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match *self {
            Color::Black => "90",
            Color::Red => "91",
            Color::Green => "92",
            Color::Yellow => "93",
            Color::Blue => "94",
            Color::Magenta => "95",
            Color::Cyan => "96",
            Color::White => "97",
        })
    }
}

/// `Colorable` defines a set of simple color functions for a given type
pub trait Colorable {
    // Set the style to use for the foreground
    fn set_fg_style(self, color: Color) -> ColorString
    where
        Self: Sized;

    // Black functions
    // -------------------------------------------------------------------------
    fn black(self) -> ColorString
    where
        Self: Sized,
    {
        self.set_fg_style(Color::Black)
    }

    // Red functions
    // -------------------------------------------------------------------------
    fn red(self) -> ColorString
    where
        Self: Sized,
    {
        self.set_fg_style(Color::Red)
    }

    // Green functions
    // -------------------------------------------------------------------------
    fn green(self) -> ColorString
    where
        Self: Sized,
    {
        self.set_fg_style(Color::Green)
    }

    // Yellow functions
    // -------------------------------------------------------------------------
    fn yellow(self) -> ColorString
    where
        Self: Sized,
    {
        self.set_fg_style(Color::Yellow)
    }

    // Blue functions
    // -------------------------------------------------------------------------
    fn blue(self) -> ColorString
    where
        Self: Sized,
    {
        self.set_fg_style(Color::Blue)
    }

    // Magenta functions
    // -------------------------------------------------------------------------
    fn magenta(self) -> ColorString
    where
        Self: Sized,
    {
        self.set_fg_style(Color::Magenta)
    }

    // Cyan functions
    // -------------------------------------------------------------------------
    fn cyan(self) -> ColorString
    where
        Self: Sized,
    {
        self.set_fg_style(Color::Cyan)
    }

    // White functions
    // -------------------------------------------------------------------------
    fn white(self) -> ColorString
    where
        Self: Sized,
    {
        self.set_fg_style(Color::White)
    }
}

/// Wrapper around the String type to provide colors and styles.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColorString {
    inner: String,
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

    /// Clear the color styling from the String
    #[allow(dead_code)]
    fn clear(&self) -> String {
        self.inner.clone()
    }
}

// Implement Deref to make ColorString behave like String
impl core::ops::Deref for ColorString {
    type Target = str;
    fn deref(&self) -> &str {
        &self.inner
    }
}

// Implement the Colorable trait for chaining of operations
impl Colorable for ColorString {
    // Update the color to use for the foreground
    fn set_fg_style(mut self, color: Color) -> ColorString
    where
        Self: Sized,
    {
        self.fg_color = Some(color);
        self
    }
}

// Implement the Default trait
impl Default for ColorString {
    fn default() -> Self {
        ColorString {
            inner: String::default(), // Actual string value
            fg_color: None,           // Foreground color
        }
    }
}

// Write out the color string
impl std::fmt::Display for ColorString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // If color is disabled fallback on String's implementation
        if self.fg_color.is_none() || Color::force_off() || (!Color::enabled() && !Color::force_on()) {
            return <String as std::fmt::Display>::fmt(&self.inner, f);
        }

        // Needed to allow the ensure call to mutate the formatter
        let d = std::cell::RefCell::new(f);

        // Ensure the reset escape sequence gets written out regardless of success
        let _ensure = private::ensure(|| d.borrow_mut().write_str("\x1B[0m"));

        // Start escape sequence
        d.borrow_mut().write_str("\x1B[")?;

        // Always set bold to keep it bright and simple
        d.borrow_mut().write_str("1;")?;

        // Write out foreground color
        d.borrow_mut().write_str(&self.color())?;

        // Close escape sequence
        d.borrow_mut().write_str("m")?;

        // Write out the actual String
        d.borrow_mut().write_str(&self.inner)?;

        // Write out color strings using terminal escape sequences
        Ok(())
    }
}

// Implement the Colorable Trait for &str
impl<'a> Colorable for &'a str {
    // Set the style to use for the foreground
    fn set_fg_style(self, color: Color) -> ColorString
    where
        Self: Sized,
    {
        ColorString { inner: String::from(self), fg_color: Some(color) }
    }
}

// Private implementation
// -------------------------------------------------------------------------------------------------
pub(crate) mod private {
    use lazy_static::*;
    use std::ffi::OsStr;
    use std::sync::Mutex;
    use std::{env, fmt};

    lazy_static! {
        /// `TERM_COLOR` will be true if the environment is a tty and the
        /// environment variable `TERM_COLOR` is not set to something falsy.
        pub static ref TERM_COLOR: bool = hastty() && flag_default("TERM_COLOR", true);

        // Force color to be enabled or disabled based on this additional flag
        pub static ref FORCE_COLOR: Mutex<Option<bool>> = Mutex::new(None);
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

    #[test]
    fn test_colors() {
        // Force color
        assert!(!Color::force_on());
        assert!(!Color::force_off());
        Color::force(Some(true));
        assert!(Color::force_on());
        assert!(!Color::force_off());

        // Clear color
        let foo = String::from("foo").red();
        assert_eq!("\u{1b}[1;91m\u{1b}[0m", "".red().to_string());
        assert_eq!(String::from("foo"), foo.clear());

        // Deref
        assert_eq!(String::from("foo"), *foo);

        // Update color
        assert_eq!("\u{1b}[1;91m\u{1b}[0m", "".black().red().to_string());

        assert_eq!("\u{1b}[1;90m\u{1b}[0m", "".black().to_string());
        assert_eq!("\u{1b}[1;92m\u{1b}[0m", "".green().to_string());
        assert_eq!("\u{1b}[1;93m\u{1b}[0m", "".yellow().to_string());
        assert_eq!("\u{1b}[1;94m\u{1b}[0m", "".blue().to_string());
        assert_eq!("\u{1b}[1;95m\u{1b}[0m", "".magenta().to_string());
        assert_eq!("\u{1b}[1;96m\u{1b}[0m", "".cyan().to_string());
        assert_eq!("\u{1b}[1;97m\u{1b}[0m", "".white().to_string());

        Color::force(Some(false));
        assert!(Color::force_off());
        assert!(!Color::force_on());
        assert_eq!("", "".black().to_string());
        assert_eq!("", "".red().to_string());
        assert_eq!("", "".green().to_string());
        assert_eq!("", "".yellow().to_string());
        assert_eq!("", "".blue().to_string());
        assert_eq!("", "".magenta().to_string());
        assert_eq!("", "".cyan().to_string());
        assert_eq!("", "".white().to_string());

        Color::force(None);
    }
}
