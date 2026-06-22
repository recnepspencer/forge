use worth_kernel::workload_composition::{
    PlanarBooleanOutcomeKind, PlanarBooleanOutcomeReceipt,
};

fn main() {
    let _ = PlanarBooleanOutcomeReceipt {
        kind: PlanarBooleanOutcomeKind::Admitted,
        declaration: panic!("declaration is private"),
        support: panic!("support is private"),
        user_outcome: panic!("user outcome is private"),
        blocker_provenance: panic!("blocker provenance is private"),
    };
}
