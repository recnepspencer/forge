use worth_ui_dsl::UiDslLoweringReceipt;

use crate::declaration::{
    UiDeclarationContainmentIntent, UiDeclarationFamilyAdmission, UiDeclarationFamilyKind,
    UiDeclarationOrderingGuarantee, UiDeclarationRepetitionPosture,
    UiDeclarationSlotParticipationIntent, UiDeclarationStructuralRole,
};

use super::{
    UiDeclarationStructuralSemantics, UiDeclarationStructuralSemanticsAdmission,
    UiDeclarationStructuralSemanticsAdmissionDenial,
};

pub(crate) fn admit_declaration_structural_semantics(
    semantic_receipt: &UiDslLoweringReceipt,
    family_admission: &UiDeclarationFamilyAdmission,
) -> UiDeclarationStructuralSemanticsAdmission {
    let family_kind = match family_admission.admitted_family() {
        Ok(family) => family.kind(),
        Err(denial) => {
            return UiDeclarationStructuralSemanticsAdmission::Denied(
                UiDeclarationStructuralSemanticsAdmissionDenial::FamilyNotAdmitted {
                    denial: denial.clone(),
                },
            );
        }
    };

    let structural_role = match structural_role_for_family(family_kind) {
        Some(role) => role,
        None => {
            return UiDeclarationStructuralSemanticsAdmission::Denied(
                UiDeclarationStructuralSemanticsAdmissionDenial::FamilyDoesNotProjectStructuralSemantics {
                    family: family_kind,
                },
            );
        }
    };

    let semantic_artifact = semantic_receipt.semantic_artifact();
    let structural_tokens = semantic_artifact
        .structural_tokens()
        .iter()
        .map(|token| token.as_str())
        .collect::<Vec<_>>();
    let expected_prefix = family_structural_prefix(family_kind);
    let family_claim = structural_tokens
        .iter()
        .copied()
        .find(|token| token.starts_with(expected_prefix));
    let slot_claims = structural_tokens
        .iter()
        .copied()
        .filter(|token| token.starts_with("slot:"))
        .collect::<Vec<_>>();

    let slot_participation_intent = match slot_claims.as_slice() {
        [] => UiDeclarationSlotParticipationIntent::None,
        claims if !slot_participation_is_admitted_for_family(family_kind) => {
            return UiDeclarationStructuralSemanticsAdmission::Denied(
                UiDeclarationStructuralSemanticsAdmissionDenial::SlotParticipationNotAdmittedForFamily {
                    family: family_kind,
                    observed: claims.iter().map(|claim| (*claim).to_owned()).collect(),
                },
            );
        }
        [claim] if claim.len() == "slot:".len() => {
            return UiDeclarationStructuralSemanticsAdmission::Denied(
                UiDeclarationStructuralSemanticsAdmissionDenial::InvalidSlotParticipationClaim {
                    family: family_kind,
                    observed: vec![(*claim).to_owned()],
                },
            );
        }
        [claim] => UiDeclarationSlotParticipationIntent::DeclaredSlotParticipant {
            slot_name: claim["slot:".len()..].into(),
        },
        claims => {
            return UiDeclarationStructuralSemanticsAdmission::Denied(
                UiDeclarationStructuralSemanticsAdmissionDenial::ContradictorySlotParticipationClaims {
                    family: family_kind,
                    observed: claims.iter().map(|claim| (*claim).to_owned()).collect(),
                },
            );
        }
    };

    let unsupported_tokens = structural_tokens
        .iter()
        .copied()
        .filter(|token| Some(*token) != family_claim && !token.starts_with("slot:"))
        .collect::<Vec<_>>();
    if !unsupported_tokens.is_empty() {
        return UiDeclarationStructuralSemanticsAdmission::Denied(
            UiDeclarationStructuralSemanticsAdmissionDenial::UnsupportedStructuralTokens {
                family: family_kind,
                observed: unsupported_tokens
                    .iter()
                    .map(|token| (*token).to_owned())
                    .collect(),
            },
        );
    }

    let Some(family_claim) = family_claim else {
        return UiDeclarationStructuralSemanticsAdmission::Denied(
            UiDeclarationStructuralSemanticsAdmissionDenial::InvalidStructuralMembershipClaim {
                family: family_kind,
                observed: structural_tokens
                    .iter()
                    .map(|token| (*token).to_owned())
                    .collect(),
            },
        );
    };
    let Some(containment_intent) = containment_intent_for_family(family_kind, family_claim) else {
        return UiDeclarationStructuralSemanticsAdmission::Denied(
            UiDeclarationStructuralSemanticsAdmissionDenial::InvalidStructuralMembershipClaim {
                family: family_kind,
                observed: vec![family_claim.to_owned()],
            },
        );
    };

    UiDeclarationStructuralSemanticsAdmission::Admitted(UiDeclarationStructuralSemantics::new(
        family_kind,
        structural_role,
        containment_intent,
        slot_participation_intent,
        UiDeclarationOrderingGuarantee::NotSemanticallyClaimed,
        UiDeclarationRepetitionPosture::NotAdmitted,
    ))
}

