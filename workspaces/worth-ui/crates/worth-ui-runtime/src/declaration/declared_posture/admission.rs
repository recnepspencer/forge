use worth_ui_dsl::UiDslLoweringReceipt;
use worth_ui_host_contract::WorthUiHostCapability;

use crate::declaration::{UiDeclarationFamilyAdmission, UiDeclarationFamilyKind};

use super::{
    measurement_policy::admit_measurement_policy_lane, UiDeclaredHostCapabilityPosture,
    UiDeclaredPostureAdmission, UiDeclaredPostureAdmissionDenial, UiDeclaredPostureApplicability,
    UiDeclaredPostureContract, UiDeclaredPostureLane, UiDeclaredPostureLaneKind,
    UiDeclaredQueryBindingPosture, UiDeclaredServiceUsagePosture, UiDeclaredTouchMeaningPosture,
};

pub(crate) fn admit_declared_posture_contract(
    semantic_receipt: &UiDslLoweringReceipt,
    family_admission: &UiDeclarationFamilyAdmission,
) -> UiDeclaredPostureAdmission {
    let family_kind = match family_admission.admitted_family() {
        Ok(family) => family.kind(),
        Err(denial) => {
            return UiDeclaredPostureAdmission::Denied(
                UiDeclaredPostureAdmissionDenial::FamilyNotAdmitted {
                    denial: denial.clone(),
                },
            );
        }
    };

    let posture_tokens = semantic_receipt
        .semantic_artifact()
        .posture_tokens()
        .iter()
        .map(|token| token.as_str())
        .collect::<Vec<_>>();

    let query_binding = match admit_query_binding_lane(family_kind, &posture_tokens) {
        Ok(lane) => lane,
        Err(denial) => return UiDeclaredPostureAdmission::Denied(denial),
    };
    let service_usage = match admit_service_usage_lane(family_kind, &posture_tokens) {
        Ok(lane) => lane,
        Err(denial) => return UiDeclaredPostureAdmission::Denied(denial),
    };
    let touch_meaning = match admit_touch_meaning_lane(family_kind, &posture_tokens) {
        Ok(lane) => lane,
        Err(denial) => return UiDeclaredPostureAdmission::Denied(denial),
    };
    let measurement_policy = match admit_measurement_policy_lane(family_kind, &posture_tokens) {
        Ok(lane) => lane,
        Err(denial) => return UiDeclaredPostureAdmission::Denied(denial),
    };
    let host_capability = match admit_host_capability_lane(family_kind, &posture_tokens) {
        Ok(lane) => lane,
        Err(denial) => return UiDeclaredPostureAdmission::Denied(denial),
    };

    UiDeclaredPostureAdmission::Admitted(UiDeclaredPostureContract::new(
        query_binding,
        service_usage,
        touch_meaning,
        measurement_policy,
        host_capability,
    ))
}

fn admit_query_binding_lane(
    family: UiDeclarationFamilyKind,
    posture_tokens: &[&str],
) -> Result<UiDeclaredPostureLane<UiDeclaredQueryBindingPosture>, UiDeclaredPostureAdmissionDenial>
{
    let claims = posture_tokens
        .iter()
        .copied()
        .filter(|token| token.starts_with("query-binding:"))
        .collect::<Vec<_>>();
    let applicability = match family {
        UiDeclarationFamilyKind::Page
        | UiDeclarationFamilyKind::PageSet
        | UiDeclarationFamilyKind::Region
        | UiDeclarationFamilyKind::Mosaic
        | UiDeclarationFamilyKind::LocalComposition
        | UiDeclarationFamilyKind::Control => UiDeclaredPostureApplicability::Optional,
        UiDeclarationFamilyKind::QueryBinding => UiDeclaredPostureApplicability::Required,
        UiDeclarationFamilyKind::Intent | UiDeclarationFamilyKind::DiagnosticSurface => {
            UiDeclaredPostureApplicability::NotApplicable
        }
    };

    admit_lane(
        family,
        UiDeclaredPostureLaneKind::QueryBinding,
        applicability,
        claims,
        |claim| match claim {
            "query-binding:standalone" => Some(UiDeclaredQueryBindingPosture::StandaloneBinding),
            "query-binding:attached:view" => {
                Some(UiDeclaredQueryBindingPosture::AttachedViewBinding)
            }
            _ => None,
        },
    )
}

