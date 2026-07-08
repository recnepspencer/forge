use worth_ui_inspection::{
    UiEvidenceAuthorityArtifactIdentity, UiEvidenceAuthorityBinding, UiEvidenceAuthorityGeneration,
    UiEvidenceAuthorityKind,
};

pub(crate) fn evidence_authority_binding(
    authority_kind: UiEvidenceAuthorityKind,
    authority_digest: u64,
    authority_generation: UiEvidenceAuthorityGeneration,
    world: Option<worth_ui_inspection::UiInspectionSupportWorld>,
) -> UiEvidenceAuthorityBinding {
    UiEvidenceAuthorityBinding::new(
        UiEvidenceAuthorityArtifactIdentity::new(authority_kind, authority_digest),
        authority_generation,
        world,
    )
}