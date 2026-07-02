use super::mismatch::EvidenceLookupRouteMismatch;
use super::packet::EvidenceLookupRoutePacket;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EvidenceLookupRouteAdmissionError {
    CurrentRouteUnavailable {
        detail: String,
    },
    RouteMismatch {
        detail: String,
        mismatch: EvidenceLookupRouteMismatch,
    },
}

impl EvidenceLookupRouteAdmissionError {
    pub(crate) fn current_route_unavailable(detail: impl Into<String>) -> Self {
        Self::CurrentRouteUnavailable {
            detail: detail.into(),
        }
    }

    pub(crate) fn route_mismatch(
        detail: impl Into<String>,
        mismatch: EvidenceLookupRouteMismatch,
    ) -> Self {
        Self::RouteMismatch {
            detail: detail.into(),
            mismatch,
        }
    }

    pub fn detail(&self) -> &str {
        match self {
            Self::CurrentRouteUnavailable { detail } | Self::RouteMismatch { detail, .. } => detail,
        }
    }

    pub fn mismatch(&self) -> Option<&EvidenceLookupRouteMismatch> {
        match self {
            Self::CurrentRouteUnavailable { .. } => None,
            Self::RouteMismatch { mismatch, .. } => Some(mismatch),
        }
    }
}

impl EvidenceLookupRoutePacket {
    #[allow(clippy::too_many_arguments)]
    pub fn require_matches_selected_contract(
        &self,
        expected_route_authority_digest: &str,
        expected_route_family_identity: &str,
        expected_right_route_family_identity: &str,
        expected_stage_receipt_family_identity: &str,
        expected_right_stage_receipt_identity: &str,
        expected_selected_lookup_plan_digest: &str,
        expected_right_lookup_execution_receipt_digest: &str,
        expected_compiled_product_identity_digest: &str,
        expected_equivalence_policy_identity_digest: &str,
        expected_selected_equivalence_family_identity: &str,
        expected_selected_equivalence_basis_identity_digest: &str,
        expected_selected_compatibility_basis_identity_digest: &str,
        expected_selected_reuse_basis_identity_digest: &str,
        expected_topology_support_digest: &str,
        expected_query_support_digest: &str,
        expected_right_authority_stage_index_identity: &str,
    ) -> Result<(), EvidenceLookupRouteAdmissionError> {
        require_match(
            expected_route_authority_digest,
            self.route_authority_digest(),
            |expected, actual| EvidenceLookupRouteMismatch::RouteAuthorityIdentity {
                expected,
                actual,
            },
        )?;
        require_match(
            expected_route_family_identity,
            self.route_family_identity(),
            |expected, actual| EvidenceLookupRouteMismatch::RouteFamilyIdentity {
                expected,
                actual,
            },
        )?;
        require_match(
            expected_right_route_family_identity,
            self.right_route_family_identity(),
            |expected, actual| EvidenceLookupRouteMismatch::RightRouteFamilyIdentity {
                expected,
                actual,
            },
        )?;
        require_match(
            expected_stage_receipt_family_identity,
            self.stage_receipt_family_identity(),
            |expected, actual| EvidenceLookupRouteMismatch::StageReceiptFamilyIdentity {
                expected,
                actual,
            },
        )?;
        require_match(
            expected_right_stage_receipt_identity,
            self.right_stage_receipt_identity(),
            |expected, actual| EvidenceLookupRouteMismatch::RightStageReceiptIdentity {
                expected,
                actual,
            },
        )?;
        require_match(
            expected_selected_lookup_plan_digest,
            self.selected_lookup_plan_digest(),
            |expected, actual| EvidenceLookupRouteMismatch::SelectedPlanIdentity {
                expected,
                actual,
            },
        )?;
        require_match(
            expected_right_lookup_execution_receipt_digest,
            self.right_lookup_execution_receipt_digest(),
            |expected, actual| EvidenceLookupRouteMismatch::RightLookupExecutionReceiptDigest {
                expected,
                actual,
            },
        )?;
        require_match(
            expected_compiled_product_identity_digest,
            self.compiled_product_identity_digest(),
            |expected, actual| EvidenceLookupRouteMismatch::CompiledProductIdentity {
                expected,
                actual,
            },
        )?;
        require_match(
            expected_equivalence_policy_identity_digest,
            self.equivalence_policy_identity_digest(),
            |expected, actual| EvidenceLookupRouteMismatch::EquivalencePolicyIdentity {
                expected,
                actual,
            },
        )?;
        require_match(
            expected_selected_equivalence_family_identity,
            self.selected_equivalence_family_identity(),
            |expected, actual| EvidenceLookupRouteMismatch::SelectedEquivalenceFamilyIdentity {
                expected,
                actual,
            },
        )?;
        require_match(
            expected_selected_equivalence_basis_identity_digest,
            self.selected_equivalence_basis_identity_digest(),
            |expected, actual| EvidenceLookupRouteMismatch::SelectedEquivalenceBasisIdentity {
                expected,
                actual,
            },
        )?;
        require_match(
            expected_selected_compatibility_basis_identity_digest,
            self.selected_compatibility_basis_identity_digest(),
            |expected, actual| EvidenceLookupRouteMismatch::SelectedCompatibilityBasisIdentity {
                expected,
                actual,
            },
        )?;
        require_match(
            expected_selected_reuse_basis_identity_digest,
            self.selected_reuse_basis_identity_digest(),
            |expected, actual| EvidenceLookupRouteMismatch::SelectedReuseBasisIdentity {
                expected,
                actual,
            },
        )?;
        require_match(
            expected_topology_support_digest,
            self.topology_support_digest(),
            |expected, actual| EvidenceLookupRouteMismatch::TopologySupportDigest {
                expected,
                actual,
            },
        )?;
        require_match(
            expected_query_support_digest,
            self.query_support_digest(),
            |expected, actual| EvidenceLookupRouteMismatch::QuerySupportDigest { expected, actual },
        )?;
        require_match(
            expected_right_authority_stage_index_identity,
            self.right_authority_stage_index_identity(),
            |expected, actual| EvidenceLookupRouteMismatch::RightAuthorityStageIndexIdentity {
                expected,
                actual,
            },
        )?;
        Ok(())
    }
}

fn require_match(
    expected: &str,
    actual: &str,
    build: impl FnOnce(String, String) -> EvidenceLookupRouteMismatch,
) -> Result<(), EvidenceLookupRouteAdmissionError> {
    if expected == actual {
        return Ok(());
    }
    Err(EvidenceLookupRouteAdmissionError::route_mismatch(
        "evidence lookup route authority mismatch",
        build(expected.to_string(), actual.to_string()),
    ))
}
