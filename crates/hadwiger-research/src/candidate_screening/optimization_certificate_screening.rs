use crate::domain_artifacts::HadwigerArtifactReference;
use crate::query_entry::HadwigerResearchHandle;

use super::evaluation::{
    CandidateScreeningEvaluation, CandidateScreeningEvaluationMode, CandidateScreeningVerdict,
};
use super::optimization::{
    AutocorrelationOverlapCertificate, DensityCapCertificate, LocalDensityWindowCertificate,
    PeriodicColorClassMeasureModel, ScreeningRational,
};
use super::{CandidateScreeningError, CandidateScreeningInvariantCatalog};
use crate::candidate_screening::CandidateScreeningInvariantFamily;

pub fn evaluate_autocorrelation_zero_screening_checked(
    _handle: &HadwigerResearchHandle,
    catalog: &CandidateScreeningInvariantCatalog,
    subject: HadwigerArtifactReference,
    model: PeriodicColorClassMeasureModel,
    certificate: AutocorrelationOverlapCertificate,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let family = CandidateScreeningInvariantFamily::AutocorrelationZero;
    require_catalog_family(catalog, family)?;
    replay_autocorrelation_certificate(&model, &certificate)?;
    measure_evaluation(catalog, family, subject, &model, certificate.stable_token())
}

pub fn evaluate_density_cap_screening_checked(
    _handle: &HadwigerResearchHandle,
    catalog: &CandidateScreeningInvariantCatalog,
    subject: HadwigerArtifactReference,
    model: PeriodicColorClassMeasureModel,
    certificate: DensityCapCertificate,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let family = CandidateScreeningInvariantFamily::DensityCapEachColorClass;
    require_catalog_family(catalog, family)?;
    replay_density_cap_certificate(&model, &certificate)?;
    measure_evaluation(catalog, family, subject, &model, certificate.stable_token())
}

pub fn evaluate_local_density_window_screening_checked(
    _handle: &HadwigerResearchHandle,
    catalog: &CandidateScreeningInvariantCatalog,
    subject: HadwigerArtifactReference,
    model: PeriodicColorClassMeasureModel,
    certificate: LocalDensityWindowCertificate,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let family = CandidateScreeningInvariantFamily::LocalDensityWindow;
    require_catalog_family(catalog, family)?;
    replay_local_density_window_certificate(&model, &certificate)?;
    measure_evaluation(catalog, family, subject, &model, certificate.stable_token())
}

fn measure_evaluation(
    catalog: &CandidateScreeningInvariantCatalog,
    family: CandidateScreeningInvariantFamily,
    subject: HadwigerArtifactReference,
    model: &PeriodicColorClassMeasureModel,
    certificate_token: String,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    CandidateScreeningEvaluation::new(
        catalog,
        family,
        subject,
        CandidateScreeningVerdict::Rejected,
        CandidateScreeningEvaluationMode::SolverBackedCertificate,
        format!(
            "periodic_measure_model={};certificate={}",
            model.stable_token(),
            certificate_token
        ),
    )
    .map_err(Into::into)
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

fn replay_autocorrelation_certificate(
    model: &PeriodicColorClassMeasureModel,
    certificate: &AutocorrelationOverlapCertificate,
) -> Result<(), CandidateScreeningError> {
    let squared_length = certificate
        .dx()
        .mul(certificate.dx())
        .add(&certificate.dy().mul(certificate.dy()));
    if squared_length.cmp_integer(1).is_ne() {
        return Err(replay_error(
            CandidateScreeningInvariantFamily::AutocorrelationZero,
            "autocorrelation_displacement_not_unit",
        ));
    }
    let mut overlap = ScreeningRational::integer(0);
    let same_color = model
        .cells()
        .iter()
        .filter(|cell| cell.color_id() == certificate.color_id())
        .collect::<Vec<_>>();
    for left in &same_color {
        for right in &same_color {
            overlap = overlap.add(&left.overlap_area_after_translation(
                right,
                certificate.dx(),
                certificate.dy(),
            ));
        }
    }
    if overlap != *certificate.claimed_overlap_area() || !overlap.is_positive() {
        return Err(replay_error(
            CandidateScreeningInvariantFamily::AutocorrelationZero,
            "autocorrelation_overlap_not_positive",
        ));
    }
    Ok(())
}

fn replay_density_cap_certificate(
    model: &PeriodicColorClassMeasureModel,
    certificate: &DensityCapCertificate,
) -> Result<(), CandidateScreeningError> {
    let color_area = model.color_area(certificate.color_id());
    let allowed = model.period_area().mul(certificate.density_cap());
    if color_area <= allowed {
        return Err(replay_error(
            CandidateScreeningInvariantFamily::DensityCapEachColorClass,
            "density_does_not_exceed_cap",
        ));
    }
    Ok(())
}

fn replay_local_density_window_certificate(
    model: &PeriodicColorClassMeasureModel,
    certificate: &LocalDensityWindowCertificate,
) -> Result<(), CandidateScreeningError> {
    let color_area = model
        .cells()
        .iter()
        .filter(|cell| cell.color_id() == certificate.color_id())
        .fold(ScreeningRational::integer(0), |sum, cell| {
            sum.add(&certificate.window().overlap_area(cell))
        });
    let allowed = certificate.window().area().mul(certificate.density_cap());
    if color_area <= allowed {
        return Err(replay_error(
            CandidateScreeningInvariantFamily::LocalDensityWindow,
            "local_density_does_not_exceed_cap",
        ));
    }
    Ok(())
}

fn replay_error(
    family: CandidateScreeningInvariantFamily,
    reason: &'static str,
) -> CandidateScreeningError {
    CandidateScreeningError::CertificateReplayRejected { family, reason }
}
