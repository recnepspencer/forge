use schema::facade::platform::authority::touched_graph_conflict::{
    BatchAdmissionPlannerRouteWitnessKind, ConflictIndependencePlannerRouteWitnessKind,
};
use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityFamilyKind;

use crate::workload_composition::touched_graph_parity_closeout::family_contributors::{
    KernelTouchedGraphParityCoverageContributor, KernelTouchedGraphParityCoverageError,
    KernelTouchedGraphParityQuerySurfaceKind,
};

use super::error::{
    ConflictFamilyContributorCatalogError, ConflictFamilyContributorCatalogErrorKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConflictFamilyContributorRowKind {
    Conflict,
    Independence,
    BatchAdmission,
}

impl ConflictFamilyContributorRowKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Conflict => "conflict",
            Self::Independence => "independence",
            Self::BatchAdmission => "batch-admission",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConflictFamilyDenialWitnessKind {
    ConflictIndependence(ConflictIndependencePlannerRouteWitnessKind),
    BatchAdmission(BatchAdmissionPlannerRouteWitnessKind),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConflictFamilyContributorCatalogRow {
    kind: ConflictFamilyContributorRowKind,
    family_kind: TouchedGraphParityFamilyKind,
    current_packet_or_identity_source: &'static str,
    carried_overlap_or_plan_source: &'static str,
    carried_witness_source: &'static str,
    ordinary_path_live_caller_surface: &'static str,
    ordinary_path_live_caller_path: &'static str,
    current_packet_identity: String,
    supporting_packet_identities: Vec<String>,
    overlap_identity_digests: Vec<String>,
    selected_conflict_plan_digests: Vec<String>,
    independence_proof_digests: Vec<String>,
    selected_batch_plan_digest: String,
    denial_witness_identity: Option<String>,
    denial_witness_kind: Option<ConflictFamilyDenialWitnessKind>,
    selected_identity_fields_produced: &'static [&'static str],
    denial_witness_fields_produced: &'static [&'static str],
    coverage_contributor: KernelTouchedGraphParityCoverageContributor,
}

impl ConflictFamilyContributorCatalogRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        kind: ConflictFamilyContributorRowKind,
        current_packet_or_identity_source: &'static str,
        carried_overlap_or_plan_source: &'static str,
        carried_witness_source: &'static str,
        ordinary_path_live_caller_surface: &'static str,
        ordinary_path_live_caller_path: &'static str,
        current_packet_identity: String,
        supporting_packet_identities: Vec<String>,
        overlap_identity_digests: Vec<String>,
        selected_conflict_plan_digests: Vec<String>,
        independence_proof_digests: Vec<String>,
        selected_batch_plan_digest: String,
        denial_witness_identity: Option<String>,
        denial_witness_kind: Option<ConflictFamilyDenialWitnessKind>,
        selected_identity_fields_produced: &'static [&'static str],
        denial_witness_fields_produced: &'static [&'static str],
    ) -> Result<Self, ConflictFamilyContributorCatalogError> {
        if current_packet_or_identity_source.is_empty()
            || carried_overlap_or_plan_source.is_empty()
            || carried_witness_source.is_empty()
            || ordinary_path_live_caller_surface.is_empty()
            || ordinary_path_live_caller_path.is_empty()
            || current_packet_identity.is_empty()
            || selected_identity_fields_produced.is_empty()
        {
            return Err(ConflictFamilyContributorCatalogError::new(
                ConflictFamilyContributorCatalogErrorKind::MissingCarriedIdentity,
                "conflict-family contributor row requires exact current, carried plan/overlap, witness, and ordinary caller identities",
            ));
        }
        if denial_witness_identity.is_some() != denial_witness_kind.is_some() {
            return Err(ConflictFamilyContributorCatalogError::new(
                ConflictFamilyContributorCatalogErrorKind::MissingCarriedIdentity,
                "conflict-family contributor row must carry witness identity and witness kind together",
            ));
        }

        let current_surface = match kind {
            ConflictFamilyContributorRowKind::Conflict => {
                "current_worth_touched_graph_conflict_selected_route_packet::selected_conflict_plan_digests"
            }
            ConflictFamilyContributorRowKind::Independence => {
                "current_worth_touched_graph_conflict_selected_route_packet::independence_proof_digests"
            }
            ConflictFamilyContributorRowKind::BatchAdmission => {
                "current_worth_touched_graph_conflict_selected_route_packet::selected_batch_plan_digest"
            }
        };

        Ok(Self {
            kind,
            family_kind: TouchedGraphParityFamilyKind::ConflictIndependenceBatchAdmission,
            current_packet_or_identity_source,
            carried_overlap_or_plan_source,
            carried_witness_source,
            ordinary_path_live_caller_surface,
            ordinary_path_live_caller_path,
            current_packet_identity,
            supporting_packet_identities,
            overlap_identity_digests,
            selected_conflict_plan_digests,
            independence_proof_digests,
            selected_batch_plan_digest,
            denial_witness_identity,
            denial_witness_kind,
            selected_identity_fields_produced,
            denial_witness_fields_produced,
            coverage_contributor: KernelTouchedGraphParityCoverageContributor::new(
                current_surface,
                "crates/worth-kernel/src/workload_composition/planner_owned_routing/selected_route/current.rs",
                current_packet_or_identity_source,
                carried_overlap_or_plan_source,
                "selected_route_concurrency_family",
                "crates/worth-kernel/src/workload_composition/planner_owned_routing/",
                selected_identity_fields_produced,
                KernelTouchedGraphParityQuerySurfaceKind::NotQuery,
                ordinary_path_live_caller_surface,
                ordinary_path_live_caller_path,
            ),
        })
    }

    pub const fn kind(&self) -> ConflictFamilyContributorRowKind {
        self.kind
    }

    pub const fn family_kind(&self) -> TouchedGraphParityFamilyKind {
        self.family_kind
    }

    pub const fn current_packet_or_identity_source(&self) -> &'static str {
        self.current_packet_or_identity_source
    }

    pub const fn carried_overlap_or_plan_source(&self) -> &'static str {
        self.carried_overlap_or_plan_source
    }

    pub const fn carried_witness_source(&self) -> &'static str {
        self.carried_witness_source
    }

    pub const fn ordinary_path_live_caller_surface(&self) -> &'static str {
        self.ordinary_path_live_caller_surface
    }

    pub const fn ordinary_path_live_caller_path(&self) -> &'static str {
        self.ordinary_path_live_caller_path
    }

    pub fn current_packet_identity(&self) -> &str {
        &self.current_packet_identity
    }

    pub fn supporting_packet_identities(&self) -> &[String] {
        &self.supporting_packet_identities
    }

    pub fn overlap_identity_digests(&self) -> &[String] {
        &self.overlap_identity_digests
    }

    pub fn selected_conflict_plan_digests(&self) -> &[String] {
        &self.selected_conflict_plan_digests
    }

    pub fn independence_proof_digests(&self) -> &[String] {
        &self.independence_proof_digests
    }

    pub fn selected_batch_plan_digest(&self) -> &str {
        &self.selected_batch_plan_digest
    }

    pub fn denial_witness_identity(&self) -> Option<&str> {
        self.denial_witness_identity.as_deref()
    }

    pub const fn denial_witness_kind(&self) -> Option<ConflictFamilyDenialWitnessKind> {
        self.denial_witness_kind
    }

    pub const fn selected_identity_fields_produced(&self) -> &'static [&'static str] {
        self.selected_identity_fields_produced
    }

    pub const fn denial_witness_fields_produced(&self) -> &'static [&'static str] {
        self.denial_witness_fields_produced
    }

    pub const fn coverage_contributor(&self) -> &KernelTouchedGraphParityCoverageContributor {
        &self.coverage_contributor
    }

    #[cfg(test)]
    pub(crate) fn with_test_identity_override(
        mut self,
        current_packet_identity: &str,
        overlap_identity_digests: &[&str],
        selected_conflict_plan_digests: &[&str],
        independence_proof_digests: &[&str],
        selected_batch_plan_digest: &str,
        denial_witness_identity: Option<&str>,
        denial_witness_kind: Option<ConflictFamilyDenialWitnessKind>,
    ) -> Self {
        self.current_packet_identity = current_packet_identity.to_string();
        self.overlap_identity_digests = overlap_identity_digests
            .iter()
            .map(|v| (*v).to_string())
            .collect();
        self.selected_conflict_plan_digests = selected_conflict_plan_digests
            .iter()
            .map(|v| (*v).to_string())
            .collect();
        self.independence_proof_digests = independence_proof_digests
            .iter()
            .map(|v| (*v).to_string())
            .collect();
        self.selected_batch_plan_digest = selected_batch_plan_digest.to_string();
        self.denial_witness_identity = denial_witness_identity.map(str::to_string);
        self.denial_witness_kind = denial_witness_kind;
        self
    }
}

pub(crate) fn conflict_family_coverage_contributor_rows_from_catalog(
    rows: &[ConflictFamilyContributorCatalogRow],
) -> Result<Vec<KernelTouchedGraphParityCoverageContributor>, KernelTouchedGraphParityCoverageError>
{
    Ok(rows
        .iter()
        .map(|row| row.coverage_contributor().clone())
        .collect())
}
