use crate::compiled_product_family::TopologyCompiledProductFamilyIdentity;
use crate::facade::current_topology_query_backed_consumer_cutover;
use crate::query_domain::TopologyReadRequestFamily;
use crate::selected_equivalence_family::current_topology_selected_equivalence_family_catalog;
use crate::selected_equivalence_family::{
    TopologyFreshnessRequirementPosture, TopologyRenderedOutputComparisonPosture,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyPublicCloseoutSeedSupportError {
    detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyPublicCloseoutAlignmentSummary {
    cutover_digest: String,
    public_read_family_row_digest: String,
    support_snapshot_digest: String,
    compiled_product_identity_digest: String,
    equivalence_policy_identity_digest: String,
    selected_equivalence_family_identity: String,
    selected_equivalence_basis_identity_digest: String,
    selected_compatibility_basis_identity_digest: String,
    selected_reuse_basis_identity_digest: String,
    reuse_decision_identity_digest: Option<String>,
    rebuild_denial_identity_digest: Option<String>,
    route_authority_digest: String,
    query_posture_digest: String,
    freshness_requirement_posture: TopologyPublicCloseoutFreshnessRequirementPosture,
    rendered_output_comparison_posture: TopologyPublicCloseoutRenderedOutputComparisonPosture,
    query_execution_count: usize,
    row_scan_fallback_count: usize,
    whole_view_fallback_count: usize,
    repeated_rediscovery_denied_count: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyPublicCloseoutFreshnessRequirementPosture {
    SameAdmittedAuthorityAndLocalityRequired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyPublicCloseoutRenderedOutputComparisonPosture {
    DerivedOutputDigestsRequired,
}

pub fn current_topology_public_closeout_alignment_summary(
) -> Result<TopologyPublicCloseoutAlignmentSummary, TopologyPublicCloseoutSeedSupportError> {
    current_topology_public_closeout_alignment_summary_with_cutover_loader(
        current_topology_query_backed_consumer_cutover,
    )
}

pub(crate) fn current_topology_public_closeout_alignment_summary_with_cutover_loader<F>(
    load_cutover: F,
) -> Result<TopologyPublicCloseoutAlignmentSummary, TopologyPublicCloseoutSeedSupportError>
where
    F: FnOnce() -> Result<
        crate::facade::TopologyQueryBackedConsumerCutover,
        crate::facade::TopologyQueryBackedConsumerCutoverCurrentError,
    >,
{
    let cutover = load_cutover()
        .map_err(|error| TopologyPublicCloseoutSeedSupportError::new(error.detail()))?;
    let public_read_row = cutover
        .family_rows()
        .iter()
        .find(|row| row.request_family() == TopologyReadRequestFamily::LoopCycleNeighborhood)
        .ok_or_else(|| {
            TopologyPublicCloseoutSeedSupportError::new(
                "topology public closeout seed support requires a loop-cycle query row",
            )
        })?;
    let family_identity = public_read_row
        .selected_equivalence_family_identity()
        .ok_or_else(|| {
            TopologyPublicCloseoutSeedSupportError::new(
                "topology public closeout seed support requires a selected equivalence family",
            )
        })?;
    let catalog = current_topology_selected_equivalence_family_catalog();
    let family = catalog
        .family_for_compiled_product(
            TopologyCompiledProductFamilyIdentity::DerivedTopologyEquivalenceContract,
        )
        .ok_or_else(|| {
            TopologyPublicCloseoutSeedSupportError::new(
                "topology public closeout seed support could not resolve family declaration",
            )
        })?;
    if family.identity().as_str() != family_identity {
        return Err(TopologyPublicCloseoutSeedSupportError::new(
            "topology public closeout seed support found a loop-cycle row with a mismatched selected equivalence family identity",
        ));
    }
    let compiled_product_identity_digest = public_read_row
        .compiled_product_identity_digest()
        .ok_or_else(|| {
            TopologyPublicCloseoutSeedSupportError::new(
                "topology public closeout seed support requires compiled-product identity",
            )
        })?
        .to_string();
    let selected_equivalence_basis_identity_digest = public_read_row
        .selected_equivalence_basis_identity_digest()
        .ok_or_else(|| {
            TopologyPublicCloseoutSeedSupportError::new(
                "topology public closeout seed support requires equivalence-basis identity",
            )
        })?
        .to_string();
    let selected_compatibility_basis_identity_digest = public_read_row
        .selected_compatibility_basis_identity_digest()
        .ok_or_else(|| {
            TopologyPublicCloseoutSeedSupportError::new(
                "topology public closeout seed support requires compatibility-basis identity",
            )
        })?
        .to_string();
    let selected_reuse_basis_identity_digest = public_read_row
        .selected_reuse_basis_identity_digest()
        .ok_or_else(|| {
            TopologyPublicCloseoutSeedSupportError::new(
                "topology public closeout seed support requires reuse-basis identity",
            )
        })?
        .to_string();
    let support_snapshot_digest = cutover.support_snapshot_digest().to_string();
    let route_authority_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-topo:touched-graph-parity-route-authority:v1".to_string(),
            format!("family:{family_identity}"),
            format!("product:{compiled_product_identity_digest}"),
            format!("equivalence-basis:{selected_equivalence_basis_identity_digest}"),
            format!("compatibility-basis:{selected_compatibility_basis_identity_digest}"),
            format!("reuse-basis:{selected_reuse_basis_identity_digest}"),
        ],
    );
    let query_posture_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth:touched-graph-topology-query-posture:v1".to_string(),
            format!("support-snapshot:{support_snapshot_digest}"),
            format!(
                "query-executions:{}",
                public_read_row.query_execution_count()
            ),
            format!(
                "row-scan-fallbacks:{}",
                public_read_row.row_scan_fallback_count()
            ),
            format!(
                "whole-view-fallbacks:{}",
                public_read_row.whole_view_fallback_count()
            ),
            format!(
                "repeated-rediscovery-denied:{}",
                public_read_row.repeated_rediscovery_denied_count()
            ),
        ],
    );
    Ok(TopologyPublicCloseoutAlignmentSummary {
        cutover_digest: cutover.closeout_digest().to_string(),
        public_read_family_row_digest: public_read_row.row_digest().to_string(),
        support_snapshot_digest,
        compiled_product_identity_digest,
        equivalence_policy_identity_digest: public_read_row
            .equivalence_policy_identity_digest()
            .ok_or_else(|| {
                TopologyPublicCloseoutSeedSupportError::new(
                    "topology public closeout seed support requires equivalence-policy identity",
                )
            })?
            .to_string(),
        selected_equivalence_family_identity: family_identity.to_string(),
        selected_equivalence_basis_identity_digest,
        selected_compatibility_basis_identity_digest,
        selected_reuse_basis_identity_digest,
        reuse_decision_identity_digest: public_read_row
            .reuse_decision_identity_digest()
            .map(str::to_string),
        rebuild_denial_identity_digest: public_read_row
            .rebuild_denial_identity_digest()
            .map(str::to_string),
        route_authority_digest,
        query_posture_digest,
        freshness_requirement_posture: family.freshness_requirement_posture().into(),
        rendered_output_comparison_posture: family.rendered_output_comparison_posture().into(),
        query_execution_count: public_read_row.query_execution_count(),
        row_scan_fallback_count: public_read_row.row_scan_fallback_count(),
        whole_view_fallback_count: public_read_row.whole_view_fallback_count(),
        repeated_rediscovery_denied_count: public_read_row.repeated_rediscovery_denied_count(),
    })
}

