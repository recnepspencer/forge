#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ThemeColorValue {
    hex: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ThemeColorValueError {
    value: String,
}

impl ThemeColorValue {
    pub fn hex(value: impl Into<String>) -> Result<Self, ThemeColorValueError> {
        let value = value.into();
        if is_hex_color_literal(&value) {
            Ok(Self { hex: value })
        } else {
            Err(ThemeColorValueError { value })
        }
    }

    pub fn invalid_for_diagnostics(value: impl Into<String>) -> Self {
        Self { hex: value.into() }
    }

    pub fn as_str(&self) -> &str {
        &self.hex
    }

    pub(crate) fn is_valid(&self) -> bool {
        is_hex_color_literal(&self.hex)
    }

    pub(crate) fn digest_basis(&self) -> String {
        length_prefixed(&self.hex)
    }
}

impl ThemeColorValueError {
    pub fn value(&self) -> &str {
        &self.value
    }
}

fn is_hex_color_literal(value: &str) -> bool {
    let Some(hex_digits) = value.strip_prefix('#') else {
        return false;
    };
    matches!(hex_digits.len(), 6 | 8)
        && hex_digits
            .chars()
            .all(|character| character.is_ascii_hexdigit())
}

fn length_prefixed(value: &str) -> String {
    format!("{}:{value}", value.len())
}
