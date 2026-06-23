use crate::runtime::WorthUiPrimitiveSourceSpan;
use crate::source::WorthUiSurfaceAuthoringValue;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiAuthoredSurfacePropValue {
    Identifier(String),
    NumberLiteral(u32),
    StringLiteral(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAuthoredSurfacePropEntry {
    surface_id: String,
    key: String,
    value: WorthUiAuthoredSurfacePropValue,
    source_span: Option<WorthUiPrimitiveSourceSpan>,
    digest: u64,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorthUiAuthoredSurfacePropsCatalog {
    entries: Vec<WorthUiAuthoredSurfacePropEntry>,
}

impl WorthUiAuthoredSurfacePropsCatalog {
    pub(crate) fn from_entries(mut entries: Vec<WorthUiAuthoredSurfacePropEntry>) -> Self {
        entries.sort_by(|left, right| {
            left.surface_id()
                .cmp(right.surface_id())
                .then_with(|| left.key().cmp(right.key()))
                .then_with(|| left.value().cmp(right.value()))
                .then_with(|| left.digest().cmp(&right.digest()))
        });
        entries.dedup();
        Self { entries }
    }

    pub fn entries(&self) -> &[WorthUiAuthoredSurfacePropEntry] {
        &self.entries
    }

    pub fn entries_for_surface<'a>(
        &'a self,
        surface_id: &str,
    ) -> impl Iterator<Item = &'a WorthUiAuthoredSurfacePropEntry> + 'a {
        let surface_id = surface_id.to_owned();
        self.entries
            .iter()
            .filter(move |entry| entry.surface_id() == surface_id)
    }

    pub fn string_prop<'a>(&'a self, surface_id: &str, key: &str) -> Option<&'a str> {
        self.entries_for_surface(surface_id).find_map(move |entry| {
            if entry.key() != key {
                return None;
            }
            match entry.value() {
                WorthUiAuthoredSurfacePropValue::Identifier(value)
                | WorthUiAuthoredSurfacePropValue::StringLiteral(value) => Some(value.as_str()),
                WorthUiAuthoredSurfacePropValue::NumberLiteral(_) => None,
            }
        })
    }

    pub fn string_props<'a>(&'a self, surface_id: &str, key: &str) -> Vec<&'a str> {
        self.entries_for_surface(surface_id)
            .filter(move |entry| entry.key() == key)
            .filter_map(|entry| match entry.value() {
                WorthUiAuthoredSurfacePropValue::Identifier(value)
                | WorthUiAuthoredSurfacePropValue::StringLiteral(value) => Some(value.as_str()),
                WorthUiAuthoredSurfacePropValue::NumberLiteral(_) => None,
            })
            .collect()
    }

    pub fn number_prop(&self, surface_id: &str, key: &str) -> Option<u32> {
        self.entries_for_surface(surface_id).find_map(|entry| {
            if entry.key() != key {
                return None;
            }
            match entry.value() {
                WorthUiAuthoredSurfacePropValue::NumberLiteral(value) => Some(*value),
                WorthUiAuthoredSurfacePropValue::Identifier(_)
                | WorthUiAuthoredSurfacePropValue::StringLiteral(_) => None,
            }
        })
    }

    pub fn surface_digest(&self, surface_id: &str) -> Option<u64> {
        let mut digest = None;
        for entry in self.entries_for_surface(surface_id) {
            digest = Some(fold_digest(
                digest.unwrap_or(0xcbf2_9ce4_8422_2325),
                entry.digest(),
            ));
        }
        digest
    }

    pub(crate) fn digest_basis(&self) -> Vec<String> {
        self.entries
            .iter()
            .map(|entry| {
                format!(
                    "authored_surface_prop|surface:{}|key:{}|value:{}|digest:{}",
                    entry.surface_id(),
                    entry.key(),
                    entry.value().digest_basis(),
                    entry.digest()
                )
            })
            .collect()
    }
}

impl WorthUiAuthoredSurfacePropEntry {
    pub(crate) fn new(
        surface_id: impl Into<String>,
        key: impl Into<String>,
        value: WorthUiAuthoredSurfacePropValue,
        source_span: Option<WorthUiPrimitiveSourceSpan>,
        digest: u64,
    ) -> Self {
        Self {
            surface_id: surface_id.into(),
            key: key.into(),
            value,
            source_span,
            digest,
        }
    }

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &WorthUiAuthoredSurfacePropValue {
        &self.value
    }

    pub fn source_span(&self) -> Option<WorthUiPrimitiveSourceSpan> {
        self.source_span
    }

    pub fn digest(&self) -> u64 {
        self.digest
    }
}

impl WorthUiAuthoredSurfacePropValue {
    pub(crate) fn from_source_value(value: WorthUiSurfaceAuthoringValue<'_>) -> Self {
        match value {
            WorthUiSurfaceAuthoringValue::Identifier(value) => Self::Identifier(value.to_owned()),
            WorthUiSurfaceAuthoringValue::NumberLiteral(value) => Self::NumberLiteral(value),
            WorthUiSurfaceAuthoringValue::StringLiteral(value) => {
                Self::StringLiteral(value.to_owned())
            }
        }
    }

    pub(crate) fn digest_basis(&self) -> String {
        match self {
            Self::Identifier(value) => format!("identifier:{value}"),
            Self::NumberLiteral(value) => format!("number:{value}"),
            Self::StringLiteral(value) => format!("string:{value}"),
        }
    }
}

impl Ord for WorthUiAuthoredSurfacePropValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.digest_basis().cmp(&other.digest_basis())
    }
}

impl PartialOrd for WorthUiAuthoredSurfacePropValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn fold_digest(current: u64, next: u64) -> u64 {
    current.rotate_left(7) ^ next.wrapping_mul(0x0000_0100_0000_01b3)
}
