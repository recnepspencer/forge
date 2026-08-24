use worth_foundational::facade::{
    canonicalization, CanonicalDigestAlgorithmId, CanonicalDigestDerivationDenial,
    CanonicalDigestId, CanonicalDigestWorkBudget,
};
use worth_query_declaration::facade::application_schema::ApplicationSchemaIdentity;

use crate::canonical_work::WorthQueryCanonicalWorkEvidence;

const INSTALLATION_BUDGET: CanonicalDigestWorkBudget =
    match CanonicalDigestWorkBudget::new(32_768, 4 * 1_024 * 1_024) {
        Some(budget) => budget,
        None => panic!("fixed application-schema installation budget is valid"),
    };

pub(crate) fn derive_installed_schema_identity(
    identity: &ApplicationSchemaIdentity,
) -> Result<(CanonicalDigestId, WorthQueryCanonicalWorkEvidence), CanonicalDigestDerivationDenial> {
    derive_with_budget(identity, INSTALLATION_BUDGET)
}

fn derive_with_budget(
    identity: &ApplicationSchemaIdentity,
    budget: CanonicalDigestWorkBudget,
) -> Result<(CanonicalDigestId, WorthQueryCanonicalWorkEvidence), CanonicalDigestDerivationDenial> {
    let ready = canonicalization()
        .digest()
        .for_sequence_with_budget(
            identity.canonical_basis().clone(),
            CanonicalDigestAlgorithmId::sha256(),
            budget,
        )
        .into_result()?;
    let derived = canonicalization().digest().derive(ready);
    Ok((
        CanonicalDigestId::new(*derived.value().bytes()),
        WorthQueryCanonicalWorkEvidence::one_digest(derived.metadata().work()),
    ))
}

#[cfg(test)]
pub(crate) fn derive_installed_schema_identity_with_budget(
    identity: &ApplicationSchemaIdentity,
    budget: CanonicalDigestWorkBudget,
) -> Result<(CanonicalDigestId, WorthQueryCanonicalWorkEvidence), CanonicalDigestDerivationDenial> {
    derive_with_budget(identity, budget)
}
