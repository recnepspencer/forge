use worth_foundational::facade::{
    canonicalization, prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisValue, CanonicalDigestAlgorithmId,
    CanonicalDigestDerivationDenial, CanonicalDigestId, CanonicalDigestWorkBudget,
    CanonicalIntegerWidth, CanonicalizationRuleVersion,
};
use worth_query_declaration::facade::application_schema::ApplicationSchemaBindingIdentity;
use worth_query_installation::facade::WorthQueryCanonicalWorkEvidence;

use super::{WorthQueryAuthenticationAudience, WorthQueryAuthenticationMethod};

const DOMAIN: CanonicalBasisDomain =
    CanonicalBasisDomain::Future("worth-query.authentication-adapter");
const RULE: &str = "worth-query-authentication-adapter-v2";
const ADAPTER_BUDGET: CanonicalDigestWorkBudget =
    match CanonicalDigestWorkBudget::new(7, 64 * 1_024) {
        Some(budget) => budget,
        None => panic!("fixed authentication adapter canonical-work budget is valid"),
    };

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryAuthenticationAdapterIdentity {
    digest: CanonicalDigestId,
    work: WorthQueryCanonicalWorkEvidence,
}

impl WorthQueryAuthenticationAdapterIdentity {
    pub const fn digest(&self) -> &CanonicalDigestId {
        &self.digest
    }

    pub const fn canonical_work(&self) -> WorthQueryCanonicalWorkEvidence {
        self.work
    }

    pub fn render_support_hex(&self) -> String {
        self.digest.render_hex()
    }
}

pub(super) fn adapter_identity(
    binding: &ApplicationSchemaBindingIdentity,
    configuration: &str,
    audience: &WorthQueryAuthenticationAudience,
    method: &WorthQueryAuthenticationMethod,
) -> Result<WorthQueryAuthenticationAdapterIdentity, CanonicalDigestDerivationDenial> {
    let entries = vec![
        text("configuration", configuration),
        digest("package", binding.package_identity()),
        digest("schema", binding.schema_identity()),
        text("audience", audience.as_str()),
        text("method", method.as_str()),
        unsigned("runtime", binding.runtime_ordinal()),
        unsigned("generation", binding.generation()),
    ];
    let version =
        CanonicalizationRuleVersion::new(RULE).expect("the fixed adapter identity rule is valid");
    let basis = prepare_canonical_basis_sequence(version, DOMAIN, entries)
        .into_result()
        .expect("adapter identity always has a nonempty canonical basis");
    let ready = canonicalization()
        .digest()
        .for_sequence_with_budget(basis, CanonicalDigestAlgorithmId::sha256(), ADAPTER_BUDGET)
        .into_result()?;
    let derived = canonicalization().digest().derive(ready);
    Ok(WorthQueryAuthenticationAdapterIdentity {
        digest: CanonicalDigestId::new(*derived.value().bytes()),
        work: WorthQueryCanonicalWorkEvidence::one_digest(derived.metadata().work()),
    })
}

fn text(locus: &'static str, value: &str) -> CanonicalBasisEntry {
    entry(
        locus,
        CanonicalBasisValue::ExactText(value.to_owned().into()),
    )
}

fn unsigned(locus: &'static str, value: u64) -> CanonicalBasisEntry {
    entry(
        locus,
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits64,
            value: value.into(),
        },
    )
}

fn digest(locus: &'static str, value: &CanonicalDigestId) -> CanonicalBasisEntry {
    entry(locus, CanonicalBasisValue::BytesDigest(*value))
}

fn entry(locus: &'static str, value: CanonicalBasisValue) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        DOMAIN,
        CanonicalBasisLocus::Named(locus.into()),
        CanonicalBasisEntryKind::Field,
        value,
    )
}
