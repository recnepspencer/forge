use crate::workload_platform::evidence_lookup_input_admission::{
    EvidenceLookupTopologyAdmissionSupport, EvidenceLookupTopologySupportState,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EvidenceLookupPlanTopologyPostureState {
    NotRequired,
    NotEvaluatedForUnaffectedFamily,
    Satisfied {
        seed_digest: String,
        receipt_ref_digest: String,
        family_identity: &'static str,
    },
    RequiredButMissing {
        family_identity: &'static str,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupPlanTopologyPosture {
    state: EvidenceLookupPlanTopologyPostureState,
}

impl EvidenceLookupPlanTopologyPosture {
    pub(crate) fn not_required() -> Self {
        Self {
            state: EvidenceLookupPlanTopologyPostureState::NotRequired,
        }
    }

    pub(crate) const fn not_evaluated_for_unaffected_family() -> Self {
        Self {
            state: EvidenceLookupPlanTopologyPostureState::NotEvaluatedForUnaffectedFamily,
        }
    }

    pub(crate) fn from_support(
        support: Option<&EvidenceLookupTopologyAdmissionSupport>,
        required_family_identity: Option<&'static str>,
    ) -> Self {
        match support.map(EvidenceLookupTopologyAdmissionSupport::state) {
            Some(EvidenceLookupTopologySupportState::Satisfied {
                seed_digest,
                receipt_ref_digest,
                family_identity,
            }) => Self {
                state: EvidenceLookupPlanTopologyPostureState::Satisfied {
                    seed_digest: seed_digest.clone(),
                    receipt_ref_digest: receipt_ref_digest.clone(),
                    family_identity: family_identity.as_str(),
                },
            },
            _ if required_family_identity.is_some() => Self {
                state: EvidenceLookupPlanTopologyPostureState::RequiredButMissing {
                    family_identity: required_family_identity.expect("checked above"),
                },
            },
            _ => Self::not_required(),
        }
    }

    pub const fn state(&self) -> &EvidenceLookupPlanTopologyPostureState {
        &self.state
    }

    #[cfg(test)]
    pub(crate) fn from_state_for_tests(state: EvidenceLookupPlanTopologyPostureState) -> Self {
        Self { state }
    }

    pub const fn is_missing_required_topology_posture(&self) -> bool {
        matches!(
            self.state,
            EvidenceLookupPlanTopologyPostureState::RequiredButMissing { .. }
        )
    }

    pub(crate) fn digest_part(&self) -> String {
        match &self.state {
            EvidenceLookupPlanTopologyPostureState::NotRequired => {
                "topology:not-required".to_string()
            }
            EvidenceLookupPlanTopologyPostureState::NotEvaluatedForUnaffectedFamily => {
                "topology:not-evaluated-unaffected-family".to_string()
            }
            EvidenceLookupPlanTopologyPostureState::Satisfied {
                seed_digest,
                receipt_ref_digest,
                family_identity,
            } => format!("topology:satisfied:{seed_digest}:{receipt_ref_digest}:{family_identity}"),
            EvidenceLookupPlanTopologyPostureState::RequiredButMissing { family_identity } => {
                format!("topology:required-missing:{family_identity}")
            }
        }
    }
}
