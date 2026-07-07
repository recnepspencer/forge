use forge_query::facade::ForgeQueryApplicationFacade;
use schema::facade::platform::relations::TopologyRelationKind;
use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;
use serde_json::Value;
use std::sync::OnceLock;

use super::TopologyQueryBackedReadFamilyRouteInput;
#[cfg(any(test, feature = "test-support-lowering"))]
use super::{
    admit_topology_query_backed_read_family_route,
    admit_topology_query_backed_read_family_route_with_selected_route_authority,
    TopologyQueryBackedConsumerCutover, TopologyQueryBackedReadFamilySelectedRouteAuthority,
};
#[cfg(not(any(test, feature = "test-support-lowering")))]
use super::{admit_topology_query_backed_read_family_route, TopologyQueryBackedConsumerCutover};
use crate::certification::support::historical_query_snapshot::historical_query_snapshot_for_read_basis;
use crate::certification::support::read_basis_query_runtime::HistoricalReadBasisQueryRuntime;
use crate::compiled_product_family::{
    current_topology_compiled_product_family_catalog, select_topology_compiled_product_family,
    TopologyCompiledProductConsumer,
};
use crate::derived_invalidation_compiled_product_admission::{
    admit_topology_compiled_product_input, TopologyCompiledProductAdmissionRequest,
};
use crate::derived_topology::compiled_product_consumer_cutover::{
    build_derived_equivalence_contract_report, DerivedEquivalenceContractReport,
};
use crate::facade::{topology_runtime, TopologyRuntimeAdapters};
use crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces;
use crate::projection::runtime_boundary::query_support::{
    relation_kind_name, topology_source_identity,
};
use crate::query_domain::{
    topology_current_head_authoritative_context, topology_current_head_query_basis_evidence,
    topology_query_domain_entry, TopologyCurrentHeadReadHandleExt, TopologyReadAnchorIdentity,
};
use crate::selected_equivalence_family::{
    current_topology_selected_equivalence_family_catalog, select_topology_equivalence_family,
};
use crate::test_support::schema_topology_authoring_boundary::seed_milestone_one_primitive_through_schema_execution;
use crate::validation::reference_integrity::build_milestone_one_runtime;

