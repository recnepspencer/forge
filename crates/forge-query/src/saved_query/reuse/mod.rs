mod matrix;

use crate::authoring::ResultShapeFamily;
use crate::composition::{CompositionDigest, ScopeLineageDigest, TemplateBindingDigest};
use crate::identity::SchemaBasisDigest;
use crate::identity_evolution::InspectorIdentityDigest;
use crate::query_context::QueryContextFamily;
use crate::saved_query::future_support::{
    derive_runtime_backed_saved_query_surface_posture, SchemaBasisEquivalenceEvidence,
};
use crate::saved_query::{SavedQueryArtifact, SavedQueryFailureClass};
use crate::view_shape::{ViewShapeDigest, ViewShapeFamily, ViewShapeIdentityConsumption};

use self::matrix::{
    evaluate_basis_family, evaluate_composition_lineage, evaluate_identity_consumption,
    evaluate_result_shape_family, evaluate_schema_basis, evaluate_support_profile,
    evaluate_template_slot_set, evaluate_template_slot_value, evaluate_temporal_async_surface,
    evaluate_view_family, row,
};
pub use self::matrix::{SavedQueryBindingMatrixArtifact, SavedQueryBindingMatrixRow};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SavedQueryRebindingDimension {
    SchemaBasisDigest,
    BasisFamily,
    TemplateSlotValue,
    TemplateSlotSet,
    ViewFamily,
    IdentityConsumption,
    ResultShapeFamily,
    CompositionLineage,
    SupportProfile,
    TemporalAsyncSurface,
}

impl SavedQueryRebindingDimension {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SchemaBasisDigest => "schema_basis_digest",
            Self::BasisFamily => "basis_family",
            Self::TemplateSlotValue => "template_slot_value",
            Self::TemplateSlotSet => "template_slot_set",
            Self::ViewFamily => "view_family",
            Self::IdentityConsumption => "identity_consumption",
            Self::ResultShapeFamily => "result_shape_family",
            Self::CompositionLineage => "composition_lineage",
            Self::SupportProfile => "support_profile",
            Self::TemporalAsyncSurface => "temporal_async_surface",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SavedQueryRebindingLegality {
    LegalNoSemanticChange,
    LegalRequiresFreshFreeze,
    IllegalSemanticDrift,
}

impl SavedQueryRebindingLegality {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LegalNoSemanticChange => "legal_no_semantic_change",
            Self::LegalRequiresFreshFreeze => "legal_requires_fresh_freeze",
            Self::IllegalSemanticDrift => "illegal_semantic_drift",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SavedQueryReuseDescriptor {
    schema_basis_digest: SchemaBasisDigest,
    schema_basis_equivalence: Option<SchemaBasisEquivalenceEvidence>,
    basis_family: Option<QueryContextFamily>,
    template_binding_digest: Option<TemplateBindingDigest>,
    template_slot_count: usize,
    view_shape_digest: ViewShapeDigest,
    view_shape_family: ViewShapeFamily,
    identity_consumption: ViewShapeIdentityConsumption,
    identity_consumption_digest: InspectorIdentityDigest,
    inspector_identity_classification_digest: InspectorIdentityDigest,
    result_shape_family: ResultShapeFamily,
    composition_digest: CompositionDigest,
    scope_lineage_digest: Option<ScopeLineageDigest>,
    support_profile_digest: String,
    capability_family_identity: String,
}

impl SavedQueryReuseDescriptor {
    pub fn new(
        schema_basis_digest: SchemaBasisDigest,
        basis_family: Option<QueryContextFamily>,
        template_binding_digest: Option<TemplateBindingDigest>,
        template_slot_count: usize,
        view_shape_digest: ViewShapeDigest,
        view_shape_family: ViewShapeFamily,
        result_shape_family: ResultShapeFamily,
        composition_digest: CompositionDigest,
        scope_lineage_digest: Option<ScopeLineageDigest>,
        support_profile_digest: impl Into<String>,
        capability_family_identity: impl Into<String>,
    ) -> Self {
        Self {
            schema_basis_digest,
            schema_basis_equivalence: None,
            basis_family,
            template_binding_digest,
            template_slot_count,
            view_shape_digest,
            view_shape_family,
            identity_consumption: ViewShapeIdentityConsumption::none(),
            identity_consumption_digest: InspectorIdentityDigest::from_parts(&[
                "identity_consumption:none".to_string(),
            ]),
            inspector_identity_classification_digest: InspectorIdentityDigest::from_parts(&[
                "identity_classification:none".to_string(),
            ]),
            result_shape_family,
            composition_digest,
            scope_lineage_digest,
            support_profile_digest: support_profile_digest.into(),
            capability_family_identity: capability_family_identity.into(),
        }
    }

    pub fn with_identity_consumption(
        mut self,
        identity_consumption: ViewShapeIdentityConsumption,
    ) -> Self {
        self.identity_consumption_digest = identity_consumption.digest();
        self.inspector_identity_classification_digest =
            InspectorIdentityDigest::from_parts(&[format!(
                "classification:{}",
                identity_consumption
                    .classification()
                    .map(|classification| classification.as_str())
                    .unwrap_or("none")
            )]);
        self.identity_consumption = identity_consumption;
        self
    }

    pub fn with_schema_basis_equivalence(
        mut self,
        evidence: SchemaBasisEquivalenceEvidence,
    ) -> Self {
        self.schema_basis_equivalence = Some(evidence);
        self
    }

