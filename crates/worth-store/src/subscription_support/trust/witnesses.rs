use super::epochs::SupportTrustFreshnessWitness;
use super::failure::{SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture};
use super::taxonomy::{
    SupportTrustProvenance, SupportTrustStrength, SupportTrustStrengthProvenance,
};
use super::translation::SupportExactTrustTranslation;
use crate::subscription_support::{
    SubscriptionSupportFamilyId, SubscriptionSupportOperationalBasis,
    SubscriptionSupportOperationalVerdict,
};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExactSupportTrustWitness {
    translation: SupportExactTrustTranslation,
    trust: SupportTrustStrengthProvenance,
    freshness: SupportTrustFreshnessWitness,
}

impl ExactSupportTrustWitness {
    #[allow(dead_code)]
    pub(crate) fn from_exact_translation(
        translation: SupportExactTrustTranslation,
        provenance: SupportTrustProvenance,
        freshness: SupportTrustFreshnessWitness,
    ) -> Result<Self, SupportTrustFailure> {
        if provenance.requires_equivalence_for_exact() {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustEquivalenceMissing,
                SupportTrustRecoveryPosture::RetryWithFresherReceipts,
                "transformed exact support trust requires an equivalence witness",
            ));
        }
        Ok(Self {
            translation,
            trust: SupportTrustStrengthProvenance::new(
                super::taxonomy::SupportTrustStrength::Exact,
                provenance,
            ),
            freshness,
        })
    }

    #[allow(dead_code)]
    pub(crate) fn from_equivalent_operational_basis(
        translation: SupportExactTrustTranslation,
        provenance: SupportTrustProvenance,
        freshness: SupportTrustFreshnessWitness,
        equivalence: SupportTrustEquivalenceWitness,
    ) -> Result<Self, SupportTrustFailure> {
        if !provenance.requires_equivalence_for_exact() {
            return Self::from_exact_translation(translation, provenance, freshness);
        }
        if equivalence.source_basis() != translation.basis() {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustBasisMismatch,
                SupportTrustRecoveryPosture::RetryWithFresherReceipts,
                "support trust equivalence witness must be bound to the exact source basis",
            ));
        }
        if equivalence.operational_verdict()
            != SubscriptionSupportOperationalVerdict::ExactResumePreserved
        {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustOperationalVerdictMismatch,
                SupportTrustRecoveryPosture::RetryWithFresherReceipts,
                "support trust equivalence witness must preserve exact operational verdict",
            ));
        }
        Ok(Self {
            translation,
            trust: SupportTrustStrengthProvenance::new(
                super::taxonomy::SupportTrustStrength::Exact,
                provenance,
            ),
            freshness,
        })
    }

    pub fn basis(&self) -> &SubscriptionSupportOperationalBasis {
        self.translation.basis()
    }

    pub fn translation(&self) -> &SupportExactTrustTranslation {
        &self.translation
    }

    pub fn trust(&self) -> SupportTrustStrengthProvenance {
        self.trust
    }

    pub fn freshness(&self) -> SupportTrustFreshnessWitness {
        self.freshness
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DegradedSupportTrustWitness {
    basis: SubscriptionSupportOperationalBasis,
    freshness: SupportTrustFreshnessWitness,
}

impl DegradedSupportTrustWitness {
    pub(crate) fn new(
        basis: SubscriptionSupportOperationalBasis,
        freshness: SupportTrustFreshnessWitness,
    ) -> Self {
        Self { basis, freshness }
    }

    pub fn basis(&self) -> &SubscriptionSupportOperationalBasis {
        &self.basis
    }

    pub fn freshness(&self) -> SupportTrustFreshnessWitness {
        self.freshness
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RebuildDerivedSupportTrustWitness {
    basis: SubscriptionSupportOperationalBasis,
    freshness: SupportTrustFreshnessWitness,
}

impl RebuildDerivedSupportTrustWitness {
    pub(crate) fn new(
        basis: SubscriptionSupportOperationalBasis,
        freshness: SupportTrustFreshnessWitness,
    ) -> Self {
        Self { basis, freshness }
    }

    pub fn basis(&self) -> &SubscriptionSupportOperationalBasis {
        &self.basis
    }

    pub fn freshness(&self) -> SupportTrustFreshnessWitness {
        self.freshness
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RejectedSupportTrustWitness {
    basis: SubscriptionSupportOperationalBasis,
    freshness: SupportTrustFreshnessWitness,
}

impl RejectedSupportTrustWitness {
    pub(crate) fn new(
        basis: SubscriptionSupportOperationalBasis,
        freshness: SupportTrustFreshnessWitness,
    ) -> Self {
        Self { basis, freshness }
    }

    pub fn basis(&self) -> &SubscriptionSupportOperationalBasis {
        &self.basis
    }

    pub fn freshness(&self) -> SupportTrustFreshnessWitness {
        self.freshness
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum SupportTrustOperationalWitness {
    Exact(ExactSupportTrustWitness),
    Degraded(DegradedSupportTrustWitness),
    RebuildDerived(RebuildDerivedSupportTrustWitness),
    Rejected(RejectedSupportTrustWitness),
}

impl SupportTrustOperationalWitness {
    pub fn basis(&self) -> &SubscriptionSupportOperationalBasis {
        match self {
            Self::Exact(witness) => witness.basis(),
            Self::Degraded(witness) => witness.basis(),
            Self::RebuildDerived(witness) => witness.basis(),
            Self::Rejected(witness) => witness.basis(),
        }
    }

    pub fn freshness(&self) -> SupportTrustFreshnessWitness {
        match self {
            Self::Exact(witness) => witness.freshness(),
            Self::Degraded(witness) => witness.freshness(),
            Self::RebuildDerived(witness) => witness.freshness(),
            Self::Rejected(witness) => witness.freshness(),
        }
    }

    pub fn trust_strength(&self) -> SupportTrustStrength {
        match self {
            Self::Exact(_) => SupportTrustStrength::Exact,
            Self::Degraded(_) => SupportTrustStrength::Degraded,
            Self::RebuildDerived(_) => SupportTrustStrength::RebuildOnly,
            Self::Rejected(_) => SupportTrustStrength::Rejected,
        }
    }

    pub fn exact(&self) -> Option<&ExactSupportTrustWitness> {
        match self {
            Self::Exact(witness) => Some(witness),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CertifiedSupportTrustWitness {
    operational: SupportTrustOperationalWitness,
}

impl CertifiedSupportTrustWitness {
    #[allow(dead_code)]
    pub(crate) fn new(operational: SupportTrustOperationalWitness) -> Self {
        Self { operational }
    }

    pub fn operational(&self) -> &SupportTrustOperationalWitness {
        &self.operational
    }

    pub fn exact(&self) -> Option<&ExactSupportTrustWitness> {
        self.operational.exact()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportTrustEquivalenceWitness {
    source_basis: SubscriptionSupportOperationalBasis,
    target_family_id: SubscriptionSupportFamilyId,
    operational_verdict: SubscriptionSupportOperationalVerdict,
    equivalence_digest: String,
}

impl SupportTrustEquivalenceWitness {
    #[allow(dead_code)]
    pub(crate) fn new(
        source_basis: SubscriptionSupportOperationalBasis,
        target_family_id: SubscriptionSupportFamilyId,
        operational_verdict: SubscriptionSupportOperationalVerdict,
        equivalence_digest: impl Into<String>,
    ) -> Result<Self, SupportTrustFailure> {
        let equivalence_digest = equivalence_digest.into();
        if equivalence_digest.trim().is_empty() {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustEquivalenceMissing,
                SupportTrustRecoveryPosture::RetryWithFresherReceipts,
                "support trust equivalence digest must be non-empty",
            ));
        }
        Ok(Self {
            source_basis,
            target_family_id,
            operational_verdict,
            equivalence_digest,
        })
    }

    pub fn source_basis(&self) -> &SubscriptionSupportOperationalBasis {
        &self.source_basis
    }

    pub fn source_family_id(&self) -> &SubscriptionSupportFamilyId {
        self.source_basis.family_id()
    }

    pub fn target_family_id(&self) -> &SubscriptionSupportFamilyId {
        &self.target_family_id
    }

    pub fn operational_verdict(&self) -> SubscriptionSupportOperationalVerdict {
        self.operational_verdict
    }

    pub fn equivalence_digest(&self) -> &str {
        &self.equivalence_digest
    }
}
