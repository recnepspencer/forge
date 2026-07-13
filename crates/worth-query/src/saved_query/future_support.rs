use super::artifact::SavedQueryPersistenceFamily;
use super::error::SavedQueryError;
use crate::query_context::QueryContextFamily;
use crate::view_shape::{
    runtime_backed_view_shape_temporal_async_support_posture, ViewShapeFamily,
    ViewShapeTemporalAsyncSupportPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SavedQueryTemporalAsyncSurfacePosture {
    OrdinaryOnly,
    FuturePreservingRuntimeBacked,
    VisibleButDeferred,
}

impl SavedQueryTemporalAsyncSurfacePosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OrdinaryOnly => "ordinary_only",
            Self::FuturePreservingRuntimeBacked => "future_preserving_runtime_backed",
            Self::VisibleButDeferred => "visible_but_deferred",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SchemaBasisEquivalenceEvidence {
    same_admitted_basis_family: bool,
    same_projection_legality_surface: bool,
}

impl SchemaBasisEquivalenceEvidence {
    pub fn explicit_same_surface() -> Self {
        Self {
            same_admitted_basis_family: true,
            same_projection_legality_surface: true,
        }
    }

    pub fn same_admitted_basis_family(&self) -> bool {
        self.same_admitted_basis_family
    }

    pub fn same_projection_legality_surface(&self) -> bool {
        self.same_projection_legality_surface
    }
}

pub fn runtime_backed_saved_query_future_support_profile() -> Vec<(
    SavedQueryPersistenceFamily,
    SavedQueryTemporalAsyncSurfacePosture,
)> {
    vec![(
        SavedQueryPersistenceFamily::EphemeralProcessOwned,
        SavedQueryTemporalAsyncSurfacePosture::FuturePreservingRuntimeBacked,
    )]
}

pub(crate) fn derive_runtime_backed_saved_query_surface_posture(
    basis_family: Option<&QueryContextFamily>,
    view_shape_family: ViewShapeFamily,
) -> SavedQueryTemporalAsyncSurfacePosture {
    if basis_family.is_none() {
        return SavedQueryTemporalAsyncSurfacePosture::OrdinaryOnly;
    }

    match runtime_backed_view_shape_temporal_async_support_posture(view_shape_family) {
        ViewShapeTemporalAsyncSupportPosture::FuturePreserving => {
            SavedQueryTemporalAsyncSurfacePosture::FuturePreservingRuntimeBacked
        }
        ViewShapeTemporalAsyncSupportPosture::VisibleButDeferred => {
            SavedQueryTemporalAsyncSurfacePosture::VisibleButDeferred
        }
    }
}

pub(crate) fn admit_runtime_backed_future_surface_posture(
    persistence_family: SavedQueryPersistenceFamily,
    posture: SavedQueryTemporalAsyncSurfacePosture,
) -> Result<(), SavedQueryError> {
    match (persistence_family, posture) {
        (_, SavedQueryTemporalAsyncSurfacePosture::OrdinaryOnly)
        | (
            SavedQueryPersistenceFamily::EphemeralProcessOwned,
            SavedQueryTemporalAsyncSurfacePosture::FuturePreservingRuntimeBacked,
        ) => Ok(()),
        (_, SavedQueryTemporalAsyncSurfacePosture::VisibleButDeferred) => {
            Err(SavedQueryError::temporal_async_surface_deferred(
                "saved query freeze denied because this temporal/async reuse surface remains visible but deferred in Milestone 9.4",
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authoring::{
        AspectFieldSelector, AuthoredResultShapeField, GuidedAuthoringPath, RootEntityKey,
    };
    use crate::composition::{GuidedCompositionPath, QueryScopeDescriptor};
    use crate::harness::fixtures::execution_preflights;
    use crate::query_context::{
        admit_and_scope_legacy_query_basis_context_for_test, bind_legacy_query_basis_context,
        QueryBasisContextRequest, QueryContextBindingSource,
    };
    use crate::saved_query::{
        evaluate_saved_query_reuse, SavedQueryFailureClass, SavedQueryFreezeContext,
        SavedQueryReuseDescriptor, SavedQueryReuseOutcome,
    };
    use crate::view_shape::{
        admit_view_shape, plan_admitted_view_shape,
        validate_canonical_bundle_for_admitted_view_shape, ViewShapeDescriptor,
    };

    #[test]
    fn runtime_backed_saved_query_future_support_is_explicit() {
        let profile = runtime_backed_saved_query_future_support_profile();
        assert_eq!(profile.len(), 1);
        assert_eq!(
            profile[0],
            (
                SavedQueryPersistenceFamily::EphemeralProcessOwned,
                SavedQueryTemporalAsyncSurfacePosture::FuturePreservingRuntimeBacked,
            )
        );
    }

    fn detail_schema_view() -> crate::schema_view::QuerySchemaView {
        crate::schema_view::QuerySchemaView::new(
            "saved-query-future-support-detail",
            [
                crate::schema_view::SchemaFieldView::new(
                    crate::authoring::AspectName::new("identity")
                        .expect("schema aspect literal must be valid"),
                    crate::authoring::FieldName::new("id")
                        .expect("schema field literal must be valid"),
                    crate::schema_view::SchemaFieldKind::String,
                ),
                crate::schema_view::SchemaFieldView::new(
                    crate::authoring::AspectName::new("profile")
                        .expect("schema aspect literal must be valid"),
                    crate::authoring::FieldName::new("display_name")
                        .expect("schema field literal must be valid"),
                    crate::schema_view::SchemaFieldKind::String,
                )
                .text_predicate_queryable(),
            ],
            [],
        )
    }

    fn direct_detail() -> crate::canonicalization::CanonicalQueryBundle {
        GuidedAuthoringPath::canonicalize_detail(
            crate::authoring::RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
                .project(AspectFieldSelector::new("identity", "id").unwrap())
                .project(AspectFieldSelector::new("profile", "display_name").unwrap())
                .build()
                .unwrap(),
            crate::authoring::RawAuthoredResultShape::detail_builder()
                .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
                .field(
                    AuthoredResultShapeField::new("profile", "display_name", "display_name")
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .unwrap()
    }

    fn basis_intent() -> crate::basis::ExecutionBasisIntent {
        crate::basis::ExecutionBasisIntent::new(
            crate::basis::BasisAuthorityFamily::Runtime,
            crate::basis::SnapshotLineageClass::CurrentHead,
            false,
        )
    }

    fn basis_aware_composed_detail() -> crate::composition::ComposedCanonicalQueryBundle {
        let preflight = execution_preflights::direct_runtime_preflight();
        let binding = bind_legacy_query_basis_context(
            QueryBasisContextRequest::current_branch_head(),
            QueryContextBindingSource::RuntimeCurrent(&preflight),
        )
        .unwrap();
        let admitted = admit_and_scope_legacy_query_basis_context_for_test(binding).unwrap();
        let direct = direct_detail();
        let evidence =
            crate::composition::BasisScopeEvidence::from_admitted_context_for_canonical_query(
                &admitted,
                direct.query().digest(),
            );
        let (_artifact, expanded) = GuidedCompositionPath::expand_detail_scopes(
            crate::authoring::RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
                .project(AspectFieldSelector::new("identity", "id").unwrap())
                .project(AspectFieldSelector::new("profile", "display_name").unwrap())
                .build()
                .unwrap(),
            crate::authoring::RawAuthoredResultShape::detail_builder()
                .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
                .field(
                    AuthoredResultShapeField::new("profile", "display_name", "display_name")
                        .unwrap(),
                )
                .build()
                .unwrap(),
            [QueryScopeDescriptor::basis_aware("current_basis", evidence)],
        )
        .unwrap();

        GuidedCompositionPath::canonicalize_expanded(expanded).unwrap()
    }

    #[test]
    fn runtime_backed_future_preserving_saved_query_freeze_retains_surface_posture() {
        let composed = basis_aware_composed_detail();
        let view = plan_admitted_view_shape(
            validate_canonical_bundle_for_admitted_view_shape(
                composed.canonical(),
                detail_schema_view(),
                admit_view_shape(composed.canonical(), ViewShapeDescriptor::detail()).unwrap(),
            )
            .unwrap(),
            basis_intent(),
        )
        .unwrap();

        let saved = crate::saved_query::freeze_composed_saved_query(
            &composed,
            &view,
            SavedQueryFreezeContext::new("test-support", "query_composition"),
        )
        .unwrap();

        assert_eq!(
            saved.metadata().temporal_async_surface_posture(),
            SavedQueryTemporalAsyncSurfacePosture::FuturePreservingRuntimeBacked
        );
    }

    #[test]
    fn deferred_temporal_async_saved_query_surface_denies_freeze_and_reuse_drift() {
        let composed = basis_aware_composed_detail();
        let deferred_view = plan_admitted_view_shape(
            validate_canonical_bundle_for_admitted_view_shape(
                composed.canonical(),
                detail_schema_view(),
                admit_view_shape(
                    composed.canonical(),
                    ViewShapeDescriptor::inspector_detail_focused(
                        worth_foundational::facade::AspectKey::new("profile").unwrap(),
                    ),
                )
                .unwrap(),
            )
            .unwrap(),
            basis_intent(),
        )
        .unwrap();

        let error = crate::saved_query::freeze_composed_saved_query(
            &composed,
            &deferred_view,
            SavedQueryFreezeContext::new("test-support", "query_composition"),
        )
        .unwrap_err();
        assert_eq!(
            error.failure_class(),
            &SavedQueryFailureClass::TemporalAsyncSurfaceDeferred
        );

        let detail_view = plan_admitted_view_shape(
            validate_canonical_bundle_for_admitted_view_shape(
                composed.canonical(),
                detail_schema_view(),
                admit_view_shape(composed.canonical(), ViewShapeDescriptor::detail()).unwrap(),
            )
            .unwrap(),
            basis_intent(),
        )
        .unwrap();

        let saved = crate::saved_query::freeze_composed_saved_query(
            &composed,
            &detail_view,
            SavedQueryFreezeContext::new("test-support", "query_composition"),
        )
        .unwrap();
        let descriptor = SavedQueryReuseDescriptor::new(
            saved.metadata().schema_basis_digest().clone(),
            None,
            saved.metadata().template_binding_digest().cloned(),
            saved.metadata().template_slot_count(),
            saved.metadata().view_shape_digest().clone(),
            saved.metadata().view_shape_family(),
            saved.metadata().result_shape_family().clone(),
            saved.metadata().composition_digest().clone(),
            saved.metadata().scope_lineage_digest().cloned(),
            saved.metadata().support_profile_digest().to_string(),
            saved.metadata().capability_family_identity().to_string(),
        );

        let outcome = evaluate_saved_query_reuse(&saved, &descriptor);
        let SavedQueryReuseOutcome::Denied(denial) = outcome else {
            panic!("temporal/async surface drift should deny reuse");
        };
        assert_eq!(
            denial.failure_class(),
            &SavedQueryFailureClass::IllegalSemanticDrift
        );
    }
}
