use worth_relational::facade::branch::AdmittedRelationalForkSourceBasis;
use worth_relational::facade::runtime::RelationalRuntime;

fn main() {
    let runtime: &mut RelationalRuntime = todo!();
    let basis: AdmittedRelationalForkSourceBasis = todo!();
    let _ = runtime.snapshots().historical_snapshot_for_identity(&basis);
}
