use worth_foundational::canonicalization_api::lower_lane::basis::{
    prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisValue, CanonicalIntegerWidth,
    CanonicalizationRuleVersion,
};
use worth_foundational::canonicalization_api::lower_lane::digest::{
    admit_canonical_sequence_digest_derivation, derive_canonical_digest, CanonicalDerivedDigest,
    CanonicalDigestAlgorithmId, CanonicalDigestDerivationDenial,
    CanonicalSingleSequenceDigestAlgorithmSlot,
};
use worth_foundational::{CanonicalBasisConstructionDenial, CanonicalDigestId};
use worth_proof::TransitionOutcome;

use super::{
    ExecutedPhysicalIsolationFinding, ExecutedPhysicalIsolationRequiredCounters,
    ExecutedPhysicalIsolationSourceBasis,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S5FoundationalCanonicalBasis {
    digest: CanonicalDerivedDigest,
    basis: ExecutedPhysicalIsolationSourceBasis,
    counters: ExecutedPhysicalIsolationRequiredCounters,
}

impl S5FoundationalCanonicalBasis {
    pub(crate) fn from_finding(
        finding: &ExecutedPhysicalIsolationFinding,
    ) -> Result<Self, S5CanonicalMaterializationDenial> {
        let version = rule_version();
        let domain = domain();
        let sequence = match prepare_canonical_basis_sequence(
            version.clone(),
            domain,
            canonical_entries(finding),
        ) {
            TransitionOutcome::Success(sequence) => sequence,
            TransitionOutcome::Denied(denial) => {
                return Err(S5CanonicalMaterializationDenial::Basis(denial));
            }
            _ => unreachable!("canonical basis preparation only returns success or denial"),
        };
        let slot = CanonicalSingleSequenceDigestAlgorithmSlot::single_sequence(
            CanonicalDigestAlgorithmId::sha256(),
            domain,
            version,
        );
        let ready = match admit_canonical_sequence_digest_derivation(sequence, slot) {
            TransitionOutcome::Success(ready) => ready,
            TransitionOutcome::Denied(denial) => {
                return Err(S5CanonicalMaterializationDenial::Digest(denial));
            }
            _ => unreachable!("canonical digest admission only returns success or denial"),
        };
        Ok(Self {
            digest: derive_canonical_digest(ready),
            basis: finding.basis().clone(),
            counters: finding.counters(),
        })
    }

    pub const fn digest(&self) -> &CanonicalDerivedDigest {
        &self.digest
    }

    pub const fn basis(&self) -> &ExecutedPhysicalIsolationSourceBasis {
        &self.basis
    }

    pub const fn counters(&self) -> ExecutedPhysicalIsolationRequiredCounters {
        self.counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S5CanonicalMaterializationDenial {
    Basis(CanonicalBasisConstructionDenial),
    Digest(CanonicalDigestDerivationDenial),
}

fn canonical_entries(finding: &ExecutedPhysicalIsolationFinding) -> Vec<CanonicalBasisEntry> {
    let basis = finding.basis();
    let counters = finding.counters();
    vec![
        text("family", basis.family()),
        digest("plan", *basis.plan_digest()),
        digest("schedule", *basis.schedule_digest()),
        digest("transcript", *basis.transcript_digest()),
        digest("replay_basis", *basis.replay_basis_digest()),
        unsigned("outcome", counters.outcome_count()),
        unsigned("retry", counters.retry_count()),
        unsigned("latch", counters.latch_count()),
        unsigned("reclaim", counters.reclaim_count()),
    ]
}

fn text(locus: &'static str, value: &'static str) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        domain(),
        CanonicalBasisLocus::Named(locus.into()),
        CanonicalBasisEntryKind::Future("store-s5-executed-isolation"),
        CanonicalBasisValue::ExactText(value.into()),
    )
}

fn digest(locus: &'static str, bytes: [u8; 32]) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        domain(),
        CanonicalBasisLocus::Named(locus.into()),
        CanonicalBasisEntryKind::Future("store-s5-executed-isolation"),
        CanonicalBasisValue::BytesDigest(CanonicalDigestId::new(bytes)),
    )
}

fn unsigned(locus: &'static str, value: u64) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        domain(),
        CanonicalBasisLocus::Named(locus.into()),
        CanonicalBasisEntryKind::Future("store-s5-executed-isolation"),
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits64,
            value: u128::from(value),
        },
    )
}

fn rule_version() -> CanonicalizationRuleVersion {
    CanonicalizationRuleVersion::new("worth-store.s5.executed-isolation.v1")
        .expect("static S5 evidence canonicalization rule version is valid")
}

const fn domain() -> CanonicalBasisDomain {
    CanonicalBasisDomain::Future("worth-store.s5.executed-isolation")
}
