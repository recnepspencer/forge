use forge_relational::facade::runtime::{RelationalReadView, RelationalRuntime};
use worth_schema::facade::{
    CertifiedTopologyInterpretation, DerivedTopologyReadBasis, PersistedTopologyTruthBatch,
    VerifiedTopologyCommit, WorthBoundaryEnvelope, WorthBoundaryFailure, WorthDecisionTrace,
    WorthDerivedTraceAnchor, WorthDerivedTraceEvidence, WorthIntegrityMarkers,
    WorthNamedCounter, WorthPerformanceAccounting, WorthTopologyReadArtifact,
    WorthTraceAvailability,
};

use crate::facade::{
    build_derived_read_diagnostics,
    build_derived_equivalence_contract, build_topology_read_artifact, certify_topology_view,
    interpret_topology_view,
    validate_interpreted_topology,
};
use crate::diagnostics::{build_derived_invalidation_report, WorthDerivedReadDiagnostics};
use crate::interpretation::InterpretedTopologyView;
use crate::materialization::MaterializedTopologyView;
use crate::parity::WorthDerivedEquivalenceContractReport;
use crate::materialization::{WorthTopologyMaterializationError, WorthTopologyMaterializer};
use crate::validators::{DerivedTopologyValidationReport, WorthTopologyValidationError};

#[derive(Debug)]
pub enum WorthTopologyReadError {
    ReadView(String),
    Materialization(WorthTopologyMaterializationError),
    Validation(WorthTopologyValidationError),
}

impl std::fmt::Display for WorthTopologyReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReadView(error) => write!(f, "read view: {error}"),
            Self::Materialization(error) => write!(f, "materialization: {error}"),
            Self::Validation(error) => write!(f, "validation: {error}"),
        }
    }
}

impl std::error::Error for WorthTopologyReadError {}

impl From<WorthTopologyMaterializationError> for WorthTopologyReadError {
    fn from(value: WorthTopologyMaterializationError) -> Self {
        Self::Materialization(value)
    }
}

impl From<WorthTopologyValidationError> for WorthTopologyReadError {
    fn from(value: WorthTopologyValidationError) -> Self {
        Self::Validation(value)
    }
}

pub type WorthTracedTopologyReadArtifact = WorthBoundaryEnvelope<WorthTopologyReadArtifact>;
pub type WorthTracedCertifiedTopologyInterpretation =
    WorthBoundaryEnvelope<CertifiedTopologyInterpretation>;
pub type WorthTracedMaterializedTopologyView = WorthBoundaryEnvelope<MaterializedTopologyView>;
pub type WorthTracedDerivedEquivalenceContract =
    WorthBoundaryEnvelope<WorthDerivedEquivalenceContractReport>;
pub type WorthTracedDerivedReadDiagnostics = WorthBoundaryEnvelope<WorthDerivedReadDiagnostics>;

#[derive(Debug, Clone)]
pub struct StagedWorthTopologyRead {
    materialized: MaterializedTopologyView,
    interpreted: InterpretedTopologyView,
    validation: DerivedTopologyValidationReport,
}

impl StagedWorthTopologyRead {
    pub(crate) fn new(
        materialized: MaterializedTopologyView,
        interpreted: InterpretedTopologyView,
        validation: DerivedTopologyValidationReport,
    ) -> Self {
        Self {
            materialized,
            interpreted,
            validation,
        }
    }

    pub fn materialized(&self) -> &MaterializedTopologyView {
        &self.materialized
    }

    pub fn interpreted(&self) -> &InterpretedTopologyView {
        &self.interpreted
    }

    pub fn validation(&self) -> &DerivedTopologyValidationReport {
        &self.validation
    }
}

pub(crate) fn stage_topology_read_from_view(
    read_view: &RelationalReadView,
) -> Result<StagedWorthTopologyRead, WorthTopologyReadError> {
    let materialized = WorthTopologyMaterializer::materialize_from_truth(read_view)?;
    let interpreted = interpret_topology_view(&materialized);
    let validation = validate_interpreted_topology(&materialized, &interpreted)?;
    Ok(StagedWorthTopologyRead::new(
        materialized,
        interpreted,
        validation,
    ))
}

