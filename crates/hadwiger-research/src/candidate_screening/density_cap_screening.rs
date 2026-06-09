use crate::domain_artifacts::core_artifact::canonical_digest_token;
use crate::domain_artifacts::{HadwigerArtifactReference, HadwigerDeclaredFamilyCheckedExt};
use crate::domain_declarations::{
    declare_research_request_checked, DensityCapScreeningDeclaration,
};
use crate::query_entry::HadwigerResearchHandle;

use super::evaluation::{
    CandidateScreeningEvaluation, CandidateScreeningEvaluationMode, CandidateScreeningVerdict,
};
use super::optimization::{DensityCapCertificate, PeriodicColorClassMeasureModel};
use super::{CandidateScreeningError, CandidateScreeningInvariantCatalog};
use crate::candidate_screening::CandidateScreeningInvariantFamily;

pub fn evaluate_density_cap_screening_checked(
    handle: &HadwigerResearchHandle,
    catalog: &CandidateScreeningInvariantCatalog,
    subject: HadwigerArtifactReference,
    model: PeriodicColorClassMeasureModel,
    certificate: DensityCapCertificate,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let family = CandidateScreeningInvariantFamily::DensityCapEachColorClass;
    require_catalog_family(catalog, family)?;
    let declaration = declare_research_request_checked(
        handle,
        DensityCapScreeningDeclaration::new(
            subject.stable_token(),
            model.stable_token(),
            certificate.color_id(),
            certificate.retained_cap_reference(),
        ),
    )
    .admitted()
    .ok_or(CandidateScreeningError::SolverCandidateUnavailable {
        family,
        reason: "query_density_cap_screening_declaration_not_admitted",
    })?;
    replay_density_cap_certificate(&model, &certificate)?;
    CandidateScreeningEvaluation::new(
        catalog,
        family,
        subject,
        CandidateScreeningVerdict::Rejected,
        CandidateScreeningEvaluationMode::SolverBackedCertificate,
        format!(
            "query_declaration_digest={};periodic_measure_model={};density_cap_certificate={}",
            canonical_digest_token(declaration.declaration_digest()),
            model.stable_token(),
            certificate.stable_token()
        ),
    )
    .map_err(Into::into)
}

fn replay_density_cap_certificate(
    model: &PeriodicColorClassMeasureModel,
    certificate: &DensityCapCertificate,
) -> Result<(), CandidateScreeningError> {
    let color_area = model.color_area(certificate.color_id());
    let allowed = model.period_area().mul(certificate.density_cap());
    if color_area <= allowed {
        return Err(replay_error("density_does_not_exceed_cap"));
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
        family: CandidateScreeningInvariantFamily::DensityCapEachColorClass,
        reason,
    }
}
