use worth_query_host::facade::{installed, runtime::WorthQueryWorkspace};
use worth_query_replay::facade::WorthQueryCertificationReplayCounters;

fn ordinary_entry(workspace: &mut WorthQueryWorkspace) {
    let _root = workspace.observe_operating_world();
    let _ = installed::operation::project_facts().entity_identities();
}

fn certification_entry(counters: WorthQueryCertificationReplayCounters) {
    let _ = counters;
}

fn main() {}
