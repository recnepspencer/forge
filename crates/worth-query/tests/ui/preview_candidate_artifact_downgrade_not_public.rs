use worth_query::facade::policy::AuthoritativePreviewComparisonCandidate;
use worth_query::facade::PreviewComparisonCandidateArtifact;

fn main() {
    let _: fn(&AuthoritativePreviewComparisonCandidate) -> &PreviewComparisonCandidateArtifact =
        AuthoritativePreviewComparisonCandidate::artifact;
}
