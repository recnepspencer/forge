#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerProductOperationBaseDigest {
    value: String,
    canonical_digest: String,
}

impl ForgeServerProductOperationBaseDigest {
    pub fn new(value: impl Into<String>) -> Result<Self, &'static str> {
        let value = value.into().trim().to_string();
        if value.is_empty() {
            return Err("product operation base digest may not be blank");
        }
        if !value.starts_with("basis:") {
            return Err("product operation base digest must start with `basis:`");
        }
        if value
            .chars()
            .any(|ch| ch.is_control() || !ch.is_ascii() || ch.is_whitespace())
        {
            return Err("product operation base digest must stay ASCII-printable");
        }
        Ok(Self {
            canonical_digest: format!("forge-server-product-base-digest-v1|value:{value}"),
            value,
        })
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
