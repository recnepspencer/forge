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
use worth_foundational::CanonicalBasisConstructionDenial;
use worth_proof::TransitionOutcome;

use super::S6CertificationEvidenceSources;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S6CanonicalEvidenceBasis {
    digest: CanonicalDerivedDigest,
    execution_identity_tag: u64,
    lane_binding_mask: u16,
    backend_profile_tag: u64,
    backend_evidence_class_tag: u64,
    queue_submitted: u64,
    flush_rows: usize,
    qualification_rows: usize,
    later_handoff_count: usize,
    access_policy_rows: usize,
    post_admission_violation_rows: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S6CanonicalMaterializationDenial {
    Basis(CanonicalBasisConstructionDenial),
    Digest(CanonicalDigestDerivationDenial),
}

impl S6CanonicalEvidenceBasis {
    pub(crate) fn from_sources(
        sources: &S6CertificationEvidenceSources,
    ) -> Result<Self, S6CanonicalMaterializationDenial> {
        let version = rule_version();
        let domain = domain();
        let sequence = match prepare_canonical_basis_sequence(
            version.clone(),
            domain,
            canonical_entries(sources),
        ) {
            TransitionOutcome::Success(sequence) => sequence,
            TransitionOutcome::Denied(denial) => {
                return Err(S6CanonicalMaterializationDenial::Basis(denial));
            }
            _ => unreachable!("canonical basis preparation only succeeds or denies"),
        };
        let slot = CanonicalSingleSequenceDigestAlgorithmSlot::single_sequence(
            CanonicalDigestAlgorithmId::test_stable_fixture(),
            domain,
            version,
        );
        let ready = match admit_canonical_sequence_digest_derivation(sequence, slot) {
            TransitionOutcome::Success(ready) => ready,
            TransitionOutcome::Denied(denial) => {
                return Err(S6CanonicalMaterializationDenial::Digest(denial));
            }
            _ => unreachable!("canonical digest admission only succeeds or denies"),
        };
        Ok(Self {
            digest: derive_canonical_digest(ready),
            execution_identity_tag: sources.binding().execution_identity_tag(),
            lane_binding_mask: sources.binding().required_lane_mask(),
            backend_profile_tag: sources.binding().backend_profile_tag(),
            backend_evidence_class_tag: sources.binding().backend_evidence_class_tag(),
            queue_submitted: sources.queue_execution().counters().submitted_units(),
            flush_rows: sources.flush_durability().len(),
            qualification_rows: sources.qualification_matrix().row_count(),
            later_handoff_count: sources.later_handoffs().destination_count(),
            access_policy_rows: sources.access_policy_rows().len(),
            post_admission_violation_rows: sources.post_admission_violations().len(),
        })
    }

    pub const fn digest(&self) -> &CanonicalDerivedDigest {
        &self.digest
    }

    pub const fn execution_identity_tag(&self) -> u64 {
        self.execution_identity_tag
    }

    pub const fn lane_binding_mask(&self) -> u16 {
        self.lane_binding_mask
    }

    pub const fn backend_profile_tag(&self) -> u64 {
        self.backend_profile_tag
    }

    pub const fn backend_evidence_class_tag(&self) -> u64 {
        self.backend_evidence_class_tag
    }

    pub const fn queue_submitted(&self) -> u64 {
        self.queue_submitted
    }

    pub const fn flush_rows(&self) -> usize {
        self.flush_rows
    }

    pub const fn qualification_rows(&self) -> usize {
        self.qualification_rows
    }

    pub const fn later_handoff_count(&self) -> usize {
        self.later_handoff_count
    }

    pub const fn access_policy_rows(&self) -> usize {
        self.access_policy_rows
    }

    pub const fn post_admission_violation_rows(&self) -> usize {
        self.post_admission_violation_rows
    }
}

fn canonical_entries(sources: &S6CertificationEvidenceSources) -> Vec<CanonicalBasisEntry> {
    vec![
        unsigned(
            "store_execution_identity",
            sources.binding().execution_identity_tag(),
        ),
        unsigned(
            "store_lane_binding_mask",
            u64::from(sources.binding().required_lane_mask()),
        ),
        unsigned(
            "bound_backend_profile",
            sources.binding().backend_profile_tag(),
        ),
        unsigned(
            "bound_backend_evidence_class",
            sources.binding().backend_evidence_class_tag(),
        ),
        unsigned(
            "queue_submitted",
            sources.queue_execution().counters().submitted_units(),
        ),
        unsigned("flush_rows", sources.flush_durability().len() as u64),
        unsigned(
            "qualification_rows",
            sources.qualification_matrix().row_count() as u64,
        ),
        unsigned(
            "later_handoff_count",
            sources.later_handoffs().destination_count() as u64,
        ),
        unsigned(
            "access_policy_rows",
            sources.access_policy_rows().len() as u64,
        ),
        unsigned(
            "post_admission_violation_rows",
            sources.post_admission_violations().len() as u64,
        ),
        unsigned(
            "secure_io_scope_checks",
            sources.secure_io_preservation().counters().scope_checks(),
        ),
        unsigned(
            "secure_io_backend_posture_checks",
            sources
                .secure_io_preservation()
                .counters()
                .backend_posture_checks(),
        ),
        unsigned(
            "access_policy_security_scope_preservations",
            sources
                .access_policy_rows()
                .iter()
                .map(|row| row.counters().security_scope_preservations())
                .sum(),
        ),
        unsigned(
            "post_admission_violations",
            sources
                .post_admission_violations()
                .iter()
                .map(|row| row.observed_violations())
                .sum(),
        ),
        unsigned(
            "queue_peak_depth",
            u64::from(sources.queue_execution().counters().peak_queue_depth()),
        ),
    ]
}

fn unsigned(locus: &'static str, value: u64) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        domain(),
        CanonicalBasisLocus::Named(locus.into()),
        CanonicalBasisEntryKind::Future("store-s6-certification-evidence"),
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits64,
            value: u128::from(value),
        },
    )
}

fn rule_version() -> CanonicalizationRuleVersion {
    CanonicalizationRuleVersion::new("worth-store.s6.certification-evidence.v1")
        .expect("static S6 evidence canonicalization rule version is valid")
}

const fn domain() -> CanonicalBasisDomain {
    CanonicalBasisDomain::Future("worth-store.s6.certification-evidence")
}
