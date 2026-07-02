use super::admitted_route::TopologyQueryBackedReadFamilyAdmissionAuthority;
use super::digest::{closeout_digest, family_row_digest};
use super::reuse_posture::{TopologyReadModelReusePosture, TopologyReadModelTypedReuseDecision};
use super::route_input::TopologyObservedQueryBackedReadFamilyRow;
use crate::projection::read_views::domain::{
    TopologyReadCloseoutStatus, TopologyReadRequestFamily,
};
use crate::selected_equivalence_family::TopologySelectedEquivalenceFamilyIdentity;
use schema::facade::platform::authority::compiled_product_semantic_graph::{
    CompiledProductEquivalencePolicyIdentity, CompiledProductIdentity,
    CompiledProductRebuildDenialIdentity, CompiledProductReuseDecisionIdentity,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyQueryBackedConsumerFamilyRow {
    request_family: TopologyReadRequestFamily,
    closeout_status: TopologyReadCloseoutStatus,
    reuse_posture: TopologyReadModelReusePosture,
    compiled_product_identity: Option<CompiledProductIdentity>,
    equivalence_policy_identity: Option<CompiledProductEquivalencePolicyIdentity>,
    selected_equivalence_family_identity: Option<TopologySelectedEquivalenceFamilyIdentity>,
    selected_equivalence_basis_identity_digest: Option<String>,
    selected_compatibility_basis_identity_digest: Option<String>,
    selected_reuse_basis_identity_digest: Option<String>,
    reuse_decision_identity: Option<CompiledProductReuseDecisionIdentity>,
    rebuild_denial_identity: Option<CompiledProductRebuildDenialIdentity>,
    query_execution_count: usize,
    row_scan_fallback_count: usize,
    whole_view_fallback_count: usize,
    repeated_rediscovery_denied_count: usize,
    closeout_row_digest: String,
    row_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyQueryBackedConsumerCutover {
    handle_identity_digest: String,
    operating_context_identity_digest: String,
    support_snapshot_digest: String,
    query_executed_debt_free_family_count: usize,
    debt_family_count: usize,
    parity_verified_count: usize,
    family_rows: Vec<TopologyQueryBackedConsumerFamilyRow>,
    closeout_digest: String,
}

impl TopologyQueryBackedConsumerFamilyRow {
    pub(crate) fn from_observed_route_row(
        row: &TopologyObservedQueryBackedReadFamilyRow,
        authority: &TopologyQueryBackedReadFamilyAdmissionAuthority,
    ) -> Self {
        let typed_decision = TopologyReadModelTypedReuseDecision::lower(row, authority);
        let row_digest = family_row_digest(
            row.request_family(),
            row.closeout_status().as_str(),
            typed_decision.posture().as_str(),
            authority
                .compiled_product_digest_for_admission()
                .unwrap_or("missing-compiled-product-proof"),
            authority
                .equivalence_policy_digest_for_admission()
                .unwrap_or("missing-equivalence-policy-proof"),
            authority
                .selected_equivalence_family_for_admission()
                .map(|identity| identity.as_str().to_string())
                .unwrap_or_else(|| "missing-selected-equivalence-family".to_string()),
            authority
                .selected_equivalence_basis_digest_for_admission()
                .unwrap_or("missing-selected-equivalence-basis"),
            authority
                .selected_compatibility_basis_digest_for_admission()
                .unwrap_or("missing-selected-compatibility-basis"),
            authority
                .selected_reuse_basis_digest_for_admission()
                .unwrap_or("missing-selected-reuse-basis"),
            typed_decision
                .reuse_decision_identity()
                .map(|identity| identity.identity_digest())
                .unwrap_or("missing-reuse-decision"),
            typed_decision
                .rebuild_denial_identity()
                .map(|identity| identity.identity_digest())
                .unwrap_or("missing-rebuild-denial"),
            row.query_execution_count(),
            row.row_scan_fallback_count(),
            row.whole_view_fallback_count(),
            row.repeated_rediscovery_denied_count(),
            row.closeout_row_digest(),
        );
        Self {
            request_family: row.request_family(),
            closeout_status: row.closeout_status(),
            reuse_posture: typed_decision.posture(),
            compiled_product_identity: authority.compiled_product_identity_for_admission().cloned(),
            equivalence_policy_identity: authority
                .equivalence_policy_identity_for_admission()
                .cloned(),
            selected_equivalence_family_identity: authority
                .selected_equivalence_family_for_admission(),
            selected_equivalence_basis_identity_digest: authority
                .selected_equivalence_basis_digest_for_admission()
                .map(str::to_string),
            selected_compatibility_basis_identity_digest: authority
                .selected_compatibility_basis_digest_for_admission()
                .map(str::to_string),
            selected_reuse_basis_identity_digest: authority
                .selected_reuse_basis_digest_for_admission()
                .map(str::to_string),
            reuse_decision_identity: typed_decision.reuse_decision_identity().cloned(),
            rebuild_denial_identity: typed_decision.rebuild_denial_identity().cloned(),
            query_execution_count: row.query_execution_count(),
            row_scan_fallback_count: row.row_scan_fallback_count(),
            whole_view_fallback_count: row.whole_view_fallback_count(),
            repeated_rediscovery_denied_count: row.repeated_rediscovery_denied_count(),
            closeout_row_digest: row.closeout_row_digest().to_string(),
            row_digest,
        }
    }

    pub(crate) fn refresh_row_digest(&mut self) {
        self.row_digest = family_row_digest(
            self.request_family,
            self.closeout_status.as_str(),
            self.reuse_posture.as_str(),
            self.compiled_product_identity
                .as_ref()
                .map(|identity| identity.identity_digest())
                .unwrap_or("missing-compiled-product-proof"),
            self.equivalence_policy_identity
                .as_ref()
                .map(|identity| identity.identity_digest())
                .unwrap_or("missing-equivalence-policy-proof"),
            self.selected_equivalence_family_identity
                .as_ref()
                .map(|identity| identity.as_str().to_string())
                .unwrap_or_else(|| "missing-selected-equivalence-family".to_string()),
            self.selected_equivalence_basis_identity_digest
                .as_deref()
                .unwrap_or("missing-selected-equivalence-basis"),
            self.selected_compatibility_basis_identity_digest
                .as_deref()
                .unwrap_or("missing-selected-compatibility-basis"),
            self.selected_reuse_basis_identity_digest
                .as_deref()
                .unwrap_or("missing-selected-reuse-basis"),
            self.reuse_decision_identity
                .as_ref()
                .map(|identity| identity.identity_digest())
                .unwrap_or("missing-reuse-decision"),
            self.rebuild_denial_identity
                .as_ref()
                .map(|identity| identity.identity_digest())
                .unwrap_or("missing-rebuild-denial"),
            self.query_execution_count,
            self.row_scan_fallback_count,
            self.whole_view_fallback_count,
            self.repeated_rediscovery_denied_count,
            &self.closeout_row_digest,
        );
    }

    pub fn request_family(&self) -> TopologyReadRequestFamily {
        self.request_family
    }

    pub const fn reuse_posture(&self) -> TopologyReadModelReusePosture {
        self.reuse_posture
    }

    pub const fn compiled_product_identity(&self) -> Option<&CompiledProductIdentity> {
        self.compiled_product_identity.as_ref()
    }

    pub fn compiled_product_identity_digest(&self) -> Option<&str> {
        self.compiled_product_identity
            .as_ref()
            .map(|identity| identity.identity_digest())
    }

    pub const fn equivalence_policy_identity(
        &self,
    ) -> Option<&CompiledProductEquivalencePolicyIdentity> {
        self.equivalence_policy_identity.as_ref()
    }

    pub fn equivalence_policy_identity_digest(&self) -> Option<&str> {
        self.equivalence_policy_identity
            .as_ref()
            .map(|identity| identity.identity_digest())
    }

    pub const fn selected_equivalence_family_identity_kind(
        &self,
    ) -> Option<TopologySelectedEquivalenceFamilyIdentity> {
        self.selected_equivalence_family_identity
    }

    pub fn selected_equivalence_family_identity(&self) -> Option<&str> {
        self.selected_equivalence_family_identity
            .as_ref()
            .map(|identity| identity.as_str())
    }

    pub fn selected_equivalence_basis_identity_digest(&self) -> Option<&str> {
        self.selected_equivalence_basis_identity_digest.as_deref()
    }

    pub fn selected_compatibility_basis_identity_digest(&self) -> Option<&str> {
        self.selected_compatibility_basis_identity_digest.as_deref()
    }

    pub fn selected_reuse_basis_identity_digest(&self) -> Option<&str> {
        self.selected_reuse_basis_identity_digest.as_deref()
    }

    pub const fn reuse_decision_identity(&self) -> Option<&CompiledProductReuseDecisionIdentity> {
        self.reuse_decision_identity.as_ref()
    }

    pub fn reuse_decision_identity_digest(&self) -> Option<&str> {
        self.reuse_decision_identity
            .as_ref()
            .map(|identity| identity.identity_digest())
    }

    pub const fn rebuild_denial_identity(&self) -> Option<&CompiledProductRebuildDenialIdentity> {
        self.rebuild_denial_identity.as_ref()
    }

    pub fn rebuild_denial_identity_digest(&self) -> Option<&str> {
        self.rebuild_denial_identity
            .as_ref()
            .map(|identity| identity.identity_digest())
    }

    pub fn query_execution_count(&self) -> usize {
        self.query_execution_count
    }

    pub fn row_scan_fallback_count(&self) -> usize {
        self.row_scan_fallback_count
    }

    pub fn whole_view_fallback_count(&self) -> usize {
        self.whole_view_fallback_count
    }

    pub fn repeated_rediscovery_denied_count(&self) -> usize {
        self.repeated_rediscovery_denied_count
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

impl TopologyQueryBackedConsumerCutover {
    pub(crate) fn new(
        handle_identity_digest: String,
        operating_context_identity_digest: String,
        support_snapshot_digest: String,
        query_executed_debt_free_family_count: usize,
        debt_family_count: usize,
        parity_verified_count: usize,
        family_rows: Vec<TopologyQueryBackedConsumerFamilyRow>,
    ) -> Self {
        let closeout_digest = closeout_digest(
            &family_rows,
            &handle_identity_digest,
            &support_snapshot_digest,
            &operating_context_identity_digest,
            parity_verified_count,
        );
        Self {
            handle_identity_digest,
            operating_context_identity_digest,
            support_snapshot_digest,
            query_executed_debt_free_family_count,
            debt_family_count,
            parity_verified_count,
            family_rows,
            closeout_digest,
        }
    }

    pub(crate) fn refresh_closeout_digest(&mut self) {
        self.closeout_digest = closeout_digest(
            &self.family_rows,
            &self.handle_identity_digest,
            &self.support_snapshot_digest,
            &self.operating_context_identity_digest,
            self.parity_verified_count,
        );
    }

    pub fn handle_identity_digest(&self) -> &str {
        &self.handle_identity_digest
    }

    pub fn operating_context_identity_digest(&self) -> &str {
        &self.operating_context_identity_digest
    }

    pub fn support_snapshot_digest(&self) -> &str {
        &self.support_snapshot_digest
    }

    pub fn query_executed_debt_free_family_count(&self) -> usize {
        self.query_executed_debt_free_family_count
    }

    pub fn debt_family_count(&self) -> usize {
        self.debt_family_count
    }

    pub fn parity_verified_count(&self) -> usize {
        self.parity_verified_count
    }

    pub fn family_rows(&self) -> &[TopologyQueryBackedConsumerFamilyRow] {
        &self.family_rows
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }

    pub fn with_test_loop_cycle_selected_compatibility_basis_identity_override(
        mut self,
        identity_digest: &str,
    ) -> Self {
        let row = self
            .family_rows
            .iter_mut()
            .find(|row| row.request_family == TopologyReadRequestFamily::LoopCycleNeighborhood)
            .expect("loop-cycle family row should exist");
        row.selected_compatibility_basis_identity_digest = Some(identity_digest.to_string());
        row.refresh_row_digest();
        self.refresh_closeout_digest();
        self
    }

    pub fn with_test_family_fallback_counts(
        mut self,
        family: TopologyReadRequestFamily,
        row_scan_fallback_count: usize,
        whole_view_fallback_count: usize,
    ) -> Self {
        let row = self
            .family_rows
            .iter_mut()
            .find(|row| row.request_family == family)
            .expect("requested family row should exist");
        row.row_scan_fallback_count = row_scan_fallback_count;
        row.whole_view_fallback_count = whole_view_fallback_count;
        row.refresh_row_digest();
        self.refresh_closeout_digest();
        self
    }
}
