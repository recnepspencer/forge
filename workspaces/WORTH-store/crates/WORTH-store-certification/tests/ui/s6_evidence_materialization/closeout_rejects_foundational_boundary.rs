use worth_store_certification::S6FoundationalAuthorityBoundary;
use worth_store_certification::adopt_materialized_s6_certification_evidence_for_closeout;

fn main() {
    let boundary = S6FoundationalAuthorityBoundary::CertificationEvidenceOnly;
    let _ = adopt_materialized_s6_certification_evidence_for_closeout(&boundary);
}
