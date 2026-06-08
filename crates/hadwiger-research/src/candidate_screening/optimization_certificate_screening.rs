use crate::domain_artifacts::{
    GraphVersion, HadwigerArtifactReference, HadwigerArtifactShapeError, HadwigerCanonicalArtifact,
};
use crate::query_entry::HadwigerResearchHandle;

use super::evaluation::{
    CandidateScreeningEvaluation, CandidateScreeningEvaluationMode, CandidateScreeningVerdict,
};
use super::finite_graph_view::FiniteGraphView;
use super::optimization::{
    AutocorrelationOverlapCertificate, DensityCapCertificate, LocalDensityWindowCertificate,
    LovaszThetaCertificate, PeriodicColorClassMeasureModel, ScreeningMatrixCertificate,
    ScreeningRational, ScreeningSolverTranscript,
};
use super::{CandidateScreeningError, CandidateScreeningInvariantCatalog};
use crate::candidate_screening::CandidateScreeningInvariantFamily;

pub fn evaluate_lovasz_theta_screening_checked(
    _handle: &HadwigerResearchHandle,
    catalog: &CandidateScreeningInvariantCatalog,
    graph: &GraphVersion,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let graph_view = FiniteGraphView::from_graph_version(graph);
    let transcript = solver_transcript("clarabel", "lovasz_theta_sdp", graph)?;
    let certificate = lovasz_certificate_for_complete_graph(&graph_view, transcript)?;
    evaluate_lovasz_theta_certificate_checked(catalog, graph, certificate)
}

pub fn evaluate_lovasz_theta_certificate_checked(
    catalog: &CandidateScreeningInvariantCatalog,
    graph: &GraphVersion,
    certificate: LovaszThetaCertificate,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let family = CandidateScreeningInvariantFamily::LovaszThetaBound;
    require_catalog_family(catalog, family)?;
    let graph_view = FiniteGraphView::from_graph_version(graph);
    replay_lovasz_certificate(&graph_view, &certificate)?;
    CandidateScreeningEvaluation::new(
        catalog,
        family,
        graph.reference(),
        verdict_bool(certificate.lower_bound().cmp_integer(6).is_gt()),
        CandidateScreeningEvaluationMode::SolverBackedCertificate,
        format!("lovasz_theta_certificate={}", certificate.stable_token()),
    )
    .map_err(Into::into)
}

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

fn verdict_bool(rejects: bool) -> CandidateScreeningVerdict {
    if rejects {
        CandidateScreeningVerdict::Rejected
    } else {
        CandidateScreeningVerdict::Passed
    }
}

fn solver_transcript(
    solver_name: &str,
    lane: &str,
    graph: &GraphVersion,
) -> Result<ScreeningSolverTranscript, HadwigerArtifactShapeError> {
    ScreeningSolverTranscript::new(
        solver_name,
        "workspace",
        format!("{}:{}", lane, graph.reference().stable_token()),
        "certificate_candidate_generated",
    )
}

fn lovasz_certificate_for_complete_graph(
    graph_view: &FiniteGraphView,
    transcript: ScreeningSolverTranscript,
) -> Result<LovaszThetaCertificate, CandidateScreeningError> {
    if !graph_view.is_complete() {
        return Err(replay_error(
            CandidateScreeningInvariantFamily::LovaszThetaBound,
            "native_theta_generation_requires_complete_graph_special_case",
        ));
    }
    let dimension = graph_view.vertex_count();
    let mut entries = vec![vec![ScreeningRational::integer(0); dimension]; dimension];
    for (index, row) in entries.iter_mut().enumerate() {
        row[index] = ScreeningRational::integer(1);
    }
    let matrix = ScreeningMatrixCertificate::new(entries)?;
    Ok(LovaszThetaCertificate::new(
        "complete-graph-complement-empty-theta",
        ScreeningRational::integer(dimension as i128),
        matrix,
        transcript,
    )?)
}

fn replay_lovasz_certificate(
    graph_view: &FiniteGraphView,
    certificate: &LovaszThetaCertificate,
) -> Result<(), CandidateScreeningError> {
    let matrix = certificate.psd_witness();
    if matrix.dimension() != graph_view.vertex_count() {
        return Err(replay_error(
            CandidateScreeningInvariantFamily::LovaszThetaBound,
            "theta_matrix_dimension_mismatch",
        ));
    }
    for row in 0..matrix.dimension() {
        for column in 0..matrix.dimension() {
            if matrix.entry(row, column) != matrix.entry(column, row) {
                return Err(replay_error(
                    CandidateScreeningInvariantFamily::LovaszThetaBound,
                    "theta_matrix_not_symmetric",
                ));
            }
            if row != column && !matrix.entry(row, column).is_zero() {
                return Err(replay_error(
                    CandidateScreeningInvariantFamily::LovaszThetaBound,
                    "theta_psd_witness_not_diagonal_gram",
                ));
            }
        }
        if matrix.entry(row, row).is_negative() {
            return Err(replay_error(
                CandidateScreeningInvariantFamily::LovaszThetaBound,
                "theta_psd_diagonal_negative",
            ));
        }
    }
    let expected = graph_view.vertex_count() as i128;
    if graph_view.is_complete() && !certificate.lower_bound().cmp_integer(expected).is_eq() {
        return Err(replay_error(
            CandidateScreeningInvariantFamily::LovaszThetaBound,
            "complete_graph_theta_bound_mismatch",
        ));
    }
    Ok(())
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
