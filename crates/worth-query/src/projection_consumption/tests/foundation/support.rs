use super::super::super::{
    ProjectMaterializedFacts, ProjectionConsumptionBindingContext, ProjectionConsumptionSource,
    ProjectionConsumptionWarnings, ProjectionFactKind, ProjectionSourceFamily,
};

pub(super) fn test_binding(visible_fields: &[&str]) -> ProjectionConsumptionBindingContext {
    ProjectionConsumptionBindingContext::test_only(
        "result-shape:test",
        "authorized-projection:test",
        crate::projection_consumption::test_authorized_field_paths(visible_fields),
    )
}

pub(super) fn test_binding_with_projection_metadata(
    result_shape_digest: &str,
    query_digest: &str,
    visible_fields: &[&str],
) -> ProjectionConsumptionBindingContext {
    ProjectionConsumptionBindingContext::test_only_with_projection_metadata(
        result_shape_digest,
        query_digest,
        result_shape_digest,
        "authorized-projection:test",
        "narrowed-result-shape:test",
        "policy:test",
        "tenant-schema:test",
        crate::projection_consumption::test_authorized_field_paths(visible_fields),
    )
}

pub(super) fn test_source(family: ProjectionSourceFamily) -> ProjectionConsumptionSource {
    match family {
        ProjectionSourceFamily::QueryReadReceipt => ProjectionConsumptionSource::test_only(
            family,
            Some("query:test"),
            Some("basis:test"),
            Some("result:test"),
            Some("result-shape:test"),
            "read-graph:test",
        ),
        ProjectionSourceFamily::QueryLiveReadReceipt => ProjectionConsumptionSource::test_only(
            family,
            Some("query:test"),
            Some("snapshot:test"),
            Some("result:test"),
            Some("result-shape:test"),
            "installation:test",
        ),
        ProjectionSourceFamily::QueryWriteReceipt => ProjectionConsumptionSource::test_only(
            family,
            None,
            Some("snapshot:test"),
            None,
            None,
            "commit:test",
        ),
        ProjectionSourceFamily::QueryContextExecution => ProjectionConsumptionSource::test_only(
            family,
            Some("query:test"),
            Some("basis:test"),
            Some("result:test"),
            Some("result-shape:test"),
            "query-context:test",
        ),
        ProjectionSourceFamily::RelationalRowSet => ProjectionConsumptionSource::test_only(
            family,
            None,
            Some("snapshot:test"),
            None,
            None,
            "relational-row-set:test",
        ),
        ProjectionSourceFamily::RelationalGroupedProjection => {
            ProjectionConsumptionSource::test_only(
                family,
                None,
                Some("snapshot:test"),
                None,
                None,
                "relational-grouped-projection:test",
            )
        }
        ProjectionSourceFamily::BridgeTruthViewRowSet => ProjectionConsumptionSource::test_only(
            family,
            None,
            Some("snapshot:test"),
            None,
            None,
            "bridge-row-set:test",
        ),
        ProjectionSourceFamily::BridgeGroupedTruthView => ProjectionConsumptionSource::test_only(
            family,
            None,
            Some("snapshot:test"),
            None,
            None,
            "bridge-grouped-truth-view:test",
        ),
        ProjectionSourceFamily::RetainedDerivedArtifactBinding => {
            ProjectionConsumptionSource::test_only(
                family,
                None,
                Some("snapshot:test"),
                None,
                None,
                "retained-binding:test",
            )
        }
        ProjectionSourceFamily::LiveArtifactBinding => ProjectionConsumptionSource::test_only(
            family,
            None,
            Some("snapshot:test"),
            None,
            None,
            "live-binding:test",
        ),
    }
}

pub(super) fn all_source_families() -> [ProjectionSourceFamily; 4] {
    [
        ProjectionSourceFamily::QueryReadReceipt,
        ProjectionSourceFamily::QueryLiveReadReceipt,
        ProjectionSourceFamily::QueryWriteReceipt,
        ProjectionSourceFamily::QueryContextExecution,
    ]
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
                worth_foundational::facade::FieldKey::new("profile")
                    .expect("projection fact field segment should admit"),
                worth_foundational::facade::FieldKey::new("display_name")
                    .expect("projection fact field segment should admit"),
            ]),
        ),
        ProjectionFactKind::DerivedField => ProjectMaterializedFacts::declare().derived_field_path(
            crate::projection_consumption::projection_fact_field_path_from_segments([
                worth_foundational::facade::FieldKey::new("profile")
                    .expect("projection fact field segment should admit"),
                worth_foundational::facade::FieldKey::new("display_name")
                    .expect("projection fact field segment should admit"),
            ]),
        ),
    }
}

pub(super) fn visible_fields_for_kind(kind: ProjectionFactKind) -> Vec<&'static str> {
    match kind {
        ProjectionFactKind::DisplayField | ProjectionFactKind::DerivedField => {
            vec!["profile.display_name"]
        }
        _ => vec!["identity.id"],
    }
}

pub(super) fn assert_warning_matches_posture(
    warnings: &ProjectionConsumptionWarnings,
    expected: crate::projection_consumption::ProjectionConsumptionWarningKind,
) {
    assert_eq!(warnings.warning_kinds(), [expected]);
}
