use super::affected_artifact::PlanarBooleanSplitAffectedArtifact as Artifact;
use super::counters::PlanarBooleanSplitDecisionLogCounters;
use super::decision_reason::PlanarBooleanSplitDecisionReason as Reason;
use super::identity::{decision_identity, decision_identity_with_detail};
use super::input::PlanarBooleanSplitDecisionLogInput;
use super::kind::PlanarBooleanSplitDecisionKind as Kind;
use super::phase::PlanarBooleanSplitDecisionPhase as Phase;
use super::row::PlanarBooleanSplitDecisionRow;
use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanMicroIntervalAction, PlanarBooleanSplitNamedArtifactKind,
};

pub(super) fn push_query_declaration_row(
    input: &PlanarBooleanSplitDecisionLogInput<'_>,
    rows: &mut Vec<PlanarBooleanSplitDecisionRow>,
    counters: &mut PlanarBooleanSplitDecisionLogCounters,
) {
    rows.push(row(
        Phase::QueryDeclaration,
        Kind::QueryDecisionLogDeclared,
        Artifact::QueryDeclaration,
        input.declaration().declaration_identity(),
        input.declaration().split_request_identity(),
        "",
        "",
        Vec::new(),
        Vec::new(),
        vec![input.declaration().lowered_plan_identity().to_string()],
        Reason::QueryDecisionLogDeclared,
    ));
    counters.emitted_decision_row();
}

pub(super) fn push_endpoint_rows(
    input: &PlanarBooleanSplitDecisionLogInput<'_>,
    rows: &mut Vec<PlanarBooleanSplitDecisionRow>,
    counters: &mut PlanarBooleanSplitDecisionLogCounters,
) {
    for decision in input
        .endpoint_boundary_schedules()
        .endpoint_contact_decisions()
    {
        rows.push(row(
            Phase::EndpointBoundaryNormalization,
            Kind::EndpointNoOpRecorded,
            Artifact::EndpointContactDecision,
            decision.decision_identity(),
            input.endpoint_boundary_schedules().schedule_set_identity(),
            decision.source_edge_identity(),
            decision.carrier_identity(),
            Vec::new(),
            decision.event_group_identities().to_vec(),
            decision.provenance_entry_identities().to_vec(),
            Reason::EndpointContactDecision,
        ));
        counters.emitted_decision_row();
        counters.recorded_endpoint_decision();
    }
}

pub(super) fn push_interval_rows(
    input: &PlanarBooleanSplitDecisionLogInput<'_>,
    rows: &mut Vec<PlanarBooleanSplitDecisionRow>,
    counters: &mut PlanarBooleanSplitDecisionLogCounters,
) {
    for schedule in input.interval_subdivision_schedules().schedules() {
        for subdivision in schedule.interval_subdivisions() {
            let kind = match subdivision.action() {
                PlanarBooleanMicroIntervalAction::Retain => Kind::IntervalSubdivisionRetained,
                PlanarBooleanMicroIntervalAction::AdmittedCollapse => Kind::MicroIntervalCollapsed,
                PlanarBooleanMicroIntervalAction::PolicyRequired => {
                    Kind::MicroIntervalPolicyRequired
                }
            };
            rows.push(row_with_policy(
                Phase::IntervalSubdivisionNormalization,
                kind,
                Artifact::IntervalSubdivision,
                subdivision.subdivision_identity(),
                input
                    .interval_subdivision_schedules()
                    .schedule_set_identity(),
                subdivision.source_edge_identity(),
                subdivision.carrier_identity(),
                vec![subdivision.interval_event_identity().to_string()],
                subdivision.event_group_identities().to_vec(),
                subdivision.provenance_entry_identities().to_vec(),
                interval_reason(kind),
                Some(format!("{:?}", subdivision.action())),
            ));
            counters.emitted_decision_row();
            counters.recorded_interval_subdivision_decision();
            if kind != Kind::IntervalSubdivisionRetained {
                counters.recorded_micro_interval_policy_decision();
            }
        }
    }
}

