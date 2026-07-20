use serde_json::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerProductResultBody {
    value: Value,
    canonical_bytes: Vec<u8>,
}

impl WorthServerProductResultBody {
    pub(crate) fn canonical_json(value: Value) -> Result<Self, serde_json::Error> {
        let (value, canonical_bytes) = super::canonicalize_json(value)?;
        Ok(Self {
            value,
            canonical_bytes,
        })
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn canonical_bytes(&self) -> &[u8] {
        &self.canonical_bytes
    }

    pub fn byte_len(&self) -> usize {
        self.canonical_bytes.len()
    }
}
