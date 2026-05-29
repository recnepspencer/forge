use crate::transactions::data::{
    AspectEvaluationTrace, AspectEvaluationTraceRow, AspectLifecycleTransitionClass,
    AspectTraceEvidence,
};

use super::data::{
    AuthoritativeDeltaPatchOperation, AuthoritativeDeltaPatchSetValue,
    CanonicalAspectDeltaEvidence, CanonicalRecordAspectDelta, EvaluatedAspectBinding,
    LifecycleTransitionClass,
};

impl CanonicalRecordAspectDelta {
    pub(crate) fn evaluation_trace(&self) -> AspectEvaluationTrace {
        AspectEvaluationTrace {
            target: self.target.clone(),
            kind_id: self.kind_id,
            plan_revision: self.plan_revision,
            structural_change: self.structural_change,
            changed_aspects: self.changed_aspects.clone(),
            contains_opaque_aspect: self.contains_opaque_aspect,
            binding_rows: self
                .evaluated_bindings
                .iter()
                .map(EvaluatedAspectBinding::trace_row)
                .collect(),
        }
    }
}

impl EvaluatedAspectBinding {
    fn trace_row(&self) -> AspectEvaluationTraceRow {
        AspectEvaluationTraceRow {
            aspect_key: self.aspect_key.clone(),
            changed: self.changed,
            aspect_shape: self.aspect_shape.clone(),
            evidence: self.evidence.trace_evidence(),
        }
    }
}

impl CanonicalAspectDeltaEvidence {
    fn trace_evidence(&self) -> AspectTraceEvidence {
        match self {
            Self::ScalarAspectValueTransition {
                old_present,
                new_present,
                old_value,
                new_value,
                ..
            } => AspectTraceEvidence::ScalarAspectPresenceOrValue {
                old_present: *old_present,
                new_present: *new_present,
                old_value: old_value.clone(),
                new_value: new_value.clone(),
            },
            Self::StructAspectValueTransition {
                old_present,
                new_present,
                old_value,
                new_value,
                ..
            } => AspectTraceEvidence::StructAspectPresenceOrValue {
                old_present: *old_present,
                new_present: *new_present,
                old_value: old_value.clone(),
                new_value: new_value.clone(),
            },
            Self::EndpointIdentity { old, new, .. } => AspectTraceEvidence::EndpointIdentity {
                old: *old,
                new: *new,
            },
            Self::Lifecycle { transition, .. } => AspectTraceEvidence::Lifecycle {
                transition: transition.trace_transition(),
            },
            Self::AuthoritativePatchOperation { operation, .. } => {
                AspectTraceEvidence::AuthoritativePatchOperation {
                    operation: operation.trace_operation(),
                }
            }
        }
    }
}

impl AuthoritativeDeltaPatchOperation {
    fn trace_operation(&self) -> crate::transactions::data::AspectTracePatchOperation {
        match self {
            Self::WholeAspectSet { value } => {
                crate::transactions::data::AspectTracePatchOperation::WholeAspectSet {
                    value: value.trace_patch_set_value(),
                }
            }
            Self::WholeAspectClear => {
                crate::transactions::data::AspectTracePatchOperation::WholeAspectClear
            }
            Self::FieldLevelPatch {
                field_sets,
                field_clears,
            } => crate::transactions::data::AspectTracePatchOperation::FieldLevelPatch {
                field_sets: field_sets
                    .iter()
                    .map(|field_set| (field_set.field.clone(), field_set.value.clone()))
                    .collect(),
                field_clears: field_clears
                    .iter()
                    .filter_map(|locator| locator.field_path().fields().first().cloned())
                    .collect(),
            },
        }
    }
}

impl AuthoritativeDeltaPatchSetValue {
    fn trace_patch_set_value(&self) -> crate::transactions::data::AspectTracePatchSetValue {
        match self {
            Self::Scalar(value) => {
                crate::transactions::data::AspectTracePatchSetValue::Scalar(value.clone())
            }
            Self::Struct(value) => {
                crate::transactions::data::AspectTracePatchSetValue::Struct(value.clone())
            }
        }
    }
}

impl LifecycleTransitionClass {
    fn trace_transition(self) -> AspectLifecycleTransitionClass {
        match self {
            Self::NoTransition => AspectLifecycleTransitionClass::NoTransition,
            Self::Create => AspectLifecycleTransitionClass::Create,
            Self::Delete => AspectLifecycleTransitionClass::Delete,
            Self::RetainForAudit => AspectLifecycleTransitionClass::RetainForAudit,
        }
    }
}
