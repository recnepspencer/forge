use std::collections::BTreeSet;
use worth_ui_certification::topology::{
    audit_milestone_37_structural_inventory, milestone_37_active_failure_modes,
    milestone_37_cleared_finding_ids, milestone_37_critical_finding_ids,
    rejected_cosmetic_candidate_ids, structural_inventory_digest,
};

fn workspace_root() -> &'static worth_ui_certification::topology::WorkspaceSourceInventory {
    super::workspace_source_inventory()
}

#[test]
fn milestone_37_structural_inventory_is_deterministic() {
    let first = audit_milestone_37_structural_inventory(workspace_root());
    let second = audit_milestone_37_structural_inventory(workspace_root());
    assert_eq!(first, second);
    assert_eq!(
        structural_inventory_digest(&first),
        structural_inventory_digest(&second)
    );
}

#[test]
fn milestone_37_structural_inventory_names_critical_blockers() {
    let findings = audit_milestone_37_structural_inventory(workspace_root());
    let observed_ids = findings
        .iter()
        .map(|finding| finding.id.as_str())
        .collect::<BTreeSet<_>>();
    let expected_ids = milestone_37_critical_finding_ids();

    let missing = expected_ids
        .difference(&observed_ids)
        .copied()
        .collect::<Vec<_>>();
    assert!(
        missing.is_empty(),
        "structural inventory missing critical findings: {missing:?}\nobserved: {observed_ids:?}"
    );
}

#[test]
fn milestone_37_structural_inventory_covers_active_failure_modes() {
    let findings = audit_milestone_37_structural_inventory(workspace_root());
    let modes = findings
        .iter()
        .map(|finding| finding.failure_mode)
        .collect::<BTreeSet<_>>();

    for mode in milestone_37_active_failure_modes() {
        assert!(modes.contains(&mode), "missing failure mode {mode:?}");
    }
}

#[test]
fn milestone_37_structural_inventory_phase3_cleared_findings_are_absent() {
    let findings = audit_milestone_37_structural_inventory(workspace_root());
    let observed_ids = findings
        .iter()
        .map(|finding| finding.id.as_str())
        .collect::<BTreeSet<_>>();
    let phase3_cleared = BTreeSet::from(["T-01", "T-02", "T-04", "O-01", "B-02"]);
    let leaked = phase3_cleared
        .intersection(&observed_ids)
        .copied()
        .collect::<Vec<_>>();
    assert!(
        leaked.is_empty(),
        "phase 3 cleared findings still present in inventory: {leaked:?}"
    );
}

#[test]
fn milestone_37_structural_inventory_phase2_cleared_findings_are_absent() {
    let findings = audit_milestone_37_structural_inventory(workspace_root());
    let observed_ids = findings
        .iter()
        .map(|finding| finding.id.as_str())
        .collect::<BTreeSet<_>>();
    let cleared = milestone_37_cleared_finding_ids();
    let leaked = cleared
        .intersection(&observed_ids)
        .copied()
        .collect::<Vec<_>>();
    assert!(
        leaked.is_empty(),
        "phase 2 cleared findings still present in inventory: {leaked:?}"
    );
}

#[test]
fn milestone_37_structural_inventory_rejects_cosmetic_only_candidates() {
    let findings = audit_milestone_37_structural_inventory(workspace_root());
    let observed_ids = findings
        .iter()
        .map(|finding| finding.id.as_str())
        .collect::<BTreeSet<_>>();

    let cosmetic = rejected_cosmetic_candidate_ids();
    let leaked = cosmetic
        .intersection(&observed_ids)
        .copied()
        .collect::<Vec<_>>();
    assert!(
        leaked.is_empty(),
        "cosmetic-only candidates leaked into critical inventory: {leaked:?}"
    );
}

#[test]
fn milestone_37_concept_freeze_excludes_38_scope_tokens_from_inventory_paths() {
    let findings = audit_milestone_37_structural_inventory(workspace_root());
    let forbidden = [
        "allocation_receipt",
        "incremental_replan",
        "scroll_portal_churn",
        "continuous_interaction",
    ];

    for finding in findings {
        let haystack = format!("{} {}", finding.id, finding.summary).to_ascii_lowercase();
        for token in forbidden {
            assert!(
                !haystack.contains(token),
                "finding {} appears to widen into 3.8 scope via `{token}`",
                finding.id
            );
        }
    }
}
