use worth_query::facade::foundation::CorrespondenceEvidenceResolved;
use worth_query::facade::runtime::{materialize_correspondence_evidence_resolved, WorthQueryDeclarationBoundContributionTarget, WorthQueryDomainCapabilityTransitionOutcome, WorthQueryMaterializationReadyContinuityContribution};

fn main() {
    let _continuity_correspondence_materializer: fn(
        WorthQueryMaterializationReadyContinuityContribution<
            WorthQueryDeclarationBoundContributionTarget,
        >,
    ) -> WorthQueryDomainCapabilityTransitionOutcome<CorrespondenceEvidenceResolved> =
        materialize_correspondence_evidence_resolved;
}