impl TopologyPublicCloseoutAlignmentSummary {
    pub fn cutover_digest(&self) -> &str {
        &self.cutover_digest
    }

    pub fn public_read_family_row_digest(&self) -> &str {
        &self.public_read_family_row_digest
    }

    pub fn support_snapshot_digest(&self) -> &str {
        &self.support_snapshot_digest
    }

    pub fn compiled_product_identity_digest(&self) -> &str {
        &self.compiled_product_identity_digest
    }

    pub fn equivalence_policy_identity_digest(&self) -> &str {
        &self.equivalence_policy_identity_digest
    }

    pub fn selected_equivalence_family_identity(&self) -> &str {
        &self.selected_equivalence_family_identity
    }

    pub fn selected_equivalence_basis_identity_digest(&self) -> &str {
        &self.selected_equivalence_basis_identity_digest
    }

    pub fn selected_compatibility_basis_identity_digest(&self) -> &str {
        &self.selected_compatibility_basis_identity_digest
    }

    pub fn selected_reuse_basis_identity_digest(&self) -> &str {
        &self.selected_reuse_basis_identity_digest
    }

    pub fn reuse_decision_identity_digest(&self) -> Option<&str> {
        self.reuse_decision_identity_digest.as_deref()
    }

    pub fn rebuild_denial_identity_digest(&self) -> Option<&str> {
        self.rebuild_denial_identity_digest.as_deref()
    }

    pub fn route_authority_digest(&self) -> &str {
        &self.route_authority_digest
    }

    pub fn query_posture_digest(&self) -> &str {
        &self.query_posture_digest
    }

    pub const fn freshness_requirement_posture(
        &self,
    ) -> TopologyPublicCloseoutFreshnessRequirementPosture {
        self.freshness_requirement_posture
    }

    pub const fn rendered_output_comparison_posture(
        &self,
    ) -> TopologyPublicCloseoutRenderedOutputComparisonPosture {
        self.rendered_output_comparison_posture
    }

    pub const fn query_execution_count(&self) -> usize {
        self.query_execution_count
    }

    pub const fn row_scan_fallback_count(&self) -> usize {
        self.row_scan_fallback_count
    }

    pub const fn whole_view_fallback_count(&self) -> usize {
        self.whole_view_fallback_count
    }

    pub const fn repeated_rediscovery_denied_count(&self) -> usize {
        self.repeated_rediscovery_denied_count
    }
}

impl TopologyPublicCloseoutSeedSupportError {
    pub(crate) fn new(detail: impl Into<String>) -> Self {
        Self {
            detail: detail.into(),
        }
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl From<TopologyFreshnessRequirementPosture>
    for TopologyPublicCloseoutFreshnessRequirementPosture
{
    fn from(value: TopologyFreshnessRequirementPosture) -> Self {
        match value {
            TopologyFreshnessRequirementPosture::SameAdmittedAuthorityAndLocalityRequired => {
                Self::SameAdmittedAuthorityAndLocalityRequired
            }
        }
    }
}

impl From<TopologyRenderedOutputComparisonPosture>
    for TopologyPublicCloseoutRenderedOutputComparisonPosture
{
    fn from(value: TopologyRenderedOutputComparisonPosture) -> Self {
        match value {
            TopologyRenderedOutputComparisonPosture::DerivedOutputDigestsRequired => {
                Self::DerivedOutputDigestsRequired
            }
        }
    }
}
