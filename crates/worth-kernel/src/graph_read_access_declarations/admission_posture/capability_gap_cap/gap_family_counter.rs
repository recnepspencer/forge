use super::super::query_admission_projection::WorthGraphReadAdmissionCapabilityGapKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAdmissionGapFamilyCounter {
    kind: WorthGraphReadAdmissionCapabilityGapKind,
    current_count: usize,
    must_not_exceed_count: usize,
    cap_ledger_digest_part: String,
}

impl WorthGraphReadAdmissionGapFamilyCounter {
    pub(crate) fn new(
        kind: WorthGraphReadAdmissionCapabilityGapKind,
        current_count: usize,
        must_not_exceed_count: usize,
        cap_ledger_digest_part: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            current_count,
            must_not_exceed_count,
            cap_ledger_digest_part: cap_ledger_digest_part.into(),
        }
    }

    pub(crate) const fn is_within_cap(&self) -> bool {
        self.current_count <= self.must_not_exceed_count
    }

    pub const fn kind(&self) -> WorthGraphReadAdmissionCapabilityGapKind {
        self.kind
    }

    pub const fn current_count(&self) -> usize {
        self.current_count
    }

    pub const fn must_not_exceed_count(&self) -> usize {
        self.must_not_exceed_count
    }

    pub fn cap_ledger_digest_part(&self) -> &str {
        &self.cap_ledger_digest_part
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "{}:{}:{}:{}",
            self.kind.as_str(),
            self.current_count,
            self.must_not_exceed_count,
            self.cap_ledger_digest_part
        )
    }
}
