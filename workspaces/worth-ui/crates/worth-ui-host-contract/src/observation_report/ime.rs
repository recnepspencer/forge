use std::ops::Range;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostUnicodeScalarRange {
    start: u32,
    end: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostUtf8ByteRange {
    start: u32,
    end: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostImeRangeConversionReceipt {
    source: UiHostUnicodeScalarRange,
    canonical: UiHostUtf8ByteRange,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostImePreeditSelection {
    Unspecified,
    Converted(UiHostImeRangeConversionReceipt),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiHostImePreedit {
    text: Box<str>,
    selection: UiHostImePreeditSelection,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiHostImeCompositionPhase {
    Preedit(UiHostImePreedit),
    Commit(Box<str>),
    Cancel,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostImePreeditConstructionDenial {
    EmptyPreedit,
    RangeReversed,
    RangeOutsideText,
    CoordinateOverflow,
}

impl UiHostImePreedit {
    pub fn from_unicode_scalar_range(
        text: impl Into<Box<str>>,
        active_range: Option<Range<usize>>,
    ) -> Result<Self, UiHostImePreeditConstructionDenial> {
        let text = text.into();
        if text.is_empty() {
            return Err(UiHostImePreeditConstructionDenial::EmptyPreedit);
        }
        let selection = match active_range {
            Some(range) => UiHostImePreeditSelection::Converted(convert_range(&text, range)?),
            None => UiHostImePreeditSelection::Unspecified,
        };
        Ok(Self { text, selection })
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub const fn selection(&self) -> UiHostImePreeditSelection {
        self.selection
    }

    pub(crate) fn encoded_len(&self) -> usize {
        self.text.len()
            + 1
            + usize::from(matches!(
                self.selection,
                UiHostImePreeditSelection::Converted(_)
            )) * 16
    }
}

impl UiHostUnicodeScalarRange {
    pub const fn start(self) -> u32 {
        self.start
    }

    pub const fn end(self) -> u32 {
        self.end
    }
}

impl UiHostUtf8ByteRange {
    pub const fn start(self) -> u32 {
        self.start
    }

    pub const fn end(self) -> u32 {
        self.end
    }
}

impl UiHostImeRangeConversionReceipt {
    pub const fn source(self) -> UiHostUnicodeScalarRange {
        self.source
    }

    pub const fn canonical(self) -> UiHostUtf8ByteRange {
        self.canonical
    }
}

fn convert_range(
    text: &str,
    range: Range<usize>,
) -> Result<UiHostImeRangeConversionReceipt, UiHostImePreeditConstructionDenial> {
    if range.start > range.end {
        return Err(UiHostImePreeditConstructionDenial::RangeReversed);
    }
    let scalar_count = text.chars().count();
    if range.end > scalar_count {
        return Err(UiHostImePreeditConstructionDenial::RangeOutsideText);
    }
    let source = UiHostUnicodeScalarRange {
        start: u32::try_from(range.start)
            .map_err(|_| UiHostImePreeditConstructionDenial::CoordinateOverflow)?,
        end: u32::try_from(range.end)
            .map_err(|_| UiHostImePreeditConstructionDenial::CoordinateOverflow)?,
    };
    let canonical = UiHostUtf8ByteRange {
        start: byte_offset(text, range.start)?,
        end: byte_offset(text, range.end)?,
    };
    Ok(UiHostImeRangeConversionReceipt { source, canonical })
}

fn byte_offset(
    text: &str,
    scalar_offset: usize,
) -> Result<u32, UiHostImePreeditConstructionDenial> {
    let offset = if scalar_offset == text.chars().count() {
        text.len()
    } else {
        text.char_indices()
            .nth(scalar_offset)
            .map(|(offset, _)| offset)
            .ok_or(UiHostImePreeditConstructionDenial::RangeOutsideText)?
    };
    u32::try_from(offset).map_err(|_| UiHostImePreeditConstructionDenial::CoordinateOverflow)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unicode_scalar_range_converts_to_exact_utf8_boundaries() {
        let preedit = UiHostImePreedit::from_unicode_scalar_range("aé🦀z", Some(1..3))
            .expect("valid scalar range");
        let UiHostImePreeditSelection::Converted(receipt) = preedit.selection() else {
            panic!("range should carry a conversion receipt");
        };
        assert_eq!(
            receipt.source(),
            UiHostUnicodeScalarRange { start: 1, end: 3 }
        );
        assert_eq!(
            receipt.canonical(),
            UiHostUtf8ByteRange { start: 1, end: 7 }
        );
    }

    #[test]
    fn empty_reversed_and_outside_preedit_coordinates_are_typed_denials() {
        assert_eq!(
            UiHostImePreedit::from_unicode_scalar_range("", None),
            Err(UiHostImePreeditConstructionDenial::EmptyPreedit)
        );
        assert_eq!(
            UiHostImePreedit::from_unicode_scalar_range("abc", Some(2..1)),
            Err(UiHostImePreeditConstructionDenial::RangeReversed)
        );
        assert_eq!(
            UiHostImePreedit::from_unicode_scalar_range("abc", Some(0..4)),
            Err(UiHostImePreeditConstructionDenial::RangeOutsideText)
        );
    }
}
