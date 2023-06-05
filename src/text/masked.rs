use std::{
    borrow::Cow,
    fmt::{self, Debug, Display},
};

use super::Text;

/// A wrapper around a string that is masked when displayed.
///
/// The masked string is displayed as a series of the same character.
/// This might be used to display a password field or similar secure data.
///
/// # Examples
///
/// ```rust
/// use ratatui::{buffer::Buffer, layout::Rect, text::Masked, widgets::{Paragraph, Widget}};
///
/// let mut buffer = Buffer::empty(Rect::new(0, 0, 5, 1));
/// let password = Masked::new("12345", 'x');
///
/// Paragraph::new(password).render(buffer.area, &mut buffer);
/// assert_eq!(buffer, Buffer::with_lines(vec!["xxxxx"]));
/// ```
#[derive(Clone)]
pub struct Masked<'a> {
    inner: Cow<'a, str>,
    mask_char: char,
}

impl<'a> Masked<'a> {
    pub fn new(s: impl Into<Cow<'a, str>>, mask_char: char) -> Self {
        Self {
            inner: s.into(),
            mask_char,
        }
    }

    /// The character to use for masking.
    pub fn mask_char(&self) -> char {
        self.mask_char
    }

    /// The underlying string, with all characters masked.
    pub fn value(&self) -> Cow<'a, str> {
        self.inner.chars().map(|_| self.mask_char).collect()
    }
}

impl Debug for Masked<'_> {
    /// Debug representation of a masked string is the underlying string
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.inner).map_err(|_| fmt::Error)
    }
}

impl Display for Masked<'_> {
    /// Display representation of a masked string is the masked string
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.value()).map_err(|_| fmt::Error)
    }
}

impl<'a> From<&'a Masked<'a>> for Cow<'a, str> {
    fn from(masked: &'a Masked) -> Cow<'a, str> {
        masked.value()
    }
}

impl<'a> From<Masked<'a>> for Cow<'a, str> {
    fn from(masked: Masked<'a>) -> Cow<'a, str> {
        masked.value()
    }
}

impl<'a> From<&'a Masked<'_>> for Text<'a> {
    fn from(masked: &'a Masked) -> Text<'a> {
        Text::raw(masked.value())
    }
}

impl<'a> From<Masked<'a>> for Text<'a> {
    fn from(masked: Masked<'a>) -> Text<'a> {
        Text::raw(masked.value())
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use super::*;
    use crate::text::Line;

    #[test]
    fn test_masked_value() {
        let masked = Masked::new("12345", 'x');
        assert_eq!(masked.value(), "xxxxx");
    }

    #[test]
    fn test_masked_debug() {
        let masked = Masked::new("12345", 'x');
        assert_eq!(format!("{masked:?}"), "12345");
    }

    #[test]
    fn test_masked_display() {
        let masked = Masked::new("12345", 'x');
        assert_eq!(format!("{masked}"), "xxxxx");
    }

    #[test]
    fn test_masked_conversions() {
        let masked = Masked::new("12345", 'x');

        let text: Text = masked.borrow().into();
        assert_eq!(text.lines, vec![Line::from("xxxxx")]);

        let text: Text = masked.to_owned().into();
        assert_eq!(text.lines, vec![Line::from("xxxxx")]);

        let cow: Cow<str> = masked.borrow().into();
        assert_eq!(cow, "xxxxx");

        let cow: Cow<str> = masked.to_owned().into();
        assert_eq!(cow, "xxxxx");
    }
}