fn structural_role_for_family(
    family: UiDeclarationFamilyKind,
) -> Option<UiDeclarationStructuralRole> {
    match family {
        UiDeclarationFamilyKind::Page => Some(UiDeclarationStructuralRole::Page),
        UiDeclarationFamilyKind::PageSet => Some(UiDeclarationStructuralRole::PageSet),
        UiDeclarationFamilyKind::Region => Some(UiDeclarationStructuralRole::Region),
        UiDeclarationFamilyKind::Mosaic => Some(UiDeclarationStructuralRole::Mosaic),
        UiDeclarationFamilyKind::LocalComposition => {
            Some(UiDeclarationStructuralRole::LocalComposition)
        }
        UiDeclarationFamilyKind::Control => Some(UiDeclarationStructuralRole::Control),
        UiDeclarationFamilyKind::DiagnosticSurface => {
            Some(UiDeclarationStructuralRole::DiagnosticSurface)
        }
        UiDeclarationFamilyKind::QueryBinding | UiDeclarationFamilyKind::Intent => None,
    }
}

fn containment_intent_for_family(
    family: UiDeclarationFamilyKind,
    family_claim: &str,
) -> Option<UiDeclarationContainmentIntent> {
    let claim_name = family_claim[family_structural_prefix(family).len()..].trim();

    match family {
        UiDeclarationFamilyKind::Page if claim_name == "product-root" => {
            Some(UiDeclarationContainmentIntent::RootTopology)
        }
        UiDeclarationFamilyKind::PageSet if !claim_name.is_empty() => {
            Some(UiDeclarationContainmentIntent::DeclaredPageSetMembership {
                page_set_name: claim_name.into(),
            })
        }
        UiDeclarationFamilyKind::Region if !claim_name.is_empty() => {
            Some(UiDeclarationContainmentIntent::DeclaredRegionMembership {
                region_name: claim_name.into(),
            })
        }
        UiDeclarationFamilyKind::Mosaic if !claim_name.is_empty() => {
            Some(UiDeclarationContainmentIntent::DeclaredMosaicMembership {
                mosaic_name: claim_name.into(),
            })
        }
        UiDeclarationFamilyKind::LocalComposition if !claim_name.is_empty() => Some(
            UiDeclarationContainmentIntent::DeclaredLocalCompositionMembership {
                local_composition_name: claim_name.into(),
            },
        ),
        UiDeclarationFamilyKind::Control if !claim_name.is_empty() => {
            Some(UiDeclarationContainmentIntent::DeclaredControlAttachment {
                control_name: claim_name.into(),
            })
        }
        UiDeclarationFamilyKind::DiagnosticSurface if !claim_name.is_empty() => Some(
            UiDeclarationContainmentIntent::DeclaredDiagnosticAttachment {
                diagnostic_surface_name: claim_name.into(),
            },
        ),
        UiDeclarationFamilyKind::QueryBinding | UiDeclarationFamilyKind::Intent => {
            unreachable!("non-structural families do not admit structural semantics")
        }
        _ => None,
    }
}

fn slot_participation_is_admitted_for_family(family: UiDeclarationFamilyKind) -> bool {
    matches!(family, UiDeclarationFamilyKind::Control)
}

fn family_structural_prefix(family: UiDeclarationFamilyKind) -> &'static str {
    match family {
        UiDeclarationFamilyKind::Page => "page:",
        UiDeclarationFamilyKind::PageSet => "page-set:",
        UiDeclarationFamilyKind::Region => "region:",
        UiDeclarationFamilyKind::Mosaic => "mosaic:",
        UiDeclarationFamilyKind::LocalComposition => "local-composition:",
        UiDeclarationFamilyKind::Control => "control:",
        UiDeclarationFamilyKind::DiagnosticSurface => "diagnostic-surface:",
        UiDeclarationFamilyKind::QueryBinding | UiDeclarationFamilyKind::Intent => "",
    }
}
