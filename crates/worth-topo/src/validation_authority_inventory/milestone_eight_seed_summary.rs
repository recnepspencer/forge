#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthValidationAuthorityMilestoneEightSeedSummary {
    seed_digest: String,
    claims_validator_selection: bool,
    receipt_context_present: bool,
    posture_context_present: bool,
}

impl WorthValidationAuthorityMilestoneEightSeedSummary {
    pub(crate) fn from_parts(
        seed_digest: impl Into<String>,
        claims_validator_selection: bool,
        receipt_context_present: bool,
        posture_context_present: bool,
    ) -> Self {
        Self {
            seed_digest: seed_digest.into(),
            claims_validator_selection,
            receipt_context_present,
            posture_context_present,
        }
    }

    pub fn imported_public_closeout(
        seed_digest: impl Into<String>,
        receipt_context_present: bool,
        posture_context_present: bool,
    ) -> Self {
        Self::from_parts(
            seed_digest,
            false,
            receipt_context_present,
            posture_context_present,
        )
    }

    pub fn current_imported_public_closeout() -> Self {
        Self::imported_public_closeout(
            "worth-graph-read-access-plan-adoption-milestone-eight-public-closeout",
            true,
            true,
        )
    }

    pub fn seed_digest(&self) -> &str {
        &self.seed_digest
    }

    pub const fn claims_validator_selection(&self) -> bool {
        self.claims_validator_selection
    }

    pub const fn receipt_context_present(&self) -> bool {
        self.receipt_context_present
    }

    pub const fn posture_context_present(&self) -> bool {
        self.posture_context_present
    }
}
