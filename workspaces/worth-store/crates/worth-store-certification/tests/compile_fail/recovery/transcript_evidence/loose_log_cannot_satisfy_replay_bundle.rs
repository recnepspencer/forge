use worth_store_physical_certification::SimulationReplayBundle;

fn requires_replay(_: SimulationReplayBundle) {}

fn main() {
    let loose_log = String::from("seed=8 transcript=passed");
    requires_replay(loose_log);
}
