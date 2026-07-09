use crate::subscription_support::{SubscriptionSupportFamilyId, SubscriptionSupportRole};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupportTrustStrength {
    Exact,
    Degraded,
    RebuildOnly,
    Rejected,
    Unsupported,
}

pub type SubscriptionSupportTrustClass = SupportTrustClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupportTrustProvenance {
    NativePublished,
    Rebuilt,
    Migrated,
    Replicated,
    Imported,
    Omitted,
    PolicyExpired,
}

impl SupportTrustProvenance {
    #[allow(dead_code)]
    pub(crate) fn requires_equivalence_for_exact(self) -> bool {
        matches!(
            self,
            Self::Rebuilt | Self::Migrated | Self::Replicated | Self::Imported
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportTrustStrengthProvenance {
    strength: SupportTrustStrength,
    provenance: SupportTrustProvenance,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportRoleTrustPosture {
    family_id: SubscriptionSupportFamilyId,
    support_role: SubscriptionSupportRole,
    trust: SupportTrustStrengthProvenance,
}

impl SupportRoleTrustPosture {
    pub fn new(
        family_id: SubscriptionSupportFamilyId,
        support_role: SubscriptionSupportRole,
        strength: SupportTrustStrength,
        provenance: SupportTrustProvenance,
    ) -> Self {
        Self {
            family_id,
            support_role,
            trust: SupportTrustStrengthProvenance::new(strength, provenance),
        }
    }

    pub fn family_id(&self) -> &SubscriptionSupportFamilyId {
        &self.family_id
    }

    pub fn support_role(&self) -> SubscriptionSupportRole {
        self.support_role
    }

    pub fn trust(&self) -> SupportTrustStrengthProvenance {
        self.trust
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupportTrustDowngradeReason {
    ResumeClassificationDegraded,
    RebuildRequired,
    OperationalVerdictDegraded,
    EquivalenceMissing,
    EpochExpired,
    CertificationCoverageMissing,
    PortabilityScopeIncomplete,
    PolicyRejected,
    UnsupportedFamily,
}

impl SupportTrustStrengthProvenance {
    #[allow(dead_code)]
    pub(crate) fn new(strength: SupportTrustStrength, provenance: SupportTrustProvenance) -> Self {
        Self {
            strength,
            provenance,
        }
    }

    pub fn strength(&self) -> SupportTrustStrength {
        self.strength
    }

    pub fn provenance(&self) -> SupportTrustProvenance {
        self.provenance
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupportTrustClass {
    ExactSupportTrusted,
    DegradedSupportTrusted,
    RebuildDerivedSupport,
    ReplicatedSupportTrusted,
    MigratedSupportTrusted,
    StaleSupportRejected,
    PolicyRejectedSupport,
    UnsupportedSupportTrust,
}

impl SupportTrustClass {
    #[allow(dead_code)]
    pub(crate) fn from_strength_provenance(
        pair: SupportTrustStrengthProvenance,
    ) -> SupportTrustClass {
        match (pair.strength(), pair.provenance()) {
            (SupportTrustStrength::Exact, SupportTrustProvenance::Replicated) => {
                SupportTrustClass::ReplicatedSupportTrusted
            }
            (SupportTrustStrength::Exact, SupportTrustProvenance::Migrated) => {
                SupportTrustClass::MigratedSupportTrusted
            }
            (SupportTrustStrength::Exact, _) => SupportTrustClass::ExactSupportTrusted,
            (SupportTrustStrength::Degraded, _) => SupportTrustClass::DegradedSupportTrusted,
            (SupportTrustStrength::RebuildOnly, _) => SupportTrustClass::RebuildDerivedSupport,
            (SupportTrustStrength::Rejected, SupportTrustProvenance::PolicyExpired) => {
                SupportTrustClass::PolicyRejectedSupport
            }
            (SupportTrustStrength::Rejected, _) => SupportTrustClass::StaleSupportRejected,
            (SupportTrustStrength::Unsupported, _) => SupportTrustClass::UnsupportedSupportTrust,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupportTrustUseBoundary {
    StoreLocalOperational,
    CertifiedPlatform,
}