fn admit_service_usage_lane(
    family: UiDeclarationFamilyKind,
    posture_tokens: &[&str],
) -> Result<UiDeclaredPostureLane<UiDeclaredServiceUsagePosture>, UiDeclaredPostureAdmissionDenial>
{
    let claims = posture_tokens
        .iter()
        .copied()
        .filter(|token| token.starts_with("service:"))
        .collect::<Vec<_>>();
    let applicability = match family {
        UiDeclarationFamilyKind::Control => UiDeclaredPostureApplicability::Optional,
        UiDeclarationFamilyKind::Page
        | UiDeclarationFamilyKind::PageSet
        | UiDeclarationFamilyKind::Region
        | UiDeclarationFamilyKind::Mosaic
        | UiDeclarationFamilyKind::LocalComposition => {
            UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted
        }
        UiDeclarationFamilyKind::DiagnosticSurface => {
            UiDeclaredPostureApplicability::DiagnosticOnly
        }
        UiDeclarationFamilyKind::QueryBinding | UiDeclarationFamilyKind::Intent => {
            UiDeclaredPostureApplicability::NotApplicable
        }
    };

    admit_lane(
        family,
        UiDeclaredPostureLaneKind::ServiceUsage,
        applicability,
        claims,
        |claim| match claim {
            "service:portal" => Some(UiDeclaredServiceUsagePosture::Portal),
            "service:scroll" => Some(UiDeclaredServiceUsagePosture::Scroll),
            "service:focus-routing" => Some(UiDeclaredServiceUsagePosture::FocusRouting),
            "service:motion" => Some(UiDeclaredServiceUsagePosture::Motion),
            _ => None,
        },
    )
}

fn admit_touch_meaning_lane(
    family: UiDeclarationFamilyKind,
    posture_tokens: &[&str],
) -> Result<UiDeclaredPostureLane<UiDeclaredTouchMeaningPosture>, UiDeclaredPostureAdmissionDenial>
{
    let claims = posture_tokens
        .iter()
        .copied()
        .filter(|token| token.starts_with("touch:"))
        .collect::<Vec<_>>();
    let applicability = match family {
        UiDeclarationFamilyKind::Control => UiDeclaredPostureApplicability::Optional,
        UiDeclarationFamilyKind::Page
        | UiDeclarationFamilyKind::PageSet
        | UiDeclarationFamilyKind::Region
        | UiDeclarationFamilyKind::Mosaic
        | UiDeclarationFamilyKind::LocalComposition => {
            UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted
        }
        UiDeclarationFamilyKind::DiagnosticSurface => {
            UiDeclaredPostureApplicability::DiagnosticOnly
        }
        UiDeclarationFamilyKind::QueryBinding | UiDeclarationFamilyKind::Intent => {
            UiDeclaredPostureApplicability::NotApplicable
        }
    };

    admit_lane(
        family,
        UiDeclaredPostureLaneKind::TouchMeaning,
        applicability,
        claims,
        |claim| match claim {
            "touch:press" => Some(UiDeclaredTouchMeaningPosture::Press),
            "touch:text-entry" => Some(UiDeclaredTouchMeaningPosture::TextEntry),
            _ => None,
        },
    )
}

