use std::sync::OnceLock;

use topology::touched_graph_parity_closeout::{
    TopologyFamilyContributorCatalogRow, TopologyTouchedGraphParityCoverageContributor,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};
use worth_spatial::touched_graph_parity_closeout::{
    SpatialFamilyContributorCatalogRow, SpatialTouchedGraphParityCoverageContributor,
};

use super::row::{
    CrossFamilyCoverageFamilyKind as FamilyKind, CrossFamilyCoverageQuerySurfaceKind as QueryKind,
    CrossFamilyCoverageResidueClassification as ResidueClassification, CrossFamilyCoverageRow,
};
use super::validation::validate_rows;
use crate::workload_composition::performance_trace::trace_scope;
use crate::workload_composition::planner_owned_routing::{
    current_worth_touched_graph_conflict_public_facade_with_artifact_policy,
    current_worth_touched_graph_conflict_selected_route_packet,
    current_worth_workload_ordinary_consumer_cutover,
    WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy,
    WorthTouchedGraphConflictPublicFacade, WorthWorkloadOrdinaryConsumerCutover,
};
use crate::workload_composition::touched_graph_parity_closeout::family_contributors::{
    conflict_family_coverage_contributor_rows, current_spatial_family_contributor_catalog,
    current_topology_family_contributor_catalog,
    public_projection_family_coverage_contributor_rows_from_public_facade,
    replay_undo_coverage_contributor_rows_from_authorities, reuse_family_coverage_contributor_rows,
    spatial_coverage_contributor_rows, KernelTouchedGraphParityCoverageContributor,
};
use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityCoverageContributor;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CrossFamilyCoverageInventory {
    rows: Vec<CrossFamilyCoverageRow>,
    selected_route_identity_digest: String,
    inventory_digest: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CrossFamilyCoverageInventoryError {
    CurrentSurfaceUnavailable(&'static str),
    MismatchedAuthorityChain,
    InvalidCoverageRow,
    MissingLiveCallerProof,
    HiddenSecondOntology,
}

pub fn current_cross_family_coverage_inventory(
) -> Result<CrossFamilyCoverageInventory, CrossFamilyCoverageInventoryError> {
    static CACHE: OnceLock<CrossFamilyCoverageInventory> = OnceLock::new();
    if let Some(cached) = CACHE.get() {
        return Ok(cached.clone());
    }

    let inventory = build_current_cross_family_coverage_inventory()?;
    let _ = CACHE.set(inventory.clone());
    Ok(inventory)
}

fn build_current_cross_family_coverage_inventory(
) -> Result<CrossFamilyCoverageInventory, CrossFamilyCoverageInventoryError> {
    let selected_route = trace_scope("coverage_inventory_selected_route_packet", || {
        current_worth_touched_graph_conflict_selected_route_packet().map_err(|_| {
            CrossFamilyCoverageInventoryError::CurrentSurfaceUnavailable(
                "current_worth_touched_graph_conflict_selected_route_packet",
            )
        })
    })?;
    let public_facade = trace_scope("coverage_inventory_public_facade", || {
        current_worth_touched_graph_conflict_public_facade_with_artifact_policy(
            WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy::MinimalOperationalTruth,
        )
        .map_err(|_| {
            CrossFamilyCoverageInventoryError::CurrentSurfaceUnavailable(
                "current_worth_touched_graph_conflict_public_facade_with_artifact_policy",
            )
        })
    })?;
    let cutover = trace_scope("coverage_inventory_cutover", || {
        current_worth_workload_ordinary_consumer_cutover().map_err(|_| {
            CrossFamilyCoverageInventoryError::CurrentSurfaceUnavailable(
                "current_worth_workload_ordinary_consumer_cutover",
            )
        })
    })?;
    trace_scope("cross_family_coverage_inventory_from_authorities", || {
        cross_family_coverage_inventory_from_authorities(&selected_route, &public_facade, &cutover)
    })
}

pub(crate) fn cross_family_coverage_inventory_from_authorities(
    selected_route: &crate::workload_composition::WorthTouchedGraphConflictSelectedRoutePacket,
    public_facade: &WorthTouchedGraphConflictPublicFacade,
    cutover: &WorthWorkloadOrdinaryConsumerCutover,
) -> Result<CrossFamilyCoverageInventory, CrossFamilyCoverageInventoryError> {
    let public_proof = public_facade.public_proof();
    let diagnostics = public_facade.derived_diagnostics();
    let topology_catalog = trace_scope("cross_family_topology_catalog", || {
        current_topology_family_contributor_catalog().map_err(|_| {
            CrossFamilyCoverageInventoryError::CurrentSurfaceUnavailable(
                "current_topology_family_contributor_catalog",
            )
        })
    })?;
    let _spatial_catalog = trace_scope("cross_family_spatial_catalog", || {
        current_spatial_family_contributor_catalog().map_err(|_| {
            CrossFamilyCoverageInventoryError::CurrentSurfaceUnavailable(
                "current_spatial_family_contributor_catalog",
            )
        })
    })?;
    let topology_row = topology_catalog
        .rows()
        .iter()
        .find(|row| row.family_kind().as_str() == "read-routing")
        .ok_or(CrossFamilyCoverageInventoryError::MismatchedAuthorityChain)?;

    if public_proof.selected_route_identity_digest() != diagnostics.selected_route_identity_digest()
        || public_proof.selected_route_identity_digest()
            != selected_route.selected_route_identity_digest()
        || public_proof
            .milestone_fifteen_seed()
            .batch_execution_receipt_digest()
            != cutover.batch_execution_receipt().execution_receipt_digest()
        || !topology_row
            .selected_identity_fields_produced()
            .contains(&"selected_equivalence_family_identity")
        || !topology_row
            .selected_identity_fields_produced()
            .contains(&"compiled_product_identity_digest")
    {
        return Err(CrossFamilyCoverageInventoryError::MismatchedAuthorityChain);
    }

    let mut rows: Vec<CrossFamilyCoverageRow> = trace_scope("cross_family_topology_rows", || {
        topology_catalog
            .rows()
            .iter()
            .map(row_from_topology_catalog_row)
            .collect::<Result<Vec<_>, _>>()
    })?;
    rows.extend(trace_scope("cross_family_spatial_rows", || {
        spatial_coverage_contributor_rows()
            .map_err(|_| {
                CrossFamilyCoverageInventoryError::CurrentSurfaceUnavailable(
                    "current_spatial_family_contributor_catalog",
                )
            })?
            .iter()
            .map(row_from_spatial_catalog_row)
            .collect::<Result<Vec<_>, _>>()
    })?);
    rows.extend(trace_scope("cross_family_replay_undo_rows", || {
        replay_undo_coverage_contributor_rows_from_authorities(selected_route, cutover)
            .map_err(|_| {
                CrossFamilyCoverageInventoryError::CurrentSurfaceUnavailable(
                    "current_replay_undo_family_contributor_catalog",
                )
            })?
            .iter()
            .map(|row: &KernelTouchedGraphParityCoverageContributor| {
                row_from_kernel_contributor(FamilyKind::ReplayUndo, row.clone())
            })
            .collect::<Result<Vec<_>, _>>()
    })?);
    rows.extend(trace_scope("cross_family_conflict_rows", || {
        conflict_family_coverage_contributor_rows()
            .map_err(|_| {
                CrossFamilyCoverageInventoryError::CurrentSurfaceUnavailable(
                    "current_conflict_family_contributor_catalog",
                )
            })?
            .iter()
            .map(|row| {
                row_from_kernel_contributor(
                    FamilyKind::ConflictIndependenceBatchAdmission,
                    row.clone(),
                )
            })
            .collect::<Result<Vec<_>, _>>()
    })?);
    rows.extend(trace_scope("cross_family_reuse_rows", || {
        reuse_family_coverage_contributor_rows()
            .map_err(|_| {
                CrossFamilyCoverageInventoryError::CurrentSurfaceUnavailable(
                    "current_reuse_family_contributor_catalog",
                )
            })?
            .iter()
            .map(|row| row_from_kernel_contributor(FamilyKind::CompiledProductReuse, row.clone()))
            .collect::<Result<Vec<_>, _>>()
    })?);
    rows.extend(trace_scope("cross_family_public_projection_rows", || {
        public_projection_family_coverage_contributor_rows_from_public_facade(public_facade)
            .map_err(|_| {
                CrossFamilyCoverageInventoryError::CurrentSurfaceUnavailable(
                    "current_public_projection_contributor_catalog",
                )
            })?
            .into_iter()
            .map(|row: KernelTouchedGraphParityCoverageContributor| {
                let family_kind = if row.current_surface() == "public_proof_inspection" {
                    FamilyKind::PublicProof
                } else {
                    FamilyKind::DerivedDiagnostics
                };
                row_from_kernel_contributor(family_kind, row)
            })
            .collect::<Result<Vec<_>, _>>()
    })?);

    trace_scope("cross_family_validate_rows", || validate_rows(&rows))?;

    let inventory_digest = trace_scope("cross_family_inventory_digest", || {
        truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &rows
                .iter()
                .flat_map(|row: &CrossFamilyCoverageRow| {
                    [
                        format!("family-kind:{}", row.family_kind().as_str()),
                        format!("surface:{}", row.current_surface()),
                        format!("source:{}", row.source_path()),
                        format!("caller-surface:{}", row.ordinary_path_live_caller_surface()),
                        format!("caller-path:{}", row.ordinary_path_live_caller_path()),
                        format!("replacement:{}", row.replacement_lane()),
                        format!("query-surface:{}", row.query_surface_kind().as_str()),
                    ]
                })
                .chain(std::iter::once(format!(
                    "selected-route:{}",
                    selected_route.selected_route_identity_digest()
                )))
                .chain(std::iter::once(format!(
                    "public-proof:{}",
                    public_proof.proof_chain_digest()
                )))
                .chain(std::iter::once(format!(
                    "diagnostics:{}",
                    diagnostics.decision_trace_identity_digest()
                )))
                .collect::<Vec<_>>(),
        )
    });

    Ok(CrossFamilyCoverageInventory {
        rows,
        selected_route_identity_digest: selected_route.selected_route_identity_digest().to_string(),
        inventory_digest,
    })
}

fn row(
    family_kind: FamilyKind,
    current_surface: &'static str,
    source_path: &'static str,
    current_owner_crate: &'static str,
    upstream_authority_source: &'static str,
    selected_route_or_equivalence_source: &'static str,
    public_or_internal_consumer_kind: &'static str,
    replacement_lane: &'static str,
    selected_identity_fields_consumed: &'static [&'static str],
    query_surface_kind: QueryKind,
    ordinary_path_live_caller_surface: &'static str,
    ordinary_path_live_caller_path: &'static str,
    ordinary_path_reachable: bool,
) -> CrossFamilyCoverageRow {
    CrossFamilyCoverageRow::from_contributor(
        family_kind,
        current_owner_crate,
        TouchedGraphParityCoverageContributor::new(
            current_surface,
            source_path,
            upstream_authority_source,
            selected_route_or_equivalence_source,
            public_or_internal_consumer_kind,
            replacement_lane,
            selected_identity_fields_consumed,
            query_surface_kind,
            ordinary_path_live_caller_surface,
            ordinary_path_live_caller_path,
        ),
        ordinary_path_reachable,
        ResidueClassification::OrdinaryPathCarried,
    )
}

