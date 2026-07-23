use crate::domain_computation::{
    WorthQueryCandidateSearchPosture as Posture, WorthQueryPortableArtifactContract,
};

use super::{
    portable_text, WorthQueryArtifactContractValidationDenial,
    WorthQueryArtifactContractValidationDenialKind as Kind,
};

pub(super) fn validate(
    contract: &WorthQueryPortableArtifactContract,
) -> Result<(), WorthQueryArtifactContractValidationDenial> {
    let search = &contract.search;
    let declared_fields = [
        search.universe_family(),
        search.termination_family(),
        search.feasibility_family(),
        search.comparison_family(),
    ];
    let fields_match = match search.search_posture() {
        Posture::NotApplicable => declared_fields.iter().all(|field| field.is_none()),
        _ => declared_fields
            .iter()
            .all(|field| field.is_some_and(portable_text)),
    };
    (fields_match && search.postures_are_coherent())
        .then_some(())
        .ok_or_else(|| {
            WorthQueryArtifactContractValidationDenial::new(
                Kind::InvalidSearchContract,
                contract.family.as_str(),
            )
        })
}
