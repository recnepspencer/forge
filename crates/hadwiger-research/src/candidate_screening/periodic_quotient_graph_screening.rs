use crate::domain_artifacts::core_artifact::canonical_digest_token;
use crate::domain_artifacts::{HadwigerArtifactReference, HadwigerDeclaredFamilyCheckedExt};
use crate::domain_declarations::{
    declare_research_request_checked, PeriodicQuotientGraphScreeningDeclaration,
};
use crate::query_entry::HadwigerResearchHandle;

use super::evaluation::{
    CandidateScreeningEvaluation, CandidateScreeningEvaluationMode, CandidateScreeningVerdict,
};
use super::optimization::{PeriodicQuotientConflictCertificate, PeriodicQuotientRectangleModel};
use super::{CandidateScreeningError, CandidateScreeningInvariantCatalog};
use crate::candidate_screening::CandidateScreeningInvariantFamily;

pub fn evaluate_periodic_quotient_graph_screening_checked(
    handle: &HadwigerResearchHandle,
    catalog: &CandidateScreeningInvariantCatalog,
    subject: HadwigerArtifactReference,
    model: PeriodicQuotientRectangleModel,
    certificate: PeriodicQuotientConflictCertificate,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let family = CandidateScreeningInvariantFamily::PeriodicQuotientGraph;
    require_catalog_family(catalog, family)?;
    let declaration = declare_research_request_checked(
        handle,
        PeriodicQuotientGraphScreeningDeclaration::new(
            subject.stable_token(),
            model.stable_token(),
            certificate.stable_token(),
        ),
    )
    .admitted()
    .ok_or(CandidateScreeningError::SolverCandidateUnavailable {
        family,
        reason: "query_periodic_quotient_screening_declaration_not_admitted",
    })?;
    replay_periodic_quotient_conflict(&model, &certificate)?;
    CandidateScreeningEvaluation::new(
        catalog,
        family,
        subject,
        CandidateScreeningVerdict::Rejected,
        CandidateScreeningEvaluationMode::SolverBackedCertificate,
        format!(
            "query_declaration_digest={};periodic_quotient_model={};periodic_quotient_conflict_certificate={}",
            canonical_digest_token(declaration.declaration_digest()),
            model.stable_token(),
            certificate.stable_token()
        ),
    )
    .map_err(Into::into)
}

fn replay_periodic_quotient_conflict(
    model: &PeriodicQuotientRectangleModel,
    certificate: &PeriodicQuotientConflictCertificate,
) -> Result<(), CandidateScreeningError> {
    let left = model
        .tile(certificate.left_tile_id())
        .ok_or_else(|| replay_error("unknown_left_periodic_tile"))?;
    let right = model
        .tile(certificate.right_tile_id())
        .ok_or_else(|| replay_error("unknown_right_periodic_tile"))?;
    if left.color_id() != right.color_id() {
        return Err(replay_error("periodic_conflict_tiles_not_same_color"));
    }
    let translated = right
        .region()
        .translated(certificate.translation_dx(), certificate.translation_dy());
    if !left.region().unit_circle_intersects_difference(&translated) {
        return Err(replay_error(
            "periodic_translated_pair_has_no_unit_conflict",
        ));
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
        family: CandidateScreeningInvariantFamily::PeriodicQuotientGraph,
        reason,
    }
}
