use std::collections::BTreeSet;

use crate::declaration::UiDeclarationFamilyKind;

use super::{
    basis_source::measurement_basis_source_claim,
    constraint_modifier::measurement_constraint_modifier_claim,
    evidence_requirement::measurement_evidence_requirement_claim, mode::measurement_mode_claim,
    ownership_posture::measurement_ownership_posture_claim, UiDeclaredMeasurementBasisSource,
    UiDeclaredMeasurementEvidenceRequirement, UiDeclaredMeasurementOwnershipPosture,
    UiDeclaredMeasurementPolicyPosture,
};
use crate::declaration::{
    UiDeclaredPostureAdmissionDenial, UiDeclaredPostureApplicability, UiDeclaredPostureLane,
    UiDeclaredPostureLaneKind,
};

pub(crate) fn admit_measurement_policy_lane(
    family: UiDeclarationFamilyKind,
    posture_tokens: &[&str],
) -> Result<
    UiDeclaredPostureLane<UiDeclaredMeasurementPolicyPosture>,
    UiDeclaredPostureAdmissionDenial,
> {
    let claims = posture_tokens
        .iter()
        .copied()
        .filter(|token| token.starts_with("measurement:"))
        .collect::<Vec<_>>();
    let applicability = match family {
        UiDeclarationFamilyKind::Control
        | UiDeclarationFamilyKind::Page
        | UiDeclarationFamilyKind::PageSet
        | UiDeclarationFamilyKind::Region
        | UiDeclarationFamilyKind::Mosaic
        | UiDeclarationFamilyKind::LocalComposition
        | UiDeclarationFamilyKind::DiagnosticSurface => UiDeclaredPostureApplicability::Optional,
        UiDeclarationFamilyKind::QueryBinding | UiDeclarationFamilyKind::Intent => {
            UiDeclaredPostureApplicability::NotApplicable
        }
    };

    match claims.as_slice() {
        [] => Ok(UiDeclaredPostureLane::new(applicability, None)),
        observed if matches!(applicability, UiDeclaredPostureApplicability::NotApplicable) => Err(
            UiDeclaredPostureAdmissionDenial::LaneNotApplicableForFamily {
                family,
                lane: UiDeclaredPostureLaneKind::MeasurementPolicy,
                observed: observed.iter().map(|claim| (*claim).to_owned()).collect(),
            },
        ),
        observed => admit_measurement_claims(family, applicability, observed),
    }
}

fn admit_measurement_claims(
    family: UiDeclarationFamilyKind,
    applicability: UiDeclaredPostureApplicability,
    claims: &[&str],
) -> Result<
    UiDeclaredPostureLane<UiDeclaredMeasurementPolicyPosture>,
    UiDeclaredPostureAdmissionDenial,
> {
    let mut mode = None;
    let mut constraint_modifier = None;
    let mut basis_source = None;
    let mut ownership_posture = None;
    let mut evidence_requirements = BTreeSet::new();
    let mut observed_claims = BTreeSet::new();

    for claim in claims {
        if !observed_claims.insert(*claim) {
            return contradictory_measurement_claims(family, claims);
        }

        if let Some(mode_claim) = measurement_mode_claim(claim) {
            assign_measurement_axis(&mut mode, mode_claim, family, claims)?;
            continue;
        }
        if let Some(constraint_claim) = measurement_constraint_modifier_claim(claim) {
            assign_measurement_axis(&mut constraint_modifier, constraint_claim, family, claims)?;
            continue;
        }
        if let Some(basis_claim) = measurement_basis_source_claim(claim) {
            assign_measurement_axis(&mut basis_source, basis_claim, family, claims)?;
            continue;
        }
        if let Some(ownership_claim) = measurement_ownership_posture_claim(claim) {
            assign_measurement_axis(&mut ownership_posture, ownership_claim, family, claims)?;
            continue;
        }
        if let Some(evidence_claim) = measurement_evidence_requirement_claim(claim) {
            evidence_requirements.insert(evidence_claim);
            continue;
        }

        match *claim {
            "measurement:scroll-owned" => {
                assign_measurement_axis(
                    &mut basis_source,
                    UiDeclaredMeasurementBasisSource::ScrollViewport,
                    family,
                    claims,
                )?;
                assign_measurement_axis(
                    &mut ownership_posture,
                    UiDeclaredMeasurementOwnershipPosture::ScrollContainerBasis,
                    family,
                    claims,
                )?;
                evidence_requirements
                    .insert(UiDeclaredMeasurementEvidenceRequirement::ScrollContentExtent);
            }
            "measurement:portal-anchored" => {
                assign_measurement_axis(
                    &mut basis_source,
                    UiDeclaredMeasurementBasisSource::PortalAnchor,
                    family,
                    claims,
                )?;
                assign_measurement_axis(
                    &mut ownership_posture,
                    UiDeclaredMeasurementOwnershipPosture::PortalAnchorBasisRequired,
                    family,
                    claims,
                )?;
                evidence_requirements
                    .insert(UiDeclaredMeasurementEvidenceRequirement::PortalAnchorMetrics);
            }
            _ => return invalid_measurement_claim(family, claim),
        }
    }

    validate_measurement_axis_combinations(
        family,
        claims,
        basis_source,
        ownership_posture,
        evidence_requirements.iter().copied().collect(),
    )?;

    let admitted = UiDeclaredMeasurementPolicyPosture::new(
        mode,
        constraint_modifier,
        basis_source,
        ownership_posture,
        evidence_requirements.into_iter().collect(),
    )
    .expect("measurement admission must contain at least one semantic claim");

    Ok(UiDeclaredPostureLane::new(applicability, Some(admitted)))
}

