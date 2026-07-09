use super::failure::{SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture};
use super::witnesses::SupportTrustEquivalenceWitness;
use crate::subscription_support::{
    SubscriptionResumeClassification, SubscriptionSupportArtifactId, SubscriptionSupportFamilyId,
    SubscriptionSupportOperationalBasis, SubscriptionSupportOperationalVerdict,
    SubscriptionSupportRole,
};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SupportTrustEquivalenceLane {
    Rebuild,
    Migration,
    Replication,
    Import,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportTrustEquivalenceContract {
    lane: SupportTrustEquivalenceLane,
    source_basis: SubscriptionSupportOperationalBasis,
    target_family_id: SubscriptionSupportFamilyId,
    target_support_role: SubscriptionSupportRole,
    target_artifact_id: SubscriptionSupportArtifactId,
    target_basis_digest: String,
    target_cursor_digest: String,
    target_checkpoint_digest: String,
    target_compatibility_digest: String,
    target_portability_digest: String,
    resume_classification: SubscriptionResumeClassification,
    operational_verdict: SubscriptionSupportOperationalVerdict,
    equivalence_digest: String,
}

impl SupportTrustEquivalenceContract {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        lane: SupportTrustEquivalenceLane,
        source_basis: SubscriptionSupportOperationalBasis,
        target_family_id: SubscriptionSupportFamilyId,
        target_support_role: SubscriptionSupportRole,
        target_artifact_id: SubscriptionSupportArtifactId,
        target_basis_digest: impl Into<String>,
        target_cursor_digest: impl Into<String>,
        target_checkpoint_digest: impl Into<String>,
        target_compatibility_digest: impl Into<String>,
        target_portability_digest: impl Into<String>,
        resume_classification: SubscriptionResumeClassification,
        operational_verdict: SubscriptionSupportOperationalVerdict,
        equivalence_digest: impl Into<String>,
    ) -> Result<Self, SupportTrustFailure> {
        Ok(Self {
            lane,
            source_basis,
            target_family_id,
            target_support_role,
            target_artifact_id,
            target_basis_digest: require_non_empty("target basis digest", target_basis_digest)?,
            target_cursor_digest: require_non_empty("target cursor digest", target_cursor_digest)?,
            target_checkpoint_digest: require_non_empty(
                "target checkpoint digest",
                target_checkpoint_digest,
            )?,
            target_compatibility_digest: require_non_empty(
                "target compatibility digest",
                target_compatibility_digest,
            )?,
            target_portability_digest: require_non_empty(
                "target portability digest",
                target_portability_digest,
            )?,
            resume_classification,
            operational_verdict,
            equivalence_digest: require_non_empty("equivalence digest", equivalence_digest)?,
        })
    }

    pub fn lane(&self) -> SupportTrustEquivalenceLane {
        self.lane
    }

    pub fn source_basis(&self) -> &SubscriptionSupportOperationalBasis {
        &self.source_basis
    }

    pub fn target_family_id(&self) -> &SubscriptionSupportFamilyId {
        &self.target_family_id
    }

    pub fn target_support_role(&self) -> SubscriptionSupportRole {
        self.target_support_role
    }

    pub fn target_artifact_id(&self) -> &SubscriptionSupportArtifactId {
        &self.target_artifact_id
    }

    pub fn target_basis_digest(&self) -> &str {
        &self.target_basis_digest
    }

    pub fn target_cursor_digest(&self) -> &str {
        &self.target_cursor_digest
    }

    pub fn target_checkpoint_digest(&self) -> &str {
        &self.target_checkpoint_digest
    }

    pub fn target_compatibility_digest(&self) -> &str {
        &self.target_compatibility_digest
    }

    pub fn target_portability_digest(&self) -> &str {
        &self.target_portability_digest
    }

    pub fn resume_classification(&self) -> SubscriptionResumeClassification {
        self.resume_classification
    }

    pub fn operational_verdict(&self) -> SubscriptionSupportOperationalVerdict {
        self.operational_verdict
    }

    pub fn equivalence_digest(&self) -> &str {
        &self.equivalence_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
pub struct SupportTrustEquivalenceEvidence {
    rebuild: Option<SupportTrustEquivalenceContract>,
    migration: Option<SupportTrustEquivalenceContract>,
    replication: Option<SupportTrustEquivalenceContract>,
    import: Option<SupportTrustEquivalenceContract>,
}

impl SupportTrustEquivalenceEvidence {
    pub fn none() -> Self {
        Self::default()
    }

    pub fn with_contract(
        mut self,
        contract: SupportTrustEquivalenceContract,
    ) -> Result<Self, SupportTrustFailure> {
        let slot = match contract.lane() {
            SupportTrustEquivalenceLane::Rebuild => &mut self.rebuild,
            SupportTrustEquivalenceLane::Migration => &mut self.migration,
            SupportTrustEquivalenceLane::Replication => &mut self.replication,
            SupportTrustEquivalenceLane::Import => &mut self.import,
        };
        if slot.is_some() {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustEquivalenceMissing,
                SupportTrustRecoveryPosture::RetryWithFresherReceipts,
                "support trust equivalence evidence cannot contain duplicate lane contracts",
            ));
        }
        *slot = Some(contract);
        Ok(self)
    }

    pub(crate) fn contract_for(
        &self,
        lane: SupportTrustEquivalenceLane,
    ) -> Option<&SupportTrustEquivalenceContract> {
        match lane {
            SupportTrustEquivalenceLane::Rebuild => self.rebuild.as_ref(),
            SupportTrustEquivalenceLane::Migration => self.migration.as_ref(),
            SupportTrustEquivalenceLane::Replication => self.replication.as_ref(),
            SupportTrustEquivalenceLane::Import => self.import.as_ref(),
        }
    }

    pub(crate) fn contract_count(&self) -> u64 {
        u64::from(self.rebuild.is_some())
            + u64::from(self.migration.is_some())
            + u64::from(self.replication.is_some())
            + u64::from(self.import.is_some())
    }
}

