use super::admitted_route::TopologyQueryBackedReadFamilyAdmissionAuthority;
use super::route_input::TopologyObservedQueryBackedReadFamilyRow;
use schema::facade::platform::authority::compiled_product_semantic_graph::{
    CompiledProductRebuildDenialIdentity, CompiledProductReuseDecisionIdentity,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyReadModelReusePosture {
    ReuseAdmitted,
    FreshRebuildRequired,
    CompatibilityWithoutReuse,
    Denied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TopologyReadModelTypedReuseDecision {
    posture: TopologyReadModelReusePosture,
    reuse_decision_identity: Option<CompiledProductReuseDecisionIdentity>,
    rebuild_denial_identity: Option<CompiledProductRebuildDenialIdentity>,
}

impl TopologyReadModelReusePosture {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReuseAdmitted => "reused",
            Self::FreshRebuildRequired => "fresh_rebuild_required",
            Self::CompatibilityWithoutReuse => "compatibility_without_reuse",
            Self::Denied => "denied",
        }
    }
}

impl TopologyReadModelTypedReuseDecision {
    pub(crate) fn lower(
        row: &TopologyObservedQueryBackedReadFamilyRow,
        authority: &TopologyQueryBackedReadFamilyAdmissionAuthority,
    ) -> Self {
        let has_selected_family_contract = authority
            .selected_equivalence_family_for_admission()
            .is_some()
            && authority
                .selected_equivalence_basis_digest_for_admission()
                .is_some()
            && authority
                .selected_reuse_basis_digest_for_admission()
                .is_some();
        let has_execution_gap =
            row.request_count() == 0 || row.query_execution_count() != row.request_count();
        let has_runtime_debt = row.debt_row_count() > 0
            || row.locality_claim_mismatch_count() > 0
            || row.row_scan_fallback_count() > 0
            || row.whole_view_fallback_count() > 0
            || row.repeated_rediscovery_denied_count() > 0;
        let has_reuse_identity = authority.reuse_decision_identity_for_admission().is_some();
        let posture = if has_execution_gap || !has_selected_family_contract {
            TopologyReadModelReusePosture::Denied
        } else if has_runtime_debt {
            TopologyReadModelReusePosture::CompatibilityWithoutReuse
        } else if has_reuse_identity {
            TopologyReadModelReusePosture::ReuseAdmitted
        } else {
            TopologyReadModelReusePosture::FreshRebuildRequired
        };
        let rebuild_denial_identity = if posture == TopologyReadModelReusePosture::ReuseAdmitted {
            None
        } else {
            authority
                .compiled_product_identity_for_admission()
                .map(|compiled_product_identity| {
                    let denial_reason = if has_execution_gap {
                        "topology-query-backed-read-family-execution-gap"
                    } else if !has_selected_family_contract {
                        "topology-query-backed-read-family-missing-contract"
                    } else {
                        "topology-query-backed-read-family-runtime-debt"
                    };
                    authority.rebuild_required_identity(compiled_product_identity, denial_reason)
                })
        };
        Self {
            posture,
            reuse_decision_identity: if posture == TopologyReadModelReusePosture::ReuseAdmitted {
                authority.reuse_decision_identity_for_admission().cloned()
            } else {
                None
            },
            rebuild_denial_identity,
        }
    }

    pub(crate) const fn posture(&self) -> TopologyReadModelReusePosture {
        self.posture
    }

    pub(crate) const fn reuse_decision_identity(
        &self,
    ) -> Option<&CompiledProductReuseDecisionIdentity> {
        self.reuse_decision_identity.as_ref()
    }

    pub(crate) const fn rebuild_denial_identity(
        &self,
    ) -> Option<&CompiledProductRebuildDenialIdentity> {
        self.rebuild_denial_identity.as_ref()
    }
}