pub(super) fn push_vertex_rows(
    input: &PlanarBooleanSplitDecisionLogInput<'_>,
    rows: &mut Vec<PlanarBooleanSplitDecisionRow>,
    counters: &mut PlanarBooleanSplitDecisionLogCounters,
) {
    for decision in input.split_vertices().coalescence_decisions() {
        rows.push(row_with_policy(
            Phase::SplitVertexIdentity,
            Kind::SplitVertexCoalesced,
            Artifact::SplitVertex,
            decision.split_vertex_identity(),
            input.split_vertices().split_vertex_identity_set_identity(),
            decision.source_edge_identity(),
            decision.carrier_identity(),
            Vec::new(),
            decision.event_group_identities().to_vec(),
            decision.input_identities().to_vec(),
            Reason::SplitVertexCoalesced(format!("{:?}", decision.reason())),
            Some(format!("{:?}", decision.reason())),
        ));
        counters.emitted_decision_row();
        counters.recorded_coalescence_decision();
    }
}

pub(super) fn push_fragment_rows(
    input: &PlanarBooleanSplitDecisionLogInput<'_>,
    rows: &mut Vec<PlanarBooleanSplitDecisionRow>,
    counters: &mut PlanarBooleanSplitDecisionLogCounters,
) {
    for fragment in input.split_fragments().fragments() {
        rows.push(row(
            Phase::SplitEdgeFragmentConstruction,
            Kind::SplitFragmentCreated,
            Artifact::SplitFragment,
            fragment.fragment_identity(),
            input.split_fragments().fragment_set_identity(),
            fragment.source_edge_identity(),
            fragment.carrier_identity(),
            Vec::new(),
            fragment.event_group_identities().to_vec(),
            fragment.cause_provenance_identities().to_vec(),
            Reason::SplitFragmentCreated,
        ));
        counters.emitted_decision_row();
        counters.recorded_fragment_decision();
    }
}

pub(super) fn push_coverage_rows(
    input: &PlanarBooleanSplitDecisionLogInput<'_>,
    rows: &mut Vec<PlanarBooleanSplitDecisionRow>,
    counters: &mut PlanarBooleanSplitDecisionLogCounters,
) {
    for coverage in input.split_chain_validation().fragment_coverage_rows() {
        rows.push(row(
            Phase::SplitChainValidation,
            Kind::SplitFragmentCoverageValidated,
            Artifact::SplitFragmentCoverage,
            coverage.row_identity(),
            input.split_chain_validation().receipt_identity(),
            coverage.source_edge_identity(),
            coverage.carrier_identity(),
            Vec::new(),
            Vec::new(),
            vec![coverage.schedule_identity().to_string()],
            Reason::SplitFragmentCoverageValidated,
        ));
        counters.emitted_decision_row();
        counters.recorded_coverage_decision();
    }
    for coverage in input.split_chain_validation().overlap_coverage_rows() {
        let decision_identity = decision_identity_with_detail(
            Phase::SplitChainValidation,
            Kind::OverlapChainCoverageValidated,
            Artifact::OverlapChainCoverage,
            coverage.chain_identity(),
            input.split_chain_validation().receipt_identity(),
            &[format!("coverage-row:{}", coverage.row_identity())],
        );
        rows.push(row_with_identity(
            decision_identity,
            Phase::SplitChainValidation,
            Kind::OverlapChainCoverageValidated,
            Artifact::OverlapChainCoverage,
            coverage.chain_identity(),
            input.split_chain_validation().receipt_identity(),
            coverage.source_edge_identity(),
            coverage.carrier_identity(),
            vec![coverage.interval_event_identity().to_string()],
            Vec::new(),
            vec![coverage.chain_identity().to_string()],
            Reason::OverlapChainCoverageValidated,
            None,
        ));
        counters.emitted_decision_row();
        counters.recorded_coverage_decision();
    }
}

pub(super) fn push_persistent_name_rows(
    input: &PlanarBooleanSplitDecisionLogInput<'_>,
    rows: &mut Vec<PlanarBooleanSplitDecisionRow>,
    counters: &mut PlanarBooleanSplitDecisionLogCounters,
) {
    for name_row in input.split_persistent_names().persistent_name_rows() {
        let artifact = match name_row.artifact_kind() {
            PlanarBooleanSplitNamedArtifactKind::SplitFragment => Artifact::SplitFragment,
            PlanarBooleanSplitNamedArtifactKind::SplitVertex => Artifact::SplitVertex,
            PlanarBooleanSplitNamedArtifactKind::OverlapChain
            | PlanarBooleanSplitNamedArtifactKind::RetainedInterval
            | PlanarBooleanSplitNamedArtifactKind::EventCause => Artifact::PersistentName,
        };
        let decision_identity = decision_identity_with_detail(
            Phase::SplitPersistentNaming,
            Kind::PersistentNamePropagated,
            artifact,
            name_row.artifact_identity(),
            input.split_persistent_names().receipt_identity(),
            &[format!("naming-row:{}", name_row.row_identity())],
        );
        rows.push(row_with_identity(
            decision_identity,
            Phase::SplitPersistentNaming,
            Kind::PersistentNamePropagated,
            artifact,
            name_row.artifact_identity(),
            input.split_persistent_names().receipt_identity(),
            name_row.source_edge_identity(),
            "",
            name_row.event_cause_identities().to_vec(),
            Vec::new(),
            vec![name_row.row_identity().to_string()],
            Reason::PersistentNamePropagated,
            None,
        ));
        counters.emitted_decision_row();
        counters.recorded_persistent_name_decision();
    }
}

