use crate::capability::ThemeColorValue;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiStyleValueError {
    raw_value: String,
    reason: WorthUiStyleValueErrorReason,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiStyleValueErrorReason {
    MissingPxUnit,
    InvalidNumber,
    InvalidTokenCount,
    UnsupportedKeyword,
    NegativeValue,
    OutOfRange,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthUiLengthValue {
    milli_points: u32,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthUiFontSizeValue {
    length: WorthUiLengthValue,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthUiSpacingValue {
    length: WorthUiLengthValue,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthUiBorderWidthValue {
    length: WorthUiLengthValue,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthUiCornerRadiusValue {
    length: WorthUiLengthValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPaddingValue {
    top: WorthUiLengthValue,
    right: WorthUiLengthValue,
    bottom: WorthUiLengthValue,
    left: WorthUiLengthValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiShadowValue {
    color: ThemeColorValue,
    offset_x_points: i8,
    offset_y_points: i8,
    blur_points: u8,
    spread_points: u8,
}

impl WorthUiStyleValueError {
    fn new(raw_value: impl Into<String>, reason: WorthUiStyleValueErrorReason) -> Self {
        Self {
            raw_value: raw_value.into(),
            reason,
        }
    }

    pub fn raw_value(&self) -> &str {
        &self.raw_value
    }

    pub fn reason(&self) -> WorthUiStyleValueErrorReason {
        self.reason
    }
}

impl WorthUiLengthValue {
    pub fn from_px(raw_value: impl AsRef<str>) -> Result<Self, WorthUiStyleValueError> {
        let milli_points = parse_px_milli_points(raw_value.as_ref())?;
        Ok(Self { milli_points })
    }

    pub fn points(&self) -> f32 {
        self.milli_points as f32 / 1000.0
    }

    pub(crate) fn digest_basis(&self) -> String {
        format!("mp:{}", self.milli_points)
    }
}

impl WorthUiFontSizeValue {
    pub fn from_px(raw_value: impl AsRef<str>) -> Result<Self, WorthUiStyleValueError> {
        Ok(Self {
            length: WorthUiLengthValue::from_px(raw_value)?,
        })
    }

    pub fn points(&self) -> f32 {
        self.length.points()
    }

    pub(crate) fn digest_basis(&self) -> String {
        format!("font_size:{}", self.length.digest_basis())
    }
}

impl WorthUiSpacingValue {
    pub fn from_px(raw_value: impl AsRef<str>) -> Result<Self, WorthUiStyleValueError> {
        Ok(Self {
            length: WorthUiLengthValue::from_px(raw_value)?,
        })
    }

    pub fn points(&self) -> f32 {
        self.length.points()
    }

    pub(crate) fn digest_basis(&self) -> String {
        format!("spacing:{}", self.length.digest_basis())
    }
}

impl WorthUiBorderWidthValue {
    pub fn from_px(raw_value: impl AsRef<str>) -> Result<Self, WorthUiStyleValueError> {
        Ok(Self {
            length: WorthUiLengthValue::from_px(raw_value)?,
        })
    }

    pub fn points(&self) -> f32 {
        self.length.points()
    }

    pub(crate) fn digest_basis(&self) -> String {
        format!("border_width:{}", self.length.digest_basis())
    }
}

impl WorthUiCornerRadiusValue {
    pub fn from_px(raw_value: impl AsRef<str>) -> Result<Self, WorthUiStyleValueError> {
        Ok(Self {
            length: WorthUiLengthValue::from_px(raw_value)?,
        })
    }

    pub fn points(&self) -> f32 {
        self.length.points()
    }

    pub(crate) fn digest_basis(&self) -> String {
        format!("corner_radius:{}", self.length.digest_basis())
    }
}

impl WorthUiPaddingValue {
    pub fn from_shorthand_px(raw_value: impl AsRef<str>) -> Result<Self, WorthUiStyleValueError> {
        let raw_value = raw_value.as_ref();
        let parts = raw_value.split_whitespace().collect::<Vec<_>>();
        match parts.as_slice() {
            [all] => {
                let value = WorthUiLengthValue::from_px(all)?;
                Ok(Self::from_edges(value, value, value, value))
            }
            [vertical, horizontal] => {
                let vertical = WorthUiLengthValue::from_px(vertical)?;
                let horizontal = WorthUiLengthValue::from_px(horizontal)?;
                Ok(Self::from_edges(vertical, horizontal, vertical, horizontal))
            }
            [top, right, bottom, left] => Ok(Self::from_edges(
                WorthUiLengthValue::from_px(top)?,
                WorthUiLengthValue::from_px(right)?,
                WorthUiLengthValue::from_px(bottom)?,
                WorthUiLengthValue::from_px(left)?,
            )),
            _ => Err(WorthUiStyleValueError::new(
                raw_value,
                WorthUiStyleValueErrorReason::InvalidTokenCount,
            )),
        }
    }

    pub fn top(&self) -> WorthUiLengthValue {
        self.top
    }

    pub fn right(&self) -> WorthUiLengthValue {
        self.right
    }

    pub fn bottom(&self) -> WorthUiLengthValue {
        self.bottom
    }

    pub fn left(&self) -> WorthUiLengthValue {
        self.left
    }

    pub fn horizontal_points(&self) -> f32 {
        self.left.points().max(self.right.points())
    }

    pub fn vertical_points(&self) -> f32 {
        self.top.points().max(self.bottom.points())
    }

    pub(crate) fn digest_basis(&self) -> String {
        format!(
            "padding:{}:{}:{}:{}",
            self.top.digest_basis(),
            self.right.digest_basis(),
            self.bottom.digest_basis(),
            self.left.digest_basis()
        )
    }

    fn from_edges(
        top: WorthUiLengthValue,
        right: WorthUiLengthValue,
        bottom: WorthUiLengthValue,
        left: WorthUiLengthValue,
    ) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }
}

impl WorthUiShadowValue {
    pub fn from_authored_parts(
        color: ThemeColorValue,
        offset_x: impl AsRef<str>,
        offset_y: impl AsRef<str>,
        blur: impl AsRef<str>,
        spread: impl AsRef<str>,
    ) -> Result<Self, WorthUiStyleValueError> {
        Ok(Self {
            color,
            offset_x_points: parse_i8_px(offset_x.as_ref())?,
            offset_y_points: parse_i8_px(offset_y.as_ref())?,
            blur_points: parse_u8_px(blur.as_ref())?,
            spread_points: parse_u8_px(spread.as_ref())?,
        })
    }

    pub fn color(&self) -> &ThemeColorValue {
        &self.color
    }

    pub fn offset_x_points(&self) -> i8 {
        self.offset_x_points
    }

    pub fn offset_y_points(&self) -> i8 {
        self.offset_y_points
    }

    pub fn blur_points(&self) -> u8 {
        self.blur_points
    }

    pub fn spread_points(&self) -> u8 {
        self.spread_points
    }

    pub(crate) fn digest_basis(&self) -> String {
        format!(
            "shadow:{}:{}:{}:{}:{}",
            self.color.digest_basis(),
            self.offset_x_points,
            self.offset_y_points,
            self.blur_points,
            self.spread_points
        )
    }
}

fn parse_px_milli_points(raw_value: &str) -> Result<u32, WorthUiStyleValueError> {
    let Some(number_text) = raw_value.strip_suffix("px") else {
        return Err(WorthUiStyleValueError::new(
            raw_value,
            WorthUiStyleValueErrorReason::MissingPxUnit,
        ));
    };
    parse_decimal_milli_points(number_text.trim(), raw_value)
}

fn parse_decimal_milli_points(
    number_text: &str,
    raw_value: &str,
) -> Result<u32, WorthUiStyleValueError> {
    if number_text.is_empty() {
        return Err(WorthUiStyleValueError::new(
            raw_value,
            WorthUiStyleValueErrorReason::InvalidNumber,
        ));
    }
    if number_text.starts_with('-') {
        return Err(WorthUiStyleValueError::new(
            raw_value,
            WorthUiStyleValueErrorReason::NegativeValue,
        ));
    }
    let parts = number_text.split('.').collect::<Vec<_>>();
    if parts.len() > 2 || parts.iter().any(|part| part.is_empty()) {
        return Err(WorthUiStyleValueError::new(
            raw_value,
            WorthUiStyleValueErrorReason::InvalidNumber,
        ));
    }
    if !parts
        .iter()
        .all(|part| part.chars().all(|ch| ch.is_ascii_digit()))
    {
        return Err(WorthUiStyleValueError::new(
            raw_value,
            WorthUiStyleValueErrorReason::InvalidNumber,
        ));
    }

    let whole = parts[0].parse::<u32>().map_err(|_| {
        WorthUiStyleValueError::new(raw_value, WorthUiStyleValueErrorReason::OutOfRange)
    })?;
    let fractional = match parts.get(1).copied() {
        None => 0,
        Some(text) => fractional_milli_points(text, raw_value)?,
    };
    whole
        .checked_mul(1000)
        .and_then(|whole| whole.checked_add(fractional))
        .ok_or_else(|| {
            WorthUiStyleValueError::new(raw_value, WorthUiStyleValueErrorReason::OutOfRange)
        })
}

fn fractional_milli_points(digits: &str, raw_value: &str) -> Result<u32, WorthUiStyleValueError> {
    if digits.len() > 3 {
        return Err(WorthUiStyleValueError::new(
            raw_value,
            WorthUiStyleValueErrorReason::InvalidNumber,
        ));
    }
    let padded = format!("{digits:0<3}");
    padded.parse::<u32>().map_err(|_| {
        WorthUiStyleValueError::new(raw_value, WorthUiStyleValueErrorReason::InvalidNumber)
    })
}

fn parse_i8_px(raw_value: &str) -> Result<i8, WorthUiStyleValueError> {
    let Some(number_text) = raw_value.strip_suffix("px") else {
        return Err(WorthUiStyleValueError::new(
            raw_value,
            WorthUiStyleValueErrorReason::MissingPxUnit,
        ));
    };
    if number_text.is_empty()
        || !number_text
            .chars()
            .enumerate()
            .all(|(index, ch)| ch.is_ascii_digit() || (index == 0 && ch == '-'))
    {
        return Err(WorthUiStyleValueError::new(
            raw_value,
            WorthUiStyleValueErrorReason::InvalidNumber,
        ));
    }
    number_text.parse::<i8>().map_err(|_| {
        WorthUiStyleValueError::new(raw_value, WorthUiStyleValueErrorReason::OutOfRange)
    })
}

fn parse_u8_px(raw_value: &str) -> Result<u8, WorthUiStyleValueError> {
    let milli_points = parse_px_milli_points(raw_value)?;
    if milli_points % 1000 != 0 {
        return Err(WorthUiStyleValueError::new(
            raw_value,
            WorthUiStyleValueErrorReason::InvalidNumber,
        ));
    }
    u8::try_from(milli_points / 1000).map_err(|_| {
        WorthUiStyleValueError::new(raw_value, WorthUiStyleValueErrorReason::OutOfRange)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn px_lengths_canonicalize_equivalent_decimal_forms() {
        assert_eq!(
            WorthUiLengthValue::from_px("12px").unwrap(),
            WorthUiLengthValue::from_px("12.0px").unwrap()
        );
        assert_eq!(
            WorthUiLengthValue::from_px("12.34px").unwrap().points(),
            12.34
        );
    }

    #[test]
    fn padding_shorthand_canonicalizes_to_explicit_edges() {
        assert_eq!(
            WorthUiPaddingValue::from_shorthand_px("4px 8px").unwrap(),
            WorthUiPaddingValue::from_shorthand_px("4px 8px 4px 8px").unwrap()
        );
    }

    #[test]
    fn invalid_units_and_negative_lengths_are_rejected() {
        assert!(WorthUiLengthValue::from_px("12").is_err());
        assert!(WorthUiLengthValue::from_px("-1px").is_err());
        assert!(WorthUiPaddingValue::from_shorthand_px("4em").is_err());
    }
}
