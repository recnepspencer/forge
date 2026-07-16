use std::collections::BTreeSet;

use worth_store_formal_models::runner::{
    validate_canonical_protocol_trace, CanonicalProtocolAction, CanonicalProtocolTrace,
    CanonicalProtocolTraceDenial, ProtocolFrontierIdentity, ProtocolTraceValidationDenial,
    ProtocolTraceValidationReceipt,
};
use worth_store_formal_models::{
    compose_compaction_action, compose_durability_action, compose_import_action,
    compose_lease_action, compose_quarantine_state, compose_replication_action,
    compose_source_precedence_action, ProtocolFamily, SharedFrontierAction,
};

use crate::courtroom::protocol_models::{
    compaction_visibility::scenarios::{
        execute_compaction_visibility_legal_traces, execute_compaction_visibility_owner_cases,
    },
    durability_recovery::scenario::{
        execute_ordinary_durability_recovery, execute_ordinary_durability_recovery_traces,
    },
    import_publication::scenario::{
        execute_ordinary_import_publication, execute_ordinary_import_publication_traces,
    },
    lease_reclaim::scenario::{
        execute_ordinary_lease_lifecycle, execute_ordinary_lease_lifecycle_traces,
    },
    quarantine_readmission::scenario::{
        execute_ordinary_quarantine_entry, execute_ordinary_quarantine_entry_traces,
    },
    replication_admission::scenarios::{
        ordinary_replication_admission_actions, ordinary_replication_admission_traces,
    },
    source_precedence::scenario::{
        execute_ordinary_source_precedence, execute_ordinary_source_precedence_traces,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrdinaryProtocolExecutionSuite {
    protocol: ProtocolFamily,
    coverage_actions: Vec<CanonicalProtocolAction>,
    legal_traces: Vec<CanonicalProtocolTrace>,
    validation_receipts: Vec<ProtocolTraceValidationReceipt>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrdinaryProtocolExecutionDenial {
    EmptyCoverage,
    EmptyTraceSuite,
    TraceConstruction(CanonicalProtocolTraceDenial),
    IllegalTrace(ProtocolTraceValidationDenial),
    TraceActionMissingFromExecutedCoverage,
    SharedActionMissingFromExecutedProjection(SharedFrontierAction),
}

pub(in crate::courtroom::protocol_models) fn execute_ordinary_protocol_suite(
    protocol: ProtocolFamily,
) -> Result<OrdinaryProtocolExecutionSuite, OrdinaryProtocolExecutionDenial> {
    let (frontier, coverage_actions, trace_actions) = match protocol {
        ProtocolFamily::DurabilityRecovery => (
            ProtocolFrontierIdentity::Durability,
            execute_ordinary_durability_recovery()
                .into_iter()
                .map(CanonicalProtocolAction::DurabilityRecovery)
                .collect(),
            canonicalize_traces(
                execute_ordinary_durability_recovery_traces(),
                CanonicalProtocolAction::DurabilityRecovery,
            ),
        ),
        ProtocolFamily::RecoverySourcePrecedence => (
            ProtocolFrontierIdentity::RecoveryPrecedence,
            execute_ordinary_source_precedence()
                .into_iter()
                .map(CanonicalProtocolAction::RecoverySourcePrecedence)
                .collect(),
            canonicalize_traces(
                execute_ordinary_source_precedence_traces(),
                CanonicalProtocolAction::RecoverySourcePrecedence,
            ),
        ),
        ProtocolFamily::CompactionVisibility => {
            let coverage = execute_compaction_visibility_owner_cases()
                .mapped_cases()
                .map(|mapped| CanonicalProtocolAction::CompactionVisibility(mapped.action()))
                .collect();
            (
                ProtocolFrontierIdentity::Visibility,
                coverage,
                canonicalize_traces(
                    execute_compaction_visibility_legal_traces(),
                    CanonicalProtocolAction::CompactionVisibility,
                ),
            )
        }
        ProtocolFamily::LeaseReclaim => (
            ProtocolFrontierIdentity::Reachability,
            execute_ordinary_lease_lifecycle()
                .into_iter()
                .map(CanonicalProtocolAction::LeaseReclaim)
                .collect(),
            canonicalize_traces(
                execute_ordinary_lease_lifecycle_traces(),
                CanonicalProtocolAction::LeaseReclaim,
            ),
        ),
        ProtocolFamily::QuarantineReadmission => (
            ProtocolFrontierIdentity::Quarantine,
            execute_ordinary_quarantine_entry()
                .into_iter()
                .map(CanonicalProtocolAction::QuarantineReadmission)
                .collect(),
            canonicalize_traces(
                execute_ordinary_quarantine_entry_traces(),
                CanonicalProtocolAction::QuarantineReadmission,
            ),
        ),
        ProtocolFamily::ImportPublication => (
            ProtocolFrontierIdentity::Admission,
            execute_ordinary_import_publication()
                .into_iter()
                .map(CanonicalProtocolAction::ImportPublication)
                .collect(),
            canonicalize_traces(
                execute_ordinary_import_publication_traces(),
                CanonicalProtocolAction::ImportPublication,
            ),
        ),
        ProtocolFamily::ReplicationAdmission => (
            ProtocolFrontierIdentity::Admission,
            ordinary_replication_admission_actions()
                .into_iter()
                .map(CanonicalProtocolAction::ReplicationAdmission)
                .collect(),
            canonicalize_traces(
                ordinary_replication_admission_traces(),
                CanonicalProtocolAction::ReplicationAdmission,
            ),
        ),
        ProtocolFamily::SharedFrontiers => return execute_shared_frontier_suite(),
    };
    admit_execution_suite(protocol, frontier, coverage_actions, trace_actions)
}

pub(super) fn exact_modeled_action_coverage(suite: &OrdinaryProtocolExecutionSuite) -> bool {
    match suite.protocol() {
        ProtocolFamily::DurabilityRecovery => {
            let observed = suite.coverage_actions().iter().filter_map(|action| match action {
                CanonicalProtocolAction::DurabilityRecovery(action) => Some(*action),
                _ => None,
            });
            BTreeSet::from_iter(observed)
                == BTreeSet::from(worth_store_formal_models::DurabilityRecoveryAction::all())
        }
        ProtocolFamily::RecoverySourcePrecedence => {
            let observed = suite.coverage_actions().iter().filter_map(|action| match action {
                CanonicalProtocolAction::RecoverySourcePrecedence(action) => Some(action.kind()),
                _ => None,
            });
            BTreeSet::from_iter(observed)
                == BTreeSet::from(worth_store_formal_models::SourcePrecedenceActionKind::all())
        }
        ProtocolFamily::CompactionVisibility => crate::courtroom::protocol_models::
            compaction_visibility::adjudicate_compaction_visibility_refinement()
            .is_ok(),
        ProtocolFamily::LeaseReclaim => {
            let observed = suite.coverage_actions().iter().filter_map(|action| match action {
                CanonicalProtocolAction::LeaseReclaim(action) => Some(action.kind()),
                _ => None,
            });
            BTreeSet::from_iter(observed)
                == BTreeSet::from(worth_store_formal_models::LeaseReclaimActionKind::all())
        }
        ProtocolFamily::QuarantineReadmission => {
            let observed = suite.coverage_actions().iter().filter_map(|action| match action {
                CanonicalProtocolAction::QuarantineReadmission(state) => Some(*state),
                _ => None,
            });
            BTreeSet::from_iter(observed)
                == BTreeSet::from(worth_store_formal_models::QuarantineReadmissionState::all())
        }
        ProtocolFamily::ImportPublication => {
            let observed = suite.coverage_actions().iter().filter_map(|action| match action {
                CanonicalProtocolAction::ImportPublication(action) => Some(*action),
                _ => None,
            });
            BTreeSet::from_iter(observed)
                == BTreeSet::from(worth_store_formal_models::ImportPublicationAction::all())
        }
        ProtocolFamily::ReplicationAdmission => {
            let observed = suite.coverage_actions().iter().filter_map(|action| match action {
                CanonicalProtocolAction::ReplicationAdmission(action) => Some(*action),
                _ => None,
            });
            BTreeSet::from_iter(observed)
                == BTreeSet::from(worth_store_formal_models::ReplicationAdmissionAction::all())
        }
        ProtocolFamily::SharedFrontiers => {
            let observed = suite.coverage_actions().iter().filter_map(|action| match action {
                CanonicalProtocolAction::SharedFrontier(action) => Some(*action),
                _ => None,
            });
            BTreeSet::from_iter(observed)
                == BTreeSet::from(worth_store_formal_models::SharedFrontierAction::all())
        }
    }
}

fn execute_shared_frontier_suite(
) -> Result<OrdinaryProtocolExecutionSuite, OrdinaryProtocolExecutionDenial> {
    let coverage = shared_actions_from_executed_owner_evidence();
    let action = |required| require_shared_action(&coverage, required);
    let traces = vec![
        vec![
            action(SharedFrontierAction::RecoveryPrecedencePreserved)?,
            action(SharedFrontierAction::LiveLeaseAcquired)?,
            action(SharedFrontierAction::CompactionCutover)?,
            action(SharedFrontierAction::Crash)?,
            action(SharedFrontierAction::Reopen)?,
            action(SharedFrontierAction::ReclaimDeferred)?,
            action(SharedFrontierAction::LeaseReleased)?,
            action(SharedFrontierAction::ReclaimReleased)?,
            action(SharedFrontierAction::GenerationReused)?,
        ],
        vec![
            action(SharedFrontierAction::DurabilityAdmitted)?,
            action(SharedFrontierAction::QuarantineSealed)?,
            action(SharedFrontierAction::QuarantineVerificationStarted)?,
            action(SharedFrontierAction::QuarantineReadmitted)?,
            action(SharedFrontierAction::CheckpointPublicationRequested)?,
        ],
        vec![
            action(SharedFrontierAction::ImportAdmissionPending)?,
            action(SharedFrontierAction::ExternalDurabilityAdmitted)?,
            action(SharedFrontierAction::ExternalPublicationRequested)?,
        ],
        vec![
            action(SharedFrontierAction::ReplicationAdmissionPending)?,
            action(SharedFrontierAction::ExternalDurabilityAdmitted)?,
            action(SharedFrontierAction::ReplicationDivergenceDetected)?,
        ],
    ];
    admit_execution_suite(
        ProtocolFamily::SharedFrontiers,
        ProtocolFrontierIdentity::Reachability,
        coverage
            .into_iter()
            .map(CanonicalProtocolAction::SharedFrontier)
            .collect(),
        canonicalize_traces(traces, CanonicalProtocolAction::SharedFrontier),
    )
}

fn shared_actions_from_executed_owner_evidence() -> Vec<SharedFrontierAction> {
    execute_ordinary_durability_recovery()
        .into_iter()
        .filter_map(compose_durability_action)
        .chain(
            execute_ordinary_source_precedence()
                .into_iter()
                .filter_map(compose_source_precedence_action),
        )
        .chain(
            execute_compaction_visibility_owner_cases()
                .mapped_cases()
                .filter_map(|mapped| compose_compaction_action(mapped.action())),
        )
        .chain(
            execute_ordinary_lease_lifecycle()
                .into_iter()
                .filter_map(compose_lease_action),
        )
        .chain(
            execute_ordinary_quarantine_entry()
                .into_iter()
                .filter_map(compose_quarantine_state),
        )
        .chain(
            execute_ordinary_import_publication()
                .into_iter()
                .filter_map(compose_import_action),
        )
        .chain(
            ordinary_replication_admission_actions()
                .into_iter()
                .filter_map(compose_replication_action),
        )
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn require_shared_action(
    coverage: &[SharedFrontierAction],
    required: SharedFrontierAction,
) -> Result<SharedFrontierAction, OrdinaryProtocolExecutionDenial> {
    coverage
        .contains(&required)
        .then_some(required)
        .ok_or(OrdinaryProtocolExecutionDenial::SharedActionMissingFromExecutedProjection(required))
}

fn admit_execution_suite(
    protocol: ProtocolFamily,
    frontier: ProtocolFrontierIdentity,
    coverage_actions: Vec<CanonicalProtocolAction>,
    trace_actions: Vec<Vec<CanonicalProtocolAction>>,
) -> Result<OrdinaryProtocolExecutionSuite, OrdinaryProtocolExecutionDenial> {
    if coverage_actions.is_empty() {
        return Err(OrdinaryProtocolExecutionDenial::EmptyCoverage);
    }
    if trace_actions.is_empty() {
        return Err(OrdinaryProtocolExecutionDenial::EmptyTraceSuite);
    }
    let mut legal_traces = Vec::with_capacity(trace_actions.len());
    let mut validation_receipts = Vec::with_capacity(trace_actions.len());
    for actions in trace_actions {
        if actions
            .iter()
            .any(|action| !coverage_actions.contains(action))
        {
            return Err(OrdinaryProtocolExecutionDenial::TraceActionMissingFromExecutedCoverage);
        }
        let trace = CanonicalProtocolTrace::admit(protocol, frontier, actions)
            .map_err(OrdinaryProtocolExecutionDenial::TraceConstruction)?;
        let validation = validate_canonical_protocol_trace(&trace)
            .map_err(OrdinaryProtocolExecutionDenial::IllegalTrace)?;
        legal_traces.push(trace);
        validation_receipts.push(validation);
    }
    Ok(OrdinaryProtocolExecutionSuite {
        protocol,
        coverage_actions,
        legal_traces,
        validation_receipts,
    })
}

fn canonicalize_traces<A: Copy>(
    traces: Vec<Vec<A>>,
    project: impl Fn(A) -> CanonicalProtocolAction + Copy,
) -> Vec<Vec<CanonicalProtocolAction>> {
    traces
        .into_iter()
        .map(|trace| trace.into_iter().map(project).collect())
        .collect()
}

impl OrdinaryProtocolExecutionSuite {
    pub const fn protocol(&self) -> ProtocolFamily {
        self.protocol
    }

    pub fn coverage_actions(&self) -> &[CanonicalProtocolAction] {
        &self.coverage_actions
    }

    pub fn legal_traces(&self) -> &[CanonicalProtocolTrace] {
        &self.legal_traces
    }

    pub fn validation_receipts(&self) -> &[ProtocolTraceValidationReceipt] {
        &self.validation_receipts
    }
}
