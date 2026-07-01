use forge_store_physical_certification::S5HarnessMaturityDependencyEvidence;
use forge_store_readiness::S5HarnessMaturityDependency;

fn main() {
    let _evidence = S5HarnessMaturityDependencyEvidence {
        dependency: S5HarnessMaturityDependency::MutationValidation,
        coverage_row_digest: [0; 32],
    };
}
