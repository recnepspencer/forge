use worth_ui::facade::{
    WorthUiAdmittedRuntimeChangeEvidence, WorthUiRuntimeChangeActivationPosture,
    WorthUiRuntimeChangeCounters, WorthUiRuntimeChangeFamily, WorthUiRuntimeChangeFamilyStatus,
    WorthUiRuntimeFactId,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationRuntimeChangeEvidence {
    digest: u64,
    posture: ValidationRuntimeChangePostureEvidence,
    counters: ValidationRuntimeChangeCountersEvidence,
    rows: Vec<ValidationRuntimeChangeFamilyRowEvidence>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationRuntimeChangePostureEvidence {
    EquivalentNoOp,
    ReadyForFrameBoundary,
    Activated,
    Denied,
    Mixed(ValidationRuntimeChangeMixedPostureEvidence),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidationRuntimeChangeMixedPostureEvidence {
    activated_family_count: usize,
    denied_family_count: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidationRuntimeChangeCountersEvidence {
    family_row_count: usize,
    changed_fact_count: usize,
    denied_family_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationRuntimeChangeFamilyRowEvidence {
    family: WorthUiRuntimeChangeFamily,
    status: WorthUiRuntimeChangeFamilyStatus,
    changed_facts: Vec<WorthUiRuntimeFactId>,
    denial_detail: Option<String>,
}

impl ValidationRuntimeChangeEvidence {
    pub fn from_admitted_change(evidence: &WorthUiAdmittedRuntimeChangeEvidence) -> Self {
        Self {
            digest: evidence.digest().value(),
            posture: ValidationRuntimeChangePostureEvidence::from_runtime(evidence.posture()),
            counters: ValidationRuntimeChangeCountersEvidence::from_runtime(evidence.counters()),
            rows: evidence
                .family_rows()
                .iter()
                .map(ValidationRuntimeChangeFamilyRowEvidence::from_runtime)
                .collect(),
        }
    }

    pub fn digest(&self) -> u64 {
        self.digest
    }

    pub fn posture(&self) -> ValidationRuntimeChangePostureEvidence {
        self.posture
    }

    pub fn counters(&self) -> ValidationRuntimeChangeCountersEvidence {
        self.counters
    }

    pub fn rows(&self) -> &[ValidationRuntimeChangeFamilyRowEvidence] {
        &self.rows
    }

    pub fn stable_digest(&self) -> u64 {
        let mut digest = 0xcbf2_9ce4_8422_2325u64;
        let mut bases = self
            .rows
            .iter()
            .map(ValidationRuntimeChangeFamilyRowEvidence::digest_basis)
            .collect::<Vec<_>>();
        bases.sort();
        for basis in bases {
            for byte in basis.as_bytes() {
                digest ^= u64::from(*byte);
                digest = digest.wrapping_mul(0x0000_0100_0000_01b3);
            }
        }
        digest
    }
}

impl ValidationRuntimeChangePostureEvidence {
    fn from_runtime(posture: WorthUiRuntimeChangeActivationPosture) -> Self {
        match posture {
            WorthUiRuntimeChangeActivationPosture::EquivalentNoOp => Self::EquivalentNoOp,
            WorthUiRuntimeChangeActivationPosture::ReadyForFrameBoundary => {
                Self::ReadyForFrameBoundary
            }
            WorthUiRuntimeChangeActivationPosture::Activated => Self::Activated,
            WorthUiRuntimeChangeActivationPosture::Denied => Self::Denied,
            WorthUiRuntimeChangeActivationPosture::Mixed(mixed) => {
                Self::Mixed(ValidationRuntimeChangeMixedPostureEvidence {
                    activated_family_count: mixed.activated_family_count(),
                    denied_family_count: mixed.denied_family_count(),
                })
            }
        }
    }
}

impl ValidationRuntimeChangeMixedPostureEvidence {
    pub fn activated_family_count(self) -> usize {
        self.activated_family_count
    }

    pub fn denied_family_count(self) -> usize {
        self.denied_family_count
    }
}

impl ValidationRuntimeChangeCountersEvidence {
    fn from_runtime(counters: WorthUiRuntimeChangeCounters) -> Self {
        Self {
            family_row_count: counters.family_row_count(),
            changed_fact_count: counters.changed_fact_count(),
            denied_family_count: counters.denied_family_count(),
        }
    }

    pub fn family_row_count(self) -> usize {
        self.family_row_count
    }

    pub fn changed_fact_count(self) -> usize {
        self.changed_fact_count
    }

    pub fn denied_family_count(self) -> usize {
        self.denied_family_count
    }
}

impl ValidationRuntimeChangeFamilyRowEvidence {
    fn from_runtime(row: &worth_ui::facade::WorthUiRuntimeChangeFamilyRow) -> Self {
        Self {
            family: row.family(),
            status: row.status(),
            changed_facts: row.changed_facts().facts().facts().cloned().collect(),
            denial_detail: row.denial_detail().map(str::to_owned),
        }
    }

    pub fn family(&self) -> WorthUiRuntimeChangeFamily {
        self.family
    }

    pub fn status(&self) -> WorthUiRuntimeChangeFamilyStatus {
        self.status
    }

    pub fn changed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.changed_facts
    }

    pub fn denial_detail(&self) -> Option<&str> {
        self.denial_detail.as_deref()
    }

    fn digest_basis(&self) -> String {
        let changed_facts = self
            .changed_facts
            .iter()
            .map(|fact| format!("{fact:?}"))
            .collect::<Vec<_>>()
            .join("|");
        format!(
            "{:?}|{:?}|{}|{}",
            self.family,
            self.status,
            changed_facts,
            self.denial_detail.as_deref().unwrap_or("")
        )
    }
}
