use crate::domain_artifacts::HadwigerArtifactReference;
use crate::domain_declarations::SubstitutionConsistencyScreeningDeclaration;
use crate::query_entry::HadwigerResearchHandle;

use super::evaluation::{CandidateScreeningEvaluation, CandidateScreeningVerdict};
use super::graph_embedding_screening_support::{
    declare_screening_request, require_catalog_family, screening_evaluation,
};
use super::optimization::SubstitutionConsistencyCertificate;
use super::{CandidateScreeningError, CandidateScreeningInvariantCatalog};
use crate::candidate_screening::CandidateScreeningInvariantFamily;

pub fn evaluate_substitution_consistency_screening_checked(
    handle: &HadwigerResearchHandle,
    catalog: &CandidateScreeningInvariantCatalog,
    subject: HadwigerArtifactReference,
    certificate: SubstitutionConsistencyCertificate,
) -> Result<CandidateScreeningEvaluation, CandidateScreeningError> {
    let family = CandidateScreeningInvariantFamily::SubstitutionConsistency;
    require_catalog_family(catalog, family)?;
    let query_digest = declare_screening_request(
        handle,
        family,
        SubstitutionConsistencyScreeningDeclaration::new(
            subject.stable_token(),
            certificate.stable_token(),
        ),
        "query_substitution_consistency_screening_declaration_not_admitted",
    )?;
    let failures = certificate
        .failures()
        .iter()
        .map(|failure| failure.as_str())
        .collect::<Vec<_>>();
    screening_evaluation(
        catalog,
        family,
        subject,
        if failures.is_empty() {
            CandidateScreeningVerdict::Passed
        } else {
            CandidateScreeningVerdict::Rejected
        },
        &query_digest,
        format!("substitution_failures={}", failures.join(".")),
    )
}
