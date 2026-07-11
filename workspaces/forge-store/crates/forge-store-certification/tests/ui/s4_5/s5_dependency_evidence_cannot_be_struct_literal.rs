use forge_store_physical_certification::{
    PhysicalIsolationHarnessMaturityDependency, PhysicalIsolationHarnessMaturityDependencyEvidence,
};

fn main() {
    let _evidence = PhysicalIsolationHarnessMaturityDependencyEvidence {
        dependency: PhysicalIsolationHarnessMaturityDependency::MutationValidation,
        coverage_row_digest: [0; 32],
    };
}
