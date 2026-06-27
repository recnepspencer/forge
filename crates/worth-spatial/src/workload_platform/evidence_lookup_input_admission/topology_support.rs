use topology::derived_invalidation_family_catalog::DerivedTopologyProductFamilyIdentity;
use topology::derived_invalidation_milestone_ten_closeout::DerivedInvalidationMilestoneElevenSeed;

use crate::workload_platform::evidence_lookup_family_catalog::EvidenceLookupTopologyInputPosture;

use super::error::{EvidenceLookupInputAdmissionError, EvidenceLookupInputAdmissionErrorKind};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupTopologyAdmissionSupport {
    family_identity: String,
    state: EvidenceLookupTopologySupportState,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EvidenceLookupTopologySupportState {
    NotRequired,
    Satisfied {
        seed_digest: String,
        receipt_ref_digest: String,
        family_identity: DerivedTopologyProductFamilyIdentity,
    },
}

impl EvidenceLookupTopologyAdmissionSupport {
    pub(crate) fn not_required(family_identity: impl Into<String>) -> Self {
        Self {
            family_identity: family_identity.into(),
            state: EvidenceLookupTopologySupportState::NotRequired,
        }
    }

    pub(crate) fn from_required_posture(
        family_identity: impl Into<String>,
        posture: &EvidenceLookupTopologyInputPosture,
        seed: Option<&DerivedInvalidationMilestoneElevenSeed>,
    ) -> Result<Self, EvidenceLookupInputAdmissionError> {
        let family_identity = family_identity.into();
        let Some(required_family) = posture.required_family() else {
            return Ok(Self::not_required(family_identity));
        };
        let Some(seed) = seed else {
            return Err(EvidenceLookupInputAdmissionError::new(
                EvidenceLookupInputAdmissionErrorKind::MissingTopologySeed,
                required_family.as_str(),
            ));
        };
        let Some(receipt_ref) = seed
            .topology_derived_product_receipts()
            .iter()
            .find(|receipt| receipt.family_identity() == required_family)
        else {
            return Err(EvidenceLookupInputAdmissionError::new(
                EvidenceLookupInputAdmissionErrorKind::MissingRequiredTopologyReceipt,
                required_family.as_str(),
            ));
        };
        Ok(Self {
            family_identity,
            state: EvidenceLookupTopologySupportState::Satisfied {
                seed_digest: seed.seed_digest().to_string(),
                receipt_ref_digest: receipt_ref.ref_digest().to_string(),
                family_identity: required_family,
            },
        })
    }

    pub fn family_identity(&self) -> &str {
        &self.family_identity
    }

    pub const fn state(&self) -> &EvidenceLookupTopologySupportState {
        &self.state
    }

    pub const fn claims_lookup_product_authority(&self) -> bool {
        false
    }

    pub(crate) fn digest_summary_part(&self) -> String {
        match &self.state {
            EvidenceLookupTopologySupportState::NotRequired => {
                format!("topology:{}:not-required", self.family_identity)
            }
            EvidenceLookupTopologySupportState::Satisfied {
                seed_digest,
                receipt_ref_digest,
                family_identity,
            } => format!(
                "topology:{}:satisfied:{}:{}:{}",
                self.family_identity,
                seed_digest,
                receipt_ref_digest,
                family_identity.as_str()
            ),
        }
    }
}
