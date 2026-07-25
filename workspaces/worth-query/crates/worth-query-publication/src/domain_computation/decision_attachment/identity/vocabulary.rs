use worth_foundational::facade::RetentionDeliveryProfile;
use worth_query_installation::facade::{
    WorthQueryArtifactClassification, WorthQueryArtifactDeletionPosture,
    WorthQueryArtifactLegalHoldPosture, WorthQueryArtifactRedactionPosture,
    WorthQueryCandidateOptimalityPosture, WorthQueryCandidateSearchPosture,
    WorthQuerySourceOutputCorrespondence, WorthQueryTransformationDisposition,
    WorthQueryTransformationErrorPosture, WorthQueryTransformationLossPosture,
};

use super::super::super::{
    WorthQueryCandidateFeasibilityClass, WorthQueryCandidateIncumbentDisposition,
    WorthQueryCandidateTerminationClass,
};

pub(super) fn termination_name(value: WorthQueryCandidateTerminationClass) -> &'static str {
    match value {
        WorthQueryCandidateTerminationClass::Completed => "completed",
        WorthQueryCandidateTerminationClass::Exhausted => "exhausted",
        WorthQueryCandidateTerminationClass::BoundReached => "bound-reached",
        WorthQueryCandidateTerminationClass::SampleCompleted => "sample-completed",
        WorthQueryCandidateTerminationClass::HeuristicStop => "heuristic-stop",
        WorthQueryCandidateTerminationClass::Interrupted => "interrupted",
    }
}

pub(super) fn feasibility_name(value: WorthQueryCandidateFeasibilityClass) -> &'static str {
    match value {
        WorthQueryCandidateFeasibilityClass::NotApplicable => "not-applicable",
        WorthQueryCandidateFeasibilityClass::NoFeasibleCandidate => "none-feasible",
        WorthQueryCandidateFeasibilityClass::FeasibleCandidateFound => "feasible-found",
        WorthQueryCandidateFeasibilityClass::AllConsideredFeasible => "all-considered-feasible",
        WorthQueryCandidateFeasibilityClass::Unknown => "unknown",
    }
}

pub(super) fn incumbent_name(value: WorthQueryCandidateIncumbentDisposition) -> &'static str {
    match value {
        WorthQueryCandidateIncumbentDisposition::NotApplicable => "not-applicable",
        WorthQueryCandidateIncumbentDisposition::None => "none",
        WorthQueryCandidateIncumbentDisposition::Selected => "selected",
        WorthQueryCandidateIncumbentDisposition::Reused => "reused",
        WorthQueryCandidateIncumbentDisposition::RejectedAll => "rejected-all",
    }
}

pub(super) fn search_name(value: &WorthQueryCandidateSearchPosture) -> String {
    match value {
        WorthQueryCandidateSearchPosture::NotApplicable => "not-applicable".into(),
        WorthQueryCandidateSearchPosture::Exhaustive => "exhaustive".into(),
        WorthQueryCandidateSearchPosture::ProvenTopK { count } => {
            format!("proven-top-k:{count}")
        }
        WorthQueryCandidateSearchPosture::Bounded { bound_identity } => {
            format!("bounded:{bound_identity}")
        }
        WorthQueryCandidateSearchPosture::Sampled { sample_identity } => {
            format!("sampled:{sample_identity}")
        }
        WorthQueryCandidateSearchPosture::Heuristic => "heuristic".into(),
        WorthQueryCandidateSearchPosture::Incomplete => "incomplete".into(),
    }
}

pub(super) fn optimality_name(value: &WorthQueryCandidateOptimalityPosture) -> String {
    match value {
        WorthQueryCandidateOptimalityPosture::NotApplicable => "not-applicable".into(),
        WorthQueryCandidateOptimalityPosture::ProvenOptimal => "proven-optimal".into(),
        WorthQueryCandidateOptimalityPosture::ProvenTopK { count } => {
            format!("proven-top-k:{count}")
        }
        WorthQueryCandidateOptimalityPosture::BoundedGap { bound_identity } => {
            format!("bounded-gap:{bound_identity}")
        }
        WorthQueryCandidateOptimalityPosture::BestInDeclaredSample { sample_identity } => {
            format!("best-in-sample:{sample_identity}")
        }
        WorthQueryCandidateOptimalityPosture::ParetoForDeclaredSet { set_identity } => {
            format!("pareto:{set_identity}")
        }
        WorthQueryCandidateOptimalityPosture::FeasibleOnly => "feasible-only".into(),
        WorthQueryCandidateOptimalityPosture::Unknown => "unknown".into(),
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

pub(super) fn retention_name(value: RetentionDeliveryProfile) -> &'static str {
    match value {
        RetentionDeliveryProfile::Ephemeral => "ephemeral",
        RetentionDeliveryProfile::Retained => "retained",
        RetentionDeliveryProfile::Durable => "durable",
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