macro_rules! lane_witness {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
        pub struct $name {
            witness: SupportTrustEquivalenceWitness,
        }

        impl $name {
            pub(crate) fn new(witness: SupportTrustEquivalenceWitness) -> Self {
                Self { witness }
            }

            pub fn witness(&self) -> &SupportTrustEquivalenceWitness {
                &self.witness
            }
        }
    };
}

lane_witness!(SupportRebuildEquivalenceWitness);
lane_witness!(SupportMigrationEquivalenceWitness);
lane_witness!(SupportReplicationEquivalenceWitness);
lane_witness!(SupportImportEquivalenceWitness);

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum SupportTrustTransformedEquivalenceWitness {
    Rebuild(SupportRebuildEquivalenceWitness),
    Migration(SupportMigrationEquivalenceWitness),
    Replication(SupportReplicationEquivalenceWitness),
    Import(SupportImportEquivalenceWitness),
}

impl SupportTrustTransformedEquivalenceWitness {
    pub(crate) fn from_contract(
        contract: &SupportTrustEquivalenceContract,
    ) -> Result<Self, SupportTrustFailure> {
        let witness = SupportTrustEquivalenceWitness::new(
            contract.source_basis().clone(),
            contract.target_family_id().clone(),
            contract.operational_verdict(),
            contract.equivalence_digest(),
        )?;
        Ok(match contract.lane() {
            SupportTrustEquivalenceLane::Rebuild => {
                Self::Rebuild(SupportRebuildEquivalenceWitness::new(witness))
            }
            SupportTrustEquivalenceLane::Migration => {
                Self::Migration(SupportMigrationEquivalenceWitness::new(witness))
            }
            SupportTrustEquivalenceLane::Replication => {
                Self::Replication(SupportReplicationEquivalenceWitness::new(witness))
            }
            SupportTrustEquivalenceLane::Import => {
                Self::Import(SupportImportEquivalenceWitness::new(witness))
            }
        })
    }

    pub(crate) fn into_operational_witness(self) -> SupportTrustEquivalenceWitness {
        match self {
            Self::Rebuild(witness) => witness.witness,
            Self::Migration(witness) => witness.witness,
            Self::Replication(witness) => witness.witness,
            Self::Import(witness) => witness.witness,
        }
    }
}

fn require_non_empty(
    label: &'static str,
    value: impl Into<String>,
) -> Result<String, SupportTrustFailure> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustEquivalenceMissing,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            format!("support trust equivalence {label} must be non-empty"),
        ));
    }
    Ok(value)
}
