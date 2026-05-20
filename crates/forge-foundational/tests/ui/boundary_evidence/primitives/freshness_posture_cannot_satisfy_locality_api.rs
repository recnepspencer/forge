use forge_foundational::{
    FoundationalBoundaryEvidenceFreshnessPosture, FoundationalBoundaryEvidenceLocality,
};

fn needs_locality(_locality: FoundationalBoundaryEvidenceLocality) {}

fn main() {
    let freshness = FoundationalBoundaryEvidenceFreshnessPosture::FreshRetained;
    needs_locality(freshness);
}
