use std::collections::BTreeSet;

use super::affected_artifact::PlanarBooleanSplitAffectedArtifact as Artifact;
use super::denial::{PlanarBooleanSplitDecisionLogDenial, PlanarBooleanSplitDecisionLogDenialKind};
use super::input::PlanarBooleanSplitDecisionLogInput;
use super::identity::{decision_identity, decision_identity_with_detail};
use super::kind::PlanarBooleanSplitDecisionKind as Kind;
use super::phase::PlanarBooleanSplitDecisionPhase as Phase;
use super::row::PlanarBooleanSplitDecisionRow;
use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanMicroIntervalAction, PlanarBooleanSplitNamedArtifactKind,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct PlanarBooleanSplitDecisionCoverageExpectation {
    decision_identity: String,
    phase: Phase,
    kind: Kind,
    artifact: Artifact,
    artifact_identity: String,
    upstream_receipt_identity: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSplitDecisionCoverageManifest {
    manifest_identity: String,
    expectations: Vec<PlanarBooleanSplitDecisionCoverageExpectation>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSplitDecisionCoverageReceipt {
    manifest_identity: String,
    decision_log_receipt_identity: String,
    expected_rows: usize,
    observed_rows: usize,
}

impl PlanarBooleanSplitDecisionCoverageManifest {
    pub(crate) fn from_input(input: &PlanarBooleanSplitDecisionLogInput<'_>) -> Self {
        let mut expectations = Vec::new();
        push_query_expectation(input, &mut expectations);
        push_endpoint_expectations(input, &mut expectations);
        push_interval_expectations(input, &mut expectations);
        push_vertex_expectations(input, &mut expectations);
        push_fragment_expectations(input, &mut expectations);
        push_coverage_expectations(input, &mut expectations);
        push_persistent_name_expectations(input, &mut expectations);
        push_phase_stop_expectations(input, &mut expectations);
        expectations.sort();
        let manifest_identity = manifest_identity(input.declaration().declaration_identity());
        Self {
            manifest_identity,
            expectations,
        }
    }

    pub fn manifest_identity(&self) -> &str {
        &self.manifest_identity
    }
    pub fn expectations(&self) -> &[PlanarBooleanSplitDecisionCoverageExpectation] {
        &self.expectations
    }

    pub(crate) fn validate_rows(
        &self,
        decision_log_receipt_identity: &str,
        rows: &[PlanarBooleanSplitDecisionRow],
    ) -> Result<PlanarBooleanSplitDecisionCoverageReceipt, PlanarBooleanSplitDecisionLogDenial>
    {
        let expected = self.expectations.iter().cloned().collect::<BTreeSet<_>>();
        let observed_rows = rows
            .iter()
            .map(PlanarBooleanSplitDecisionCoverageExpectation::from_row)
            .collect::<BTreeSet<_>>();
        if expected != observed_rows {
            return Err(PlanarBooleanSplitDecisionLogDenial::new(
                PlanarBooleanSplitDecisionLogDenialKind::MissingDecisionCoverage,
                &self.manifest_identity,
                Default::default(),
                "split decision log rows must exactly match the Query coverage manifest",
            ));
        }
        Ok(PlanarBooleanSplitDecisionCoverageReceipt {
            manifest_identity: self.manifest_identity.clone(),
            decision_log_receipt_identity: decision_log_receipt_identity.to_string(),
            expected_rows: expected.len(),
            observed_rows: observed_rows.len(),
        })
    }
}

impl PlanarBooleanSplitDecisionCoverageExpectation {
    fn new(
        decision_identity: impl Into<String>,
        phase: Phase,
        kind: Kind,
        artifact: Artifact,
        artifact_identity: impl Into<String>,
        upstream_receipt_identity: impl Into<String>,
    ) -> Self {
        Self {
            decision_identity: decision_identity.into(),
            phase,
            kind,
            artifact,
            artifact_identity: artifact_identity.into(),
            upstream_receipt_identity: upstream_receipt_identity.into(),
        }
    }

    fn from_row(row: &PlanarBooleanSplitDecisionRow) -> Self {
        Self::new(
            row.decision_identity(),
            row.phase(),
            row.kind(),
            row.affected_artifact(),
            row.affected_artifact_identity(),
            row.upstream_receipt_identity(),
        )
    }

    pub fn phase(&self) -> Phase {
        self.phase
    }
    pub fn decision_identity(&self) -> &str {
        &self.decision_identity
    }
    pub fn kind(&self) -> Kind {
        self.kind
    }
    pub fn artifact(&self) -> Artifact {
        self.artifact
    }
    pub fn artifact_identity(&self) -> &str {
        &self.artifact_identity
    }
    pub fn upstream_receipt_identity(&self) -> &str {
        &self.upstream_receipt_identity
    }
}

impl PlanarBooleanSplitDecisionCoverageReceipt {
    pub fn manifest_identity(&self) -> &str {
        &self.manifest_identity
    }
    pub fn decision_log_receipt_identity(&self) -> &str {
        &self.decision_log_receipt_identity
    }
    pub fn expected_rows(&self) -> usize {
        self.expected_rows
    }
    pub fn observed_rows(&self) -> usize {
        self.observed_rows
    }
}

fn push_query_expectation(
    input: &PlanarBooleanSplitDecisionLogInput<'_>,
    expectations: &mut Vec<PlanarBooleanSplitDecisionCoverageExpectation>,
) {
    expectations.push(PlanarBooleanSplitDecisionCoverageExpectation::new(
        decision_identity(
            Phase::QueryDeclaration,
            Kind::QueryDecisionLogDeclared,
            Artifact::QueryDeclaration,
            input.declaration().declaration_identity(),
            input.declaration().split_request_identity(),
        ),
        Phase::QueryDeclaration,
        Kind::QueryDecisionLogDeclared,
        Artifact::QueryDeclaration,
        input.declaration().declaration_identity(),
        input.declaration().split_request_identity(),
    ));
}

fn push_endpoint_expectations(
    input: &PlanarBooleanSplitDecisionLogInput<'_>,
    expectations: &mut Vec<PlanarBooleanSplitDecisionCoverageExpectation>,
) {
    for decision in input
        .endpoint_boundary_schedules()
        .endpoint_contact_decisions()
    {
        expectations.push(PlanarBooleanSplitDecisionCoverageExpectation::new(
            decision_identity(
                Phase::EndpointBoundaryNormalization,
                Kind::EndpointNoOpRecorded,
                Artifact::EndpointContactDecision,
                decision.decision_identity(),
                input.endpoint_boundary_schedules().schedule_set_identity(),
            ),
            Phase::EndpointBoundaryNormalization,
            Kind::EndpointNoOpRecorded,
            Artifact::EndpointContactDecision,
            decision.decision_identity(),
            input.endpoint_boundary_schedules().schedule_set_identity(),
        ));
    }
}

fn push_interval_expectations(
    input: &PlanarBooleanSplitDecisionLogInput<'_>,
    expectations: &mut Vec<PlanarBooleanSplitDecisionCoverageExpectation>,
) {
    for schedule in input.interval_subdivision_schedules().schedules() {
        for subdivision in schedule.interval_subdivisions() {
            expectations.push(PlanarBooleanSplitDecisionCoverageExpectation::new(
                decision_identity(
                    Phase::IntervalSubdivisionNormalization,
                    interval_kind(subdivision.action()),
                    Artifact::IntervalSubdivision,
                    subdivision.subdivision_identity(),
                    input
                        .interval_subdivision_schedules()
                        .schedule_set_identity(),
                ),
                Phase::IntervalSubdivisionNormalization,
                interval_kind(subdivision.action()),
                Artifact::IntervalSubdivision,
                subdivision.subdivision_identity(),
                input
                    .interval_subdivision_schedules()
                    .schedule_set_identity(),
            ));
        }
    }
}

fn push_vertex_expectations(
    input: &PlanarBooleanSplitDecisionLogInput<'_>,
    expectations: &mut Vec<PlanarBooleanSplitDecisionCoverageExpectation>,
) {
    for decision in input.split_vertices().coalescence_decisions() {
        expectations.push(PlanarBooleanSplitDecisionCoverageExpectation::new(
            decision_identity(
                Phase::SplitVertexIdentity,
                Kind::SplitVertexCoalesced,
                Artifact::SplitVertex,
                decision.split_vertex_identity(),
                input.split_vertices().split_vertex_identity_set_identity(),
            ),
            Phase::SplitVertexIdentity,
            Kind::SplitVertexCoalesced,
            Artifact::SplitVertex,
            decision.split_vertex_identity(),
            input.split_vertices().split_vertex_identity_set_identity(),
        ));
    }
}

fn push_fragment_expectations(
    input: &PlanarBooleanSplitDecisionLogInput<'_>,
    expectations: &mut Vec<PlanarBooleanSplitDecisionCoverageExpectation>,
) {
    for fragment in input.split_fragments().fragments() {
        expectations.push(PlanarBooleanSplitDecisionCoverageExpectation::new(
            decision_identity(
                Phase::SplitEdgeFragmentConstruction,
                Kind::SplitFragmentCreated,
                Artifact::SplitFragment,
                fragment.fragment_identity(),
                input.split_fragments().fragment_set_identity(),
            ),
            Phase::SplitEdgeFragmentConstruction,
            Kind::SplitFragmentCreated,
            Artifact::SplitFragment,
            fragment.fragment_identity(),
            input.split_fragments().fragment_set_identity(),
        ));
    }
}

fn push_coverage_expectations(
    input: &PlanarBooleanSplitDecisionLogInput<'_>,
    expectations: &mut Vec<PlanarBooleanSplitDecisionCoverageExpectation>,
) {
    for coverage in input.split_chain_validation().fragment_coverage_rows() {
        expectations.push(PlanarBooleanSplitDecisionCoverageExpectation::new(
            decision_identity(
                Phase::SplitChainValidation,
                Kind::SplitFragmentCoverageValidated,
                Artifact::SplitFragmentCoverage,
                coverage.row_identity(),
                input.split_chain_validation().receipt_identity(),
            ),
            Phase::SplitChainValidation,
            Kind::SplitFragmentCoverageValidated,
            Artifact::SplitFragmentCoverage,
            coverage.row_identity(),
            input.split_chain_validation().receipt_identity(),
        ));
    }
    for coverage in input.split_chain_validation().overlap_coverage_rows() {
        expectations.push(PlanarBooleanSplitDecisionCoverageExpectation::new(
            decision_identity_with_detail(
                Phase::SplitChainValidation,
                Kind::OverlapChainCoverageValidated,
                Artifact::OverlapChainCoverage,
                coverage.chain_identity(),
                input.split_chain_validation().receipt_identity(),
                &[format!("coverage-row:{}", coverage.row_identity())],
            ),
            Phase::SplitChainValidation,
            Kind::OverlapChainCoverageValidated,
            Artifact::OverlapChainCoverage,
            coverage.chain_identity(),
            input.split_chain_validation().receipt_identity(),
        ));
    }
}

fn push_persistent_name_expectations(
    input: &PlanarBooleanSplitDecisionLogInput<'_>,
    expectations: &mut Vec<PlanarBooleanSplitDecisionCoverageExpectation>,
) {
    for name_row in input.split_persistent_names().persistent_name_rows() {
        let artifact = persistent_name_artifact(name_row.artifact_kind());
        expectations.push(PlanarBooleanSplitDecisionCoverageExpectation::new(
            decision_identity_with_detail(
                Phase::SplitPersistentNaming,
                Kind::PersistentNamePropagated,
                artifact,
                name_row.artifact_identity(),
                input.split_persistent_names().receipt_identity(),
                &[format!("naming-row:{}", name_row.row_identity())],
            ),
            Phase::SplitPersistentNaming,
            Kind::PersistentNamePropagated,
            artifact,
            name_row.artifact_identity(),
            input.split_persistent_names().receipt_identity(),
        ));
    }
}

fn push_phase_stop_expectations(
    input: &PlanarBooleanSplitDecisionLogInput<'_>,
    expectations: &mut Vec<PlanarBooleanSplitDecisionCoverageExpectation>,
) {
    for stop in input.phase_stops() {
        expectations.push(PlanarBooleanSplitDecisionCoverageExpectation::new(
            decision_identity(
                stop.phase(),
                Kind::SplitPhaseDenied,
                Artifact::PhaseStop,
                stop.stop_identity(),
                stop.evidence_identity(),
            ),
            stop.phase(),
            Kind::SplitPhaseDenied,
            Artifact::PhaseStop,
            stop.stop_identity(),
            stop.evidence_identity(),
        ));
    }
}

fn interval_kind(action: PlanarBooleanMicroIntervalAction) -> Kind {
    match action {
        PlanarBooleanMicroIntervalAction::Retain => Kind::IntervalSubdivisionRetained,
        PlanarBooleanMicroIntervalAction::AdmittedCollapse => Kind::MicroIntervalCollapsed,
        PlanarBooleanMicroIntervalAction::PolicyRequired => Kind::MicroIntervalPolicyRequired,
    }
}

fn persistent_name_artifact(kind: PlanarBooleanSplitNamedArtifactKind) -> Artifact {
    match kind {
        PlanarBooleanSplitNamedArtifactKind::SplitFragment => Artifact::SplitFragment,
        PlanarBooleanSplitNamedArtifactKind::SplitVertex => Artifact::SplitVertex,
        PlanarBooleanSplitNamedArtifactKind::OverlapChain
        | PlanarBooleanSplitNamedArtifactKind::RetainedInterval
        | PlanarBooleanSplitNamedArtifactKind::EventCause => Artifact::PersistentName,
    }
}

fn manifest_identity(declaration_identity: &str) -> String {
    format!("split-decision-coverage-manifest:{declaration_identity}")
}
