use serde::Serialize;

use super::{
    DegradedStateReport, DurableRecoveryOutcome, DurableRecoveryPlan, MaintenanceArtifactFamily,
    MaintenanceRecoveryDisposition, MaintenanceRecoveryReport, RecoveryQuarantineScope,
    RecoverySourceKind,
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
pub struct DurableRecoverySourceSummary {
    published_authoritative_truth: usize,
    hosted_runtime_canonical_result: usize,
    intent_only: usize,
    requires_rebuild: usize,
    requires_quarantine: usize,
    maintenance_residue: usize,
}

impl DurableRecoverySourceSummary {
    pub(crate) fn from_outcome(outcome: &DurableRecoveryOutcome) -> Self {
        let mut summary = Self {
            published_authoritative_truth: 0,
            hosted_runtime_canonical_result: 0,
            intent_only: 0,
            requires_rebuild: 0,
            requires_quarantine: 0,
            maintenance_residue: 0,
        };

        for report in &outcome.source_reports {
            match report.source_kind() {
                RecoverySourceKind::PublishedAuthoritativeTruth => {
                    summary.published_authoritative_truth += 1;
                }
                RecoverySourceKind::HostedRuntimeCanonicalResult => {
                    summary.hosted_runtime_canonical_result += 1;
                }
                RecoverySourceKind::IntentOnly => {
                    summary.intent_only += 1;
                }
                RecoverySourceKind::RequiresRebuild => {
                    summary.requires_rebuild += 1;
                }
                RecoverySourceKind::RequiresQuarantine => {
                    summary.requires_quarantine += 1;
                }
                RecoverySourceKind::MaintenanceResidue => {
                    summary.maintenance_residue += 1;
                }
            }
        }

        summary
    }

    pub fn published_authoritative_truth(&self) -> usize {
        self.published_authoritative_truth
    }

    pub fn hosted_runtime_canonical_result(&self) -> usize {
        self.hosted_runtime_canonical_result
    }

    pub fn intent_only(&self) -> usize {
        self.intent_only
    }

    pub fn requires_rebuild(&self) -> usize {
        self.requires_rebuild
    }

    pub fn requires_quarantine(&self) -> usize {
        self.requires_quarantine
    }

    pub fn maintenance_residue(&self) -> usize {
        self.maintenance_residue
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RecoveryStatusReport {
    planned_mutation_count: usize,
    recovered_decision_count: usize,
    quiescent_restart: bool,
    operator_disposition: RecoveryOperatorDisposition,
    source_summary: DurableRecoverySourceSummary,
    degraded: DegradedStateReport,
    maintenance: MaintenanceRecoveryReport,
    recommended_actions: Vec<RecoveryOperatorAction>,
}

impl RecoveryStatusReport {
    pub(crate) fn new(
        plan: &DurableRecoveryPlan,
        outcome: &DurableRecoveryOutcome,
        maintenance: MaintenanceRecoveryReport,
    ) -> Self {
        let degraded = outcome.degraded_state_report();
        let source_summary = DurableRecoverySourceSummary::from_outcome(outcome);
        let operator_disposition = determine_operator_disposition(&degraded, &maintenance);
        let recommended_actions = build_recommended_actions(&degraded, &maintenance);

        Self {
            planned_mutation_count: plan.pending_durable_mutation_ids.len(),
            recovered_decision_count: outcome.decisions.len(),
            quiescent_restart: outcome.decisions.is_empty(),
            operator_disposition,
            source_summary,
            degraded,
            maintenance,
            recommended_actions,
        }
    }

    pub fn planned_mutation_count(&self) -> usize {
        self.planned_mutation_count
    }

    pub fn recovered_decision_count(&self) -> usize {
        self.recovered_decision_count
    }

    pub fn quiescent_restart(&self) -> bool {
        self.quiescent_restart
    }

    pub fn operator_disposition(&self) -> RecoveryOperatorDisposition {
        self.operator_disposition
    }

    pub fn source_summary(&self) -> &DurableRecoverySourceSummary {
        &self.source_summary
    }

    pub fn degraded(&self) -> &DegradedStateReport {
        &self.degraded
    }

    pub fn maintenance(&self) -> &MaintenanceRecoveryReport {
        &self.maintenance
    }

    pub fn recommended_actions(&self) -> &[RecoveryOperatorAction] {
        &self.recommended_actions
    }
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
    pub fn kind(&self) -> RecoveryOperatorActionKind {
        self.kind
    }

    pub fn scope_identity(&self) -> &str {
        &self.scope_identity
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

fn determine_operator_disposition(
    degraded: &DegradedStateReport,
    maintenance: &MaintenanceRecoveryReport,
) -> RecoveryOperatorDisposition {
    if !degraded.quarantines().is_empty()
        || maintenance
            .entries()
            .iter()
            .any(|entry| entry.disposition() == MaintenanceRecoveryDisposition::RequireQuarantine)
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
    {
        return RecoveryOperatorDisposition::RebuildRequired;
    }

    if !degraded.retained_without_acknowledgment().is_empty() {
        return RecoveryOperatorDisposition::RetainedWithoutAcknowledgment;
    }

    RecoveryOperatorDisposition::Clean
}

fn build_recommended_actions(
    degraded: &DegradedStateReport,
    maintenance: &MaintenanceRecoveryReport,
) -> Vec<RecoveryOperatorAction> {
    let mut actions = Vec::new();

    for degraded in degraded.retained_without_acknowledgment() {
        actions.push(RecoveryOperatorAction {
            kind: RecoveryOperatorActionKind::InspectRetainedWithoutAcknowledgment,
            scope_identity: format!("durable-mutation:{}", degraded.durable_mutation_id.0),
            reason: degraded.reason.clone(),
        });
    }

    for degraded in degraded.rebuilds() {
        actions.push(RecoveryOperatorAction {
            kind: RecoveryOperatorActionKind::RebuildMaintenanceArtifact,
            scope_identity: format!("durable-mutation:{}", degraded.durable_mutation_id.0),
            reason: degraded.reason.clone(),
        });
    }

    for degraded in degraded.quarantines() {
        actions.push(RecoveryOperatorAction {
            kind: RecoveryOperatorActionKind::QuarantineScope,
            scope_identity: format_quarantine_scope(
                degraded.scope,
                format!("durable-mutation:{}", degraded.durable_mutation_id.0),
            ),
            reason: degraded.reason.clone(),
        });
    }

    for degraded in degraded.salvages() {
        actions.push(RecoveryOperatorAction {
            kind: RecoveryOperatorActionKind::SalvageScope,
            scope_identity: format_quarantine_scope(
                degraded.scope,
                format!("durable-mutation:{}", degraded.durable_mutation_id.0),
            ),
            reason: degraded.reason.clone(),
        });
    }

    for entry in maintenance.entries() {
        match entry.disposition() {
            MaintenanceRecoveryDisposition::RequireRebuild => {
                actions.push(RecoveryOperatorAction {
                    kind: RecoveryOperatorActionKind::RebuildMaintenanceArtifact,
                    scope_identity: format_maintenance_scope(
                        entry.family(),
                        entry.scope_identity(),
                    ),
                    reason: entry.reason().to_string(),
                });
            }
            MaintenanceRecoveryDisposition::RequireQuarantine => {
                actions.push(RecoveryOperatorAction {
                    kind: RecoveryOperatorActionKind::QuarantineScope,
                    scope_identity: format_maintenance_scope(
                        entry.family(),
                        entry.scope_identity(),
                    ),
                    reason: entry.reason().to_string(),
                });
            }
            _ => {}
        }
    }

    actions
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
