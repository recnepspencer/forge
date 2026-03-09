use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::artifact::AttachmentRecord;
use crate::capture::{RecordSchemaVersion, RunRecord};
use crate::compatibility::{
    check_record_schema_with_policy, CompatibilityPolicy, CompatibilityReport, CompatibilityStatus,
};
use crate::identity::{ReplayId, RunId, ScenarioId};
use crate::scenario::{ExecutionProfile, ExecutionRequest};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReplayRequest<TargetId = String> {
    pub name: String,
    pub source_run: RunRecord<TargetId>,
    pub request: ExecutionRequest<TargetId>,
    pub profile: ExecutionProfile,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReplayRecord<TargetId = String> {
    pub schema_version: RecordSchemaVersion,
    pub replay_id: ReplayId,
    pub source_run_id: RunId,
    pub scenario_id: ScenarioId,
    pub adapter_name: String,
    pub scenario_name: String,
    pub replay_name: String,
    pub requested_targets: Vec<TargetId>,
    pub summary: Value,
    pub attachments: Vec<AttachmentRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayCompatibilityReport {
    pub status: CompatibilityStatus,
    pub source_schema: RecordSchemaVersion,
    pub expected_schema: RecordSchemaVersion,
    pub schema_report: CompatibilityReport,
    pub migration_plan: ReplayMigrationPlan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplayMigrationPolicy {
    Exact,
    BackwardCompatible,
    UpgradeToCurrent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplayMigrationSupport {
    NotRequired,
    Supported,
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayMigrationStep {
    pub from: RecordSchemaVersion,
    pub to: RecordSchemaVersion,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayMigrationPlan {
    pub policy: ReplayMigrationPolicy,
    pub support: ReplayMigrationSupport,
    pub steps: Vec<ReplayMigrationStep>,
}

pub fn plan_replay_migration(
    source_schema: RecordSchemaVersion,
    expected_schema: RecordSchemaVersion,
    policy: ReplayMigrationPolicy,
) -> ReplayMigrationPlan {
    let support = if source_schema == expected_schema {
        ReplayMigrationSupport::NotRequired
    } else if matches!(policy, ReplayMigrationPolicy::UpgradeToCurrent) {
        ReplayMigrationSupport::Supported
    } else {
        ReplayMigrationSupport::Unsupported
    };
    let steps = if matches!(support, ReplayMigrationSupport::Supported) {
        vec![ReplayMigrationStep {
            from: source_schema,
            to: expected_schema,
            description: format!(
                "upgrade replay record from {:?} to {:?}",
                source_schema, expected_schema
            ),
        }]
    } else {
        Vec::new()
    };
    ReplayMigrationPlan {
        policy,
        support,
        steps,
    }
}

pub fn check_replay_compatibility<TargetId>(
    request: &ReplayRequest<TargetId>,
    expected_schema: RecordSchemaVersion,
    policy: CompatibilityPolicy,
    migration_policy: ReplayMigrationPolicy,
) -> ReplayCompatibilityReport {
    let schema_report =
        check_record_schema_with_policy(expected_schema, request.source_run.schema_version, policy);
    let migration_plan =
        plan_replay_migration(request.source_run.schema_version, expected_schema, migration_policy);
    let status = if matches!(migration_plan.support, ReplayMigrationSupport::Supported) {
        CompatibilityStatus::Compatible
    } else {
        schema_report.status
    };
    ReplayCompatibilityReport {
        status,
        source_schema: request.source_run.schema_version,
        expected_schema,
        schema_report,
        migration_plan,
    }
}

impl<TargetId> ReplayRequest<TargetId> {
    pub fn compatibility_report(
        &self,
        expected_schema: RecordSchemaVersion,
        policy: CompatibilityPolicy,
        migration_policy: ReplayMigrationPolicy,
    ) -> ReplayCompatibilityReport {
        check_replay_compatibility(self, expected_schema, policy, migration_policy)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::capture::{
        DiagnosticsLevel, ExecutionMode, RunOutcome, RunStatus, TargetStatusRecord,
    };
    use crate::compatibility::{CompatibilityPolicy, CompatibilityStatus};
    use crate::identity::{run_id, scenario_id};

    use super::{check_replay_compatibility, plan_replay_migration, ReplayMigrationPolicy, ReplayMigrationSupport, ReplayRequest};
    use crate::capture::{ObservationStatus, RecordSchemaVersion, RunRecord};
    use crate::scenario::{ExecutionProfile, ExecutionRequest};

    #[test]
    fn replay_request_reports_schema_compatibility() {
        let scenario_id = scenario_id("fixture");
        let source_run = RunRecord {
            schema_version: RecordSchemaVersion::V1,
            run_id: run_id(&scenario_id, "profile", "request"),
            scenario_id,
            adapter_name: "double".to_string(),
            scenario_name: "fixture".to_string(),
            profile_name: "profile".to_string(),
            time_marker: None,
            feed_batch: None,
            execution_mode: ExecutionMode::Serial,
            diagnostics_level: DiagnosticsLevel::Operational,
            status: RunStatus::Succeeded,
            outcome: RunOutcome::Completed,
            budget_usage: None,
            requested_targets: vec!["target".to_string()],
            target_statuses: vec![TargetStatusRecord {
                target: "target".to_string(),
                status: ObservationStatus::Clean,
                detail: None,
            }],
            changed_targets: vec!["target".to_string()],
            attachments: Vec::new(),
            summary: json!({}),
            extensions: Default::default(),
        };
        let request = ReplayRequest {
            name: "replay".to_string(),
            source_run,
            request: ExecutionRequest::target("request", "target".to_string()),
            profile: ExecutionProfile::serial("profile"),
        };

        let report = check_replay_compatibility(
            &request,
            RecordSchemaVersion::V1,
            CompatibilityPolicy::Exact,
            ReplayMigrationPolicy::Exact,
        );
        assert_eq!(report.status, CompatibilityStatus::Compatible);
    }

    #[test]
    fn replay_migration_plan_can_represent_upgrades() {
        let plan = plan_replay_migration(
            RecordSchemaVersion::V1,
            RecordSchemaVersion::V1,
            ReplayMigrationPolicy::UpgradeToCurrent,
        );
        assert_eq!(plan.support, ReplayMigrationSupport::NotRequired);
    }
}
