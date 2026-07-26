use sha2::Sha256;

use crate::canonical_hash_encoding::hash_text_field;
use crate::domain_computation::*;

pub(super) fn hash_optional(hash: &mut Sha256, label: &'static str, value: Option<&str>) {
    hash_text_field(hash, label, value.unwrap_or("not-declared"));
}

pub(super) fn occurrence_policy(value: WorthQueryArtifactOccurrenceIdentityPolicy) -> &'static str {
    match value {
        WorthQueryArtifactOccurrenceIdentityPolicy::IndependentPerExecution => {
            "independent-per-execution"
        }
        WorthQueryArtifactOccurrenceIdentityPolicy::DomainMintedIndependent => {
            "domain-minted-independent"
        }
    }
}

pub(super) fn substitution(value: WorthQueryArtifactSubstitutionPurpose) -> &'static str {
    match value {
        WorthQueryArtifactSubstitutionPurpose::ComputationalReuse => "computational-reuse",
        WorthQueryArtifactSubstitutionPurpose::EvidentiarySubstitution => {
            "evidentiary-substitution"
        }
    }
}

pub(super) fn reproducibility(value: WorthQueryArtifactReproducibilityClass) -> &'static str {
    match value {
        WorthQueryArtifactReproducibilityClass::ExactDeterministic => "exact-deterministic",
        WorthQueryArtifactReproducibilityClass::SeededDeterministic => "seeded-deterministic",
        WorthQueryArtifactReproducibilityClass::CanonicalReduction => "canonical-reduction",
        WorthQueryArtifactReproducibilityClass::DomainComparator => "domain-comparator",
        WorthQueryArtifactReproducibilityClass::IntervalOrErrorBound => "interval-or-error-bound",
        WorthQueryArtifactReproducibilityClass::Distributional => "distributional",
        WorthQueryArtifactReproducibilityClass::NonReplayable => "non-replayable",
    }
}

pub(super) fn determinism(value: WorthQueryArtifactDeterminismPosture) -> &'static str {
    match value {
        WorthQueryArtifactDeterminismPosture::Deterministic => "deterministic",
        WorthQueryArtifactDeterminismPosture::SeededDeterministic => "seeded-deterministic",
        WorthQueryArtifactDeterminismPosture::EnvironmentDependent => "environment-dependent",
        WorthQueryArtifactDeterminismPosture::EntropyDependent => "entropy-dependent",
        WorthQueryArtifactDeterminismPosture::Nondeterministic => "nondeterministic",
    }
}

pub(super) fn move_name(value: WorthQueryArtifactMovePosture) -> &'static str {
    match value {
        WorthQueryArtifactMovePosture::Required => "required",
        WorthQueryArtifactMovePosture::Forbidden => "forbidden",
    }
}

pub(super) fn borrow_name(value: WorthQueryArtifactBorrowPosture) -> &'static str {
    match value {
        WorthQueryArtifactBorrowPosture::Forbidden => "forbidden",
        WorthQueryArtifactBorrowPosture::SharedReadOnly => "shared-read-only",
    }
}

pub(super) fn provider_transfer_name(
    value: WorthQueryArtifactProviderTransferPosture,
) -> &'static str {
    match value {
        WorthQueryArtifactProviderTransferPosture::Forbidden => "forbidden",
        WorthQueryArtifactProviderTransferPosture::MoveOwnership => "move-ownership",
    }
}

pub(super) fn incumbent_name(value: WorthQueryConvergenceIncumbentPosture) -> &'static str {
    match value {
        WorthQueryConvergenceIncumbentPosture::NoIncumbent => "none",
        WorthQueryConvergenceIncumbentPosture::FirstFeasible => "first-feasible",
        WorthQueryConvergenceIncumbentPosture::BestObserved => "best-observed",
        WorthQueryConvergenceIncumbentPosture::ParetoFrontier => "pareto-frontier",
    }
}

pub(super) fn oscillation_name(value: WorthQueryConvergenceOscillationPosture) -> &'static str {
    match value {
        WorthQueryConvergenceOscillationPosture::Impossible => "impossible",
        WorthQueryConvergenceOscillationPosture::DetectAndDeny => "detect-and-deny",
        WorthQueryConvergenceOscillationPosture::DetectAndSelectIncumbent => {
            "detect-and-select-incumbent"
        }
        WorthQueryConvergenceOscillationPosture::DomainClassified => "domain-classified",
    }
}

pub(super) fn correspondence_name(value: WorthQuerySourceOutputCorrespondence) -> &'static str {
    match value {
        WorthQuerySourceOutputCorrespondence::OneToOne => "one-to-one",
        WorthQuerySourceOutputCorrespondence::OneToMany => "one-to-many",
        WorthQuerySourceOutputCorrespondence::ManyToOne => "many-to-one",
        WorthQuerySourceOutputCorrespondence::ManyToMany => "many-to-many",
        WorthQuerySourceOutputCorrespondence::Partial => "partial",
    }
}

pub(super) fn disposition_name(value: WorthQueryTransformationDisposition) -> &'static str {
    match value {
        WorthQueryTransformationDisposition::Preserved => "preserved",
        WorthQueryTransformationDisposition::Normalized => "normalized",
        WorthQueryTransformationDisposition::Approximated => "approximated",
        WorthQueryTransformationDisposition::Repaired => "repaired",
        WorthQueryTransformationDisposition::Omitted => "omitted",
        WorthQueryTransformationDisposition::Unsupported => "unsupported",
        WorthQueryTransformationDisposition::Quarantined => "quarantined",
    }
}