fn assign_measurement_axis<T: Copy + Eq>(
    slot: &mut Option<T>,
    claim: T,
    family: UiDeclarationFamilyKind,
    claims: &[&str],
) -> Result<(), UiDeclaredPostureAdmissionDenial> {
    if let Some(existing) = *slot {
        if existing != claim {
            return contradictory_measurement_claims(family, claims);
        }
        return Ok(());
    }

    *slot = Some(claim);
    Ok(())
}

fn validate_measurement_axis_combinations(
    family: UiDeclarationFamilyKind,
    claims: &[&str],
    basis_source: Option<UiDeclaredMeasurementBasisSource>,
    ownership_posture: Option<UiDeclaredMeasurementOwnershipPosture>,
    evidence_requirements: Vec<UiDeclaredMeasurementEvidenceRequirement>,
) -> Result<(), UiDeclaredPostureAdmissionDenial> {
    validate_required_basis_for_ownership(family, claims, basis_source, ownership_posture)?;

    for evidence_requirement in evidence_requirements {
        validate_required_basis_for_evidence(family, claims, basis_source, evidence_requirement)?;
    }

    Ok(())
}

fn validate_required_basis_for_ownership(
    family: UiDeclarationFamilyKind,
    claims: &[&str],
    basis_source: Option<UiDeclaredMeasurementBasisSource>,
    ownership_posture: Option<UiDeclaredMeasurementOwnershipPosture>,
) -> Result<(), UiDeclaredPostureAdmissionDenial> {
    let Some(ownership_posture) = ownership_posture else {
        return Ok(());
    };
    let required_basis = match ownership_posture {
        UiDeclaredMeasurementOwnershipPosture::ScrollContainerBasis => {
            UiDeclaredMeasurementBasisSource::ScrollViewport
        }
        UiDeclaredMeasurementOwnershipPosture::PortalAnchorBasisRequired => {
            UiDeclaredMeasurementBasisSource::PortalAnchor
        }
    };

    validate_required_basis(
        family,
        claims,
        basis_source,
        required_basis,
        "measurement ownership posture requires a matching basis source",
    )
}

fn validate_required_basis_for_evidence(
    family: UiDeclarationFamilyKind,
    claims: &[&str],
    basis_source: Option<UiDeclaredMeasurementBasisSource>,
    evidence_requirement: UiDeclaredMeasurementEvidenceRequirement,
) -> Result<(), UiDeclaredPostureAdmissionDenial> {
    let required_basis = match evidence_requirement {
        UiDeclaredMeasurementEvidenceRequirement::HostFontMetrics => return Ok(()),
        UiDeclaredMeasurementEvidenceRequirement::ScrollContentExtent => {
            UiDeclaredMeasurementBasisSource::ScrollViewport
        }
        UiDeclaredMeasurementEvidenceRequirement::PortalAnchorMetrics => {
            UiDeclaredMeasurementBasisSource::PortalAnchor
        }
    };

    validate_required_basis(
        family,
        claims,
        basis_source,
        required_basis,
        "measurement evidence requirement requires a matching basis source",
    )
}

fn validate_required_basis(
    family: UiDeclarationFamilyKind,
    claims: &[&str],
    basis_source: Option<UiDeclaredMeasurementBasisSource>,
    required_basis: UiDeclaredMeasurementBasisSource,
    reason: &'static str,
) -> Result<(), UiDeclaredPostureAdmissionDenial> {
    match basis_source {
        Some(observed_basis) if observed_basis == required_basis => Ok(()),
        _ => impossible_measurement_claims(family, claims, reason),
    }
}

fn contradictory_measurement_claims<T>(
    family: UiDeclarationFamilyKind,
    claims: &[&str],
) -> Result<T, UiDeclaredPostureAdmissionDenial> {
    Err(UiDeclaredPostureAdmissionDenial::ContradictoryLaneClaims {
        family,
        lane: UiDeclaredPostureLaneKind::MeasurementPolicy,
        observed: claims.iter().map(|claim| (*claim).to_owned()).collect(),
    })
}

fn impossible_measurement_claims<T>(
    family: UiDeclarationFamilyKind,
    claims: &[&str],
    reason: &'static str,
) -> Result<T, UiDeclaredPostureAdmissionDenial> {
    Err(
        UiDeclaredPostureAdmissionDenial::ImpossibleLaneCombination {
            family,
            lane: UiDeclaredPostureLaneKind::MeasurementPolicy,
            observed: claims.iter().map(|claim| (*claim).to_owned()).collect(),
            reason,
        },
    )
}

fn invalid_measurement_claim<T>(
    family: UiDeclarationFamilyKind,
    claim: &str,
) -> Result<T, UiDeclaredPostureAdmissionDenial> {
    Err(UiDeclaredPostureAdmissionDenial::InvalidLaneClaim {
        family,
        lane: UiDeclaredPostureLaneKind::MeasurementPolicy,
        observed: vec![claim.to_owned()],
    })
}
