use crate::composition::{CompositionDigest, ScopeLineageDigest, TemplateBindingDigest};
use crate::identity::{CanonicalQueryDigest, CanonicalResultShapeDigest, SchemaBasisDigest};
use crate::identity_evolution::{InspectorIdentityClassification, InspectorIdentityDigest};
use crate::query_context::QueryContextFamily;
use crate::saved_query::digest::SavedQueryArtifactDigest;
use crate::saved_query::error::SavedQueryError;
use crate::saved_query::future_support::SavedQueryTemporalAsyncSurfacePosture;
use crate::view_shape::{ViewShapeDigest, ViewShapeFamily, ViewShapeIdentityConsumption};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SavedQueryPersistenceFamily {
    EphemeralProcessOwned,
}

impl SavedQueryPersistenceFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EphemeralProcessOwned => "ephemeral_process_owned",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SavedQueryPersistenceClaim {
    DurableReload,
    ImportExport,
    RestartStableContinuation,
}

impl SavedQueryPersistenceClaim {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DurableReload => "durable_reload",
            Self::ImportExport => "import_export",
            Self::RestartStableContinuation => "restart_stable_continuation",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SavedQueryMetadata {
    canonical_query_digest: CanonicalQueryDigest,
    canonical_result_shape_digest: CanonicalResultShapeDigest,
    composition_digest: CompositionDigest,
    scope_lineage_digest: Option<ScopeLineageDigest>,
    template_binding_digest: Option<TemplateBindingDigest>,
    view_shape_digest: ViewShapeDigest,
    view_shape_family: ViewShapeFamily,
    identity_consumption: ViewShapeIdentityConsumption,
    identity_consumption_digest: InspectorIdentityDigest,
    inspector_identity_classification_digest: InspectorIdentityDigest,
    schema_basis_digest: SchemaBasisDigest,
    basis_family: Option<QueryContextFamily>,
    result_shape_family: crate::authoring::ResultShapeFamily,
    support_profile_digest: String,
    capability_family_identity: String,
    template_slot_count: usize,
    temporal_async_surface_posture: SavedQueryTemporalAsyncSurfacePosture,
}

impl SavedQueryMetadata {
    pub(crate) fn new(
        canonical_query_digest: CanonicalQueryDigest,
        canonical_result_shape_digest: CanonicalResultShapeDigest,
        composition_digest: CompositionDigest,
        scope_lineage_digest: Option<ScopeLineageDigest>,
        template_binding_digest: Option<TemplateBindingDigest>,
        view_shape_digest: ViewShapeDigest,
        view_shape_family: ViewShapeFamily,
        identity_consumption: ViewShapeIdentityConsumption,
        identity_consumption_digest: InspectorIdentityDigest,
        inspector_identity_classification_digest: InspectorIdentityDigest,
        schema_basis_digest: SchemaBasisDigest,
        basis_family: Option<QueryContextFamily>,
        result_shape_family: crate::authoring::ResultShapeFamily,
        support_profile_digest: String,
        capability_family_identity: String,
        template_slot_count: usize,
        temporal_async_surface_posture: SavedQueryTemporalAsyncSurfacePosture,
    ) -> Self {
        Self {
            canonical_query_digest,
            canonical_result_shape_digest,
            composition_digest,
            scope_lineage_digest,
            template_binding_digest,
            view_shape_digest,
            view_shape_family,
            identity_consumption,
            identity_consumption_digest,
            inspector_identity_classification_digest,
            schema_basis_digest,
            basis_family,
            result_shape_family,
            support_profile_digest,
            capability_family_identity,
            template_slot_count,
            temporal_async_surface_posture,
        }
    }

    pub fn canonical_query_digest(&self) -> &CanonicalQueryDigest {
        &self.canonical_query_digest
    }

    pub fn canonical_result_shape_digest(&self) -> &CanonicalResultShapeDigest {
        &self.canonical_result_shape_digest
    }

    pub fn composition_digest(&self) -> &CompositionDigest {
        &self.composition_digest
    }

    pub fn scope_lineage_digest(&self) -> Option<&ScopeLineageDigest> {
        self.scope_lineage_digest.as_ref()
    }

    pub fn template_binding_digest(&self) -> Option<&TemplateBindingDigest> {
        self.template_binding_digest.as_ref()
    }

    pub fn view_shape_digest(&self) -> &ViewShapeDigest {
        &self.view_shape_digest
    }

    pub fn view_shape_family(&self) -> ViewShapeFamily {
        self.view_shape_family
    }

    pub fn schema_basis_digest(&self) -> &SchemaBasisDigest {
        &self.schema_basis_digest
    }

    pub fn identity_consumption(&self) -> &ViewShapeIdentityConsumption {
        &self.identity_consumption
    }

    pub fn identity_consumption_digest(&self) -> &InspectorIdentityDigest {
        &self.identity_consumption_digest
    }

    pub fn inspector_identity_classification(&self) -> Option<InspectorIdentityClassification> {
        self.identity_consumption.classification()
    }

    pub fn inspector_identity_classification_digest(&self) -> &InspectorIdentityDigest {
        &self.inspector_identity_classification_digest
    }

    pub fn basis_family(&self) -> Option<&QueryContextFamily> {
        self.basis_family.as_ref()
    }

    pub fn result_shape_family(&self) -> &crate::authoring::ResultShapeFamily {
        &self.result_shape_family
    }

    pub fn support_profile_digest(&self) -> &str {
        &self.support_profile_digest
    }

    pub fn capability_family_identity(&self) -> &str {
        &self.capability_family_identity
    }

    pub fn template_slot_count(&self) -> usize {
        self.template_slot_count
    }

    pub fn temporal_async_surface_posture(&self) -> SavedQueryTemporalAsyncSurfacePosture {
        self.temporal_async_surface_posture
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SavedQueryArtifact {
    digest: SavedQueryArtifactDigest,
    metadata: SavedQueryMetadata,
    persistence_family: SavedQueryPersistenceFamily,
}

impl SavedQueryArtifact {
    pub(crate) fn new(
        digest: SavedQueryArtifactDigest,
        metadata: SavedQueryMetadata,
        persistence_family: SavedQueryPersistenceFamily,
    ) -> Self {
        Self {
            digest,
            metadata,
            persistence_family,
        }
    }

    pub fn digest(&self) -> &SavedQueryArtifactDigest {
        &self.digest
    }

    pub fn metadata(&self) -> &SavedQueryMetadata {
        &self.metadata
    }

    pub fn persistence_family(&self) -> SavedQueryPersistenceFamily {
        self.persistence_family
    }

    pub fn admit_persistence_claim(
        &self,
        claim: SavedQueryPersistenceClaim,
    ) -> Result<(), SavedQueryError> {
        match (self.persistence_family, claim) {
            (
                SavedQueryPersistenceFamily::EphemeralProcessOwned,
                SavedQueryPersistenceClaim::DurableReload,
            )
            | (
                SavedQueryPersistenceFamily::EphemeralProcessOwned,
                SavedQueryPersistenceClaim::ImportExport,
            )
            | (
                SavedQueryPersistenceFamily::EphemeralProcessOwned,
                SavedQueryPersistenceClaim::RestartStableContinuation,
            ) => Err(SavedQueryError::durable_claim_denied(format!(
                "saved query persistence family '{}' denies '{}'",
                self.persistence_family.as_str(),
                claim.as_str()
            ))),
        }
    }
}
