use super::super::input::{
    WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationCanonicalEntryKind,
    WorthQueryDeclarationCanonicalValue,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryAsyncRequestIdentityValue {
    Text(String),
    Unsigned(u64),
    Bytes4([u8; 4]),
    Bytes32([u8; 32]),
    RangeU32 { start: u32, end: u32 },
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthQueryAsyncRequestIdentityPart {
    key: String,
    value: WorthQueryAsyncRequestIdentityValue,
}

impl WorthQueryAsyncRequestIdentityPart {
    pub fn text(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self::new(key, WorthQueryAsyncRequestIdentityValue::Text(value.into()))
    }

    pub fn unsigned(key: impl Into<String>, value: u64) -> Self {
        Self::new(key, WorthQueryAsyncRequestIdentityValue::Unsigned(value))
    }

    pub fn bytes4(key: impl Into<String>, value: [u8; 4]) -> Self {
        Self::new(key, WorthQueryAsyncRequestIdentityValue::Bytes4(value))
    }

    pub fn bytes32(key: impl Into<String>, value: [u8; 32]) -> Self {
        Self::new(key, WorthQueryAsyncRequestIdentityValue::Bytes32(value))
    }

    pub fn range_u32(key: impl Into<String>, start: u32, end: u32) -> Self {
        Self::new(
            key,
            WorthQueryAsyncRequestIdentityValue::RangeU32 { start, end },
        )
    }

    fn new(key: impl Into<String>, value: WorthQueryAsyncRequestIdentityValue) -> Self {
        Self {
            key: key.into(),
            value,
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &WorthQueryAsyncRequestIdentityValue {
        &self.value
    }

    pub(crate) fn has_blank_text_value(&self) -> bool {
        matches!(&self.value, WorthQueryAsyncRequestIdentityValue::Text(value) if value.trim().is_empty())
    }

    pub(crate) fn evidence_component(&self) -> String {
        format!(
            "{}:{}:{}",
            self.key.len(),
            self.key,
            self.value.evidence_component()
        )
    }

    pub(super) fn canonical_entries(&self, base: &str) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        let mut entries = vec![
            WorthQueryDeclarationCanonicalEntry::new(
                format!("{base}.key"),
                WorthQueryDeclarationCanonicalEntryKind::Identity,
                WorthQueryDeclarationCanonicalValue::ExactText(self.key.clone()),
            ),
            WorthQueryDeclarationCanonicalEntry::new(
                format!("{base}.value_kind"),
                WorthQueryDeclarationCanonicalEntryKind::Shape,
                WorthQueryDeclarationCanonicalValue::ExactText(self.value.kind().to_owned()),
            ),
        ];
        entries.extend(self.value.canonical_entries(base));
        entries
    }
}

impl WorthQueryAsyncRequestIdentityValue {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Text(_) => "text",
            Self::Unsigned(_) => "unsigned",
            Self::Bytes4(_) => "bytes4",
            Self::Bytes32(_) => "bytes32",
            Self::RangeU32 { .. } => "range-u32",
        }
    }

    pub fn reporting_value(&self) -> String {
        self.evidence_component()
    }

    fn evidence_component(&self) -> String {
        match self {
            Self::Text(value) => format!("text:{}:{value}", value.len()),
            Self::Unsigned(value) => format!("unsigned:{value:016x}"),
            Self::Bytes4(value) => format!("bytes4:{}", hex(value)),
            Self::Bytes32(value) => format!("bytes32:{}", hex(value)),
            Self::RangeU32 { start, end } => format!("range-u32:{start:08x}:{end:08x}"),
        }
    }

    fn canonical_entries(&self, base: &str) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        let identity = WorthQueryDeclarationCanonicalEntryKind::Identity;
        match self {
            Self::Text(value) => vec![WorthQueryDeclarationCanonicalEntry::new(
                format!("{base}.text"),
                identity,
                WorthQueryDeclarationCanonicalValue::ExactText(value.clone()),
            )],
            Self::Unsigned(value) => vec![WorthQueryDeclarationCanonicalEntry::new(
                format!("{base}.unsigned"),
                identity,
                WorthQueryDeclarationCanonicalValue::UnsignedInteger((*value).into()),
            )],
            Self::Bytes4(value) => fixed_bytes_entry(base, "bytes4", value),
            Self::Bytes32(value) => fixed_bytes_entry(base, "bytes32", value),
            Self::RangeU32 { start, end } => vec![
                WorthQueryDeclarationCanonicalEntry::new(
                    format!("{base}.range.start"),
                    identity,
                    WorthQueryDeclarationCanonicalValue::UnsignedInteger((*start).into()),
                ),
                WorthQueryDeclarationCanonicalEntry::new(
                    format!("{base}.range.end"),
                    identity,
                    WorthQueryDeclarationCanonicalValue::UnsignedInteger((*end).into()),
                ),
            ],
        }
    }
}

fn fixed_bytes_entry(
    base: &str,
    field: &str,
    value: &[u8],
) -> Vec<WorthQueryDeclarationCanonicalEntry> {
    vec![WorthQueryDeclarationCanonicalEntry::new(
        format!("{base}.{field}"),
        WorthQueryDeclarationCanonicalEntryKind::Identity,
        WorthQueryDeclarationCanonicalValue::ExactText(hex(value)),
    )]
}

fn hex(value: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(value.len() * 2);
    for byte in value {
        encoded.push(DIGITS[(byte >> 4) as usize] as char);
        encoded.push(DIGITS[(byte & 0x0f) as usize] as char);
    }
    encoded
}
