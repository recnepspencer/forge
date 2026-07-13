use worth_query::facade::certification::{BasisLifecyclePublicBoundaryAuditRow, BasisLifecyclePublicBoundarySurface};

fn main() {
    let _row = BasisLifecyclePublicBoundaryAuditRow {
        surface: BasisLifecyclePublicBoundarySurface::BranchIdentifier,
        forbidden_token: "raw branch id",
        blocked_entrypoint: "read",
        required_capability: "ScopedObservationBasis",
        enforcement_proof: "compile_fail",
        row_digest: String::new(),
    };
}
