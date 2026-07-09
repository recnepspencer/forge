use worth_query::facade::{AuthoritativePreviewComparisonCandidate, PreviewComparisonCandidateArtifact};

fn main() {
    let _: fn(&AuthoritativePreviewComparisonCandidate) -> &PreviewComparisonCandidateArtifact =
        AuthoritativePreviewComparisonCandidate::artifact;
}
