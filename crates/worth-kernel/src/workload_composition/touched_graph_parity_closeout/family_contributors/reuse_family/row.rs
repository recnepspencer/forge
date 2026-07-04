use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityFamilyKind;
use topology::facade::TopologyDerivedReuseDecisionPosture;
use worth_spatial::facade::evidence_lookup_reuse_route::EvidenceLookupReuseDecisionPosture;

use crate::workload_composition::touched_graph_parity_closeout::family_contributors::{
    KernelTouchedGraphParityCoverageContributor, KernelTouchedGraphParityCoverageError,
    KernelTouchedGraphParityQuerySurfaceKind,
};

use super::error::{ReuseFamilyContributorCatalogError, ReuseFamilyContributorCatalogErrorKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReuseFamilyContributorRowKind {
    Equivalence,
    Reuse,
}

impl ReuseFamilyContributorRowKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Equivalence => "equivalence",
            Self::Reuse => "reuse",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReuseFamilyContributorCatalogRow {
    kind: ReuseFamilyContributorRowKind,
    family_kind: TouchedGraphParityFamilyKind,
    current_packet_or_identity_source: &'static str,
    carried_equivalence_or_compatibility_source: &'static str,
    carried_reuse_or_denial_source: &'static str,
    ordinary_path_live_caller_surface: &'static str,
    ordinary_path_live_caller_path: &'static str,
    route_packet_identity: String,
    topology_selected_family_identity: String,
    topology_selected_product_identity_digest: String,
    topology_equivalence_policy_identity_digest: String,
    topology_selected_compatibility_basis_identity_digest: String,
    topology_selected_reuse_basis_identity_digest: String,
    topology_posture: TopologyDerivedReuseDecisionPosture,
    topology_reuse_decision_identity_digest: Option<String>,
    topology_rebuild_denial_identity_digest: Option<String>,
    spatial_selected_family_identity: String,
    spatial_selected_product_identity_digest: String,
    spatial_equivalence_policy_identity_digest: String,
    spatial_selected_compatibility_basis_identity_digest: String,
    spatial_selected_reuse_basis_identity_digest: String,
    spatial_posture: EvidenceLookupReuseDecisionPosture,
    spatial_reuse_decision_identity_digest: Option<String>,
    spatial_rebuild_denial_identity_digest: Option<String>,
    selected_identity_fields_produced: &'static [&'static str],
    coverage_contributor: KernelTouchedGraphParityCoverageContributor,
}

impl ReuseFamilyContributorCatalogRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        kind: ReuseFamilyContributorRowKind,
        current_packet_or_identity_source: &'static str,
        carried_equivalence_or_compatibility_source: &'static str,
        carried_reuse_or_denial_source: &'static str,
        ordinary_path_live_caller_surface: &'static str,
        ordinary_path_live_caller_path: &'static str,
        route_packet_identity: String,
        topology_selected_family_identity: String,
        topology_selected_product_identity_digest: String,
        topology_equivalence_policy_identity_digest: String,
        topology_selected_compatibility_basis_identity_digest: String,
        topology_selected_reuse_basis_identity_digest: String,
        topology_posture: TopologyDerivedReuseDecisionPosture,
        topology_reuse_decision_identity_digest: Option<String>,
        topology_rebuild_denial_identity_digest: Option<String>,
        spatial_selected_family_identity: String,
        spatial_selected_product_identity_digest: String,
        spatial_equivalence_policy_identity_digest: String,
        spatial_selected_compatibility_basis_identity_digest: String,
        spatial_selected_reuse_basis_identity_digest: String,
        spatial_posture: EvidenceLookupReuseDecisionPosture,
        spatial_reuse_decision_identity_digest: Option<String>,
        spatial_rebuild_denial_identity_digest: Option<String>,
        selected_identity_fields_produced: &'static [&'static str],
    ) -> Result<Self, ReuseFamilyContributorCatalogError> {
        if current_packet_or_identity_source.is_empty()
            || carried_equivalence_or_compatibility_source.is_empty()
            || carried_reuse_or_denial_source.is_empty()
            || ordinary_path_live_caller_surface.is_empty()
            || ordinary_path_live_caller_path.is_empty()
            || route_packet_identity.is_empty()
            || topology_selected_family_identity.is_empty()
            || topology_selected_product_identity_digest.is_empty()
            || topology_equivalence_policy_identity_digest.is_empty()
            || topology_selected_compatibility_basis_identity_digest.is_empty()
            || topology_selected_reuse_basis_identity_digest.is_empty()
            || spatial_selected_family_identity.is_empty()
            || spatial_selected_product_identity_digest.is_empty()
            || spatial_equivalence_policy_identity_digest.is_empty()
            || spatial_selected_compatibility_basis_identity_digest.is_empty()
            || spatial_selected_reuse_basis_identity_digest.is_empty()
            || selected_identity_fields_produced.is_empty()
        {
            return Err(ReuseFamilyContributorCatalogError::new(
                ReuseFamilyContributorCatalogErrorKind::MissingCarriedIdentity,
                "reuse-family contributor row requires exact route, equivalence, compatibility, reuse, and denial identities",
            ));
        }

        let current_surface = match kind {
            ReuseFamilyContributorRowKind::Equivalence => {
                "current_worth_touched_graph_conflict_compiled_product_reuse_route_packet::selected_equivalence_policy_identity_digest"
            }
            ReuseFamilyContributorRowKind::Reuse => {
                "current_worth_touched_graph_conflict_compiled_product_reuse_route_packet::selected_reuse_basis_identity_digest"
            }
        };
        Ok(Self {
            kind,
            family_kind: TouchedGraphParityFamilyKind::CompiledProductReuse,
            current_packet_or_identity_source,
            carried_equivalence_or_compatibility_source,
            carried_reuse_or_denial_source,
            ordinary_path_live_caller_surface,
            ordinary_path_live_caller_path,
            route_packet_identity,
            topology_selected_family_identity,
            topology_selected_product_identity_digest,
            topology_equivalence_policy_identity_digest,
            topology_selected_compatibility_basis_identity_digest,
            topology_selected_reuse_basis_identity_digest,
            topology_posture,
            topology_reuse_decision_identity_digest,
            topology_rebuild_denial_identity_digest,
            spatial_selected_family_identity,
            spatial_selected_product_identity_digest,
            spatial_equivalence_policy_identity_digest,
            spatial_selected_compatibility_basis_identity_digest,
            spatial_selected_reuse_basis_identity_digest,
            spatial_posture,
            spatial_reuse_decision_identity_digest,
            spatial_rebuild_denial_identity_digest,
            selected_identity_fields_produced,
            coverage_contributor: KernelTouchedGraphParityCoverageContributor::new(
                current_surface,
                "crates/worth-kernel/src/workload_composition/planner_owned_routing/compiled_product_reuse_route.rs",
                current_packet_or_identity_source,
                "current_worth_touched_graph_conflict_public_proof_input::{compiled_product_reuse_route_packet_identity,selected_reuse_basis_identity_digest,spatial_equivalence_policy_identity_digest}",
                "semantic_reuse_contract",
                "crates/worth-kernel/src/workload_composition/planner_owned_routing/compiled_product_reuse_route.rs",
                selected_identity_fields_produced,
                KernelTouchedGraphParityQuerySurfaceKind::NotQuery,
                ordinary_path_live_caller_surface,
                ordinary_path_live_caller_path,
            ),
        })
    }

    pub const fn kind(&self) -> ReuseFamilyContributorRowKind {
        self.kind
    }

    pub const fn family_kind(&self) -> TouchedGraphParityFamilyKind {
        self.family_kind
    }

    pub const fn current_packet_or_identity_source(&self) -> &'static str {
        self.current_packet_or_identity_source
    }

    pub const fn carried_equivalence_or_compatibility_source(&self) -> &'static str {
        self.carried_equivalence_or_compatibility_source
    }

    pub const fn carried_reuse_or_denial_source(&self) -> &'static str {
        self.carried_reuse_or_denial_source
    }

    pub const fn ordinary_path_live_caller_surface(&self) -> &'static str {
        self.ordinary_path_live_caller_surface
    }

    pub const fn ordinary_path_live_caller_path(&self) -> &'static str {
        self.ordinary_path_live_caller_path
    }

    pub fn route_packet_identity(&self) -> &str {
        &self.route_packet_identity
    }

    pub fn topology_selected_family_identity(&self) -> &str {
        &self.topology_selected_family_identity
    }

    pub fn topology_selected_product_identity_digest(&self) -> &str {
        &self.topology_selected_product_identity_digest
    }

    pub fn topology_equivalence_policy_identity_digest(&self) -> &str {
        &self.topology_equivalence_policy_identity_digest
    }

    pub fn certified_topology_equivalence_basis_digest(&self) -> &str {
        &self.topology_selected_compatibility_basis_identity_digest
    }

    pub fn topology_selected_reuse_basis_identity_digest(&self) -> &str {
        &self.topology_selected_reuse_basis_identity_digest
    }

    pub const fn topology_posture(&self) -> TopologyDerivedReuseDecisionPosture {
        self.topology_posture
    }

    pub fn topology_reuse_decision_identity_digest(&self) -> Option<&str> {
        self.topology_reuse_decision_identity_digest.as_deref()
    }

    pub fn topology_rebuild_denial_identity_digest(&self) -> Option<&str> {
        self.topology_rebuild_denial_identity_digest.as_deref()
    }

    pub fn spatial_selected_family_identity(&self) -> &str {
        &self.spatial_selected_family_identity
    }

    pub fn spatial_selected_product_identity_digest(&self) -> &str {
        &self.spatial_selected_product_identity_digest
    }

    pub fn spatial_equivalence_policy_identity_digest(&self) -> &str {
        &self.spatial_equivalence_policy_identity_digest
    }

    pub fn certified_spatial_equivalence_basis_digest(&self) -> &str {
        &self.spatial_selected_compatibility_basis_identity_digest
    }

    pub fn spatial_selected_reuse_basis_identity_digest(&self) -> &str {
        &self.spatial_selected_reuse_basis_identity_digest
    }

    pub const fn spatial_posture(&self) -> EvidenceLookupReuseDecisionPosture {
        self.spatial_posture
    }

    pub fn spatial_reuse_decision_identity_digest(&self) -> Option<&str> {
        self.spatial_reuse_decision_identity_digest.as_deref()
    }

    pub fn spatial_rebuild_denial_identity_digest(&self) -> Option<&str> {
        self.spatial_rebuild_denial_identity_digest.as_deref()
    }

    pub const fn selected_identity_fields_produced(&self) -> &'static [&'static str] {
        self.selected_identity_fields_produced
    }

    pub const fn coverage_contributor(&self) -> &KernelTouchedGraphParityCoverageContributor {
        &self.coverage_contributor
    }

    #[cfg(test)]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn with_test_identity_override(
        mut self,
        topology_equivalence_policy_identity_digest: &str,
        topology_selected_compatibility_basis_identity_digest: &str,
        topology_selected_reuse_basis_identity_digest: &str,
        topology_reuse_decision_identity_digest: Option<&str>,
        topology_rebuild_denial_identity_digest: Option<&str>,
        spatial_equivalence_policy_identity_digest: &str,
        spatial_selected_compatibility_basis_identity_digest: &str,
        spatial_selected_reuse_basis_identity_digest: &str,
        spatial_reuse_decision_identity_digest: Option<&str>,
        spatial_rebuild_denial_identity_digest: Option<&str>,
    ) -> Self {
        self.topology_equivalence_policy_identity_digest =
            topology_equivalence_policy_identity_digest.to_string();
        self.topology_selected_compatibility_basis_identity_digest =
            topology_selected_compatibility_basis_identity_digest.to_string();
        self.topology_selected_reuse_basis_identity_digest =
            topology_selected_reuse_basis_identity_digest.to_string();
        self.topology_reuse_decision_identity_digest =
            topology_reuse_decision_identity_digest.map(str::to_string);
        self.topology_rebuild_denial_identity_digest =
            topology_rebuild_denial_identity_digest.map(str::to_string);
        self.spatial_equivalence_policy_identity_digest =
            spatial_equivalence_policy_identity_digest.to_string();
        self.spatial_selected_compatibility_basis_identity_digest =
            spatial_selected_compatibility_basis_identity_digest.to_string();
        self.spatial_selected_reuse_basis_identity_digest =
            spatial_selected_reuse_basis_identity_digest.to_string();
        self.spatial_reuse_decision_identity_digest =
            spatial_reuse_decision_identity_digest.map(str::to_string);
        self.spatial_rebuild_denial_identity_digest =
            spatial_rebuild_denial_identity_digest.map(str::to_string);
        self
    }

    #[cfg(test)]
    pub(crate) fn with_test_posture_override(
        mut self,
        topology_posture: TopologyDerivedReuseDecisionPosture,
        spatial_posture: EvidenceLookupReuseDecisionPosture,
    ) -> Self {
        self.topology_posture = topology_posture;
        self.spatial_posture = spatial_posture;
        self
    }
}

pub(crate) fn reuse_family_coverage_contributor_rows_from_catalog(
    rows: &[ReuseFamilyContributorCatalogRow],
) -> Result<Vec<KernelTouchedGraphParityCoverageContributor>, KernelTouchedGraphParityCoverageError>
{
    Ok(rows
        .iter()
        .map(|row| row.coverage_contributor().clone())
        .collect())
}