#[derive(Clone, Debug)]
pub(crate) struct CurrentTopologyQueryBackedReadFamilyArtifacts {
    read_basis: schema::facade::topology_authoring::DerivedTopologyReadBasis,
    materialized: crate::derived_topology::materialized_graph::MaterializedTopologyView,
    interpreted: crate::derived_topology::traversal_views::InterpretedTopologyView,
    validation: crate::validation::DerivedTopologyValidationReport,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyQueryBackedConsumerCutoverCurrentError {
    detail: String,
}

pub fn current_topology_query_backed_consumer_cutover(
) -> Result<TopologyQueryBackedConsumerCutover, TopologyQueryBackedConsumerCutoverCurrentError> {
    static CACHE: OnceLock<TopologyQueryBackedConsumerCutover> = OnceLock::new();
    if let Some(cached) = CACHE.get() {
        return Ok(cached.clone());
    }

    let cutover = admit_topology_query_backed_read_family_route(
        &current_topology_query_backed_read_family_route_input()?,
    )
    .map_err(current_runtime_error)?;
    let _ = CACHE.set(cutover.clone());
    Ok(cutover)
}

#[cfg(any(test, feature = "test-support-lowering"))]
pub fn admit_current_topology_query_backed_consumer_cutover_with_selected_route_authority<
    A: TopologyQueryBackedReadFamilySelectedRouteAuthority,
>(
    authority: &A,
) -> Result<TopologyQueryBackedConsumerCutover, TopologyQueryBackedConsumerCutoverCurrentError> {
    let route_input = current_topology_query_backed_read_family_route_input()?;
    admit_topology_query_backed_read_family_route_with_selected_route_authority(
        &route_input,
        authority,
    )
    .map_err(current_runtime_error)
}

pub(crate) fn current_topology_query_backed_read_family_route_input() -> Result<
    TopologyQueryBackedReadFamilyRouteInput<'static>,
    TopologyQueryBackedConsumerCutoverCurrentError,
> {
    current_topology_query_backed_read_family_route_input_with_hostile_selected_basis_overrides(
        None, None,
    )
}

pub(crate) fn current_topology_query_backed_consumer_cutover_with_hostile_selected_basis_overrides(
    selected_compatibility_basis_identity_digest: Option<&str>,
    selected_reuse_basis_identity_digest: Option<&str>,
) -> Result<TopologyQueryBackedConsumerCutover, TopologyQueryBackedConsumerCutoverCurrentError> {
    admit_topology_query_backed_read_family_route(
        &current_topology_query_backed_read_family_route_input_with_hostile_selected_basis_overrides(
            selected_compatibility_basis_identity_digest,
            selected_reuse_basis_identity_digest,
        )?,
    )
    .map_err(current_runtime_error)
}

pub(crate) fn current_topology_query_backed_read_family_route_input_with_hostile_selected_basis_overrides(
    selected_compatibility_basis_identity_digest: Option<&str>,
    selected_reuse_basis_identity_digest: Option<&str>,
) -> Result<
    TopologyQueryBackedReadFamilyRouteInput<'static>,
    TopologyQueryBackedConsumerCutoverCurrentError,
> {
    let (runtime, historical_artifacts) =
        current_topology_query_backed_read_family_runtime_and_artifacts()?;
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(
        adapters,
        "phase13.current-topology-query-backed-consumer-cutover.runtime",
    )
    .map_err(current_runtime_error)?;
    let surfaces =
        declare_topology_query_surfaces(&mut workspace).map_err(current_runtime_error)?;
    let relation_rows = workspace.read::<Value>(surfaces.relations());
    let source_identity = relation_rows
        .iter()
        .find_map(|row| {
            (relation_kind_name(row) == TopologyRelationKind::HalfEdgeRadialNext.kind_name())
                .then(|| topology_source_identity(row).map(str::to_string))
                .flatten()
        })
        .or_else(|| {
            relation_rows.iter().find_map(|row| {
                (relation_kind_name(row) == TopologyRelationKind::HalfEdgeNext.kind_name())
                    .then(|| topology_source_identity(row).map(str::to_string))
                    .flatten()
            })
        })
        .ok_or_else(|| {
            TopologyQueryBackedConsumerCutoverCurrentError::new(
                "current topology query-backed route input could not locate a loop-cycle anchor",
            )
        })?;
    let facade = ForgeQueryApplicationFacade::runtime_backed_default();
    let basis_evidence =
        topology_current_head_query_basis_evidence(&facade).ok_or_else(|| {
            TopologyQueryBackedConsumerCutoverCurrentError::new(
                "current topology query-backed route input could not admit current-head query basis evidence",
            )
        })?;
    let handle = topology_query_domain_entry(&facade)
        .with_operating_context(topology_current_head_authoritative_context())
        .validate()
        .map_err(current_runtime_error)?
        .admit()
        .map_err(current_runtime_error)?;
    let anchor = TopologyReadAnchorIdentity::from_runtime_row_label(&source_identity);
    let mut reads = handle.topology_reads(&mut workspace);
    let _radial = reads
        .radial_half_edge_neighborhood(&anchor)
        .map_err(current_runtime_error)?;
    let _loop_cycle = reads
        .loop_cycle(&anchor, 5)
        .map_err(current_runtime_error)?;
    let equivalence_contract = build_query_backed_equivalence_contract_from_raw_inputs(
        historical_artifacts.read_basis(),
        historical_artifacts.materialized(),
        historical_artifacts.interpreted(),
        historical_artifacts.validation(),
        selected_compatibility_basis_identity_digest,
        selected_reuse_basis_identity_digest,
    )?;
    let leaked_contract = Box::leak(Box::new(equivalence_contract));
    Ok(TopologyQueryBackedReadFamilyRouteInput::new(
        &reads,
        &basis_evidence,
        leaked_contract,
    ))
}

fn build_query_backed_equivalence_contract_from_raw_inputs(
    read_basis: &schema::facade::topology_authoring::DerivedTopologyReadBasis,
    materialized: &crate::derived_topology::materialized_graph::MaterializedTopologyView,
    interpreted: &crate::derived_topology::traversal_views::InterpretedTopologyView,
    validation: &crate::validation::DerivedTopologyValidationReport,
    selected_compatibility_basis_identity_digest: Option<&str>,
    selected_reuse_basis_identity_digest: Option<&str>,
) -> Result<DerivedEquivalenceContractReport, TopologyQueryBackedConsumerCutoverCurrentError> {
    let catalog = current_topology_compiled_product_family_catalog();
    let admitted = admit_topology_compiled_product_input(
        &catalog,
        TopologyCompiledProductAdmissionRequest::for_historical_read_basis(
            TopologyCompiledProductConsumer::DerivedEquivalenceContractProjection,
            read_basis,
        ),
    )
    .map_err(current_runtime_error)?;
    let selected_equivalence_family = select_topology_equivalence_family(
        &current_topology_selected_equivalence_family_catalog(),
        &admitted,
    )
    .map_err(current_runtime_error)?
    .with_hostile_selected_basis_overrides(
        selected_compatibility_basis_identity_digest,
        selected_reuse_basis_identity_digest,
    );
    let selected_family = select_topology_compiled_product_family(
        &catalog,
        admitted.clone().into_family_admitted_input(),
    )
    .map_err(current_runtime_error)?;
    let lowered_identity = selected_family
        .compile_product_identity(materialized, interpreted, validation)
        .map_err(current_runtime_error)?;
    Ok(build_derived_equivalence_contract_report(
        admitted.source_authority_basis().authority_snapshot_id(),
        admitted
            .source_authority_basis()
            .authority_branch_id()
            .to_string(),
        read_basis.authoritative_mutation_origin(),
        read_basis.derivation_origin(),
        admitted
            .source_authority_basis()
            .truth_basis_digest_hex()
            .to_string(),
        admitted.source_authority_basis().touched_aspect_count(),
        admitted
            .locality_basis()
            .triggered_invalidation_targets()
            .to_vec(),
        admitted.source_authority_basis().precision_fallback_count(),
        admitted
            .source_authority_basis()
            .precision_budget_fallback_count(),
        Some(&selected_equivalence_family),
        Some(selected_family.declaration().identity()),
        Some(&lowered_identity),
        materialized,
        interpreted,
        validation,
    ))
}

fn current_topology_query_backed_read_family_runtime_and_artifacts() -> Result<
    (
        forge_relational::facade::runtime::RelationalRuntime,
        CurrentTopologyQueryBackedReadFamilyArtifacts,
    ),
    TopologyQueryBackedConsumerCutoverCurrentError,
> {
    let mut runtime = build_milestone_one_runtime().map_err(current_runtime_error)?;
    let verified = seed_milestone_one_primitive_through_schema_execution(
        &mut runtime,
        "phase13.current-topology-query-backed-consumer-cutover",
        &MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    )
    .map_err(current_runtime_error)?;
    let mut historical_query_runtime = HistoricalReadBasisQueryRuntime::open(
        &runtime,
        verified.read_basis().clone(),
        "phase13.current-topology-query-backed-consumer-cutover.historical",
    )
    .map_err(current_runtime_error)?;
    let historical_snapshot =
        historical_query_snapshot_for_read_basis(&mut historical_query_runtime)
            .map_err(current_runtime_error)?;
    Ok((
        runtime,
        CurrentTopologyQueryBackedReadFamilyArtifacts {
            read_basis: verified.read_basis().clone(),
            materialized: historical_snapshot.materialized().clone(),
            interpreted: historical_snapshot.interpreted().clone(),
            validation: historical_snapshot.validation().clone(),
        },
    ))
}

impl CurrentTopologyQueryBackedReadFamilyArtifacts {
    pub(crate) fn read_basis(
        &self,
    ) -> &schema::facade::topology_authoring::DerivedTopologyReadBasis {
        &self.read_basis
    }

    pub(crate) fn materialized(
        &self,
    ) -> &crate::derived_topology::materialized_graph::MaterializedTopologyView {
        &self.materialized
    }

    pub(crate) fn interpreted(
        &self,
    ) -> &crate::derived_topology::traversal_views::InterpretedTopologyView {
        &self.interpreted
    }

    pub(crate) fn validation(&self) -> &crate::validation::DerivedTopologyValidationReport {
        &self.validation
    }
}

impl TopologyQueryBackedConsumerCutoverCurrentError {
    pub(crate) fn new(detail: impl Into<String>) -> Self {
        Self {
            detail: detail.into(),
        }
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

fn current_runtime_error(
    error: impl std::fmt::Debug,
) -> TopologyQueryBackedConsumerCutoverCurrentError {
    TopologyQueryBackedConsumerCutoverCurrentError::new(format!(
        "current topology query-backed route input did not assemble: {error:?}"
    ))
}