pub(super) fn error_name(value: WorthQueryTransformationErrorPosture) -> &'static str {
    match value {
        WorthQueryTransformationErrorPosture::Exact => "exact",
        WorthQueryTransformationErrorPosture::Bounded => "bounded",
        WorthQueryTransformationErrorPosture::Estimated => "estimated",
        WorthQueryTransformationErrorPosture::Unknown => "unknown",
    }
}

pub(super) fn loss_name(value: WorthQueryTransformationLossPosture) -> &'static str {
    match value {
        WorthQueryTransformationLossPosture::Lossless => "lossless",
        WorthQueryTransformationLossPosture::DeclaredLossy => "declared-lossy",
        WorthQueryTransformationLossPosture::LossClassifiedByDomain => "domain-classified",
    }
}

pub(super) fn clone_posture_name(value: WorthQueryArtifactClonePosture) -> &'static str {
    match value {
        WorthQueryArtifactClonePosture::Forbidden => "forbidden",
        WorthQueryArtifactClonePosture::Declared { .. } => "declared",
    }
}

pub(super) fn clone_mechanism_name(value: WorthQueryArtifactCloneMechanism) -> &'static str {
    match value {
        WorthQueryArtifactCloneMechanism::DeepClone => "deep-clone",
        WorthQueryArtifactCloneMechanism::ProviderDefinedCopy => "provider-defined-copy",
    }
}

pub(super) fn clone_boundary_name(value: WorthQueryArtifactCloneBoundary) -> &'static str {
    match value {
        WorthQueryArtifactCloneBoundary::ConcurrentObserver => "concurrent-observer",
        WorthQueryArtifactCloneBoundary::Isolation => "isolation",
        WorthQueryArtifactCloneBoundary::Retry => "retry",
        WorthQueryArtifactCloneBoundary::Temporal => "temporal",
        WorthQueryArtifactCloneBoundary::ProviderTransfer => "provider-transfer",
    }
}

pub(super) fn serialization_name(value: WorthQueryArtifactSerializationPosture) -> &'static str {
    match value {
        WorthQueryArtifactSerializationPosture::Forbidden => "forbidden",
        WorthQueryArtifactSerializationPosture::CanonicalProjectionOnly => {
            "canonical-projection-only"
        }
        WorthQueryArtifactSerializationPosture::DomainPayloadWithSchema => {
            "domain-payload-with-schema"
        }
    }
}

pub(super) fn lifecycle_name(value: WorthQueryArtifactLifecycleContract) -> &'static str {
    match value {
        WorthQueryArtifactLifecycleContract::Transient => "transient",
        WorthQueryArtifactLifecycleContract::ArenaScoped => "arena-scoped",
        WorthQueryArtifactLifecycleContract::Retained => "retained",
        WorthQueryArtifactLifecycleContract::ReconstructibleDerived => "reconstructible-derived",
        WorthQueryArtifactLifecycleContract::ExternallyDurable => "externally-durable",
        WorthQueryArtifactLifecycleContract::ReconstructibleAsAuthoritative => {
            "reconstructible-as-authoritative"
        }
    }
}

pub(super) fn classification_name(value: WorthQueryArtifactClassification) -> &'static str {
    match value {
        WorthQueryArtifactClassification::Public => "public",
        WorthQueryArtifactClassification::Internal => "internal",
        WorthQueryArtifactClassification::Confidential => "confidential",
        WorthQueryArtifactClassification::Restricted => "restricted",
    }
}

pub(super) fn redaction_name(value: WorthQueryArtifactRedactionPosture) -> &'static str {
    match value {
        WorthQueryArtifactRedactionPosture::NotRequired => "not-required",
        WorthQueryArtifactRedactionPosture::CanonicalProjectionOnly => "canonical-projection-only",
        WorthQueryArtifactRedactionPosture::DomainRedactorRequired => "domain-redactor-required",
        WorthQueryArtifactRedactionPosture::NeverDisclose => "never-disclose",
    }
}

pub(super) fn retention_name(
    value: worth_foundational::facade::RetentionDeliveryProfile,
) -> &'static str {
    match value {
        worth_foundational::facade::RetentionDeliveryProfile::Ephemeral => "ephemeral",
        worth_foundational::facade::RetentionDeliveryProfile::Retained => "retained",
        worth_foundational::facade::RetentionDeliveryProfile::Durable => "durable",
    }
}

pub(super) fn deletion_name(value: WorthQueryArtifactDeletionPosture) -> &'static str {
    match value {
        WorthQueryArtifactDeletionPosture::DeleteWithRun => "delete-with-run",
        WorthQueryArtifactDeletionPosture::DeleteAfterRetention => "delete-after-retention",
        WorthQueryArtifactDeletionPosture::DomainControlled => "domain-controlled",
        WorthQueryArtifactDeletionPosture::ExternallyControlled => "externally-controlled",
    }
}

pub(super) fn legal_hold_name(value: WorthQueryArtifactLegalHoldPosture) -> &'static str {
    match value {
        WorthQueryArtifactLegalHoldPosture::NotEligible => "not-eligible",
        WorthQueryArtifactLegalHoldPosture::DomainControlled => "domain-controlled",
        WorthQueryArtifactLegalHoldPosture::RequiredWhenDirected => "required-when-directed",
    }
}