fn row_from_topology_contributor(
    family_kind: FamilyKind,
    contributor: TopologyTouchedGraphParityCoverageContributor,
) -> Result<CrossFamilyCoverageRow, CrossFamilyCoverageInventoryError> {
    Ok(row(
        family_kind,
        contributor.current_surface(),
        contributor.source_path(),
        "worth-topo",
        contributor.upstream_authority_source(),
        contributor.selected_route_or_equivalence_source(),
        contributor.public_or_internal_consumer_kind(),
        contributor.replacement_lane(),
        contributor.selected_identity_fields_consumed(),
        contributor.query_surface_kind(),
        contributor.ordinary_path_live_caller_surface(),
        contributor.ordinary_path_live_caller_path(),
        true,
    ))
}

fn row_from_topology_catalog_row(
    row: &TopologyFamilyContributorCatalogRow,
) -> Result<CrossFamilyCoverageRow, CrossFamilyCoverageInventoryError> {
    let family_kind = match row.family_kind() {
        schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityFamilyKind::ReadRouting => FamilyKind::ReadRouting,
        schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityFamilyKind::ValidatorInvariantRouting => FamilyKind::ValidatorInvariantRouting,
        schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityFamilyKind::Invalidation => FamilyKind::Invalidation,
        _ => return Err(CrossFamilyCoverageInventoryError::HiddenSecondOntology),
    };
    row_from_topology_contributor(family_kind, row.coverage_contributor().clone())
}

