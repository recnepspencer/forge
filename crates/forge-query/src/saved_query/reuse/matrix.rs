use crate::identity::hash_parts;
use crate::saved_query::{SavedQueryArtifact, SavedQueryReuseDescriptor};

use super::{binding_row, SavedQueryRebindingDimension, SavedQueryRebindingLegality};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SavedQueryBindingMatrixArtifact {
    rows: Vec<SavedQueryBindingMatrixRow>,
    digest: String,
}

impl SavedQueryBindingMatrixArtifact {
    pub fn new(rows: Vec<SavedQueryBindingMatrixRow>) -> Self {
        let digest = hash_parts(
            &rows
                .iter()
                .map(|row| {
                    format!(
                        "{}:{}:{}",
                        row.dimension().as_str(),
                        row.legality().as_str(),
                        row.message()
                    )
                })
                .collect::<Vec<_>>(),
        );
        Self { rows, digest }
    }

    pub fn rows(&self) -> &[SavedQueryBindingMatrixRow] {
        &self.rows
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SavedQueryBindingMatrixRow {
    dimension: SavedQueryRebindingDimension,
    legality: SavedQueryRebindingLegality,
    message: String,
}

impl SavedQueryBindingMatrixRow {
    pub fn dimension(&self) -> SavedQueryRebindingDimension {
        self.dimension
    }

    pub fn legality(&self) -> SavedQueryRebindingLegality {
        self.legality
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

pub fn row(
    dimension: SavedQueryRebindingDimension,
    legality: SavedQueryRebindingLegality,
    message: impl Into<String>,
) -> SavedQueryBindingMatrixRow {
    SavedQueryBindingMatrixRow {
        dimension,
        legality,
        message: message.into(),
    }
}

pub fn evaluate_schema_basis(
    artifact: &SavedQueryArtifact,
    descriptor: &SavedQueryReuseDescriptor,
) -> SavedQueryBindingMatrixRow {
    if artifact.metadata().schema_basis_digest() == descriptor.schema_basis_digest() {
        return binding_row(
            SavedQueryRebindingDimension::SchemaBasisDigest,
            SavedQueryRebindingLegality::LegalNoSemanticChange,
            "schema basis digest is unchanged",
        );
    }
    match descriptor.schema_basis_equivalence() {
        Some(evidence)
            if evidence.same_admitted_basis_family()
                && evidence.same_projection_legality_surface() =>
        {
            binding_row(
                SavedQueryRebindingDimension::SchemaBasisDigest,
                SavedQueryRebindingLegality::LegalNoSemanticChange,
                "explicit schema-basis equivalence evidence preserves the admitted projection legality surface",
            )
        }
        _ => binding_row(
            SavedQueryRebindingDimension::SchemaBasisDigest,
            SavedQueryRebindingLegality::IllegalSemanticDrift,
            "schema basis digest changed without explicit equivalence evidence",
        ),
    }
}

pub fn evaluate_basis_family(
    artifact: &SavedQueryArtifact,
    descriptor: &SavedQueryReuseDescriptor,
) -> SavedQueryBindingMatrixRow {
    if artifact.metadata().basis_family() == descriptor.basis_family() {
        return binding_row(
            SavedQueryRebindingDimension::BasisFamily,
            SavedQueryRebindingLegality::LegalNoSemanticChange,
            "basis family is unchanged",
        );
    }
    binding_row(
        SavedQueryRebindingDimension::BasisFamily,
        SavedQueryRebindingLegality::IllegalSemanticDrift,
        "basis family changes are always semantic drift in Milestone 8",
    )
}

pub fn evaluate_template_slot_value(
    artifact: &SavedQueryArtifact,
    descriptor: &SavedQueryReuseDescriptor,
) -> SavedQueryBindingMatrixRow {
    if artifact.metadata().template_binding_digest() == descriptor.template_binding_digest() {
        return binding_row(
            SavedQueryRebindingDimension::TemplateSlotValue,
            SavedQueryRebindingLegality::LegalNoSemanticChange,
            "template slot values are unchanged",
        );
    }
    binding_row(
        SavedQueryRebindingDimension::TemplateSlotValue,
        SavedQueryRebindingLegality::LegalRequiresFreshFreeze,
        "template slot value changes require a fresh freeze artifact",
    )
}

pub fn evaluate_template_slot_set(
    artifact: &SavedQueryArtifact,
    descriptor: &SavedQueryReuseDescriptor,
) -> SavedQueryBindingMatrixRow {
    if artifact.metadata().template_slot_count() == descriptor.template_slot_count() {
        return binding_row(
            SavedQueryRebindingDimension::TemplateSlotSet,
            SavedQueryRebindingLegality::LegalNoSemanticChange,
            "template slot set width is unchanged",
        );
    }
    binding_row(
        SavedQueryRebindingDimension::TemplateSlotSet,
        SavedQueryRebindingLegality::IllegalSemanticDrift,
        "template slot set changes are always semantic drift",
    )
}

pub fn evaluate_view_family(
    artifact: &SavedQueryArtifact,
    descriptor: &SavedQueryReuseDescriptor,
) -> SavedQueryBindingMatrixRow {
    if artifact.metadata().view_shape_family() == descriptor.view_shape_family()
        && artifact.metadata().view_shape_digest() == descriptor.view_shape_digest()
    {
        return binding_row(
            SavedQueryRebindingDimension::ViewFamily,
            SavedQueryRebindingLegality::LegalNoSemanticChange,
            "view shape family and digest are unchanged",
        );
    }
    binding_row(
        SavedQueryRebindingDimension::ViewFamily,
        SavedQueryRebindingLegality::LegalRequiresFreshFreeze,
        "view-shape changes require a fresh freeze artifact",
    )
}

pub fn evaluate_identity_consumption(
    artifact: &SavedQueryArtifact,
    descriptor: &SavedQueryReuseDescriptor,
) -> SavedQueryBindingMatrixRow {
    if artifact.metadata().identity_consumption() == descriptor.identity_consumption()
        && artifact.metadata().identity_consumption_digest()
            == descriptor.identity_consumption_digest()
        && artifact
            .metadata()
            .inspector_identity_classification_digest()
            == descriptor.inspector_identity_classification_digest()
    {
        return binding_row(
            SavedQueryRebindingDimension::IdentityConsumption,
            SavedQueryRebindingLegality::LegalNoSemanticChange,
            "identity-aware inspector contract is unchanged",
        );
    }
    binding_row(
        SavedQueryRebindingDimension::IdentityConsumption,
        SavedQueryRebindingLegality::LegalRequiresFreshFreeze,
        "identity-aware inspector contract changes require a fresh freeze artifact",
    )
}

pub fn evaluate_result_shape_family(
    artifact: &SavedQueryArtifact,
    descriptor: &SavedQueryReuseDescriptor,
) -> SavedQueryBindingMatrixRow {
    if artifact.metadata().result_shape_family() == descriptor.result_shape_family() {
        return binding_row(
            SavedQueryRebindingDimension::ResultShapeFamily,
            SavedQueryRebindingLegality::LegalNoSemanticChange,
            "result-shape family is unchanged",
        );
    }
    binding_row(
        SavedQueryRebindingDimension::ResultShapeFamily,
        SavedQueryRebindingLegality::IllegalSemanticDrift,
        "result-shape family changes are semantic drift",
    )
}

pub fn evaluate_composition_lineage(
    artifact: &SavedQueryArtifact,
    descriptor: &SavedQueryReuseDescriptor,
) -> SavedQueryBindingMatrixRow {
    if artifact.metadata().composition_digest() == descriptor.composition_digest()
        && artifact.metadata().scope_lineage_digest() == descriptor.scope_lineage_digest()
    {
        return binding_row(
            SavedQueryRebindingDimension::CompositionLineage,
            SavedQueryRebindingLegality::LegalNoSemanticChange,
            "composition digest and lineage are unchanged",
        );
    }
    binding_row(
        SavedQueryRebindingDimension::CompositionLineage,
        SavedQueryRebindingLegality::IllegalSemanticDrift,
        "composition digest or lineage changed",
    )
}

pub fn evaluate_support_profile(
    artifact: &SavedQueryArtifact,
    descriptor: &SavedQueryReuseDescriptor,
) -> SavedQueryBindingMatrixRow {
    if artifact.metadata().support_profile_digest() == descriptor.support_profile_digest()
        && artifact.metadata().capability_family_identity()
            == descriptor.capability_family_identity()
    {
        return binding_row(
            SavedQueryRebindingDimension::SupportProfile,
            SavedQueryRebindingLegality::LegalNoSemanticChange,
            "support profile and capability family are unchanged",
        );
    }
    binding_row(
        SavedQueryRebindingDimension::SupportProfile,
        SavedQueryRebindingLegality::IllegalSemanticDrift,
        "support posture changes are semantic drift",
    )
}

pub fn evaluate_temporal_async_surface(
    artifact: &SavedQueryArtifact,
    descriptor: &SavedQueryReuseDescriptor,
) -> SavedQueryBindingMatrixRow {
    if artifact.metadata().temporal_async_surface_posture()
        == descriptor.temporal_async_surface_posture()
    {
        return binding_row(
            SavedQueryRebindingDimension::TemporalAsyncSurface,
            SavedQueryRebindingLegality::LegalNoSemanticChange,
            "temporal/async surface posture is unchanged",
        );
    }
    binding_row(
        SavedQueryRebindingDimension::TemporalAsyncSurface,
        SavedQueryRebindingLegality::IllegalSemanticDrift,
        "temporal/async reuse posture changed",
    )
}
