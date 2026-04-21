use crate::canonicalization::CanonicalQueryBundle;
use crate::composition::{
    ComposedCanonicalQueryBundle, CompositionDigest, CompositionReport, ScopeLineageDigest,
    TemplateBindingDigest,
};
use crate::query_context::QueryContextFamily;
use crate::saved_query::artifact::{
    SavedQueryArtifact, SavedQueryMetadata, SavedQueryPersistenceFamily,
};
use crate::saved_query::digest::SavedQueryArtifactDigest;
use crate::saved_query::error::SavedQueryError;
use crate::view_shape::ViewShapePlanArtifact;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SavedQueryFreezeContext {
    support_profile_digest: String,
    capability_family_identity: String,
}

impl SavedQueryFreezeContext {
    pub fn new(
        support_profile_digest: impl Into<String>,
        capability_family_identity: impl Into<String>,
    ) -> Self {
        Self {
            support_profile_digest: support_profile_digest.into(),
            capability_family_identity: capability_family_identity.into(),
        }
    }

    pub fn support_profile_digest(&self) -> &str {
        &self.support_profile_digest
    }

    pub fn capability_family_identity(&self) -> &str {
        &self.capability_family_identity
    }
}

pub fn freeze_direct_saved_query(
    canonical: &CanonicalQueryBundle,
    view_plan: &ViewShapePlanArtifact,
    freeze_context: SavedQueryFreezeContext,
) -> Result<SavedQueryArtifact, SavedQueryError> {
    let composition_digest = CompositionDigest::from_parts(&[
        "source:direct_canonical".to_string(),
        format!("query:{}", canonical.query().digest().as_str()),
        format!(
            "result_shape:{}",
            canonical.result_shape().digest().as_str()
        ),
    ]);

    build_saved_query_artifact(
        canonical,
        None,
        composition_digest,
        None,
        None,
        view_plan,
        freeze_context,
        0,
    )
}

pub fn freeze_composed_saved_query(
    composed: &ComposedCanonicalQueryBundle,
    view_plan: &ViewShapePlanArtifact,
    freeze_context: SavedQueryFreezeContext,
) -> Result<SavedQueryArtifact, SavedQueryError> {
    let report = composed.composition();
    build_saved_query_artifact(
        composed.canonical(),
        Some(report),
        report.composition_digest().clone(),
        report.scope_lineage_digest().cloned(),
        report.template_binding_digest().cloned(),
        view_plan,
        freeze_context,
        report.counters().template_slot_count(),
    )
}

fn build_saved_query_artifact(
    canonical: &CanonicalQueryBundle,
    composition: Option<&CompositionReport>,
    composition_digest: CompositionDigest,
    scope_lineage_digest: Option<ScopeLineageDigest>,
    template_binding_digest: Option<TemplateBindingDigest>,
    view_plan: &ViewShapePlanArtifact,
    freeze_context: SavedQueryFreezeContext,
    template_slot_count: usize,
) -> Result<SavedQueryArtifact, SavedQueryError> {
    verify_freeze_invariants(canonical, view_plan)?;

    let metadata = SavedQueryMetadata::new(
        canonical.query().digest().clone(),
        canonical.result_shape().digest().clone(),
        composition_digest,
        scope_lineage_digest,
        template_binding_digest,
        view_plan.view_shape_digest().clone(),
        view_plan.family(),
        view_plan.delivery_metadata().identity_consumption().clone(),
        view_plan
            .delivery_metadata()
            .identity_consumption()
            .digest(),
        crate::identity_evolution::InspectorIdentityDigest::from_parts(&[format!(
            "classification:{}",
            view_plan
                .delivery_metadata()
                .identity_consumption()
                .classification()
                .map(|classification| classification.as_str())
                .unwrap_or("none")
        )]),
        view_plan.validated().query().schema_basis().clone(),
        composition.and_then(|report| report.basis_family().cloned()),
        canonical.result_shape().family().clone(),
        freeze_context.support_profile_digest().to_string(),
        freeze_context.capability_family_identity().to_string(),
        template_slot_count,
    );
    let digest = SavedQueryArtifactDigest::from_parts(&[
        format!(
            "canonical_query:{}",
            metadata.canonical_query_digest().as_str()
        ),
        format!(
            "canonical_result_shape:{}",
            metadata.canonical_result_shape_digest().as_str()
        ),
        format!("composition:{}", metadata.composition_digest().as_str()),
        format!("view_shape:{}", metadata.view_shape_digest().as_str()),
        format!(
            "identity_consumption:{}",
            metadata.identity_consumption_digest().as_str()
        ),
        format!(
            "identity_classification:{}",
            metadata.inspector_identity_classification_digest().as_str()
        ),
        format!("schema_basis:{}", metadata.schema_basis_digest().as_str()),
        format!("support:{}", metadata.support_profile_digest()),
        format!("capability:{}", metadata.capability_family_identity()),
        format!("template_slots:{}", metadata.template_slot_count()),
        format!(
            "basis_family:{}",
            metadata
                .basis_family()
                .map(QueryContextFamily::as_str)
                .unwrap_or("none")
        ),
        format!(
            "scope_lineage:{}",
            metadata
                .scope_lineage_digest()
                .map(ScopeLineageDigest::as_str)
                .unwrap_or("none")
        ),
        format!(
            "template_binding:{}",
            metadata
                .template_binding_digest()
                .map(TemplateBindingDigest::as_str)
                .unwrap_or("none")
        ),
    ]);

    Ok(SavedQueryArtifact::new(
        digest,
        metadata,
        SavedQueryPersistenceFamily::EphemeralProcessOwned,
    ))
}

fn verify_freeze_invariants(
    canonical: &CanonicalQueryBundle,
    view_plan: &ViewShapePlanArtifact,
) -> Result<(), SavedQueryError> {
    if canonical.query().digest() != view_plan.canonical().query().digest() {
        return Err(SavedQueryError::freeze_invariant_rejected(
            "saved-query freeze canonical query does not match the view-shaped plan canonical query",
        ));
    }
    if canonical.result_shape().digest() != view_plan.canonical().result_shape().digest() {
        return Err(SavedQueryError::freeze_invariant_rejected(
            "saved-query freeze canonical result shape does not match the view-shaped plan canonical result shape",
        ));
    }
    Ok(())
}
