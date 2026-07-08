use crate::evidence::shared::evidence_expansion::UiEvidenceExpansion;
use crate::evidence::shared::evidence_reference::UiEvidenceRef;
use worth_ui_inspection::{
    UiEvidenceAuthorityGeneration, UiEvidenceExpansionOutcome, UiEvidenceMaterializationPosture,
    UiEvidenceRetentionPosture, UiEvidenceRichness,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ExpansionAdmission {
    Discarded,
    WrongGeneration,
    NotMaterialized,
    Admitted,
}

pub(crate) fn classify_expansion_admission(
    current_generation: UiEvidenceAuthorityGeneration,
    evidence_ref: &UiEvidenceRef,
    _requested_richness: UiEvidenceRichness,
) -> ExpansionAdmission {
    if matches!(
        evidence_ref.retention_posture(),
        UiEvidenceRetentionPosture::DiscardedWithTombstone
    ) {
        return ExpansionAdmission::Discarded;
    }

    if evidence_ref.authority_generation() != current_generation {
        return ExpansionAdmission::WrongGeneration;
    }

    if !matches!(
        evidence_ref.materialization_posture(),
        UiEvidenceMaterializationPosture::SummaryAvailable
            | UiEvidenceMaterializationPosture::DetailAvailable
    ) {
        return ExpansionAdmission::NotMaterialized;
    }

    ExpansionAdmission::Admitted
}

fn build_preflight_expansion(
    evidence_ref: UiEvidenceRef,
    requested_richness: UiEvidenceRichness,
    outcome: UiEvidenceExpansionOutcome,
) -> UiEvidenceExpansion {
    UiEvidenceExpansion::new(
        evidence_ref,
        requested_richness,
        outcome,
        None,
        Box::new([]),
        None,
    )
}

pub(crate) fn preflight_evidence_expansion(
    current_generation: UiEvidenceAuthorityGeneration,
    evidence_ref: UiEvidenceRef,
    requested_richness: UiEvidenceRichness,
) -> Option<UiEvidenceExpansion> {
    match classify_expansion_admission(current_generation, &evidence_ref, requested_richness) {
        ExpansionAdmission::Discarded => Some(build_preflight_expansion(
            evidence_ref,
            requested_richness,
            UiEvidenceExpansionOutcome::Discarded {
                retention: evidence_ref.retention_posture(),
            },
        )),
        ExpansionAdmission::WrongGeneration => Some(build_preflight_expansion(
            evidence_ref,
            requested_richness,
            UiEvidenceExpansionOutcome::WrongGeneration {
                requested_generation: evidence_ref.authority_generation(),
                current_generation,
            },
        )),
        ExpansionAdmission::NotMaterialized => Some(build_preflight_expansion(
            evidence_ref,
            requested_richness,
            UiEvidenceExpansionOutcome::NotMaterialized {
                posture: evidence_ref.materialization_posture(),
            },
        )),
        ExpansionAdmission::Admitted => None,
    }
}