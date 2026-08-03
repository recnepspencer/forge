use std::collections::BTreeSet;

use worth_store_formal_models::runner::{
    require_exact_protocol_refinement_coverage, CanonicalProtocolAction, CheckedOperatorBinding,
    ExactProtocolRefinementCoverageReceipt, ProtocolRefinementCoverageDenial,
};
use worth_store_formal_models::{
    current_compaction_visibility_owner_cases, DurabilityRecoveryAction, ImportPublicationAction,
    LeaseReclaimActionKind, ProtocolFamily, QuarantineReadmissionState, ReplicationAdmissionAction,
    SharedFrontierAction, SourcePrecedenceActionKind,
};

use super::ordinary_execution::OrdinaryProtocolExecutionSuite;
use crate::courtroom::protocol_models::compaction_visibility::scenarios::execute_compaction_visibility_owner_cases;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum OwnerBindingCase {
    Durability(DurabilityRecoveryAction),
    RecoverySource(SourcePrecedenceActionKind),
    Lease(LeaseReclaimActionKind),
    Quarantine(QuarantineReadmissionState),
    Import(ImportPublicationAction),
    Replication(ReplicationAdmissionAction),
    Shared(SharedFrontierAction),
}

pub(super) fn require_exact_owner_coverage(
    suite: &OrdinaryProtocolExecutionSuite,
    checked_bindings: &[CheckedOperatorBinding],
) -> Result<ExactProtocolRefinementCoverageReceipt, ProtocolRefinementCoverageDenial> {
    if suite.protocol() == ProtocolFamily::CompactionVisibility {
        return require_exact_compaction_owner_coverage();
    }
    let declared = declared_cases(suite.protocol());
    let executed = suite
        .coverage_actions()
        .iter()
        .filter_map(owner_binding_case)
        .collect::<Vec<_>>();
    let mapped = suite
        .coverage_actions()
        .iter()
        .zip(checked_bindings)
        .filter_map(|(action, _)| owner_binding_case(action))
        .collect::<BTreeSet<_>>();
    require_exact_protocol_refinement_coverage(declared, executed, mapped)
}

fn require_exact_compaction_owner_coverage(
) -> Result<ExactProtocolRefinementCoverageReceipt, ProtocolRefinementCoverageDenial> {
    let execution = execute_compaction_visibility_owner_cases();
    require_exact_protocol_refinement_coverage(
        current_compaction_visibility_owner_cases(),
        execution.owner_cases(),
        execution.mapped_cases().map(|mapping| mapping.owner_case()),
    )
}

fn declared_cases(protocol: ProtocolFamily) -> Vec<OwnerBindingCase> {
    match protocol {
        ProtocolFamily::DurabilityRecovery => DurabilityRecoveryAction::production_owned()
            .into_iter()
            .map(OwnerBindingCase::Durability)
            .collect(),
        ProtocolFamily::RecoverySourcePrecedence => SourcePrecedenceActionKind::all()
            .into_iter()
            .map(OwnerBindingCase::RecoverySource)
            .collect(),
        ProtocolFamily::LeaseReclaim => LeaseReclaimActionKind::all()
            .into_iter()
            .map(OwnerBindingCase::Lease)
            .collect(),
        ProtocolFamily::QuarantineReadmission => QuarantineReadmissionState::all()
            .into_iter()
            .map(OwnerBindingCase::Quarantine)
            .collect(),
        ProtocolFamily::ImportPublication => ImportPublicationAction::all()
            .into_iter()
            .map(OwnerBindingCase::Import)
            .collect(),
        ProtocolFamily::ReplicationAdmission => ReplicationAdmissionAction::all()
            .into_iter()
            .map(OwnerBindingCase::Replication)
            .collect(),
        ProtocolFamily::SharedFrontiers => SharedFrontierAction::all()
            .into_iter()
            .map(OwnerBindingCase::Shared)
            .collect(),
        ProtocolFamily::CompactionVisibility => Vec::new(),
    }
}

fn owner_binding_case(action: &CanonicalProtocolAction) -> Option<OwnerBindingCase> {
    match action {
        CanonicalProtocolAction::DurabilityRecovery(action) => {
            Some(OwnerBindingCase::Durability(*action))
        }
        CanonicalProtocolAction::RecoverySourcePrecedence(action) => {
            Some(OwnerBindingCase::RecoverySource(action.kind()))
        }
        CanonicalProtocolAction::LeaseReclaim(action) => {
            Some(OwnerBindingCase::Lease(action.kind()))
        }
        CanonicalProtocolAction::QuarantineReadmission(state) => {
            Some(OwnerBindingCase::Quarantine(*state))
        }
        CanonicalProtocolAction::ImportPublication(action) => {
            Some(OwnerBindingCase::Import(*action))
        }
        CanonicalProtocolAction::ReplicationAdmission(action) => {
            Some(OwnerBindingCase::Replication(*action))
        }
        CanonicalProtocolAction::SharedFrontier(action) => Some(OwnerBindingCase::Shared(*action)),
        CanonicalProtocolAction::CompactionVisibility(_) => None,
    }
}
