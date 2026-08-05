use worth_foundational::facade::{
    CanonicalBasisDomain, CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus,
    CanonicalBasisValue, CanonicalIntegerWidth,
};

pub(super) const APPLICATION_SCHEMA_DOMAIN: CanonicalBasisDomain =
    CanonicalBasisDomain::Future("worth-query.application-schema");

pub(super) struct ApplicationSchemaCanonicalBasis {
    entries: Vec<CanonicalBasisEntry>,
}

impl ApplicationSchemaCanonicalBasis {
    pub(super) fn with_member_capacity(member_count: usize) -> Self {
        Self {
            entries: Vec::with_capacity(member_count.saturating_mul(4).saturating_add(6)),
        }
    }

    pub(super) fn text(&mut self, locus: impl Into<String>, value: &str) {
        self.push(
            locus,
            CanonicalBasisValue::ExactText(value.to_owned().into()),
        );
    }

    pub(super) fn optional_text(&mut self, locus: impl Into<String>, value: Option<&str>) {
        self.push(
            locus,
            value.map_or(CanonicalBasisValue::Null, |value| {
                CanonicalBasisValue::ExactText(value.to_owned().into())
            }),
        );
    }

    pub(super) fn bool(&mut self, locus: impl Into<String>, value: bool) {
        self.push(locus, CanonicalBasisValue::Bool(value));
    }

    pub(super) fn u32(&mut self, locus: impl Into<String>, value: u32) {
        self.push(
            locus,
            CanonicalBasisValue::UnsignedInteger {
                width: CanonicalIntegerWidth::Bits32,
                value: value.into(),
            },
        );
    }

    pub(super) fn usize(&mut self, locus: impl Into<String>, value: usize) {
        self.push(
            locus,
            CanonicalBasisValue::UnsignedInteger {
                width: CanonicalIntegerWidth::Bits64,
                value: value as u128,
            },
        );
    }

    pub(super) fn value(&mut self, locus: impl Into<String>, value: CanonicalBasisValue) {
        self.push(locus, value);
    }

    pub(super) fn into_entries(self) -> Vec<CanonicalBasisEntry> {
        self.entries
    }

    pub(super) fn extend(&mut self, entries: impl IntoIterator<Item = CanonicalBasisEntry>) {
        self.entries.extend(entries);
    }

    fn push(&mut self, locus: impl Into<String>, value: CanonicalBasisValue) {
        self.entries.push(CanonicalBasisEntry::new(
            APPLICATION_SCHEMA_DOMAIN,
            CanonicalBasisLocus::Named(locus.into().into()),
            CanonicalBasisEntryKind::Identity,
            value,
        ));
    }
}
