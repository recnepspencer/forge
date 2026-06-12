use std::collections::BTreeMap;

use crate::authorized_projection::{
    AuthorizedProjectionArtifact, AuthorizedProjectionCounters, MaskedProjectionArtifact,
    PolicyFieldInfluenceSet,
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
use serde_json::json;

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
    let retained_snapshot =
        ForgeQuerySnapshotIdentity::from_external_authority_label("snapshot-retained");
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
    let live_snapshot = ForgeQuerySnapshotIdentity::from_external_authority_label("snapshot-live");
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
                            ForgeQueryEntityIdentity::authored_command("entity-1"),
                            json!({"profile": {"display_name": "First"}}),
                        ),
                        ForgeQueryEntity::from_external_projection(
                            ForgeQueryEntityIdentity::authored_command("entity-2"),
                            json!({"profile": {"display_name": "Second"}}),
                        ),
                    ],
                    ForgeQueryLiveReadReceipt::test_only(
                        first.view_name(),
                        "installation:first",
                        "query:test",
                        "shape:first",
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
                        ForgeQueryEntityIdentity::authored_command("entity-3"),
                        json!({"profile": {"display_name": "Third"}}),
                    )],
                    ForgeQueryLiveReadReceipt::test_only(
                        second.view_name(),
                        "installation:second",
                        "query:test",
                        "shape:second",
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
