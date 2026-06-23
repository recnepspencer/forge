use forge_foundational::facade::{CanonicalFieldPath, FieldKey};

use crate::facade::{
    ForgeQueryDerivedPatch, ForgeQueryDerivedPatchPayload, ForgeQueryDerivedView,
    ForgeQueryDerivedViewMaintainer, ForgeQueryDerivedViewMaterialization, ForgeQueryMutationDelta,
};
use crate::memory_workspace::ForgeQueryCommitIdentity;
use crate::runtime::ForgeQueryRetainedFieldPath;

pub(super) struct TranscriptMaintainer {
    pub(super) prefix: &'static str,
    pub(super) replace: bool,
}

impl ForgeQueryDerivedViewMaintainer for TranscriptMaintainer {
    fn maintain(
        &mut self,
        view: &ForgeQueryDerivedView,
        delta: &ForgeQueryMutationDelta,
        materialization: &mut ForgeQueryDerivedViewMaterialization,
    ) -> ForgeQueryDerivedPatch {
        let retained_scalars = [
            (
                retained_field_path("family"),
                crate::runtime::ForgeQueryAdmittedAspectValue::native_string_value(
                    self.prefix.to_string(),
                ),
            ),
            (
                retained_field_path("entity"),
                crate::runtime::ForgeQueryAdmittedAspectValue::native_string_value(
                    delta
                        .entity_identity
                        .terminal_projection_for_reporting()
                        .to_string(),
                ),
            ),
            (
                retained_field_path("view"),
                crate::runtime::ForgeQueryAdmittedAspectValue::native_string_value(
                    view.name().to_string(),
                ),
            ),
        ];
        if self.replace {
            materialization
                .replace_retained_scalar_row(retained_scalars.clone())
                .expect("transcript row should admit native scalar values");
        } else {
            materialization
                .push_retained_scalar_row(retained_scalars.clone())
                .expect("transcript row should admit native scalar values");
        }
        ForgeQueryDerivedPatch::incremental(
            view.name(),
            transcript_commit_identity("transcript-derived-commit", self.prefix),
            delta.entity_identity.clone(),
            if view.produced_aspect_touches().is_empty() {
                delta.admitted_touched_aspects().to_vec()
            } else {
                view.produced_aspect_touches().to_vec()
            },
            ForgeQueryDerivedPatchPayload::from_retained_scalar_values(retained_scalars)
                .expect("transcript payload should admit native scalar values"),
        )
    }
}

fn retained_field_path(path: &str) -> ForgeQueryRetainedFieldPath {
    let fields = path
        .split('.')
        .map(|segment| FieldKey::new(segment.to_string()))
        .collect::<Option<Vec<_>>>()
        .expect("transcript retained field path should admit");
    let path = CanonicalFieldPath::new(fields)
        .expect("transcript retained field path should not be empty");
    ForgeQueryRetainedFieldPath::from_canonical_field_path(path)
}

fn transcript_commit_identity(namespace: &str, evidence: &str) -> ForgeQueryCommitIdentity {
    ForgeQueryCommitIdentity::from_relational_commit_id(stable_transcript_position(
        namespace, evidence,
    ))
}

fn stable_transcript_position(namespace: &str, evidence: &str) -> u64 {
    let mut acc = 14_695_981_039_346_656_037_u64;
    for byte in namespace.bytes().chain(evidence.bytes()) {
        acc ^= u64::from(byte);
        acc = acc.wrapping_mul(1_099_511_628_211_u64);
    }
    acc
}