fn row_from_spatial_contributor(
    family_kind: FamilyKind,
    contributor: SpatialTouchedGraphParityCoverageContributor,
) -> Result<CrossFamilyCoverageRow, CrossFamilyCoverageInventoryError> {
    Ok(row(
        family_kind,
        contributor.current_surface(),
        contributor.source_path(),
        "worth-spatial",
        contributor.upstream_authority_source(),
        contributor.selected_route_or_equivalence_source(),
        contributor.public_or_internal_consumer_kind(),
        contributor.replacement_lane(),
        contributor.selected_identity_fields_consumed(),
        contributor.query_surface_kind(),
        contributor.ordinary_path_live_caller_surface(),
        contributor.ordinary_path_live_caller_path(),
        true,
    ))
}

fn row_from_spatial_catalog_row(
    row: &SpatialFamilyContributorCatalogRow,
) -> Result<CrossFamilyCoverageRow, CrossFamilyCoverageInventoryError> {
    row_from_spatial_contributor(row.family_kind(), row.coverage_contributor().clone())
}

fn row_from_kernel_contributor(
    family_kind: FamilyKind,
    contributor: KernelTouchedGraphParityCoverageContributor,
) -> Result<CrossFamilyCoverageRow, CrossFamilyCoverageInventoryError> {
    Ok(row(
        family_kind,
        contributor.current_surface(),
        contributor.source_path(),
        "worth-kernel",
        contributor.upstream_authority_source(),
        contributor.selected_route_or_equivalence_source(),
        contributor.public_or_internal_consumer_kind(),
        contributor.replacement_lane(),
        contributor.selected_identity_fields_consumed(),
        contributor.query_surface_kind(),
        contributor.ordinary_path_live_caller_surface(),
        contributor.ordinary_path_live_caller_path(),
        true,
    ))
}

impl CrossFamilyCoverageInventory {
    pub fn rows(&self) -> &[CrossFamilyCoverageRow] {
        &self.rows
    }

    pub fn selected_route_identity_digest(&self) -> &str {
        &self.selected_route_identity_digest
    }

    pub fn inventory_digest(&self) -> &str {
        &self.inventory_digest
    }
}
