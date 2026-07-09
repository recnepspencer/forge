#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerProductIdempotencyKey {
    value: String,
    canonical_digest: String,
}

impl WorthServerProductIdempotencyKey {
    pub fn new(value: impl Into<String>) -> Result<Self, &'static str> {
        let value = value.into().trim().to_string();
        if value.is_empty() {
            return Err("product idempotency key may not be blank");
        }
        if value
            .chars()
            .any(|ch| ch.is_control() || !ch.is_ascii() || ch.is_whitespace())
        {
            return Err("product idempotency key must stay ASCII-printable");
        }
        Ok(Self {
            canonical_digest: format!("worth-server-product-idempotency-key-v1|value:{value}"),
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
