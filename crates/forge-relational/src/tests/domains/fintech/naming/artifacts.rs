pub(crate) fn artifact_alias(case: &str, surface: &str, stage: &str) -> String {
    format!("{case}.{surface}.{stage}")
}

pub(crate) fn read_alias(case: &str, stage: &str) -> String {
    artifact_alias(case, "read", stage)
}

pub(crate) fn replay_alias(case: &str, branch: &str) -> String {
    artifact_alias(case, "replay", branch)
}
