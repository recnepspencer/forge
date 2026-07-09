use worth_store_physical_certification::SimulationReplayBundle;

fn requires_replay(_: SimulationReplayBundle) {}

fn main() {
    let terminal_json = serde_json::json!({ "seed": 8 });
    requires_replay(terminal_json);
}
