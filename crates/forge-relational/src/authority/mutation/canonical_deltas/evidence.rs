use std::io::{self, Write};

use crate::payloads::data::RecordPayload;
use crate::publication::patch::data::{AspectKey, RecordStructuralChange};
use crate::schema::data::LoweredAspectBinding;
use crate::transactions::data::{
    AspectEvaluationTrace, AspectEvaluationTraceRow, AspectLifecycleTransitionClass,
    AspectTraceEvidence,
};

use super::data::{
    BindingEvidence, CanonicalDeltaError, CanonicalRecordAspectDelta, EvaluatedAspectBinding,
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
            contains_degraded_precision: self.contains_degraded_precision,
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
            precision: self.precision,
            evidence: self.evidence.trace_evidence(),
        }
    }
}

impl BindingEvidence {
    fn trace_evidence(&self) -> AspectTraceEvidence {
        match self {
            Self::JsonFieldPresenceOrValue {
                old_present,
                new_present,
                old_canonical_json,
                new_canonical_json,
            } => AspectTraceEvidence::JsonFieldPresenceOrValue {
                old_present: *old_present,
                new_present: *new_present,
                old_canonical_json: old_canonical_json.clone(),
                new_canonical_json: new_canonical_json.clone(),
            },
            Self::EndpointIdentity { old, new } => AspectTraceEvidence::EndpointIdentity {
                old: *old,
                new: *new,
            },
            Self::Lifecycle { transition } => AspectTraceEvidence::Lifecycle {
                transition: transition.trace_transition(),
            },
            Self::OpaquePayloadDigest {
                old_present,
                new_present,
                old_diagnostic_digest,
                new_diagnostic_digest,
            } => AspectTraceEvidence::OpaquePayloadDigest {
                old_present: *old_present,
                new_present: *new_present,
                old_diagnostic_digest: *old_diagnostic_digest,
                new_diagnostic_digest: *new_diagnostic_digest,
            },
        }
    }
}

impl LifecycleTransitionClass {
    fn trace_transition(self) -> AspectLifecycleTransitionClass {
        match self {
            Self::NoTransition => AspectLifecycleTransitionClass::NoTransition,
            Self::Create => AspectLifecycleTransitionClass::Create,
            Self::Update => AspectLifecycleTransitionClass::Update,
            Self::Delete => AspectLifecycleTransitionClass::Delete,
            Self::RetainForAudit => AspectLifecycleTransitionClass::RetainForAudit,
        }
    }
}

pub(super) fn raw_field_name<'a>(
    aspect_key: &AspectKey,
    _binding: &LoweredAspectBinding,
    value: &'a crate::symbols::data::InternedString,
) -> Result<&'a str, CanonicalDeltaError> {
    match value {
        crate::symbols::data::InternedString::Raw(raw) => Ok(raw.as_str()),
        crate::symbols::data::InternedString::Symbol(_) => {
            Err(CanonicalDeltaError::SymbolicLoweredFieldName {
                aspect_key: aspect_key.clone(),
                field: value.clone(),
            })
        }
    }
}

pub(super) fn lifecycle_transition(
    structural_change: RecordStructuralChange,
) -> LifecycleTransitionClass {
    match structural_change {
        RecordStructuralChange::Created => LifecycleTransitionClass::Create,
        RecordStructuralChange::Updated => LifecycleTransitionClass::Update,
        RecordStructuralChange::Deleted => LifecycleTransitionClass::Delete,
        RecordStructuralChange::RetainedForAudit => LifecycleTransitionClass::RetainForAudit,
    }
}

pub(super) fn serialize_json_value(
    aspect_key: &AspectKey,
    value: &serde_json::Value,
) -> Result<String, CanonicalDeltaError> {
    serde_json::to_string(value).map_err(|error| CanonicalDeltaError::JsonEvidenceSerialization {
        aspect_key: aspect_key.clone(),
        detail: format!(
            "failed to serialize canonical JSON evidence for aspect {:?}: {}",
            aspect_key, error
        ),
    })
}

pub(super) fn payload_diagnostic_digest(
    payload: &RecordPayload,
) -> Result<u128, CanonicalDeltaError> {
    match payload {
        RecordPayload::StructuredJson(value) => {
            let mut sink = Fnv128Writer::default();
            serde_json::to_writer(&mut sink, value).map_err(|error| {
                CanonicalDeltaError::JsonEvidenceSerialization {
                    aspect_key: AspectKey(crate::symbols::data::InternedString::Raw(
                        "opaque-payload".to_string(),
                    )),
                    detail: format!(
                        "failed to stream canonical payload evidence for opaque aspect digest: {}",
                        error
                    ),
                }
            })?;
            Ok(sink.finish())
        }
        RecordPayload::OpaqueBytes(bytes) => Ok(hash_bytes_u128(bytes)),
    }
}

#[derive(Default)]
struct Fnv128Writer {
    hash: u128,
}

impl Fnv128Writer {
    fn finish(self) -> u128 {
        if self.hash == 0 {
            0x6c62272e07bb014262b821756295c58d
        } else {
            self.hash
        }
    }
}

impl Write for Fnv128Writer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        const FNV_OFFSET: u128 = 0x6c62272e07bb014262b821756295c58d;
        const FNV_PRIME: u128 = 0x0000000001000000000000000000013B;
        if self.hash == 0 {
            self.hash = FNV_OFFSET;
        }
        for byte in buf {
            self.hash ^= u128::from(*byte);
            self.hash = self.hash.wrapping_mul(FNV_PRIME);
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn hash_bytes_u128(bytes: &[u8]) -> u128 {
    const FNV_OFFSET: u128 = 0x6c62272e07bb014262b821756295c58d;
    const FNV_PRIME: u128 = 0x0000000001000000000000000000013B;

    let mut hash = FNV_OFFSET;
    for byte in bytes {
        hash ^= u128::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}
