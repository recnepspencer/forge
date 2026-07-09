use worth_query::facade::{
    materialize_correspondence_evidence_resolved, CorrespondenceEvidenceResolved,
    WorthQueryDeclarationBoundContributionTarget, WorthQueryDomainCapabilityTransitionOutcome,
    WorthQueryMaterializationReadyContinuityContribution,
};

fn main() {
    let _continuity_correspondence_materializer: fn(
        WorthQueryMaterializationReadyContinuityContribution<
            WorthQueryDeclarationBoundContributionTarget,
        >,
    ) -> WorthQueryDomainCapabilityTransitionOutcome<CorrespondenceEvidenceResolved> =
        materialize_correspondence_evidence_resolved;
}
