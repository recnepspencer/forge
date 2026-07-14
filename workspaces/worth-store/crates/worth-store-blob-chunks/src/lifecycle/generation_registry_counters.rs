use worth_store_budgets::CounterEvidenceStrength;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobGenerationRegistryCounterSnapshot {
    strength: CounterEvidenceStrength,
    admissions: u64,
    publications: u64,
    observations: u64,
    classification_checks: u64,
    rebuild_admissions: u64,
    denials: u64,
}

impl BlobGenerationRegistryCounterSnapshot {
    pub(crate) const fn start() -> Self {
        Self {
            strength: CounterEvidenceStrength::Exact,
            admissions: 1,
            publications: 0,
            observations: 0,
            classification_checks: 0,
            rebuild_admissions: 0,
            denials: 0,
        }
    }

    pub(crate) const fn record_publication(self) -> Self {
        Self {
            publications: self.publications + 1,
            ..self
        }
    }

    pub(crate) const fn record_observation(self) -> Self {
        Self {
            observations: self.observations + 1,
            ..self
        }
    }

    pub(crate) const fn record_classification_check(self) -> Self {
        Self {
            classification_checks: self.classification_checks + 1,
            ..self
        }
    }

    pub(crate) const fn record_rebuild_admission(self) -> Self {
        Self {
            rebuild_admissions: self.rebuild_admissions + 1,
            ..self
        }
    }

    pub(crate) const fn record_denial(self) -> Self {
        Self {
            denials: self.denials + 1,
            ..self
        }
    }

    pub const fn strength(self) -> CounterEvidenceStrength {
        self.strength
    }

    pub const fn admissions(self) -> u64 {
        self.admissions
    }

    pub const fn publications(self) -> u64 {
        self.publications
    }

    pub const fn observations(self) -> u64 {
        self.observations
    }

    pub const fn classification_checks(self) -> u64 {
        self.classification_checks
    }

    pub const fn rebuild_admissions(self) -> u64 {
        self.rebuild_admissions
    }

    pub const fn denials(self) -> u64 {
        self.denials
    }
}
