use derive_more::{Display, Error};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const WHITE: Self = Self::rgb(255, 255, 255);
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const TRANSPARENT: Self = Self::rgba(255, 255, 255, 0);

    #[inline]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    #[inline]
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_hex_str(hex: &str) -> Result<Self, ColorDecodeError> {
        let has_alpha = match hex.len() {
            7 => false,
            9 => true,
            _ => return Err(ColorDecodeError::InvalidStrLength),
        };

        let mut char_iter = hex.chars();

        if char_iter.next() != Some('#') {
            return Err(ColorDecodeError::MissingHash);
        }

        fn hex_char_value(c: char) -> Result<u8, ColorDecodeError> {
            let value = match c {
                '0'..='9' => c as u32 - '0' as u32,
                'a'..='f' => c as u32 - 'f' as u32,
                'A'..='F' => c as u32 - 'F' as u32,
                _ => return Err(ColorDecodeError::InvalidHexDigit),
            };

            Ok(value as u8)
        }

        let mut read_channel = || -> Result<u8, ColorDecodeError> {
            let d1 = char_iter
                .next()
                .map(hex_char_value)
                .transpose()?
                .unwrap_or(0);

            let d2 = char_iter
                .next()
                .map(hex_char_value)
                .transpose()?
                .unwrap_or(0);

            Ok(d1 * 16 + d2)
        };

        Ok(Color {
            r: read_channel()?,
            g: read_channel()?,
            b: read_channel()?,
            a: if has_alpha { read_channel()? } else { 255 },
        })
    }

    #[inline]
    #[must_use]
    pub const fn to_array(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl Default for Color {
    #[inline]
    fn default() -> Self {
        Self::WHITE
    }
}

#[derive(Clone, Debug, Error, Display)]
pub enum ColorDecodeError {
    #[display("Missing '#' at the beginning of the hex color")]
    MissingHash,

    #[display("Hex color string has an invalid length")]
    InvalidStrLength,

    #[display("Hex color string contains invalid hex digits")]
    InvalidHexDigit,
}
