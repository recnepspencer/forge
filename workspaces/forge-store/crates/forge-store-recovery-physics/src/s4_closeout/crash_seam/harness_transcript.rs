use super::S4RecoveryCrashSeam;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S4LoweredCrashHarnessEvidence {
    pub(super) seam: S4RecoveryCrashSeam,
    pub(super) lowered_plan_id: String,
    pub(super) storage_boundary_id: String,
    pub(super) observer_transcript_id: String,
    pub(super) proof_oracle_id: String,
    pub(super) seed: u64,
    pub(super) backend_profile: String,
    pub(super) fault_ordinal: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S4CrashHarnessTranscriptSource {
    seam: S4RecoveryCrashSeam,
    lowered_plan_id: String,
    storage_boundary_id: String,
    observer_transcript_id: String,
    proof_oracle_id: String,
    seed: u64,
    backend_profile: String,
    fault_ordinal: u16,
}

impl S4CrashHarnessTranscriptSource {
    #[cfg(any(test, feature = "certification-test-authority"))]
    pub fn from_roadmap2_transcript(
        seam: S4RecoveryCrashSeam,
        lowered_plan_id: impl Into<String>,
        storage_boundary_id: impl Into<String>,
        observer_transcript_id: impl Into<String>,
        proof_oracle_id: impl Into<String>,
        seed: u64,
        backend_profile: impl Into<String>,
        fault_ordinal: u16,
    ) -> Result<Self, super::super::RecoveryPhysicsCloseoutDenial> {
        let source = Self {
            seam,
            lowered_plan_id: lowered_plan_id.into(),
            storage_boundary_id: storage_boundary_id.into(),
            observer_transcript_id: observer_transcript_id.into(),
            proof_oracle_id: proof_oracle_id.into(),
            seed,
            backend_profile: backend_profile.into(),
            fault_ordinal,
        };
        source.require_transcript_source_authority()?;
        Ok(source)
    }

    #[cfg(any(test, feature = "certification-test-authority"))]
    fn require_transcript_source_authority(
        &self,
    ) -> Result<(), super::super::RecoveryPhysicsCloseoutDenial> {
        if self.lowered_plan_id.is_empty()
            || self.storage_boundary_id.is_empty()
            || self.observer_transcript_id.is_empty()
            || self.proof_oracle_id.is_empty()
            || self.seed == 0
            || self.backend_profile.is_empty()
            || self.fault_ordinal == 0
        {
            return Err(
                super::super::RecoveryPhysicsCloseoutDenial::MissingCrashFaultSchedulerEvidence,
            );
        }
        Ok(())
    }
}

impl S4LoweredCrashHarnessEvidence {
    pub(crate) fn from_recovery_harness_transcript(
        source: S4CrashHarnessTranscriptSource,
    ) -> Result<Self, super::super::RecoveryPhysicsCloseoutDenial> {
        Ok(Self {
            seam: source.seam,
            lowered_plan_id: source.lowered_plan_id,
            storage_boundary_id: source.storage_boundary_id,
            observer_transcript_id: source.observer_transcript_id,
            proof_oracle_id: source.proof_oracle_id,
            seed: source.seed,
            backend_profile: source.backend_profile,
            fault_ordinal: source.fault_ordinal,
        })
    }

    pub const fn seam(&self) -> S4RecoveryCrashSeam {
        self.seam
    }

    pub fn lowered_plan_id(&self) -> &str {
        &self.lowered_plan_id
    }

    pub fn storage_boundary_id(&self) -> &str {
        &self.storage_boundary_id
    }

    pub fn observer_transcript_id(&self) -> &str {
        &self.observer_transcript_id
    }

    pub fn proof_oracle_id(&self) -> &str {
        &self.proof_oracle_id
    }

    pub const fn seed(&self) -> u64 {
        self.seed
    }

    pub fn backend_profile(&self) -> &str {
        &self.backend_profile
    }

    pub const fn fault_ordinal(&self) -> u16 {
        self.fault_ordinal
    }
}
