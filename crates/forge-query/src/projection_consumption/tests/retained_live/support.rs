use std::sync::OnceLock;

use std::collections::BTreeMap;

use crate::authorized_projection::{
    AuthorizedProjectionArtifact, AuthorizedProjectionCounters, MaskedProjectionArtifact,
    PolicyFieldInfluenceSet,
};
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::memory_workspace::{
    ForgeQueryEntity, ForgeQueryEntityIdentity, ForgeQuerySnapshotIdentity,
};
use crate::projection_consumption::{ProjectMaterializedFacts, ProjectionFactKind};
use crate::runtime::{
    ForgeQueryDerivedMaterializationBundle, ForgeQueryDerivedMaterializationReceipt,
    ForgeQueryDerivedMaterializationResult, ForgeQueryDerivedMaterializationTarget,
    ForgeQueryLiveArtifactBundle, ForgeQueryLiveArtifactTarget, ForgeQueryLiveReadReceipt,
    ForgeQueryLiveReadResult,
};
use forge_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts;
use serde_json::json;

pub(super) struct SharedTestResultShape {
    pub identity: ForgeQueryEvidenceIdentity,
    pub digest: String,
}

pub(super) fn shared_test_result_shape() -> &'static SharedTestResultShape {
    static SHARED: OnceLock<SharedTestResultShape> = OnceLock::new();
    SHARED.get_or_init(|| {
        let identity = test_result_shape_identity("result-shape:test");
        let digest = identity.as_str().to_string();
        SharedTestResultShape { identity, digest }
    })
}

pub(super) fn result_shape_identity_for_test(label: &str) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "projection_test_result_shape_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("label"), label)
        .seal()
}

pub(super) fn test_result_shape_identity(label: &str) -> ForgeQueryEvidenceIdentity {
    test_result_shape_artifact(label).result_shape_identity()
}

pub(super) fn test_result_shape_artifact(
    label: &str,
) -> crate::canonicalization::CanonicalResultShapeArtifact {
    use crate::authoring::ResultShapeFamily;
    use crate::canonicalization::CanonicalResultShapeArtifact;
    use crate::identity::CanonicalResultShapeDigest;

    CanonicalResultShapeArtifact {
        digest: CanonicalResultShapeDigest::from_parts(&[label.to_string()]),
        family: ResultShapeFamily::Detail,
        fields: Vec::new(),
    }
}

pub(super) fn test_result_shape_canonical_digest(label: &str) -> String {
    test_result_shape_artifact(label)
        .digest()
        .as_str()
        .to_string()
}

pub(super) fn test_result_shape_digest(label: &str) -> String {
    test_result_shape_identity(label).as_str().to_string()
}

pub(super) fn authorized_projection(
    query_digest: &str,
    result_shape_digest: &str,
    visible_fields: &[&str],
) -> AuthorizedProjectionArtifact {
    AuthorizedProjectionArtifact::new(
        query_digest,
        result_shape_digest,
        "policy:test",
        "tenant-schema:test",
        visible_fields
            .iter()
            .map(|field| field.to_string())
            .collect(),
        MaskedProjectionArtifact::new(Vec::new(), Vec::new()),
        "narrowed-result-shape:test".to_string(),
        PolicyFieldInfluenceSet::new(&["influence:test".to_string()], 1),
        AuthorizedProjectionCounters::default(),
    )
}

pub(super) fn retained_binding() -> crate::runtime::ForgeQueryDerivedArtifactBinding {
    let retained_snapshot = retained_live_snapshot_identity("snapshot-retained");
    let first = ForgeQueryDerivedMaterializationTarget::new("derived.first");
    let second = ForgeQueryDerivedMaterializationTarget::new("derived.second");
    let bundle = ForgeQueryDerivedMaterializationBundle::test_only(
        retained_snapshot.clone(),
        BTreeMap::from([
            (
                first.view_name().to_string(),
                ForgeQueryDerivedMaterializationResult::test_only(
                    vec![
                        json!({"profile": {"display_name": "First"}, "metrics": {"priority": 1}}),
                        json!({"profile": {"display_name": "Second"}, "metrics": {"priority": 2}}),
                    ],
                    ForgeQueryDerivedMaterializationReceipt::test_only(
                        first.view_name(),
                        retained_snapshot.clone(),
                        "derived-first-digest",
                    ),
                ),
            ),
            (
                second.view_name().to_string(),
                ForgeQueryDerivedMaterializationResult::test_only(
                    vec![json!({"profile": {"display_name": "Third"}})],
                    ForgeQueryDerivedMaterializationReceipt::test_only(
                        second.view_name(),
                        retained_snapshot.clone(),
                        "derived-second-digest",
                    ),
                ),
            ),
        ]),
    );

    bundle
        .bind_retained_artifact("retained.binding", [first.clone(), second.clone()])
        .expect("retained binding should succeed")
}