pub(crate) fn stage_topology_read_from_view_traced(
    read_view: &RelationalReadView,
    basis: &DerivedTopologyReadBasis,
) -> Result<WorthBoundaryEnvelope<StagedWorthTopologyRead>, WorthBoundaryFailure<WorthTopologyReadError>>
{
    let staged =
        stage_topology_read_from_view(read_view).map_err(|error| read_failure_for_basis(basis, error))?;
    let materialized = staged.materialized().clone();
    let interpreted = staged.interpreted().clone();
    let validation = staged.validation().clone();
    Ok(traced_read_envelope(
        staged,
        basis,
        &materialized,
        &interpreted,
        &validation,
    ))
}

pub struct WorthTopologyReader<'a> {
    runtime: &'a RelationalRuntime,
}

impl<'a> WorthTopologyReader<'a> {
    pub fn new(runtime: &'a RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn read_basis_from_persisted_truth(
        &self,
        persisted_truth: &PersistedTopologyTruthBatch,
    ) -> DerivedTopologyReadBasis {
        DerivedTopologyReadBasis::from_persisted_truth(persisted_truth)
    }

    pub fn read_basis_from_verified_commit(
        &self,
        verified: &VerifiedTopologyCommit,
    ) -> DerivedTopologyReadBasis {
        verified.read_basis.clone()
    }

    pub fn read_view(
        &self,
        basis: &DerivedTopologyReadBasis,
    ) -> Result<RelationalReadView, WorthTopologyReadError> {
        self.runtime
            .read_truth()
            .read_snapshot(basis.snapshot())
            .ok_or_else(|| {
                WorthTopologyReadError::ReadView(format!(
                    "worth topology reader could not open snapshot {:?}",
                    basis.snapshot()
                ))
            })
    }

    pub fn read_artifact_traced(
        &self,
        basis: &DerivedTopologyReadBasis,
    ) -> Result<WorthTracedTopologyReadArtifact, WorthBoundaryFailure<WorthTopologyReadError>> {
        let staged = self.stage_traced(basis)?;
        Ok(traced_read_envelope(
            build_topology_read_artifact(basis, staged.primary_result().interpreted()),
            basis,
            staged.primary_result().materialized(),
            staged.primary_result().interpreted(),
            staged.primary_result().validation(),
        ))
    }

    pub fn interpret_traced(
        &self,
        basis: &DerivedTopologyReadBasis,
    ) -> Result<
        WorthTracedCertifiedTopologyInterpretation,
        WorthBoundaryFailure<WorthTopologyReadError>,
    > {
        let staged = self.stage_traced(basis)?;
        Ok(traced_read_envelope(
            certify_topology_view(basis.clone(), staged.primary_result().interpreted()),
            basis,
            staged.primary_result().materialized(),
            staged.primary_result().interpreted(),
            staged.primary_result().validation(),
        ))
    }

    pub fn materialize_traced(
        &self,
        basis: &DerivedTopologyReadBasis,
    ) -> Result<WorthTracedMaterializedTopologyView, WorthBoundaryFailure<WorthTopologyReadError>> {
        let read_view = self.read_view_traced(basis)?;
        let materialized = WorthTopologyMaterializer::materialize_from_truth(read_view.primary_result())
            .map_err(|error| read_failure_for_basis(basis, error.into()))?;
        Ok(traced_materialized_envelope(materialized, basis))
    }

    pub fn interpret_materialized(
        &self,
        materialized: &MaterializedTopologyView,
    ) -> InterpretedTopologyView {
        interpret_topology_view(materialized)
    }

    pub fn validate_interpreted(
        &self,
        materialized: &MaterializedTopologyView,
        interpreted: &InterpretedTopologyView,
    ) -> Result<DerivedTopologyValidationReport, WorthTopologyReadError> {
        Ok(validate_interpreted_topology(materialized, interpreted)?)
    }

    pub fn equivalence_contract_traced(
        &self,
        basis: &DerivedTopologyReadBasis,
    ) -> Result<
        WorthTracedDerivedEquivalenceContract,
        WorthBoundaryFailure<WorthTopologyReadError>,
    > {
        let staged = self.stage_traced(basis)?;
        Ok(traced_read_envelope(
            build_derived_equivalence_contract(
                basis,
                staged.primary_result().materialized(),
                staged.primary_result().interpreted(),
                staged.primary_result().validation(),
            ),
            basis,
            staged.primary_result().materialized(),
            staged.primary_result().interpreted(),
            staged.primary_result().validation(),
        ))
    }

