use sha2::{Digest, Sha256};

use super::{
    BridgeAggregateMutationEvidenceDigest, BridgeAuthorityEvidenceDeferredBoundary,
    BridgeAuthorityEvidenceReadyCapability, BridgeAuthorityEvidenceVerificationGate,
    BridgeMutationEvidenceCarryForwardSection, BridgeMutationEvidenceContinuityFamily,
    BridgeMutationEvidenceExistingTruthBindingFamily, BridgeMutationEvidenceNamingFamily,
    BridgeMutationEvidenceSymbolicTargetReferenceFamily,
};

#[derive(Clone, Copy)]
enum AuthorityEvidenceDigestDomain {
    SupportDigest,
    CloseoutDigest,
    SupportReference,
    CarryForward,
    ExistingBinding,
    SymbolicTarget,
    Naming,
    Continuity,
    Aggregate,
    ReadyCapability,
    DeferredBoundary,
    VerificationGate,
}

impl AuthorityEvidenceDigestDomain {
    const fn as_bytes(self) -> &'static [u8] {
        match self {
            Self::SupportDigest => b"bridge-authoritative-mutation-evidence-support",
            Self::CloseoutDigest => b"bridge-authoritative-mutation-evidence-closeout",
            Self::SupportReference => b"support",
            Self::CarryForward => b"carry-forward",
            Self::ExistingBinding => b"existing-binding",
            Self::SymbolicTarget => b"symbolic-target",
            Self::Naming => b"naming",
            Self::Continuity => b"continuity",
            Self::Aggregate => b"aggregate",
            Self::ReadyCapability => b"ready-capability",
            Self::DeferredBoundary => b"deferred-boundary",
            Self::VerificationGate => b"verification-gate",
        }
    }
}

pub(super) fn support_digest_from_typed_evidence(
    carry_forward_sections: &[BridgeMutationEvidenceCarryForwardSection],
    existing_truth_binding_families: &[BridgeMutationEvidenceExistingTruthBindingFamily],
    symbolic_target_reference_families: &[BridgeMutationEvidenceSymbolicTargetReferenceFamily],
    naming_mutation_families: &[BridgeMutationEvidenceNamingFamily],
    continuity_mutation_families: &[BridgeMutationEvidenceContinuityFamily],
    aggregate_evidence_digests: &[BridgeAggregateMutationEvidenceDigest],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(AuthorityEvidenceDigestDomain::SupportDigest.as_bytes());
    hash_carry_forward_sections(&mut hasher, carry_forward_sections);
    hash_existing_truth_binding_families(&mut hasher, existing_truth_binding_families);
    hash_symbolic_target_reference_families(&mut hasher, symbolic_target_reference_families);
    hash_naming_mutation_families(&mut hasher, naming_mutation_families);
    hash_continuity_mutation_families(&mut hasher, continuity_mutation_families);
    hash_aggregate_evidence_digests(&mut hasher, aggregate_evidence_digests);
    let digest = hasher.finalize();
    format!("bridge-authoritative-mutation-evidence-support:sha256:{digest:x}")
}

pub(super) fn closeout_digest_from_typed_evidence(
    support_digest: &str,
    ready_capabilities: &[BridgeAuthorityEvidenceReadyCapability],
    deferred_boundaries: &[BridgeAuthorityEvidenceDeferredBoundary],
    verification_gates: &[BridgeAuthorityEvidenceVerificationGate],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(AuthorityEvidenceDigestDomain::CloseoutDigest.as_bytes());
    hash_entry(
        &mut hasher,
        AuthorityEvidenceDigestDomain::SupportReference,
        support_digest,
    );
    hash_ready_capabilities(&mut hasher, ready_capabilities);
    hash_deferred_boundaries(&mut hasher, deferred_boundaries);
    hash_verification_gates(&mut hasher, verification_gates);
    let digest = hasher.finalize();
    format!("bridge-authoritative-mutation-evidence-closeout:sha256:{digest:x}")
}

fn hash_carry_forward_sections(
    hasher: &mut Sha256,
    entries: &[BridgeMutationEvidenceCarryForwardSection],
) {
    for entry in entries {
        hash_entry(
            hasher,
            AuthorityEvidenceDigestDomain::CarryForward,
            entry.digest_entry(),
        );
    }
}

fn hash_existing_truth_binding_families(
    hasher: &mut Sha256,
    entries: &[BridgeMutationEvidenceExistingTruthBindingFamily],
) {
    for entry in entries {
        hash_entry(
            hasher,
            AuthorityEvidenceDigestDomain::ExistingBinding,
            entry.digest_entry(),
        );
    }
}

fn hash_symbolic_target_reference_families(
    hasher: &mut Sha256,
    entries: &[BridgeMutationEvidenceSymbolicTargetReferenceFamily],
) {
    for entry in entries {
        hash_entry(
            hasher,
            AuthorityEvidenceDigestDomain::SymbolicTarget,
            entry.digest_entry(),
        );
    }
}

fn hash_naming_mutation_families(
    hasher: &mut Sha256,
    entries: &[BridgeMutationEvidenceNamingFamily],
) {
    for entry in entries {
        hash_entry(
            hasher,
            AuthorityEvidenceDigestDomain::Naming,
            entry.digest_entry(),
        );
    }
}

fn hash_continuity_mutation_families(
    hasher: &mut Sha256,
    entries: &[BridgeMutationEvidenceContinuityFamily],
) {
    for entry in entries {
        hash_entry(
            hasher,
            AuthorityEvidenceDigestDomain::Continuity,
            entry.digest_entry(),
        );
    }
}

fn hash_aggregate_evidence_digests(
    hasher: &mut Sha256,
    entries: &[BridgeAggregateMutationEvidenceDigest],
) {
    for entry in entries {
        hash_entry(
            hasher,
            AuthorityEvidenceDigestDomain::Aggregate,
            entry.digest_entry(),
        );
    }
}

fn hash_ready_capabilities(
    hasher: &mut Sha256,
    entries: &[BridgeAuthorityEvidenceReadyCapability],
) {
    for entry in entries {
        hash_entry(
            hasher,
            AuthorityEvidenceDigestDomain::ReadyCapability,
            entry.digest_entry(),
        );
    }
}

fn hash_deferred_boundaries(
    hasher: &mut Sha256,
    entries: &[BridgeAuthorityEvidenceDeferredBoundary],
) {
    for entry in entries {
        hash_entry(
            hasher,
            AuthorityEvidenceDigestDomain::DeferredBoundary,
            entry.digest_entry(),
        );
    }
}

fn hash_verification_gates(
    hasher: &mut Sha256,
    entries: &[BridgeAuthorityEvidenceVerificationGate],
) {
    for entry in entries {
        hash_entry(
            hasher,
            AuthorityEvidenceDigestDomain::VerificationGate,
            entry.digest_entry(),
        );
    }
}

fn hash_entry(hasher: &mut Sha256, domain: AuthorityEvidenceDigestDomain, value: &str) {
    hasher.update(domain.as_bytes());
    hasher.update(b"\0");
    hasher.update(value.as_bytes());
    hasher.update(b"\0");
}
