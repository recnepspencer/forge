use worth_query_execution::facade::provider_session::WorthQuerySelectedInstalledInvariant;

fn bypass_state_load(selected: WorthQuerySelectedInstalledInvariant<'_, '_>) {
    let _ = selected.execute();
}

fn main() {}