    pub(crate) fn schema_basis_digest(&self) -> &SchemaBasisDigest {
        &self.schema_basis_digest
    }

    pub(crate) fn schema_basis_equivalence(&self) -> Option<&SchemaBasisEquivalenceEvidence> {
        self.schema_basis_equivalence.as_ref()
    }

    pub(crate) fn basis_family(&self) -> Option<&QueryContextFamily> {
        self.basis_family.as_ref()
    }

    pub(crate) fn template_binding_digest(&self) -> Option<&TemplateBindingDigest> {
        self.template_binding_digest.as_ref()
    }

    pub(crate) fn template_slot_count(&self) -> usize {
        self.template_slot_count
    }

    pub(crate) fn view_shape_digest(&self) -> &ViewShapeDigest {
        &self.view_shape_digest
    }

    pub(crate) fn view_shape_family(&self) -> ViewShapeFamily {
        self.view_shape_family
    }

    pub(crate) fn identity_consumption(&self) -> &ViewShapeIdentityConsumption {
        &self.identity_consumption
    }

    pub(crate) fn identity_consumption_digest(&self) -> &InspectorIdentityDigest {
        &self.identity_consumption_digest
    }

    pub(crate) fn inspector_identity_classification_digest(&self) -> &InspectorIdentityDigest {
        &self.inspector_identity_classification_digest
    }

    pub(crate) fn result_shape_family(&self) -> &ResultShapeFamily {
        &self.result_shape_family
    }

    pub(crate) fn composition_digest(&self) -> &CompositionDigest {
        &self.composition_digest
    }

    pub(crate) fn scope_lineage_digest(&self) -> Option<&ScopeLineageDigest> {
        self.scope_lineage_digest.as_ref()
    }

    pub(crate) fn support_profile_digest(&self) -> &str {
        &self.support_profile_digest
    }

    pub(crate) fn capability_family_identity(&self) -> &str {
        &self.capability_family_identity
    }

    pub(crate) fn temporal_async_surface_posture(
        &self,
    ) -> crate::saved_query::SavedQueryTemporalAsyncSurfacePosture {
        derive_runtime_backed_saved_query_surface_posture(
            self.basis_family.as_ref(),
            self.view_shape_family,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SavedQueryReuseDecision {
    overall: SavedQueryRebindingLegality,
    matrix: SavedQueryBindingMatrixArtifact,
}

impl SavedQueryReuseDecision {
    pub fn overall(&self) -> SavedQueryRebindingLegality {
        self.overall
    }

    pub fn matrix(&self) -> &SavedQueryBindingMatrixArtifact {
        &self.matrix
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SavedQueryReuseDenial {
    failure_class: SavedQueryFailureClass,
    overall: SavedQueryRebindingLegality,
    matrix: SavedQueryBindingMatrixArtifact,
    message: String,
}

impl SavedQueryReuseDenial {
    pub fn failure_class(&self) -> &SavedQueryFailureClass {
        &self.failure_class
    }

    pub fn overall(&self) -> SavedQueryRebindingLegality {
        self.overall
    }

    pub fn matrix(&self) -> &SavedQueryBindingMatrixArtifact {
        &self.matrix
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SavedQueryReuseOutcome {
    Admitted(SavedQueryReuseDecision),
    Denied(SavedQueryReuseDenial),
}

pub fn evaluate_saved_query_reuse(
    artifact: &SavedQueryArtifact,
    descriptor: &SavedQueryReuseDescriptor,
) -> SavedQueryReuseOutcome {
    let rows = vec![
        evaluate_schema_basis(artifact, descriptor),
        evaluate_basis_family(artifact, descriptor),
        evaluate_template_slot_value(artifact, descriptor),
        evaluate_template_slot_set(artifact, descriptor),
        evaluate_view_family(artifact, descriptor),
        evaluate_identity_consumption(artifact, descriptor),
        evaluate_result_shape_family(artifact, descriptor),
        evaluate_composition_lineage(artifact, descriptor),
        evaluate_support_profile(artifact, descriptor),
        evaluate_temporal_async_surface(artifact, descriptor),
    ];
    let overall = if rows
        .iter()
        .any(|row| row.legality() == SavedQueryRebindingLegality::IllegalSemanticDrift)
    {
        SavedQueryRebindingLegality::IllegalSemanticDrift
    } else if rows
        .iter()
        .any(|row| row.legality() == SavedQueryRebindingLegality::LegalRequiresFreshFreeze)
    {
        SavedQueryRebindingLegality::LegalRequiresFreshFreeze
    } else {
        SavedQueryRebindingLegality::LegalNoSemanticChange
    };
    let matrix = SavedQueryBindingMatrixArtifact::new(rows);

    if overall == SavedQueryRebindingLegality::IllegalSemanticDrift {
        return SavedQueryReuseOutcome::Denied(SavedQueryReuseDenial {
            failure_class: SavedQueryFailureClass::IllegalSemanticDrift,
            overall,
            matrix,
            message: "saved query reuse denied due to illegal semantic drift".to_string(),
        });
    }

    SavedQueryReuseOutcome::Admitted(SavedQueryReuseDecision { overall, matrix })
}

pub(crate) fn binding_row(
    dimension: SavedQueryRebindingDimension,
    legality: SavedQueryRebindingLegality,
    message: impl Into<String>,
) -> SavedQueryBindingMatrixRow {
    row(dimension, legality, message)
}
