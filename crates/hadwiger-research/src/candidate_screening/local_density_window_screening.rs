use crate::domain_artifacts::core_artifact::canonical_digest_token;
use crate::domain_artifacts::{HadwigerArtifactReference, HadwigerDeclaredFamilyCheckedExt};
use crate::domain_declarations::{
    declare_research_request_checked, LocalDensityWindowScreeningDeclaration,
};
use crate::query_entry::HadwigerResearchHandle;

use super::evaluation::{
    CandidateScreeningEvaluation, CandidateScreeningEvaluationMode, CandidateScreeningVerdict,
};
use super::optimization::{LocalDensityWindowCertificate, PeriodicColorClassMeasureModel};
use super::{CandidateScreeningError, CandidateScreeningInvariantCatalog};
use crate::candidate_screening::CandidateScreeningInvariantFamily;

pub fn evaluate_local_density_window_screening_checked(
    handle: &HadwigerResearchHandle,
    catalog: &CandidateScreeningInvariantCatalog,
    subject: HadwigerArtifactReference,
    model: PeriodicColorClassMeasureModel,
    certificate: LocalDensityWindowCertificate,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let family = CandidateScreeningInvariantFamily::LocalDensityWindow;
    require_catalog_family(catalog, family)?;
    let declaration = declare_research_request_checked(
        handle,
        LocalDensityWindowScreeningDeclaration::new(
            subject.stable_token(),
            model.stable_token(),
            certificate.window().stable_token(),
            certificate.color_id(),
            certificate.retained_bound_reference(),
        ),
    )
    .admitted()
    .ok_or(CandidateScreeningError::SolverCandidateUnavailable {
        family,
        reason: "query_local_density_window_screening_declaration_not_admitted",
    })?;
    replay_local_density_window_certificate(&model, &certificate)?;
    CandidateScreeningEvaluation::new(
        catalog,
        family,
        subject,
        CandidateScreeningVerdict::Rejected,
        CandidateScreeningEvaluationMode::SolverBackedCertificate,
        format!(
            "query_declaration_digest={};periodic_measure_model={};local_density_window_certificate={}",
            canonical_digest_token(declaration.declaration_digest()),
            model.stable_token(),
            certificate.stable_token()
        ),
    )
    .map_err(Into::into)
}

fn replay_local_density_window_certificate(
    model: &PeriodicColorClassMeasureModel,
    certificate: &LocalDensityWindowCertificate,
) -> Result<(), CandidateScreeningError> {
    let color_area = model.color_area_in_window(certificate.color_id(), certificate.window());
    let allowed = certificate.window().area().mul(certificate.density_cap());
    if color_area <= allowed {
        return Err(replay_error("local_density_does_not_exceed_cap"));
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
        family: CandidateScreeningInvariantFamily::LocalDensityWindow,
        reason,
    }
}
