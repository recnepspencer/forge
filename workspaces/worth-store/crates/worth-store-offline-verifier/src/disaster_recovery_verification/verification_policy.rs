use std::collections::BTreeSet;

use worth_store_replication::DisasterRecoveryComponent;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisasterRecoveryVerificationPolicyDenial {
    EmptySupportedFormatSet,
    EmptySupportedBackendSet,
    InvalidAssumptionIdentity,
    DuplicateAssumptionIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisasterRecoveryVerificationPolicy {
    supported_formats: BTreeSet<[u8; 32]>,
    supported_backends: BTreeSet<[u8; 32]>,
}

impl DisasterRecoveryVerificationPolicy {
    pub fn from_supported_assumptions(
        supported_formats: Vec<[u8; 32]>,
        supported_backends: Vec<[u8; 32]>,
    ) -> Result<Self, DisasterRecoveryVerificationPolicyDenial> {
        let supported_formats = admit_set(
            supported_formats,
            DisasterRecoveryVerificationPolicyDenial::EmptySupportedFormatSet,
        )?;
        let supported_backends = admit_set(
            supported_backends,
            DisasterRecoveryVerificationPolicyDenial::EmptySupportedBackendSet,
        )?;
        Ok(Self {
            supported_formats,
            supported_backends,
        })
    }

    pub(super) fn supports_format(&self, component: &DisasterRecoveryComponent) -> bool {
        self.supported_formats
            .contains(&component.evidence().format_identity())
    }

    pub(super) fn supports_backend(&self, component: &DisasterRecoveryComponent) -> bool {
        self.supported_backends
            .contains(&component.evidence().backend_assumption_identity())
    }
}

fn admit_set(
    identities: Vec<[u8; 32]>,
    empty_denial: DisasterRecoveryVerificationPolicyDenial,
) -> Result<BTreeSet<[u8; 32]>, DisasterRecoveryVerificationPolicyDenial> {
    if identities.is_empty() {
        return Err(empty_denial);
    }
    if identities.contains(&[0; 32]) {
        return Err(DisasterRecoveryVerificationPolicyDenial::InvalidAssumptionIdentity);
    }
    let original_len = identities.len();
    let admitted = identities.into_iter().collect::<BTreeSet<_>>();
    if admitted.len() != original_len {
        return Err(DisasterRecoveryVerificationPolicyDenial::DuplicateAssumptionIdentity);
    }
    Ok(admitted)
}
