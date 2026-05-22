use forge_query::facade::{
    materialize_correspondence_evidence_resolved, CorrespondenceEvidenceResolved,
    ForgeQueryDeclarationBoundContributionTarget, ForgeQueryDomainCapabilityTransitionOutcome,
    ForgeQueryMaterializationReadyContinuityContribution,
};

fn main() {
    let _continuity_correspondence_materializer: fn(
        ForgeQueryMaterializationReadyContinuityContribution<
            ForgeQueryDeclarationBoundContributionTarget,
        >,
    ) -> ForgeQueryDomainCapabilityTransitionOutcome<CorrespondenceEvidenceResolved> =
        materialize_correspondence_evidence_resolved;
}