    pub fn diagnostics_traced(
        &self,
        basis: &DerivedTopologyReadBasis,
    ) -> Result<WorthTracedDerivedReadDiagnostics, WorthBoundaryFailure<WorthTopologyReadError>> {
        let staged = self.stage_traced(basis)?;
        Ok(traced_read_envelope(
            build_derived_read_diagnostics(
                basis,
                staged.primary_result().materialized(),
                staged.primary_result().interpreted(),
                staged.primary_result().validation(),
            ),
            basis,
            staged.primary_result().materialized(),
            staged.primary_result().interpreted(),
            staged.primary_result().validation(),
        ))
    }

    pub(crate) fn stage(
        &self,
        basis: &DerivedTopologyReadBasis,
    ) -> Result<StagedWorthTopologyRead, WorthTopologyReadError> {
        self.stage_traced(basis)
            .map(WorthBoundaryEnvelope::into_primary_result)
            .map_err(WorthBoundaryFailure::into_error)
    }

    pub(crate) fn read_view_traced(
        &self,
        basis: &DerivedTopologyReadBasis,
    ) -> Result<WorthBoundaryEnvelope<RelationalReadView>, WorthBoundaryFailure<WorthTopologyReadError>>
    {
        let read_view = self
            .runtime
            .read_truth()
            .read_snapshot(basis.snapshot())
            .ok_or_else(|| {
                read_failure_for_basis(
                    basis,
                    WorthTopologyReadError::ReadView(format!(
                        "worth topology reader could not open snapshot {:?}",
                        basis.snapshot()
                    )),
                )
            })?;
        Ok(WorthBoundaryEnvelope::success(
            read_view,
            Vec::new(),
            WorthDecisionTrace {
                derived_anchor: Some(WorthDerivedTraceAnchor::from_read_basis(basis)),
                ..WorthDecisionTrace::default()
            },
            integrity_markers_for_basis(basis),
            WorthPerformanceAccounting::default(),
        ))
    }

    pub(crate) fn stage_traced(
        &self,
        basis: &DerivedTopologyReadBasis,
    ) -> Result<WorthBoundaryEnvelope<StagedWorthTopologyRead>, WorthBoundaryFailure<WorthTopologyReadError>>
    {
        let read_view = self.read_view_traced(basis)?;
        stage_topology_read_from_view_traced(read_view.primary_result(), basis)
    }
}

fn traced_materialized_envelope(
    materialized: MaterializedTopologyView,
    basis: &DerivedTopologyReadBasis,
) -> WorthTracedMaterializedTopologyView {
    let fallback_classes = materialized
        .report()
        .fallback_class
        .map(materialization_fallback_class_name)
        .into_iter()
        .collect();
    WorthBoundaryEnvelope::success(
        materialized.clone(),
        Vec::new(),
        WorthDecisionTrace {
            authority_anchor: None,
            bridge_anchor: None,
            derived_anchor: Some(WorthDerivedTraceAnchor::from_read_basis(basis)),
            signal_anchor: None,
            authority: None,
            bridge: None,
            derived: Some(WorthDerivedTraceEvidence {
                availability: WorthTraceAvailability::Present,
                invalidation_target_count: build_derived_invalidation_report(basis)
                    .triggered_target_count,
                fallback_classes,
                equivalence_digest: None,
            }),
            signal: None,
        },
        integrity_markers_for_basis(basis),
        WorthPerformanceAccounting::new([
            WorthNamedCounter::new(
                "derived.materialization.entity_count",
                materialized.report().breadth.entity_count as u64,
            ),
            WorthNamedCounter::new(
                "derived.materialization.relation_count",
                materialized.report().breadth.relation_count as u64,
            ),
            WorthNamedCounter::new(
                "derived.materialization.topology_entity_count",
                materialized.report().breadth.topology_entity_count as u64,
            ),
            WorthNamedCounter::new(
                "derived.materialization.topology_relation_count",
                materialized.report().breadth.topology_relation_count as u64,
            ),
            WorthNamedCounter::new(
                "derived.materialization.whole_view_materialization",
                u64::from(materialized.report().whole_view_materialization),
            ),
        ]),
    )
}

fn traced_read_envelope<T>(
    primary_result: T,
    basis: &DerivedTopologyReadBasis,
    materialized: &MaterializedTopologyView,
    interpreted: &InterpretedTopologyView,
    validation: &DerivedTopologyValidationReport,
) -> WorthBoundaryEnvelope<T> {
    let diagnostics = build_derived_read_diagnostics(basis, materialized, interpreted, validation);
    let derived_trace = derived_trace_evidence_for(&diagnostics);
    WorthBoundaryEnvelope::success(
        primary_result,
        Vec::new(),
        WorthDecisionTrace {
            authority_anchor: None,
            bridge_anchor: None,
            derived_anchor: Some(WorthDerivedTraceAnchor::from_read_basis(basis)),
            signal_anchor: None,
            authority: None,
            bridge: None,
            derived: Some(derived_trace),
            signal: None,
        },
        integrity_markers_for_basis(basis),
        performance_accounting_for(&diagnostics),
    )
}

