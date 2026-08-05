use worth_foundational::facade::{
    canonicalization, prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisValue, CanonicalDigestAlgorithmId,
    CanonicalDigestId, CanonicalIntegerWidth, CanonicalizationRuleVersion,
};

const DOMAIN: CanonicalBasisDomain = CanonicalBasisDomain::Future("worth-bank.proposal-identity");
const RULE_VERSION: &str = "worth-bank-proposal-identity-v2";

pub(crate) struct CanonicalProposalPayload {
    operation: &'static str,
    entries: Vec<CanonicalBasisEntry>,
}

impl CanonicalProposalPayload {
    pub(crate) fn new(operation: &'static str) -> Self {
        Self {
            operation,
            entries: vec![entry(
                "operation",
                CanonicalBasisValue::ExactText(operation.into()),
            )],
        }
    }

    pub(crate) fn u64(mut self, locus: &'static str, value: u64) -> Self {
        self.entries.push(entry(
            locus,
            CanonicalBasisValue::UnsignedInteger {
                width: CanonicalIntegerWidth::Bits64,
                value: value.into(),
            },
        ));
        self
    }

    pub(crate) fn i64(mut self, locus: &'static str, value: i64) -> Self {
        self.entries.push(entry(
            locus,
            CanonicalBasisValue::SignedInteger {
                width: CanonicalIntegerWidth::Bits64,
                value: value.into(),
            },
        ));
        self
    }

    pub(crate) fn text(mut self, locus: &'static str, value: &str) -> Self {
        self.entries.push(entry(
            locus,
            CanonicalBasisValue::ExactText(value.to_owned().into()),
        ));
        self
    }

    pub(crate) fn byte(mut self, locus: &'static str, value: u8) -> Self {
        self.entries.push(entry(
            locus,
            CanonicalBasisValue::UnsignedInteger {
                width: CanonicalIntegerWidth::Bits8,
                value: value.into(),
            },
        ));
        self
    }

    pub(crate) const fn operation(&self) -> &'static str {
        self.operation
    }

    pub(crate) fn derive_identity(self) -> CanonicalDigestId {
        let version = CanonicalizationRuleVersion::new(RULE_VERSION)
            .expect("the fixed bank proposal identity rule is valid");
        let basis = prepare_canonical_basis_sequence(version, DOMAIN, self.entries)
            .into_result()
            .expect("bank proposal identity fields have unique typed loci");
        let ready = canonicalization()
            .digest()
            .for_sequence(basis, CanonicalDigestAlgorithmId::sha256())
            .into_result()
            .expect("SHA-256 admits the typed bank proposal identity basis");
        CanonicalDigestId::new(*canonicalization().digest().derive(ready).value().bytes())
    }
}

fn entry(locus: &'static str, value: CanonicalBasisValue) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        DOMAIN,
        CanonicalBasisLocus::Named(locus.into()),
        CanonicalBasisEntryKind::Identity,
        value,
    )
}
