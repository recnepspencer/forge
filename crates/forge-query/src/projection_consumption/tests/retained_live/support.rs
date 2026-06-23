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
use crate::memory_workspace::{ForgeQueryEntity, ForgeQuerySnapshotIdentity};
use crate::projection_consumption::{ProjectMaterializedFacts, ProjectionFactKind};
use crate::runtime::{
    ForgeQueryDerivedMaterializationBundle, ForgeQueryDerivedMaterializationReceipt,
    ForgeQueryDerivedMaterializationResult, ForgeQueryDerivedMaterializationTarget,
    ForgeQueryLiveArtifactBundle, ForgeQueryLiveArtifactTarget, ForgeQueryLiveReadReceipt,
    ForgeQueryLiveReadResult, ForgeQueryRetainedFieldPath, ForgeQueryRetainedMaterializedRow,
};
use forge_foundational::facade::{AspectValue, CanonicalFieldPath, FieldKey};
use forge_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts;

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

#[allow(dead_code)]
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

#[allow(dead_code)]
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
        crate::projection_consumption::test_authorized_field_paths(visible_fields),
        MaskedProjectionArtifact::new(Vec::new(), Vec::new()),
        "narrowed-result-shape:test".to_string(),
        PolicyFieldInfluenceSet::new(&["influence:test".to_string()], 1),
        AuthorizedProjectionCounters::default(),
    )
}

pub(super) fn retained_binding() -> crate::runtime::ForgeQueryDerivedArtifactBinding {
    let retained_snapshot = retained_live_snapshot_identity("snapshot-retained");
    let first = ForgeQueryDerivedMaterializationTarget::test_only("derived.first");
    let second = ForgeQueryDerivedMaterializationTarget::test_only("derived.second");
    let bundle = ForgeQueryDerivedMaterializationBundle::test_only(
        retained_snapshot.clone(),
        BTreeMap::from([
            (
                first.clone(),
                ForgeQueryDerivedMaterializationResult::test_only_retained_rows(
                    vec![
                        retained_materialized_row([
                            ("profile.display_name", text_value("First")),
                            ("metrics.priority", AspectValue::Int64(1)),
                        ]),
                        retained_materialized_row([
                            ("profile.display_name", text_value("Second")),
                            ("metrics.priority", AspectValue::Int64(2)),
                        ]),
                    ],
                    ForgeQueryDerivedMaterializationReceipt::test_only(
                        first.view_name(),
                        retained_snapshot.clone(),
                        "derived-first-digest",
                    ),
                ),
            ),
            (
                second.clone(),
                ForgeQueryDerivedMaterializationResult::test_only_retained_rows(
                    vec![retained_materialized_row([(
                        "profile.display_name",
                        text_value("Third"),
                    )])],
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
    let first = ForgeQueryLiveArtifactTarget::test_only("live.first");
    let second = ForgeQueryLiveArtifactTarget::test_only("live.second");
    let bundle = ForgeQueryLiveArtifactBundle::test_only(
        live_snapshot.clone(),
        BTreeMap::from([
            (
                first.clone(),
                ForgeQueryLiveReadResult::test_only(
                    vec![
                        ForgeQueryEntity::from_native_field_values(
                            crate::memory_workspace::admit_authored_entity_label("entity-1"),
                            projection_values([("profile.display_name", text_value("First"))]),
                        ),
                        ForgeQueryEntity::from_native_field_values(
                            crate::memory_workspace::admit_authored_entity_label("entity-2"),
                            projection_values([("profile.display_name", text_value("Second"))]),
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
                second.clone(),
                ForgeQueryLiveReadResult::test_only(
                    vec![ForgeQueryEntity::from_native_field_values(
                        crate::memory_workspace::admit_authored_entity_label("entity-3"),
                        projection_values([("profile.display_name", text_value("Third"))]),
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

fn projection_values(
    values: impl IntoIterator<Item = (&'static str, AspectValue)>,
) -> BTreeMap<CanonicalFieldPath, AspectValue> {
    values
        .into_iter()
        .map(|(path, value)| (canonical_field_path(path), value))
        .collect()
}

fn retained_materialized_row(
    values: impl IntoIterator<Item = (&'static str, AspectValue)>,
) -> ForgeQueryRetainedMaterializedRow {
    let values = values
        .into_iter()
        .map(|(path, value)| (retained_field_path(path), value))
        .collect();
    ForgeQueryRetainedMaterializedRow::from_scalar_values(values)
        .expect("test retained materialized row should admit")
}

fn text_value(value: impl Into<String>) -> AspectValue {
    crate::runtime::ForgeQueryAdmittedAspectValue::native_string_value(value)
}

fn retained_field_path(path: &str) -> ForgeQueryRetainedFieldPath {
    ForgeQueryRetainedFieldPath::from_canonical_field_path(canonical_field_path(path))
}

fn canonical_field_path(path: &str) -> CanonicalFieldPath {
    CanonicalFieldPath::new(
        path.split('.')
            .map(|segment| FieldKey::new(segment.to_string()))
            .collect::<Option<Vec<_>>>()
            .expect("test field path should be canonical"),
    )
    .expect("test field path should not be empty")
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
        ProjectionFactKind::DisplayField => ProjectMaterializedFacts::declare().display_field_path(
            crate::projection_consumption::projection_fact_field_path_from_segments([
                forge_foundational::facade::FieldKey::new("profile")
                    .expect("projection fact field segment should admit"),
                forge_foundational::facade::FieldKey::new("display_name")
                    .expect("projection fact field segment should admit"),
            ]),
        ),
        ProjectionFactKind::DerivedScalarField => ProjectMaterializedFacts::declare()
            .derived_scalar_field_path(
                crate::projection_consumption::projection_fact_field_path_from_segments([
                    forge_foundational::facade::FieldKey::new("profile")
                        .expect("projection fact field segment should admit"),
                    forge_foundational::facade::FieldKey::new("display_name")
                        .expect("projection fact field segment should admit"),
                ]),
            ),
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