fn read_failure_for_basis(
    basis: &DerivedTopologyReadBasis,
    error: WorthTopologyReadError,
) -> WorthBoundaryFailure<WorthTopologyReadError> {
    WorthBoundaryFailure::failure(
        error,
        Vec::new(),
        WorthDecisionTrace {
            derived_anchor: Some(WorthDerivedTraceAnchor::from_read_basis(basis)),
            ..WorthDecisionTrace::default()
        },
        integrity_markers_for_basis(basis),
        WorthPerformanceAccounting::default(),
    )
}

fn integrity_markers_for_basis(basis: &DerivedTopologyReadBasis) -> WorthIntegrityMarkers {
    WorthIntegrityMarkers::new(
        Some(basis.branch_id().clone()),
        basis.touched_aspects().iter().copied().collect(),
        Some(basis.authoritative_mutation_origin()),
        Some(basis.authority.truth_basis_identity.clone()),
        basis.precision_fallbacks.len(),
        basis.precision_budget_fallbacks.len(),
    )
}

fn materialization_fallback_class_name(
    fallback: crate::materialization::MaterializationFallbackClass,
) -> String {
    match fallback {
        crate::materialization::MaterializationFallbackClass::WholeViewRebuild => {
            "WholeViewRebuild".to_string()
        }
    }
}

fn derived_trace_evidence_for(
    diagnostics: &WorthDerivedReadDiagnostics,
) -> WorthDerivedTraceEvidence {
    let fallback_classes = diagnostics
        .fallback_report
        .materialization_fallback_class
        .map(materialization_fallback_class_name)
        .into_iter()
        .collect::<Vec<_>>();
    WorthDerivedTraceEvidence {
        availability: WorthTraceAvailability::Present,
        invalidation_target_count: diagnostics.invalidation_report.triggered_target_count,
        fallback_classes,
        equivalence_digest: Some(
            diagnostics
                .equivalence_contract_report
                .materialized_topology_digest
                .digest_hex
                .clone(),
        ),
    }
}

fn performance_accounting_for(
    diagnostics: &WorthDerivedReadDiagnostics,
) -> WorthPerformanceAccounting {
    WorthPerformanceAccounting::new([
        WorthNamedCounter::new(
            "derived.invalidation.triggered_target_count",
            diagnostics.invalidation_report.triggered_target_count as u64,
        ),
        WorthNamedCounter::new(
            "derived.rebuild.topology_entity_count",
            diagnostics.rebuild_report.topology_entity_count as u64,
        ),
        WorthNamedCounter::new(
            "derived.rebuild.topology_relation_count",
            diagnostics.rebuild_report.topology_relation_count as u64,
        ),
        WorthNamedCounter::new(
            "derived.rebuild.interpreted_wire_count",
            diagnostics.rebuild_report.interpreted_wire_count as u64,
        ),
        WorthNamedCounter::new(
            "derived.rebuild.interpreted_shell_count",
            diagnostics.rebuild_report.interpreted_shell_count as u64,
        ),
        WorthNamedCounter::new(
            "derived.rebuild.boundary_interpretation_count",
            diagnostics.rebuild_report.boundary_interpretation_count as u64,
        ),
        WorthNamedCounter::new(
            "derived.rebuild.radial_interpretation_count",
            diagnostics.rebuild_report.radial_interpretation_count as u64,
        ),
        WorthNamedCounter::new(
            "derived.validation.row_count",
            diagnostics.rebuild_report.validation_row_count as u64,
        ),
        WorthNamedCounter::new(
            "derived.fallback.explicit_fallback_count",
            diagnostics.fallback_report.explicit_fallback_count as u64,
        ),
        WorthNamedCounter::new(
            "derived.fallback.precision_fallback_count",
            diagnostics.fallback_report.precision_fallback_count as u64,
        ),
        WorthNamedCounter::new(
            "derived.fallback.precision_budget_fallback_count",
            diagnostics.fallback_report.precision_budget_fallback_count as u64,
        ),
    ])
}

#[cfg(test)]
mod tests {
    use super::build_derived_invalidation_report;
    use worth_schema::facade::{
        seed_minimal_topology, seed_milestone_one_primitive, WorthMilestoneOnePrimitiveCase,
    };

