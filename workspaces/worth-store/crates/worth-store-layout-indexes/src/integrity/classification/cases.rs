#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CorruptionClassificationCaseId(pub(super) &'static str);

impl CorruptionClassificationCaseId {
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

pub(super) const CORRUPTION_CLASSIFICATION_CASES: [CorruptionClassificationCaseId; 4] = [
    CorruptionClassificationCaseId("layout.integrity.classification.rebuild_required"),
    CorruptionClassificationCaseId("layout.integrity.classification.quarantined"),
    CorruptionClassificationCaseId(
        "layout.integrity.classification.quarantine_readmission_required",
    ),
    CorruptionClassificationCaseId("layout.integrity.classification.import_readmission_required"),
];

pub fn corruption_classification_cases() -> impl Iterator<Item = CorruptionClassificationCaseId> {
    CORRUPTION_CLASSIFICATION_CASES.into_iter()
}
