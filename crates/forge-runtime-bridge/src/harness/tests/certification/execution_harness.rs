use forge_harness::facade::{ExecutionProfile, ExecutionRequest, HarnessAdapter, RunRecord};

use crate::harness::adapter::{BridgeHarnessAdapter, BridgeHarnessTargetId};
use crate::harness::fixtures::BridgeHarnessFixture;

pub(crate) fn execute_harness_run(
    fixture: forge_harness::facade::ScenarioFixture<BridgeHarnessFixture>,
    profile: ExecutionProfile,
    request_name: &str,
    target: BridgeHarnessTargetId,
) -> RunRecord<BridgeHarnessTargetId> {
    let adapter = BridgeHarnessAdapter;
    let mut runtime = adapter
        .create_runtime()
        .expect("harness runtime should construct");
    adapter
        .prepare_runtime(&mut runtime, &profile)
        .expect("harness prepare should succeed");
    adapter
        .load_fixture(&mut runtime, &fixture)
        .expect("harness fixture should load");
    adapter
        .execute(
            &mut runtime,
            &fixture,
            &ExecutionRequest::target(request_name, target),
            &profile,
        )
        .expect("harness execution should succeed")
}