    use crate::facade::{worth_milestone_one_runtime_builder, WorthTopologyReader};

    #[test]
    fn reader_builds_artifact_and_interpretation_from_persisted_truth() {
        let mut runtime = worth_milestone_one_runtime_builder()
            .expect("worth milestone one runtime builder")
            .build();
        let seeded = seed_minimal_topology(&mut runtime, "reader-seeded")
            .expect("seed minimal topology");

        let reader = WorthTopologyReader::new(&runtime);
        let basis = reader.read_basis_from_persisted_truth(&seeded.persisted_truth);
        let artifact = reader
            .read_artifact_traced(&basis)
            .expect("read artifact")
            .into_primary_result();
        let interpretation = reader
            .interpret_traced(&basis)
            .expect("interpretation")
            .into_primary_result();

        assert_eq!(artifact.snapshot, seeded.snapshot);
        assert_eq!(artifact.interpretations, interpretation.interpretations);
    }

    #[test]
    fn reader_reuses_verified_commit_basis_for_admitted_primitive() {
        let mut runtime = worth_milestone_one_runtime_builder()
            .expect("worth milestone one runtime builder")
            .build();
        let verified = seed_milestone_one_primitive(
            &mut runtime,
            "reader-verified",
            &WorthMilestoneOnePrimitiveCase::WireBranch { branch_count: 4 },
        )
        .expect("verified primitive");

        let reader = WorthTopologyReader::new(&runtime);
        let basis = reader.read_basis_from_verified_commit(&verified);
        let interpretation = reader
            .interpret_traced(&basis)
            .expect("interpretation")
            .into_primary_result();

        assert!(interpretation
            .interpretations
            .wires
            .iter()
            .any(|wire| wire.branch_vertex_ids.len() == 1));
    }

    #[test]
    fn traced_read_artifact_surfaces_schema_owned_derived_trace() {
        let mut runtime = worth_milestone_one_runtime_builder()
            .expect("worth milestone one runtime builder")
            .build();
        let seeded = seed_minimal_topology(&mut runtime, "reader-traced")
            .expect("seed minimal topology");

        let reader = WorthTopologyReader::new(&runtime);
        let basis = reader.read_basis_from_persisted_truth(&seeded.persisted_truth);
        let traced = reader
            .read_artifact_traced(&basis)
            .expect("traced read artifact");

        assert_eq!(traced.primary_result().snapshot, seeded.snapshot);
        assert_eq!(
            traced.integrity_markers().truth_basis_identity,
            Some(basis.authority.truth_basis_identity.clone())
        );
        assert_eq!(
            traced
                .decision_trace()
                .derived_anchor()
                .expect("derived anchor")
                .snapshot_id,
            basis.snapshot().snapshot_id
        );
        let reopened = traced
            .decision_trace()
            .derived_anchor()
            .expect("derived anchor")
            .open_snapshot(&runtime)
            .expect("reopen derived snapshot");
        assert!(!reopened.entities().is_empty());
        assert_eq!(
            traced
                .decision_trace()
                .derived
                .as_ref()
                .expect("derived trace")
                .invalidation_target_count,
            build_derived_invalidation_report(&basis).triggered_target_count
        );
        assert!(traced
            .performance_accounting()
            .counters
            .iter()
            .any(|counter| counter.name == "derived.validation.row_count"));
    }

    #[test]
    fn traced_diagnostics_surface_equivalence_digest_and_fallback_counters() {
        let mut runtime = worth_milestone_one_runtime_builder()
            .expect("worth milestone one runtime builder")
            .build();
        let verified = seed_milestone_one_primitive(
            &mut runtime,
            "reader-diagnostics-traced",
            &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
        )
        .expect("verified primitive");

        let reader = WorthTopologyReader::new(&runtime);
        let traced = reader
            .diagnostics_traced(&verified.read_basis)
            .expect("traced diagnostics");

        assert_eq!(
            traced
                .decision_trace()
                .derived
                .as_ref()
                .expect("derived trace")
                .equivalence_digest
                .as_deref(),
            Some(
                traced
                    .primary_result()
                    .equivalence_contract_report
                    .materialized_topology_digest
                    .digest_hex
                    .as_str()
            )
        );
        assert!(traced
            .performance_accounting()
            .counters
            .iter()
            .any(|counter| counter.name == "derived.fallback.explicit_fallback_count"));
    }
}
