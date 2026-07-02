use forge_store_physical_certification::accept_store_owned_s5_harness_readiness;
use forge_store_physical_isolation::s5_simulation_harness_readiness_requirement;

struct GenericRunner;

fn main() {
    let runner = GenericRunner;
    let _accepted = accept_store_owned_s5_harness_readiness(
        runner,
        s5_simulation_harness_readiness_requirement(),
    );
}
