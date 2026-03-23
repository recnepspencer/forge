use serde::{Deserialize, Serialize};
use serde_json::{json, to_value, Value};

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
#[must_use]
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
#[must_use]
pub struct AspectEvaluationTraceRow {
    pub aspect_key: AspectKey,
    pub changed: bool,
    pub precision: AspectPrecision,
    pub evidence: AspectTraceEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AspectTraceEvidence {
    JsonFieldPresenceOrValue {
        old_present: bool,
        new_present: bool,
        old_canonical_json: Option<String>,
        new_canonical_json: Option<String>,
    },
    EndpointIdentity {
        old: Option<EntityId>,
        new: Option<EntityId>,
    },
    Lifecycle {
        transition: AspectLifecycleTransitionClass,
    },
    OpaquePayloadDigest {
        old_present: bool,
        new_present: bool,
        old_diagnostic_digest: Option<u128>,
        new_diagnostic_digest: Option<u128>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AspectLifecycleTransitionClass {
    NoTransition,
    Create,
    Update,
    Delete,
    RetainForAudit,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub struct AspectEmissionTrace {
    pub target: RecordRef,
    pub patch_position: PatchStreamPosition,
    pub patch_record_index: u64,
    pub structural_change: RecordStructuralChange,
    pub changed_aspects: CanonicalAspectSet,
    pub contains_degraded_precision: bool,
}

impl AspectEvaluationTrace {
    pub fn diagnostic_artifact(&self) -> RelationalDiagnosticArtifact {
        let fields = AspectEvaluationTraceFields::from_trace(self);
        RelationalDiagnosticArtifact {
            scope: DiagnosticsScope::Transaction,
            kind: DiagnosticsArtifactKind::DetailedTrace,
            determinism: DeterminismExpectation::Required,
            entries: vec![RelationalDiagnosticsEntry {
                code: DiagnosticCode::AspectEvaluationTraced,
                message: "aspect evaluation trace derived from canonical commit-time delta"
                    .to_string(),
                fields: trace_fields_value(&fields, "aspect evaluation trace"),
            }],
        }
    }
}

impl AspectEmissionTrace {
    pub fn diagnostic_artifact(&self) -> RelationalDiagnosticArtifact {
        let fields = AspectEmissionTraceFields::from_trace(self);
        RelationalDiagnosticArtifact {
            scope: DiagnosticsScope::PatchPublication,
            kind: DiagnosticsArtifactKind::DetailedTrace,
            determinism: DeterminismExpectation::Required,
            entries: vec![RelationalDiagnosticsEntry {
                code: DiagnosticCode::AspectEmissionTraced,
                message: "aspect emission trace derived from canonical patch publication"
                    .to_string(),
                fields: trace_fields_value(&fields, "aspect emission trace"),
            }],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct AspectEvaluationTraceFields {
    target: RecordRef,
    kind_id: u64,
    plan_revision: String,
    structural_change: RecordStructuralChange,
    changed_aspects: CanonicalAspectSet,
    contains_degraded_precision: bool,
    binding_rows: Vec<AspectEvaluationTraceRow>,
}

impl AspectEvaluationTraceFields {
    fn from_trace(trace: &AspectEvaluationTrace) -> Self {
        Self {
            target: trace.target.clone(),
            kind_id: trace.kind_id.0 as u64,
            plan_revision: trace.plan_revision.0.to_string(),
            structural_change: trace.structural_change,
            changed_aspects: trace.changed_aspects.clone(),
            contains_degraded_precision: trace.contains_degraded_precision,
            binding_rows: trace.binding_rows.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct AspectEmissionTraceFields {
    target: RecordRef,
    patch_position: PatchStreamPosition,
    patch_record_index: u64,
    structural_change: RecordStructuralChange,
    changed_aspects: CanonicalAspectSet,
    contains_degraded_precision: bool,
}

impl AspectEmissionTraceFields {
    fn from_trace(trace: &AspectEmissionTrace) -> Self {
        Self {
            target: trace.target.clone(),
            patch_position: trace.patch_position,
            patch_record_index: trace.patch_record_index,
            structural_change: trace.structural_change,
            changed_aspects: trace.changed_aspects.clone(),
            contains_degraded_precision: trace.contains_degraded_precision,
        }
    }
}

fn trace_fields_value<T>(fields: &T, trace_kind: &str) -> Value
where
    T: Serialize,
{
    match to_value(fields) {
        Ok(value) => value,
        Err(error) => json!({
            "trace_kind": trace_kind,
            "serialization_failure": error.to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::AspectLifecycleTransitionClass;

    #[test]
    fn lifecycle_no_transition_serializes_explicitly() {
        let serialized = serde_json::to_string(&AspectLifecycleTransitionClass::NoTransition)
            .expect("serialize aspect lifecycle transition");
        assert_eq!(serialized, "\"NoTransition\"");
    }
}
