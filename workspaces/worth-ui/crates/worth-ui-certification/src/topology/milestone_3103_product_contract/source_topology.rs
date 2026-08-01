use std::collections::BTreeSet;

use crate::topology::WorkspaceSourceInventory;

const SOURCE_ROOT: &str = "apps/platform-pulse/src";

pub(super) fn audit(inventory: &WorkspaceSourceInventory) -> Result<(), String> {
    let required = [
        "application.rs",
        "launch_configuration.rs",
        "lib.rs",
        "lifecycle_observation_publication/query.rs",
        "lifecycle_observation_publication.rs",
        "main.rs",
        "native_frame/first_frame.rs",
        "native_frame/input.rs",
        "native_frame/input_reachability_tests.rs",
        "native_frame/projection.rs",
        "native_frame/query.rs",
        "native_frame/rebind.rs",
        "native_frame/source_rebind.rs",
        "native_frame/terminal_error.rs",
        "native_frame.rs",
        "observation_contract/envelope.rs",
        "observation_contract/lifecycle.rs",
        "observation_contract/mod.rs",
        "observation_contract/native_input.rs",
        "observation_contract/projection/replacement_projection.rs",
        "observation_contract/projection.rs",
        "observation_contract/projection_tests.rs",
        "observation_contract/query.rs",
        "observation_contract/query_projection.rs",
        "observation_contract/schema_transition.rs",
        "observation_contract/terminal_projection.rs",
        "observation_contract/visual.rs",
        "observation_contract/visual_projection.rs",
        "observation_contract/visual_tests.rs",
        "observation_contract/visual_value_projection.rs",
        "query_source/external_value.rs",
        "query_source/installation.rs",
        "query_source/lifecycle.rs",
        "query_source/mod.rs",
        "source_watch.rs",
        "visual_identity_adjudication.rs",
        "visual_identity_execution.rs",
        "visual_identity_execution/comparison.rs",
        "visual_identity_execution/progression.rs",
        "visual_identity_pulse.rs",
        "visual_observation_publication.rs",
    ]
    .into_iter()
    .map(|path| format!("{SOURCE_ROOT}/{path}"))
    .collect::<BTreeSet<_>>();
    let observed = inventory
        .rust_files_under(SOURCE_ROOT)
        .map(|source| source.relative_path().to_string_lossy().replace('\\', "/"))
        .collect::<BTreeSet<_>>();
    let missing = required.difference(&observed).collect::<Vec<_>>();
    if missing.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "Pulse successor topology lost required 3.10.3 product files: {missing:?}"
        ))
    }
}
