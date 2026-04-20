use crate::authoring::ResultShapeFamily;
use crate::composition::{CompositionDigest, ScopeLineageDigest, TemplateBindingDigest};
use crate::identity::SchemaBasisDigest;
use crate::query_context::QueryContextFamily;
use crate::saved_query::{SavedQueryArtifact, SavedQueryFailureClass};
use crate::view_shape::{ViewShapeDigest, ViewShapeFamily};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SavedQueryRebindingDimension {
    SchemaBasisDigest,
    BasisFamily,
    TemplateSlotValue,
    TemplateSlotSet,
    ViewFamily,
    ResultShapeFamily,
    CompositionLineage,
    SupportProfile,
}

impl SavedQueryRebindingDimension {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SchemaBasisDigest => "schema_basis_digest",
            Self::BasisFamily => "basis_family",
            Self::TemplateSlotValue => "template_slot_value",
            Self::TemplateSlotSet => "template_slot_set",
            Self::ViewFamily => "view_family",
            Self::ResultShapeFamily => "result_shape_family",
            Self::CompositionLineage => "composition_lineage",
            Self::SupportProfile => "support_profile",
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SavedQueryReuseDescriptor {
    schema_basis_digest: SchemaBasisDigest,
    schema_basis_equivalence: Option<SchemaBasisEquivalenceEvidence>,
    basis_family: Option<QueryContextFamily>,
    template_binding_digest: Option<TemplateBindingDigest>,
    template_slot_count: usize,
    view_shape_digest: ViewShapeDigest,
    view_shape_family: ViewShapeFamily,
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
            result_shape_family,
            composition_digest,
            scope_lineage_digest,
            support_profile_digest: support_profile_digest.into(),
            capability_family_identity: capability_family_identity.into(),
        }
    }

    pub fn with_schema_basis_equivalence(
        mut self,
        evidence: SchemaBasisEquivalenceEvidence,
    ) -> Self {
        self.schema_basis_equivalence = Some(evidence);
        self
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SavedQueryBindingMatrixArtifact {
    rows: Vec<SavedQueryBindingMatrixRow>,
    digest: String,
}

impl SavedQueryBindingMatrixArtifact {
    pub fn rows(&self) -> &[SavedQueryBindingMatrixRow] {
        &self.rows
    }

    pub fn digest(&self) -> &str {
        &self.digest
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
        evaluate_result_shape_family(artifact, descriptor),
        evaluate_composition_lineage(artifact, descriptor),
        evaluate_support_profile(artifact, descriptor),
    ];
    let overall = if rows
        .iter()
        .any(|row| row.legality == SavedQueryRebindingLegality::IllegalSemanticDrift)
    {
        SavedQueryRebindingLegality::IllegalSemanticDrift
    } else if rows
        .iter()
        .any(|row| row.legality == SavedQueryRebindingLegality::LegalRequiresFreshFreeze)
    {
        SavedQueryRebindingLegality::LegalRequiresFreshFreeze
    } else {
        SavedQueryRebindingLegality::LegalNoSemanticChange
    };
    let matrix = SavedQueryBindingMatrixArtifact {
        digest: crate::identity::hash_parts(
            &rows
                .iter()
                .map(|row| {
                    format!(
                        "{}:{}:{}",
                        row.dimension.as_str(),
                        row.legality.as_str(),
                        row.message
                    )
                })
                .collect::<Vec<_>>(),
        ),
        rows,
    };

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

fn evaluate_schema_basis(
    artifact: &SavedQueryArtifact,
    descriptor: &SavedQueryReuseDescriptor,
) -> SavedQueryBindingMatrixRow {
    if artifact.metadata().schema_basis_digest() == &descriptor.schema_basis_digest {
        return row(
            SavedQueryRebindingDimension::SchemaBasisDigest,
            SavedQueryRebindingLegality::LegalNoSemanticChange,
            "schema basis digest is unchanged",
        );
    }
    match &descriptor.schema_basis_equivalence {
        Some(evidence)
            if evidence.same_admitted_basis_family() && evidence.same_projection_legality_surface() =>
        {
            row(
                SavedQueryRebindingDimension::SchemaBasisDigest,
                SavedQueryRebindingLegality::LegalNoSemanticChange,
                "explicit schema-basis equivalence evidence preserves the admitted projection legality surface",
            )
        }
        _ => row(
            SavedQueryRebindingDimension::SchemaBasisDigest,
            SavedQueryRebindingLegality::IllegalSemanticDrift,
            "schema basis digest changed without explicit equivalence evidence",
        ),
    }
}

fn evaluate_basis_family(
    artifact: &SavedQueryArtifact,
    descriptor: &SavedQueryReuseDescriptor,
) -> SavedQueryBindingMatrixRow {
    if artifact.metadata().basis_family() == descriptor.basis_family.as_ref() {
        return row(
            SavedQueryRebindingDimension::BasisFamily,
            SavedQueryRebindingLegality::LegalNoSemanticChange,
            "basis family is unchanged",
        );
    }
    row(
        SavedQueryRebindingDimension::BasisFamily,
        SavedQueryRebindingLegality::IllegalSemanticDrift,
        "basis family changes are always semantic drift in Milestone 8",
    )
}

fn evaluate_template_slot_value(
    artifact: &SavedQueryArtifact,
    descriptor: &SavedQueryReuseDescriptor,
) -> SavedQueryBindingMatrixRow {
    if artifact.metadata().template_binding_digest() == descriptor.template_binding_digest.as_ref() {
        return row(
            SavedQueryRebindingDimension::TemplateSlotValue,
            SavedQueryRebindingLegality::LegalNoSemanticChange,
            "template slot values are unchanged",
        );
    }
    row(
        SavedQueryRebindingDimension::TemplateSlotValue,
        SavedQueryRebindingLegality::LegalRequiresFreshFreeze,
        "template slot value changes require a fresh freeze artifact",
    )
}

fn evaluate_template_slot_set(
    artifact: &SavedQueryArtifact,
    descriptor: &SavedQueryReuseDescriptor,
) -> SavedQueryBindingMatrixRow {
    if artifact.metadata().template_slot_count() == descriptor.template_slot_count {
        return row(
            SavedQueryRebindingDimension::TemplateSlotSet,
            SavedQueryRebindingLegality::LegalNoSemanticChange,
            "template slot set width is unchanged",
        );
    }
    row(
        SavedQueryRebindingDimension::TemplateSlotSet,
        SavedQueryRebindingLegality::IllegalSemanticDrift,
        "template slot set changes are always semantic drift",
    )
}

fn evaluate_view_family(
    artifact: &SavedQueryArtifact,
    descriptor: &SavedQueryReuseDescriptor,
) -> SavedQueryBindingMatrixRow {
    if artifact.metadata().view_shape_family() == descriptor.view_shape_family
        && artifact.metadata().view_shape_digest() == &descriptor.view_shape_digest
    {
        return row(
            SavedQueryRebindingDimension::ViewFamily,
            SavedQueryRebindingLegality::LegalNoSemanticChange,
            "view shape family and digest are unchanged",
        );
    }
    row(
        SavedQueryRebindingDimension::ViewFamily,
        SavedQueryRebindingLegality::LegalRequiresFreshFreeze,
        "view-shape changes require a fresh freeze artifact",
    )
}

fn evaluate_result_shape_family(
    artifact: &SavedQueryArtifact,
    descriptor: &SavedQueryReuseDescriptor,
) -> SavedQueryBindingMatrixRow {
    if artifact.metadata().result_shape_family() == &descriptor.result_shape_family {
        return row(
            SavedQueryRebindingDimension::ResultShapeFamily,
            SavedQueryRebindingLegality::LegalNoSemanticChange,
            "result-shape family is unchanged",
        );
    }
    row(
        SavedQueryRebindingDimension::ResultShapeFamily,
        SavedQueryRebindingLegality::IllegalSemanticDrift,
        "result-shape family changes are semantic drift",
    )
}

fn evaluate_composition_lineage(
    artifact: &SavedQueryArtifact,
    descriptor: &SavedQueryReuseDescriptor,
) -> SavedQueryBindingMatrixRow {
    if artifact.metadata().composition_digest() == &descriptor.composition_digest
        && artifact.metadata().scope_lineage_digest() == descriptor.scope_lineage_digest.as_ref()
    {
        return row(
            SavedQueryRebindingDimension::CompositionLineage,
            SavedQueryRebindingLegality::LegalNoSemanticChange,
            "composition digest and lineage are unchanged",
        );
    }
    row(
        SavedQueryRebindingDimension::CompositionLineage,
        SavedQueryRebindingLegality::IllegalSemanticDrift,
        "composition digest or lineage changed",
    )
}

fn evaluate_support_profile(
    artifact: &SavedQueryArtifact,
    descriptor: &SavedQueryReuseDescriptor,
) -> SavedQueryBindingMatrixRow {
    if artifact.metadata().support_profile_digest() == descriptor.support_profile_digest
        && artifact.metadata().capability_family_identity() == descriptor.capability_family_identity
    {
        return row(
            SavedQueryRebindingDimension::SupportProfile,
            SavedQueryRebindingLegality::LegalNoSemanticChange,
            "support profile and capability family are unchanged",
        );
    }
    row(
        SavedQueryRebindingDimension::SupportProfile,
        SavedQueryRebindingLegality::IllegalSemanticDrift,
        "support posture changes are semantic drift",
    )
}

fn row(
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