fn admit_host_capability_lane(
    family: UiDeclarationFamilyKind,
    posture_tokens: &[&str],
) -> Result<UiDeclaredPostureLane<UiDeclaredHostCapabilityPosture>, UiDeclaredPostureAdmissionDenial>
{
    let claims = posture_tokens
        .iter()
        .copied()
        .filter(|token| token.starts_with("host-capability:"))
        .collect::<Vec<_>>();
    let applicability = match family {
        UiDeclarationFamilyKind::Control => UiDeclaredPostureApplicability::Optional,
        UiDeclarationFamilyKind::Page
        | UiDeclarationFamilyKind::PageSet
        | UiDeclarationFamilyKind::Region
        | UiDeclarationFamilyKind::Mosaic
        | UiDeclarationFamilyKind::LocalComposition => {
            UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted
        }
        UiDeclarationFamilyKind::DiagnosticSurface => {
            UiDeclaredPostureApplicability::DiagnosticOnly
        }
        UiDeclarationFamilyKind::QueryBinding | UiDeclarationFamilyKind::Intent => {
            UiDeclaredPostureApplicability::NotApplicable
        }
    };

    match claims.as_slice() {
        [] => Ok(UiDeclaredPostureLane::new(applicability, None)),
        observed if matches!(applicability, UiDeclaredPostureApplicability::NotApplicable) => Err(
            UiDeclaredPostureAdmissionDenial::LaneNotApplicableForFamily {
                family,
                lane: UiDeclaredPostureLaneKind::HostCapability,
                observed: observed.iter().map(|claim| (*claim).to_owned()).collect(),
            },
        ),
        observed
            if matches!(
                applicability,
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted
            ) =>
        {
            Err(
                UiDeclaredPostureAdmissionDenial::LaneArchitecturallyOwnedButNotYetAdmitted {
                    family,
                    lane: UiDeclaredPostureLaneKind::HostCapability,
                    observed: observed.iter().map(|claim| (*claim).to_owned()).collect(),
                },
            )
        }
        observed
            if matches!(
                applicability,
                UiDeclaredPostureApplicability::DiagnosticOnly
            ) =>
        {
            Err(
                UiDeclaredPostureAdmissionDenial::LaneNotApplicableForFamily {
                    family,
                    lane: UiDeclaredPostureLaneKind::HostCapability,
                    observed: observed.iter().map(|claim| (*claim).to_owned()).collect(),
                },
            )
        }
        observed => {
            let mut capabilities = Vec::with_capacity(observed.len());
            for claim in observed {
                let Some(capability) = parse_host_capability_claim(claim) else {
                    return Err(UiDeclaredPostureAdmissionDenial::InvalidLaneClaim {
                        family,
                        lane: UiDeclaredPostureLaneKind::HostCapability,
                        observed: vec![(*claim).to_owned()],
                    });
                };
                capabilities.push(capability);
            }

            Ok(UiDeclaredPostureLane::new(
                applicability,
                Some(UiDeclaredHostCapabilityPosture::new(capabilities)),
            ))
        }
    }
}

fn parse_host_capability_claim(claim: &str) -> Option<WorthUiHostCapability> {
    match claim {
        "host-capability:text-input" => Some(WorthUiHostCapability::TextInput),
        "host-capability:ime" => Some(WorthUiHostCapability::Ime),
        "host-capability:accessibility" => Some(WorthUiHostCapability::Accessibility),
        "host-capability:font-metrics" => Some(WorthUiHostCapability::FontMetrics),
        "host-capability:visual-capture" => Some(WorthUiHostCapability::VisualCapture),
        _ => None,
    }
}

fn admit_lane<T, F>(
    family: UiDeclarationFamilyKind,
    lane: UiDeclaredPostureLaneKind,
    applicability: UiDeclaredPostureApplicability,
    claims: Vec<&str>,
    parse: F,
) -> Result<UiDeclaredPostureLane<T>, UiDeclaredPostureAdmissionDenial>
where
    F: Fn(&str) -> Option<T>,
{
    match claims.as_slice() {
        [] => Ok(UiDeclaredPostureLane::new(applicability, None)),
        observed if matches!(applicability, UiDeclaredPostureApplicability::NotApplicable) => Err(
            UiDeclaredPostureAdmissionDenial::LaneNotApplicableForFamily {
                family,
                lane,
                observed: observed.iter().map(|claim| (*claim).to_owned()).collect(),
            },
        ),
        observed
            if matches!(
                applicability,
                UiDeclaredPostureApplicability::ArchitecturallyOwnedButNotYetAdmitted
            ) =>
        {
            Err(
                UiDeclaredPostureAdmissionDenial::LaneArchitecturallyOwnedButNotYetAdmitted {
                    family,
                    lane,
                    observed: observed.iter().map(|claim| (*claim).to_owned()).collect(),
                },
            )
        }
        observed
            if matches!(
                applicability,
                UiDeclaredPostureApplicability::DiagnosticOnly
            ) =>
        {
            Err(
                UiDeclaredPostureAdmissionDenial::LaneNotApplicableForFamily {
                    family,
                    lane,
                    observed: observed.iter().map(|claim| (*claim).to_owned()).collect(),
                },
            )
        }
        [claim] => match parse(claim) {
            Some(posture) => Ok(UiDeclaredPostureLane::new(applicability, Some(posture))),
            None => Err(UiDeclaredPostureAdmissionDenial::InvalidLaneClaim {
                family,
                lane,
                observed: vec![(*claim).to_owned()],
            }),
        },
        observed => Err(UiDeclaredPostureAdmissionDenial::ContradictoryLaneClaims {
            family,
            lane,
            observed: observed.iter().map(|claim| (*claim).to_owned()).collect(),
        }),
    }
}
