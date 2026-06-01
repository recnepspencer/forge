use super::certification::ForgeQueryOrchestrationSurfaceCertificationReference;
use super::docs::ForgeQueryOrchestrationSurfaceDocReference;
use super::family::{
    ForgeQueryOrchestrationBindingProjection, ForgeQueryOrchestrationCheckedTopologyKind,
    ForgeQueryOrchestrationSupportSurface, ForgeQueryOrchestrationSurfaceFamily,
    ForgeQueryOrchestrationSurfaceVisibility, ForgeQueryOrchestrationTranscriptFamily,
};
use super::row::{ForgeQueryOrchestrationSemanticProfile, ForgeQueryOrchestrationSurfaceRow};
use super::transcript::ForgeQueryOrchestrationProofContract;

pub(crate) struct RowSpec {
    pub(crate) public_name: &'static str,
    pub(crate) canonical_base_name: &'static str,
    pub(crate) family: ForgeQueryOrchestrationSurfaceFamily,
    pub(crate) visibility: ForgeQueryOrchestrationSurfaceVisibility,
    pub(crate) ordinary_outcome_supported: bool,
    pub(crate) binding_projection: ForgeQueryOrchestrationBindingProjection,
    pub(crate) checked_type_name: &'static str,
    pub(crate) proof_type_name: &'static str,
    pub(crate) transcript_family: ForgeQueryOrchestrationTranscriptFamily,
    pub(crate) checked_topology_kind: ForgeQueryOrchestrationCheckedTopologyKind,
    pub(crate) support_surface: ForgeQueryOrchestrationSupportSurface,
    pub(crate) semantic_profile: ForgeQueryOrchestrationSemanticProfile,
    pub(crate) doc_path: &'static str,
    pub(crate) doc_section: &'static str,
    pub(crate) certification_suite: &'static str,
    pub(crate) certification_command: &'static str,
}

impl RowSpec {
    pub(crate) fn into_row(self) -> ForgeQueryOrchestrationSurfaceRow {
        ForgeQueryOrchestrationSurfaceRow::new(
            self.public_name,
            self.canonical_base_name,
            self.family,
            self.visibility,
            self.ordinary_outcome_supported,
            self.binding_projection,
            ForgeQueryOrchestrationProofContract::new(
                self.checked_type_name,
                self.proof_type_name,
                self.transcript_family,
                self.checked_topology_kind,
                self.support_surface,
            ),
            self.semantic_profile,
            ForgeQueryOrchestrationSurfaceDocReference::new(self.doc_path, self.doc_section),
            ForgeQueryOrchestrationSurfaceCertificationReference::new(
                self.certification_suite,
                self.certification_command,
            ),
        )
    }
}

pub(crate) fn leak(text: String) -> &'static str {
    text.leak()
}

pub(crate) fn push_four_lane_rows(rows: &mut Vec<RowSpec>, spec: FourLaneSpec) {
    for (public_name, visibility, doc_section) in [
        (
            spec.base_name,
            ForgeQueryOrchestrationSurfaceVisibility::Ordinary,
            spec.ordinary_section,
        ),
        (
            leak(format!("{}_outcome", spec.base_name)),
            ForgeQueryOrchestrationSurfaceVisibility::OrdinaryOutcome,
            spec.outcome_section,
        ),
        (
            leak(format!("{}_checked", spec.base_name)),
            ForgeQueryOrchestrationSurfaceVisibility::Checked,
            spec.checked_section,
        ),
        (
            leak(format!("{}_proof", spec.base_name)),
            ForgeQueryOrchestrationSurfaceVisibility::ProofVisible,
            spec.proof_section,
        ),
    ] {
        rows.push(RowSpec {
            public_name,
            canonical_base_name: spec.base_name,
            family: spec.family,
            visibility,
            ordinary_outcome_supported: true,
            binding_projection: spec.binding_projection,
            checked_type_name: spec.checked_type_name,
            proof_type_name: spec.proof_type_name,
            transcript_family: spec.transcript_family,
            checked_topology_kind: spec.checked_topology_kind,
            support_surface: spec.support_surface,
            semantic_profile: spec.semantic_profile.clone(),
            doc_path: spec.doc_path,
            doc_section,
            certification_suite: spec.certification_suite,
            certification_command: spec.certification_command,
        });
    }
}

pub(crate) struct FourLaneSpec {
    pub(crate) base_name: &'static str,
    pub(crate) family: ForgeQueryOrchestrationSurfaceFamily,
    pub(crate) transcript_family: ForgeQueryOrchestrationTranscriptFamily,
    pub(crate) checked_type_name: &'static str,
    pub(crate) proof_type_name: &'static str,
    pub(crate) support_surface: ForgeQueryOrchestrationSupportSurface,
    pub(crate) checked_topology_kind: ForgeQueryOrchestrationCheckedTopologyKind,
    pub(crate) binding_projection: ForgeQueryOrchestrationBindingProjection,
    pub(crate) semantic_profile: ForgeQueryOrchestrationSemanticProfile,
    pub(crate) doc_path: &'static str,
    pub(crate) ordinary_section: &'static str,
    pub(crate) outcome_section: &'static str,
    pub(crate) checked_section: &'static str,
    pub(crate) proof_section: &'static str,
    pub(crate) certification_suite: &'static str,
    pub(crate) certification_command: &'static str,
}
