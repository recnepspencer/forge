use worth_query::facade::domain::WorthQueryAdmittedDomainEvidence;

fn duplicate(evidence: WorthQueryAdmittedDomainEvidence) {
    let _copy = evidence.clone();
}

fn main() {}
