use super::super::probes::ObservabilityProbe;

pub(crate) fn compare_observability_overlap(
    left: &ObservabilityProbe,
    right: &ObservabilityProbe,
) -> Vec<String> {
    let mut mismatches = Vec::new();
    if left.latest_patch_present != right.latest_patch_present {
        mismatches.push("latest_patch_present".to_string());
    }
    if left.latest_replay_present != right.latest_replay_present {
        mismatches.push("latest_replay_present".to_string());
    }
    mismatches
}
