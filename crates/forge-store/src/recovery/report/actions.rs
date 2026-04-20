use serde::Serialize;

use super::super::{
    DegradedStateReport, DurableMutationIdentity, DurableRecoveryOutcome,
    MaintenanceArtifactFamily, MaintenanceRecoveryDisposition, MaintenanceRecoveryReport,
    RecoveryQuarantineScope, SupportArtifactRecoveryDisposition, SupportArtifactRecoveryReport,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum RecoveryOperatorDisposition {
    Clean,
    RetainedWithoutAcknowledgment,
    RebuildRequired,
    QuarantineRequired,
    SalvageRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum RecoveryOperatorActionKind {
    InspectRetainedWithoutAcknowledgment,
    RebuildMaintenanceArtifact,
    QuarantineScope,
    SalvageScope,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RecoveryOperatorAction {
    kind: RecoveryOperatorActionKind,
    scope_identity: String,
    reason: String,
}

impl RecoveryOperatorAction {
    pub fn kind(&self) -> RecoveryOperatorActionKind { self.kind }
    pub fn scope_identity(&self) -> &str { &self.scope_identity }
    pub fn reason(&self) -> &str { &self.reason }
}

pub(super) fn determine_operator_disposition(
    degraded: &DegradedStateReport,
    maintenance: &MaintenanceRecoveryReport,
    support_artifacts: &SupportArtifactRecoveryReport,
) -> RecoveryOperatorDisposition {
    if !degraded.quarantines().is_empty()
        || maintenance
            .entries()
            .iter()
            .any(|entry| entry.disposition() == MaintenanceRecoveryDisposition::RequireQuarantine)
        || support_artifacts.quarantines().into_iter().next().is_some()
    {
        return RecoveryOperatorDisposition::QuarantineRequired;
    }
    if !degraded.salvages().is_empty() {
        return RecoveryOperatorDisposition::SalvageRequired;
    }
    if !degraded.rebuilds().is_empty()
        || maintenance
            .entries()
            .iter()
            .any(|entry| entry.disposition() == MaintenanceRecoveryDisposition::RequireRebuild)
        || support_artifacts.rebuilds().into_iter().next().is_some()
    {
        return RecoveryOperatorDisposition::RebuildRequired;
    }
    if !degraded.retained_without_acknowledgment().is_empty() {
        return RecoveryOperatorDisposition::RetainedWithoutAcknowledgment;
    }
    RecoveryOperatorDisposition::Clean
}

pub(super) fn build_recommended_actions(
    outcome: &DurableRecoveryOutcome,
    degraded: &DegradedStateReport,
    maintenance: &MaintenanceRecoveryReport,
    support_artifacts: &SupportArtifactRecoveryReport,
) -> Vec<RecoveryOperatorAction> {
    let mut actions = Vec::new();

    for degraded in degraded.retained_without_acknowledgment() {
        actions.push(RecoveryOperatorAction {
            kind: RecoveryOperatorActionKind::InspectRetainedWithoutAcknowledgment,
            scope_identity: mutation_scope_identity(outcome, degraded.durable_mutation_id),
            reason: degraded.reason.clone(),
        });
    }
    for degraded in degraded.rebuilds() {
        actions.push(RecoveryOperatorAction {
            kind: RecoveryOperatorActionKind::RebuildMaintenanceArtifact,
            scope_identity: mutation_scope_identity(outcome, degraded.durable_mutation_id),
            reason: degraded.reason.clone(),
        });
    }
    for degraded in degraded.quarantines() {
        actions.push(RecoveryOperatorAction {
            kind: RecoveryOperatorActionKind::QuarantineScope,
            scope_identity: format_quarantine_scope(
                degraded.scope,
                mutation_scope_identity(outcome, degraded.durable_mutation_id),
            ),
            reason: degraded.reason.clone(),
        });
    }
    for degraded in degraded.salvages() {
        actions.push(RecoveryOperatorAction {
            kind: RecoveryOperatorActionKind::SalvageScope,
            scope_identity: format_quarantine_scope(
                degraded.scope,
                mutation_scope_identity(outcome, degraded.durable_mutation_id),
            ),
            reason: degraded.reason.clone(),
        });
    }

    for entry in maintenance.entries() {
        match entry.disposition() {
            MaintenanceRecoveryDisposition::RequireRebuild => {
                actions.push(RecoveryOperatorAction {
                    kind: RecoveryOperatorActionKind::RebuildMaintenanceArtifact,
                    scope_identity: format_maintenance_scope(entry.family(), entry.scope_identity()),
                    reason: entry.reason().to_string(),
                });
            }
            MaintenanceRecoveryDisposition::RequireQuarantine => {
                actions.push(RecoveryOperatorAction {
                    kind: RecoveryOperatorActionKind::QuarantineScope,
                    scope_identity: format_maintenance_scope(entry.family(), entry.scope_identity()),
                    reason: entry.reason().to_string(),
                });
            }
            _ => {}
        }
    }

    for entry in support_artifacts.entries() {
        match entry.disposition() {
            SupportArtifactRecoveryDisposition::RequireRebuild => {
                actions.push(RecoveryOperatorAction {
                    kind: RecoveryOperatorActionKind::RebuildMaintenanceArtifact,
                    scope_identity: format!("support-artifact:{}", entry.scope_identity()),
                    reason: entry.reason().to_string(),
                });
            }
            SupportArtifactRecoveryDisposition::RequireQuarantine => {
                actions.push(RecoveryOperatorAction {
                    kind: RecoveryOperatorActionKind::QuarantineScope,
                    scope_identity: format!("support-artifact:{}", entry.scope_identity()),
                    reason: entry.reason().to_string(),
                });
            }
            SupportArtifactRecoveryDisposition::RetainClean => {}
        }
    }

    actions
}

fn mutation_scope_identity(
    outcome: &DurableRecoveryOutcome,
    durable_mutation_id: crate::DurableMutationId,
) -> String {
    match outcome.mutation_identity(durable_mutation_id) {
        Some(DurableMutationIdentity::BulkChunk {
            plan_kind,
            program_id,
            plan_id,
            chunk_ordinal,
        }) => {
            let kind = match plan_kind {
                crate::bulk::BulkPlanKind::Ingest => "ingest",
                crate::bulk::BulkPlanKind::Transform => "transform",
            };
            format!("bulk:{kind}:{program_id}:{plan_id}:chunk:{chunk_ordinal}")
        }
        Some(DurableMutationIdentity::GenericOperation { operation_name }) => {
            format!("operation:{operation_name}")
        }
        None => format!("durable-mutation:{}", durable_mutation_id.0),
    }
}

fn format_quarantine_scope(scope: RecoveryQuarantineScope, identity: String) -> String {
    match scope {
        RecoveryQuarantineScope::ArtifactInstance => identity,
        RecoveryQuarantineScope::ArtifactFamily => format!("artifact-family:{identity}"),
        RecoveryQuarantineScope::Branch => format!("branch:{identity}"),
        RecoveryQuarantineScope::Tenant => format!("tenant:{identity}"),
        RecoveryQuarantineScope::StoreWide => format!("store-wide:{identity}"),
    }
}

fn format_maintenance_scope(family: MaintenanceArtifactFamily, scope_identity: &str) -> String {
    let family_name = match family {
        MaintenanceArtifactFamily::Snapshot => "snapshot",
        MaintenanceArtifactFamily::Compaction => "compaction",
        MaintenanceArtifactFamily::Reclaim => "reclaim",
        MaintenanceArtifactFamily::Capsule => "capsule",
    };
    format!("{family_name}:{scope_identity}")
}
