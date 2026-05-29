mod diagnostic_fields;

use serde::{Deserialize, Serialize};

use crate::diagnostics::data::{
    DeterminismExpectation, DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope,
    RelationalDiagnosticArtifact, RelationalDiagnosticsEntry,
};
use crate::identity::data::{EntityId, KindId};
use crate::publication::patch::data::{
    CanonicalAspectSet, PatchStreamPosition, RecordStructuralChange,
};
use crate::schema::data::AspectPlanRevision;

use super::RecordRef;
use diagnostic_fields::{emission_trace_diagnostic_fields, evaluation_trace_diagnostic_fields};
use forge_foundational::facade::{AspectKey, AspectValue, FieldKey, StructAspectValue};

#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
pub struct AspectEvaluationTrace {
    pub target: RecordRef,
    pub kind_id: KindId,
    pub plan_revision: AspectPlanRevision,
    pub structural_change: RecordStructuralChange,
    pub changed_aspects: CanonicalAspectSet,
    pub contains_opaque_aspect: bool,
    pub binding_rows: Vec<AspectEvaluationTraceRow>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
pub struct AspectEvaluationTraceRow {
    pub aspect_key: AspectKey,
    pub changed: bool,
    pub aspect_shape: forge_foundational::AspectShape,
    pub evidence: AspectTraceEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AspectTraceEvidence {
    ScalarAspectPresenceOrValue {
        old_present: bool,
        new_present: bool,
        old_value: Option<AspectValue>,
        new_value: Option<AspectValue>,
    },
    StructAspectPresenceOrValue {
        old_present: bool,
        new_present: bool,
        old_value: Option<StructAspectValue>,
        new_value: Option<StructAspectValue>,
    },
    EndpointIdentity {
        old: Option<EntityId>,
        new: Option<EntityId>,
    },
    Lifecycle {
        transition: AspectLifecycleTransitionClass,
    },
    AuthoritativePatchOperation {
        operation: AspectTracePatchOperation,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AspectTracePatchOperation {
    WholeAspectSet {
        value: Option<AspectValue>,
    },
    WholeAspectClear,
    FieldLevelPatch {
        field_sets: Vec<(FieldKey, AspectValue)>,
        field_clears: Vec<FieldKey>,
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
    pub contains_opaque_aspect: bool,
}

impl AspectEvaluationTrace {
    pub fn diagnostic_artifact(&self) -> RelationalDiagnosticArtifact {
        let fields = evaluation_trace_diagnostic_fields(self);
        RelationalDiagnosticArtifact::new(
            DiagnosticsScope::Transaction,
            DiagnosticsArtifactKind::DetailedTrace,
            DeterminismExpectation::Required,
            vec![RelationalDiagnosticsEntry::new(
                DiagnosticCode::AspectEvaluationTraced,
                "aspect evaluation trace derived from canonical commit-time delta",
                fields,
            )],
        )
    }
}

impl AspectEmissionTrace {
    pub fn diagnostic_artifact(&self) -> RelationalDiagnosticArtifact {
        let fields = emission_trace_diagnostic_fields(self);
        RelationalDiagnosticArtifact::new(
            DiagnosticsScope::PatchPublication,
            DiagnosticsArtifactKind::DetailedTrace,
            DeterminismExpectation::Required,
            vec![RelationalDiagnosticsEntry::new(
                DiagnosticCode::AspectEmissionTraced,
                "aspect emission trace derived from canonical patch publication",
                fields,
            )],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{AspectLifecycleTransitionClass, AspectTraceEvidence};

    #[test]
    fn lifecycle_no_transition_is_typed_trace_evidence() {
        let evidence = AspectTraceEvidence::Lifecycle {
            transition: AspectLifecycleTransitionClass::NoTransition,
        };

        let AspectTraceEvidence::Lifecycle { transition } = evidence else {
            panic!("expected lifecycle trace evidence");
        };
        assert_eq!(transition, AspectLifecycleTransitionClass::NoTransition);
    }
}
