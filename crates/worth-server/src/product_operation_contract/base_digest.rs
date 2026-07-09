#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerProductOperationBaseDigest {
    value: String,
    canonical_digest: String,
}

impl WorthServerProductOperationBaseDigest {
    pub fn new(value: impl Into<String>) -> Result<Self, &'static str> {
        let value = Self::canonicalize_text(value)?;
        Ok(Self {
            canonical_digest: format!("worth-server-product-base-digest-v1|value:{value}"),
            value,
        })
    }

    pub fn canonicalize_text(value: impl Into<String>) -> Result<String, &'static str> {
        let mut value = value.into().trim().to_string();
        if value.is_empty() {
            return Err("product operation base digest may not be blank");
        }
        if !value.starts_with("basis:") {
            value = format!("basis:{value}");
        }
        if value
            .chars()
            .any(|ch| ch.is_control() || !ch.is_ascii() || ch.is_whitespace())
        {
            return Err("product operation base digest must stay ASCII-printable");
        }
        Ok(value)
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
