use worth_foundational::facade::{
    canonical_basis_value_for_aspect_value, canonicalization, compare_canonical_basis,
    prepare_canonical_basis_sequence, prepare_canonical_comparison, AspectValue,
    CanonicalBasisDomain, CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus,
    CanonicalBasisReadyArtifact, CanonicalBasisValue, CanonicalComparisonOutcome,
    CanonicalDigestAlgorithmId, CanonicalDigestDerivationDenial, CanonicalDigestId,
    CanonicalDigestWorkBudget, CanonicalEquivalenceBasis, CanonicalizationRuleVersion,
};
use worth_query_installation::facade::WorthQueryCanonicalWorkEvidence;

const PARAMETER_DOMAIN: &str = "worth-query.application-query-parameters";
const PARAMETER_RULE: &str = "worth-query-application-query-parameters-v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationParameterCanonicalArtifact {
    basis: CanonicalBasisReadyArtifact,
    digest: CanonicalDigestId,
    work: WorthQueryCanonicalWorkEvidence,
}

impl WorthQueryApplicationParameterCanonicalArtifact {
    pub fn basis(&self) -> &CanonicalBasisReadyArtifact {
        &self.basis
    }

    pub const fn identity(&self) -> &CanonicalDigestId {
        &self.digest
    }

    pub const fn digest(&self) -> &CanonicalDigestId {
        &self.digest
    }

    pub const fn work(&self) -> WorthQueryCanonicalWorkEvidence {
        self.work
    }

    pub fn is_equivalent_to(&self, other: &Self) -> bool {
        let comparison = prepare_canonical_comparison(
            CanonicalEquivalenceBasis::ExactCanonicalBasis,
            self.basis.clone(),
            other.basis.clone(),
        )
        .into_result()
        .expect("application-query parameter artifacts share one comparison family");
        matches!(
            compare_canonical_basis(&comparison),
            CanonicalComparisonOutcome::Equivalent(_)
        )
    }
}

pub(super) fn prepare_parameter_basis(
    bindings: &[(&'static str, AspectValue)],
    budget: CanonicalDigestWorkBudget,
) -> Result<WorthQueryApplicationParameterCanonicalArtifact, CanonicalDigestDerivationDenial> {
    let domain = CanonicalBasisDomain::Future(PARAMETER_DOMAIN);
    let mut entries = vec![entry(
        domain,
        "binding-count".to_owned(),
        CanonicalBasisValue::UnsignedInteger {
            width: worth_foundational::facade::CanonicalIntegerWidth::Bits64,
            value: bindings.len() as u128,
        },
    )];
    for (index, (name, value)) in bindings.iter().enumerate() {
        let path = format!("parameter[{index}]");
        entries.extend([
            entry(
                domain,
                format!("{path}.name"),
                CanonicalBasisValue::ExactText((*name).into()),
            ),
            entry(
                domain,
                format!("{path}.scalar"),
                CanonicalBasisValue::ExactText(value.value_family().canonical_name().into()),
            ),
            entry(
                domain,
                format!("{path}.value"),
                canonical_basis_value_for_aspect_value(value),
            ),
        ]);
    }
    let version = CanonicalizationRuleVersion::new(PARAMETER_RULE)
        .expect("the fixed application-query parameter rule is valid");
    let basis = prepare_canonical_basis_sequence(version, domain, entries)
        .into_result()
        .expect("the application-query parameter basis always has its count entry");
    let ready = canonicalization()
        .digest()
        .for_sequence_with_budget(basis.clone(), CanonicalDigestAlgorithmId::sha256(), budget)
        .into_result()?;
    let derived = canonicalization().digest().derive(ready);
    Ok(WorthQueryApplicationParameterCanonicalArtifact {
        basis,
        digest: CanonicalDigestId::new(*derived.value().bytes()),
        work: WorthQueryCanonicalWorkEvidence::one_digest(derived.metadata().work()),
    })
}

fn entry(
    domain: CanonicalBasisDomain,
    locus: String,
    value: CanonicalBasisValue,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        domain,
        CanonicalBasisLocus::Named(locus.into()),
        CanonicalBasisEntryKind::Field,
        value,
    )
}
