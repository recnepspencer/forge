#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SharedVocabularyAdoptionEntry {
    pub adoption_point: &'static str,
    pub stronger_store_source: &'static str,
    pub shared_family: &'static str,
    pub lowering_direction: &'static str,
    pub strength_loss: &'static str,
    pub construction_authority: &'static str,
    pub reverse_flow_denial: &'static str,
    pub proof: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SharedVocabularyAdoptionLedger {
    entries: &'static [SharedVocabularyAdoptionEntry],
}

impl SharedVocabularyAdoptionLedger {
    pub(crate) const fn new(entries: &'static [SharedVocabularyAdoptionEntry]) -> Self {
        Self { entries }
    }

    pub const fn entries(self) -> &'static [SharedVocabularyAdoptionEntry] {
        self.entries
    }
}