pub(super) fn live_binding() -> crate::runtime::ForgeQueryLiveArtifactBinding {
    let live_snapshot = retained_live_snapshot_identity("snapshot-live");
    let first = ForgeQueryLiveArtifactTarget::new("live.first");
    let second = ForgeQueryLiveArtifactTarget::new("live.second");
    let bundle = ForgeQueryLiveArtifactBundle::test_only(
        live_snapshot.clone(),
        BTreeMap::from([
            (
                first.view_name().to_string(),
                ForgeQueryLiveReadResult::test_only(
                    vec![
                        ForgeQueryEntity::from_external_projection(
                            crate::memory_workspace::admit_authored_entity_label("entity-1"),
                            json!({"profile": {"display_name": "First"}}),
                        ),
                        ForgeQueryEntity::from_external_projection(
                            crate::memory_workspace::admit_authored_entity_label("entity-2"),
                            json!({"profile": {"display_name": "Second"}}),
                        ),
                    ],
                    ForgeQueryLiveReadReceipt::test_only(
                        first.view_name(),
                        "installation:first",
                        "query:test",
                        test_result_shape_artifact("shape:first").digest().clone(),
                        "subscription:first",
                        "result:first",
                        live_snapshot.clone(),
                        2,
                    ),
                ),
            ),
            (
                second.view_name().to_string(),
                ForgeQueryLiveReadResult::test_only(
                    vec![ForgeQueryEntity::from_external_projection(
                        crate::memory_workspace::admit_authored_entity_label("entity-3"),
                        json!({"profile": {"display_name": "Third"}}),
                    )],
                    ForgeQueryLiveReadReceipt::test_only(
                        second.view_name(),
                        "installation:second",
                        "query:test",
                        test_result_shape_artifact("shape:second").digest().clone(),
                        "subscription:second",
                        "result:second",
                        live_snapshot.clone(),
                        1,
                    ),
                ),
            ),
        ]),
    );

    bundle
        .bind_live_artifact("live.binding", [first.clone(), second.clone()])
        .expect("live binding should succeed")
}

fn retained_live_snapshot_identity(label: &str) -> ForgeQuerySnapshotIdentity {
    ForgeQuerySnapshotIdentity::from_relational_snapshot(
        RelationalBridgeSnapshotIdentityParts::new(
            retained_live_fixture_position("snapshot", label),
            retained_live_fixture_position("snapshot-version", label),
        ),
    )
}

fn retained_live_fixture_position(namespace: &str, evidence: &str) -> u64 {
    let mut acc = 14_695_981_039_346_656_037_u64;
    for byte in namespace.bytes().chain(evidence.bytes()) {
        acc ^= u64::from(byte);
        acc = acc.wrapping_mul(1_099_511_628_211_u64);
    }
    acc
}

pub(super) fn request_for_kind(kind: ProjectionFactKind) -> ProjectMaterializedFacts {
    match kind {
        ProjectionFactKind::EntityIdentity => {
            ProjectMaterializedFacts::declare().entity_identities()
        }
        ProjectionFactKind::ViewLocalIdentity => {
            ProjectMaterializedFacts::declare().view_local_identities()
        }
        ProjectionFactKind::TargetIdentity => ProjectMaterializedFacts::declare().target_identity(),
        ProjectionFactKind::SourceReference => {
            ProjectMaterializedFacts::declare().source_references()
        }
        ProjectionFactKind::EffectContinuity => {
            ProjectMaterializedFacts::declare().effect_continuity_facts()
        }
        ProjectionFactKind::Membership => ProjectMaterializedFacts::declare().memberships(),
        ProjectionFactKind::RelationEndpoint => {
            ProjectMaterializedFacts::declare().relation_endpoints()
        }
        ProjectionFactKind::DisplayField => {
            ProjectMaterializedFacts::declare().display_field("profile.display_name")
        }
        ProjectionFactKind::DerivedScalarField => {
            ProjectMaterializedFacts::declare().derived_scalar_field("profile.display_name")
        }
    }
}

pub(super) fn visible_fields_for_kind(kind: ProjectionFactKind) -> Vec<&'static str> {
    match kind {
        ProjectionFactKind::DisplayField | ProjectionFactKind::DerivedScalarField => {
            vec!["profile.display_name"]
        }
        _ => vec!["identity.id"],
    }
}
