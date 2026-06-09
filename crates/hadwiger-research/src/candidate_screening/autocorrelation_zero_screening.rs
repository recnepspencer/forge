use crate::domain_artifacts::core_artifact::canonical_digest_token;
use crate::domain_artifacts::{HadwigerArtifactReference, HadwigerDeclaredFamilyCheckedExt};
use crate::domain_declarations::{
    declare_research_request_checked, AutocorrelationZeroScreeningDeclaration,
};
use crate::query_entry::HadwigerResearchHandle;

use super::evaluation::{
    CandidateScreeningEvaluation, CandidateScreeningEvaluationMode, CandidateScreeningVerdict,
};
use super::optimization::{AutocorrelationOverlapCertificate, PeriodicColorClassMeasureModel};
use super::{CandidateScreeningError, CandidateScreeningInvariantCatalog};
use crate::candidate_screening::CandidateScreeningInvariantFamily;

pub fn evaluate_autocorrelation_zero_screening_checked(
    handle: &HadwigerResearchHandle,
    catalog: &CandidateScreeningInvariantCatalog,
    subject: HadwigerArtifactReference,
    model: PeriodicColorClassMeasureModel,
    certificate: AutocorrelationOverlapCertificate,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let family = CandidateScreeningInvariantFamily::AutocorrelationZero;
    require_catalog_family(catalog, family)?;
    let declaration = declare_research_request_checked(
        handle,
        AutocorrelationZeroScreeningDeclaration::new(
            subject.stable_token(),
            model.stable_token(),
            "periodic_rational_rectangular_cells_exact_unit_displacement",
        ),
    )
    .admitted()
    .ok_or(CandidateScreeningError::SolverCandidateUnavailable {
        family,
        reason: "query_autocorrelation_screening_declaration_not_admitted",
    })?;
    replay_autocorrelation_certificate(&model, &certificate)?;
    CandidateScreeningEvaluation::new(
        catalog,
        family,
        subject,
        CandidateScreeningVerdict::Rejected,
        CandidateScreeningEvaluationMode::SolverBackedCertificate,
        format!(
            "query_declaration_digest={};periodic_measure_model={};autocorrelation_certificate={}",
            canonical_digest_token(declaration.declaration_digest()),
            model.stable_token(),
            certificate.stable_token()
        ),
    )
    .map_err(Into::into)
}

fn replay_autocorrelation_certificate(
    model: &PeriodicColorClassMeasureModel,
    certificate: &AutocorrelationOverlapCertificate,
) -> Result<(), CandidateScreeningError> {
    require_exact_unit_displacement(certificate)?;
    let overlap = model.same_color_translated_overlap_area(
        certificate.color_id(),
        certificate.dx(),
        certificate.dy(),
    );
    if overlap != *certificate.claimed_overlap_area() || !overlap.is_positive() {
        return Err(replay_error("autocorrelation_overlap_not_positive"));
    }
    Ok(())
}

fn require_exact_unit_displacement(
    certificate: &AutocorrelationOverlapCertificate,
) -> Result<(), CandidateScreeningError> {
    let squared_length = certificate
        .dx()
        .mul(certificate.dx())
        .add(&certificate.dy().mul(certificate.dy()));
    if squared_length.cmp_integer(1).is_ne() {
        return Err(replay_error("autocorrelation_displacement_not_unit"));
    }
    Ok(())
}

fn require_catalog_family(
    catalog: &CandidateScreeningInvariantCatalog,
    family: CandidateScreeningInvariantFamily,
) -> Result<(), CandidateScreeningError> {
    if catalog.has_family(family) {
        Ok(())
    } else {
        Err(CandidateScreeningError::MissingInvariantFamily(family))
    }
}

fn replay_error(reason: &'static str) -> CandidateScreeningError {
    CandidateScreeningError::CertificateReplayRejected {
        family: CandidateScreeningInvariantFamily::AutocorrelationZero,
        reason,
    }
}
