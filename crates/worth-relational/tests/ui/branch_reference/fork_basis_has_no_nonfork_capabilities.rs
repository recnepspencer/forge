use worth_relational::facade::branch::AdmittedRelationalForkSourceBasis;

fn main() {
    let basis: AdmittedRelationalForkSourceBasis = todo!();
    let _ = basis.observe_branch();
    let _ = basis.begin_transaction();
    let _ = basis.publish_commit();
    let _ = basis.retain_component_basis();
    let _ = basis.readmit();
}