pub(super) fn push_phase_stop_rows(
    input: &PlanarBooleanSplitDecisionLogInput<'_>,
    rows: &mut Vec<PlanarBooleanSplitDecisionRow>,
    counters: &mut PlanarBooleanSplitDecisionLogCounters,
) {
    for stop in input.phase_stops() {
        rows.push(stop.to_decision_row());
        counters.emitted_decision_row();
        counters.recorded_phase_stop_decision();
    }
}

fn interval_reason(kind: Kind) -> Reason {
    match kind {
        Kind::IntervalSubdivisionRetained => Reason::IntervalSubdivisionRetained,
        Kind::MicroIntervalCollapsed => Reason::MicroIntervalCollapsed,
        Kind::MicroIntervalPolicyRequired => Reason::MicroIntervalPolicyRequired,
        _ => unreachable!("interval subdivision emits only interval decisions"),
    }
}

#[allow(clippy::too_many_arguments)]
fn row(
    phase: Phase,
    kind: Kind,
    artifact: Artifact,
    artifact_identity: &str,
    upstream_receipt_identity: &str,
    source_edge_identity: &str,
    carrier_identity: &str,
    event_identities: Vec<String>,
    event_group_identities: Vec<String>,
    provenance_identities: Vec<String>,
    decision_reason: Reason,
) -> PlanarBooleanSplitDecisionRow {
    row_with_policy(
        phase,
        kind,
        artifact,
        artifact_identity,
        upstream_receipt_identity,
        source_edge_identity,
        carrier_identity,
        event_identities,
        event_group_identities,
        provenance_identities,
        decision_reason,
        None,
    )
}

#[allow(clippy::too_many_arguments)]
fn row_with_policy(
    phase: Phase,
    kind: Kind,
    artifact: Artifact,
    artifact_identity: &str,
    upstream_receipt_identity: &str,
    source_edge_identity: &str,
    carrier_identity: &str,
    event_identities: Vec<String>,
    event_group_identities: Vec<String>,
    provenance_identities: Vec<String>,
    decision_reason: Reason,
    policy_or_denial_kind: Option<String>,
) -> PlanarBooleanSplitDecisionRow {
    let decision_identity = decision_identity(
        phase,
        kind,
        artifact,
        artifact_identity,
        upstream_receipt_identity,
    );
    row_with_identity(
        decision_identity,
        phase,
        kind,
        artifact,
        artifact_identity,
        upstream_receipt_identity,
        source_edge_identity,
        carrier_identity,
        event_identities,
        event_group_identities,
        provenance_identities,
        decision_reason,
        policy_or_denial_kind,
    )
}

#[allow(clippy::too_many_arguments)]
fn row_with_identity(
    decision_identity: String,
    phase: Phase,
    kind: Kind,
    artifact: Artifact,
    artifact_identity: &str,
    upstream_receipt_identity: &str,
    source_edge_identity: &str,
    carrier_identity: &str,
    event_identities: Vec<String>,
    event_group_identities: Vec<String>,
    provenance_identities: Vec<String>,
    decision_reason: Reason,
    policy_or_denial_kind: Option<String>,
) -> PlanarBooleanSplitDecisionRow {
    PlanarBooleanSplitDecisionRow::new(
        decision_identity,
        phase,
        kind,
        artifact,
        artifact_identity.to_string(),
        source_edge_identity.to_string(),
        carrier_identity.to_string(),
        event_identities,
        event_group_identities,
        provenance_identities,
        upstream_receipt_identity.to_string(),
        decision_reason,
        policy_or_denial_kind,
    )
}
