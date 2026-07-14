use worth_query::facade::runtime::WorthQueryEvidenceIdentity;

fn evidence_identity() -> WorthQueryEvidenceIdentity {
    unreachable!()
}

fn main() {
    let _label = evidence_identity().terminal_projection_for_reporting();
}
