use std::sync::Arc;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::merge::data::{
    BoundExecutableMergeRecordPlan, ExecutableAspectPlan, MaterializedAspectValue,
    MergeExecutableRecordProvenance,
};
use crate::transactions::data::RecordRef;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutedMergeRecordClass {
    AdoptSource,
    PreserveShared,
    Reconcile,
    ConvergeDeletedOnBothSides,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutedMergeAspectClass {
    AdoptSourceValue,
    PreserveSharedValue,
    ReconcileValue,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutedMergeAspectDiagnosticRow {
    pub aspect_key: crate::publication::patch::data::AspectKey,
    pub class: ExecutedMergeAspectClass,
    pub source_value: Option<MaterializedAspectValue>,
    pub target_value: Option<MaterializedAspectValue>,
    pub base_value: Option<MaterializedAspectValue>,
    pub shared_value: Option<MaterializedAspectValue>,
    pub resolved_value: Option<MaterializedAspectValue>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutedMergeRecordDiagnosticRow {
    pub class: ExecutedMergeRecordClass,
    pub source_record: Option<RecordRef>,
    pub target_record: Option<RecordRef>,
    pub record: Option<RecordRef>,
    pub provenance: MergeExecutableRecordProvenance,
    pub aspect_rows: Arc<[ExecutedMergeAspectDiagnosticRow]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeExecutionDiagnosticsPlan {
    pub executed_records: Arc<[ExecutedMergeRecordDiagnosticRow]>,
    pub digest: String,
}

pub(crate) fn diagnostics_plan_from_record_plans(
    record_plans: &[BoundExecutableMergeRecordPlan],
) -> MergeExecutionDiagnosticsPlan {
    let executed_records = Arc::from(
        record_plans
            .iter()
            .map(executed_record_row)
            .collect::<Vec<_>>(),
    );
    let digest = merge_execution_diagnostics_digest(&executed_records);
    MergeExecutionDiagnosticsPlan {
        executed_records,
        digest,
    }
}

pub(crate) fn merge_execution_diagnostics_digest(
    executed_records: &[ExecutedMergeRecordDiagnosticRow],
) -> String {
    let bytes = serde_json::to_vec(executed_records)
        .expect("merge execution diagnostics plan serialization");
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn executed_record_row(
    plan: &BoundExecutableMergeRecordPlan,
) -> ExecutedMergeRecordDiagnosticRow {
    match plan {
        BoundExecutableMergeRecordPlan::AdoptSource(plan) => ExecutedMergeRecordDiagnosticRow {
            class: ExecutedMergeRecordClass::AdoptSource,
            source_record: Some(plan.source_record.clone()),
            target_record: None,
            record: None,
            provenance: plan.provenance.clone(),
            aspect_rows: Arc::from(
                plan.aspect_plan
                    .iter()
                    .map(executed_aspect_row)
                    .collect::<Vec<_>>(),
            ),
        },
        BoundExecutableMergeRecordPlan::PreserveShared(plan) => ExecutedMergeRecordDiagnosticRow {
            class: ExecutedMergeRecordClass::PreserveShared,
            source_record: None,
            target_record: plan.target_record.clone(),
            record: Some(plan.record.clone()),
            provenance: plan.provenance.clone(),
            aspect_rows: Arc::from(
                plan.aspect_plan
                    .iter()
                    .map(executed_aspect_row)
                    .collect::<Vec<_>>(),
            ),
        },
        BoundExecutableMergeRecordPlan::Reconcile(plan) => ExecutedMergeRecordDiagnosticRow {
            class: ExecutedMergeRecordClass::Reconcile,
            source_record: Some(plan.source_record.clone()),
            target_record: Some(plan.target_record.clone()),
            record: None,
            provenance: plan.provenance.clone(),
            aspect_rows: Arc::from(
                plan.aspect_plan
                    .iter()
                    .map(executed_aspect_row)
                    .collect::<Vec<_>>(),
            ),
        },
        BoundExecutableMergeRecordPlan::ConvergeDeletedOnBothSides(plan) => {
            ExecutedMergeRecordDiagnosticRow {
                class: ExecutedMergeRecordClass::ConvergeDeletedOnBothSides,
                source_record: Some(plan.source_record.clone()),
                target_record: plan.target_record.clone(),
                record: None,
                provenance: plan.provenance.clone(),
                aspect_rows: Arc::from(Vec::<ExecutedMergeAspectDiagnosticRow>::new()),
            }
        }
    }
}

fn executed_aspect_row(plan: &ExecutableAspectPlan) -> ExecutedMergeAspectDiagnosticRow {
    match plan {
        ExecutableAspectPlan::AdoptSourceValue {
            aspect_key,
            source_value,
        } => ExecutedMergeAspectDiagnosticRow {
            aspect_key: aspect_key.clone(),
            class: ExecutedMergeAspectClass::AdoptSourceValue,
            source_value: Some(source_value.clone()),
            target_value: None,
            base_value: None,
            shared_value: None,
            resolved_value: None,
        },
        ExecutableAspectPlan::PreserveSharedValue {
            aspect_key,
            shared_value,
        } => ExecutedMergeAspectDiagnosticRow {
            aspect_key: aspect_key.clone(),
            class: ExecutedMergeAspectClass::PreserveSharedValue,
            source_value: None,
            target_value: None,
            base_value: None,
            shared_value: Some(shared_value.clone()),
            resolved_value: None,
        },
        ExecutableAspectPlan::ReconcileValue {
            aspect_key,
            source_value,
            target_value,
            base_value,
            resolved_value,
        } => ExecutedMergeAspectDiagnosticRow {
            aspect_key: aspect_key.clone(),
            class: ExecutedMergeAspectClass::ReconcileValue,
            source_value: source_value.clone(),
            target_value: target_value.clone(),
            base_value: base_value.clone(),
            shared_value: None,
            resolved_value: resolved_value.clone(),
        },
    }
}
