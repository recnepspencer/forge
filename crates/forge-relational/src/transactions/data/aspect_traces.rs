use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::diagnostics::data::{
    DeterminismExpectation, DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope,
    RelationalDiagnosticArtifact, RelationalDiagnosticsEntry,
};
use crate::identity::data::{EntityId, KindId};
use crate::publication::patch::data::{
    AspectKey, CanonicalAspectSet, PatchStreamPosition, RecordStructuralChange,
};
use crate::schema::data::{AspectPlanRevision, AspectPrecision};

use super::RecordRef;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AspectEvaluationTrace {
    pub target: RecordRef,
    pub kind_id: KindId,
    pub plan_revision: AspectPlanRevision,
    pub structural_change: RecordStructuralChange,
    pub changed_aspects: CanonicalAspectSet,
    pub contains_degraded_precision: bool,
    pub binding_rows: Vec<AspectEvaluationTraceRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AspectEvaluationTraceRow {
    pub aspect_key: AspectKey,
    pub changed: bool,
    pub precision: AspectPrecision,
    pub evidence: AspectTraceEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AspectTraceEvidence {
    JsonFieldPresenceOrValue {
        old_present: bool,
        new_present: bool,
        old_canonical_hash: Option<u64>,
        new_canonical_hash: Option<u64>,
    },
    EndpointIdentity {
        old: Option<EntityId>,
        new: Option<EntityId>,
    },
    Lifecycle {
        transition: AspectLifecycleTransitionClass,
    },
    OpaquePayload {
        old_hash: Option<u128>,
        new_hash: Option<u128>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AspectLifecycleTransitionClass {
    None,
    Create,
    Update,
    Delete,
    RetainForAudit,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AspectEmissionTrace {
    pub target: RecordRef,
    pub patch_position: PatchStreamPosition,
    pub patch_record_index: usize,
    pub structural_change: RecordStructuralChange,
    pub changed_aspects: CanonicalAspectSet,
    pub contains_degraded_precision: bool,
}

impl AspectEvaluationTrace {
    pub fn diagnostic_artifact(&self) -> RelationalDiagnosticArtifact {
        RelationalDiagnosticArtifact {
            scope: DiagnosticsScope::Transaction,
            kind: DiagnosticsArtifactKind::DetailedTrace,
            determinism: DeterminismExpectation::Required,
            entries: vec![RelationalDiagnosticsEntry {
                code: DiagnosticCode::AspectEvaluationTraced,
                message: "aspect evaluation trace derived from canonical commit-time delta"
                    .to_string(),
                fields: json!({
                    "target": self.target,
                    "kind_id": self.kind_id.0,
                    "plan_revision": self.plan_revision.0.to_string(),
                    "structural_change": self.structural_change,
                    "changed_aspects": self.changed_aspects,
                    "contains_degraded_precision": self.contains_degraded_precision,
                    "binding_rows": self.binding_rows,
                }),
            }],
        }
    }
}

impl AspectEmissionTrace {
    pub fn diagnostic_artifact(&self) -> RelationalDiagnosticArtifact {
        RelationalDiagnosticArtifact {
            scope: DiagnosticsScope::PatchPublication,
            kind: DiagnosticsArtifactKind::DetailedTrace,
            determinism: DeterminismExpectation::Required,
            entries: vec![RelationalDiagnosticsEntry {
                code: DiagnosticCode::AspectEmissionTraced,
                message: "aspect emission trace derived from canonical patch publication"
                    .to_string(),
                fields: json!({
                    "target": self.target,
                    "patch_position": self.patch_position,
                    "patch_record_index": self.patch_record_index,
                    "structural_change": self.structural_change,
                    "changed_aspects": self.changed_aspects,
                    "contains_degraded_precision": self.contains_degraded_precision,
                }),
            }],
        }
    }
}
