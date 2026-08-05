use std::sync::atomic::{AtomicUsize, Ordering};

use worth_foundational::facade::{
    admit_canonical_sequence_digest_derivation, derive_canonical_digest,
    prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisValue, CanonicalDigestAlgorithmId,
    CanonicalIntegerWidth, CanonicalSingleSequenceDigestAlgorithmSlot, CanonicalizationRuleVersion,
};
use worth_query_declaration::facade::application_capability::ApplicationCapabilityGovernedInputIdentity;

static CANONICAL_MATERIALIZATION_COUNT: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CapabilityGovernedInputIdentity {
    #[default]
    None,
    FixedU64([u64; 4]),
    Canonical,
}

impl CapabilityGovernedInputIdentity {
    pub(super) fn materialize(
        self,
        amount: u64,
    ) -> Option<ApplicationCapabilityGovernedInputIdentity> {
        match self {
            Self::None => None,
            Self::FixedU64(values) => {
                Some(ApplicationCapabilityGovernedInputIdentity::four_u64(values))
            }
            Self::Canonical => {
                CANONICAL_MATERIALIZATION_COUNT.fetch_add(1, Ordering::Relaxed);
                Some(canonical_amount_identity(amount))
            }
        }
    }
}

pub fn canonical_governed_input_materialization_count() -> usize {
    CANONICAL_MATERIALIZATION_COUNT.load(Ordering::Relaxed)
}

fn canonical_amount_identity(amount: u64) -> ApplicationCapabilityGovernedInputIdentity {
    let domain = CanonicalBasisDomain::Future("worth-query.test.capability-governed-input");
    let version = CanonicalizationRuleVersion::new("worth-query.test.capability-governed-input.v1")
        .expect("the governed-input fixture version is static and valid");
    let basis = prepare_canonical_basis_sequence(
        version.clone(),
        domain,
        [CanonicalBasisEntry::new(
            domain,
            CanonicalBasisLocus::Named("amount".into()),
            CanonicalBasisEntryKind::Value,
            CanonicalBasisValue::UnsignedInteger {
                width: CanonicalIntegerWidth::Bits64,
                value: amount.into(),
            },
        )],
    )
    .into_result()
    .expect("the governed-input fixture basis is valid");
    let ready = admit_canonical_sequence_digest_derivation(
        basis,
        CanonicalSingleSequenceDigestAlgorithmSlot::single_sequence(
            CanonicalDigestAlgorithmId::sha256(),
            domain,
            version,
        ),
    )
    .into_result()
    .expect("the governed-input fixture digest slot matches its basis");
    ApplicationCapabilityGovernedInputIdentity::canonical(&derive_canonical_digest(ready))
}
