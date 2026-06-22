use super::*;

#[test]
fn current_gate_report_exposes_inventory_and_deletion_ledger() {
    let report = current_worth_graph_authority_gate_report().expect("gate report should certify");

    assert_eq!(report.counters().inventory_rows(), 13);
    assert_eq!(report.counters().audited_sources(), 2955);
    assert_eq!(report.counters().deletion_ledger_rows(), 13);
    assert_eq!(report.counters().touched_graph_inventory_rows(), 20);
    assert_eq!(report.counters().touched_graph_deletion_ledger_rows(), 5);
    assert_eq!(report.counters().discovery_records(), 9);
    assert_eq!(report.counters().lower_authority_guard_plans(), 8);
    assert_eq!(report.counters().graph_obligation_selected_rows(), 1);
    assert_eq!(report.counters().graph_obligation_denied_rows(), 4);
    assert_eq!(report.counters().graph_obligation_residue_rows(), 3);
    assert_eq!(
        report.counters().graph_obligation_registration_full_scans(),
        0
    );
    assert!(
        report
            .counters()
            .graph_obligation_attempted_bucket_lookups()
            > 0
    );
    assert!(report.inventory().iter().any(|row| {
        row.source_path == "crates/worth-kernel/src/query_adoption"
            && row.deletion_target == WorthGraphAuthorityDeletionTarget::CompatibilityReport
    }));
    assert!(report
        .deletion_ledger()
        .iter()
        .any(|row| row.deletion_target == WorthGraphAuthorityDeletionTarget::ResidueManifest));
    for category in WorthTouchedGraphAuthorityInventoryCategory::ALL {
        assert!(
            report
                .touched_graph_inventory()
                .iter()
                .any(|row| row.category() == *category),
            "missing touched graph inventory category {category:?}"
        );
    }
}

#[test]
fn unclassified_source_inside_audited_root_fails_certification() {
    let mut audited_sources = current_worth_graph_authority_audited_source_paths();
    audited_sources
        .push("crates/worth-topo/src/topology_operators/adoption/unclassified.rs".to_string());
    audited_sources.sort();

    let violation = certify_worth_graph_authority_gate(
        current_worth_graph_authority_inventory(),
        current_worth_graph_authority_deletion_ledger(),
        current_worth_touched_graph_authority_inventory(),
        current_worth_touched_graph_deletion_ledger(),
        touched_graph_static_authority::current_worth_touched_graph_static_authority_entries(),
        touched_graph_facade_audit::current_worth_touched_graph_ordinary_public_facade_exports(),
        current_worth_graph_authority_discovery_records(),
        current_worth_lower_authority_promotion_guard_plan(),
        &audited_sources,
    )
    .expect_err("unclassified source under an audited root must fail the gate");

    assert_eq!(
        violation,
        WorthGraphAuthorityGateViolation::AuditedSourceSetManifestDrift(
            "topology.operator-catalog"
        )
    );
}

#[test]
fn root_authority_family_discovery_records_are_complete() {
    let report = current_worth_graph_authority_gate_report().expect("gate report should certify");

    for root_family in WorthGraphAuthorityRootFamily::ALL {
        assert!(
            report
                .discovery_records()
                .iter()
                .any(|record| record.root_family == root_family),
            "missing root authority discovery record for {root_family:?}"
        );
    }
}

#[test]
fn lower_authority_promotion_compile_fail_plan_is_complete() {
    let report = current_worth_graph_authority_gate_report().expect("gate report should certify");

    for promotion_case in WorthLowerAuthorityPromotionCase::ALL {
        assert!(
            report
                .lower_authority_guard_plan()
                .iter()
                .any(|plan| plan.promotion_case == promotion_case
                    && plan.planned_compile_fail_path.ends_with(".rs")),
            "missing compile-fail plan for {promotion_case:?}"
        );
    }
}

#[test]
fn lower_authority_promotion_compile_fail_paths_exist() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let report = current_worth_graph_authority_gate_report().expect("gate report should certify");

    for plan in report.lower_authority_guard_plan() {
        let fixture_path = manifest_dir.join(plan.planned_compile_fail_path());
        assert!(
            fixture_path.is_file(),
            "planned lower-authority compile-fail fixture does not exist: {}",
            plan.planned_compile_fail_path()
        );
    }
}

#[test]
fn deletion_target_keep_requires_distinct_authority_proof() {
    let mut deletion_ledger = current_worth_graph_authority_deletion_ledger();
    deletion_ledger.push(WorthGraphAuthorityDeletionLedgerRow {
        target_id: "bad.keep",
        source_path: "crates/worth-kernel/src/construction/query_authority",
        owner: WorthGraphAuthorityOwner::Kernel,
        deletion_target: WorthGraphAuthorityDeletionTarget::LocalSupportPinWrapper,
        action: WorthGraphAuthorityAction::Keep,
        replacement_or_blocker: "keep without proof",
        distinct_authority_proof: "",
        residue_owner: "",
        residue_cap: "",
        introduced_phase: "",
        removal_trigger: "",
        qa_evidence: "negative test",
    });

    let violation = certify_worth_graph_authority_gate(
        current_worth_graph_authority_inventory(),
        deletion_ledger,
        current_worth_touched_graph_authority_inventory(),
        current_worth_touched_graph_deletion_ledger(),
        touched_graph_static_authority::current_worth_touched_graph_static_authority_entries(),
        touched_graph_facade_audit::current_worth_touched_graph_ordinary_public_facade_exports(),
        current_worth_graph_authority_discovery_records(),
        current_worth_lower_authority_promotion_guard_plan(),
        &current_worth_graph_authority_audited_source_paths(),
    )
    .expect_err("deletion target keep without proof must fail");

    assert_eq!(
        violation,
        WorthGraphAuthorityGateViolation::DeletionTargetKeptWithoutDistinctProof("bad.keep")
    );
}

#[test]
fn residue_rows_require_owner_cap_phase_and_removal_trigger() {
    let mut deletion_ledger = current_worth_graph_authority_deletion_ledger();
    deletion_ledger.push(WorthGraphAuthorityDeletionLedgerRow {
        target_id: "bad.residue",
        source_path: "_docs/worth/worth-query-graph-authority-hardening-gate.md",
        owner: WorthGraphAuthorityOwner::Kernel,
        deletion_target: WorthGraphAuthorityDeletionTarget::ResidueManifest,
        action: WorthGraphAuthorityAction::Residue,
        replacement_or_blocker: "manifest",
        distinct_authority_proof: "",
        residue_owner: "",
        residue_cap: "cap",
        introduced_phase: "phase-1",
        removal_trigger: "close all rows",
        qa_evidence: "negative test",
    });

    let violation = certify_worth_graph_authority_gate(
        current_worth_graph_authority_inventory(),
        deletion_ledger,
        current_worth_touched_graph_authority_inventory(),
        current_worth_touched_graph_deletion_ledger(),
        touched_graph_static_authority::current_worth_touched_graph_static_authority_entries(),
        touched_graph_facade_audit::current_worth_touched_graph_ordinary_public_facade_exports(),
        current_worth_graph_authority_discovery_records(),
        current_worth_lower_authority_promotion_guard_plan(),
        &current_worth_graph_authority_audited_source_paths(),
    )
    .expect_err("residue without owner must fail");

    assert_eq!(
        violation,
        WorthGraphAuthorityGateViolation::ResidueWithoutOwnerCapOrRemovalTrigger("bad.residue")
    );
}

#[test]
fn query_gap_rows_require_named_blocker() {
    let mut inventory = current_worth_graph_authority_inventory();
    inventory.push(WorthGraphAuthorityInventoryRow {
        source_id: "bad.query-gap",
        source_path: "crates/forge-query/src/query_gap.rs",
        source_scope: WorthGraphAuthoritySourceScope::ExactSource,
        owner: WorthGraphAuthorityOwner::ForgeQuery,
        row_class: WorthGraphAuthorityRowClass::QueryCapabilityGap,
        deletion_target: WorthGraphAuthorityDeletionTarget::None,
        discovery_source: WorthGraphAuthorityDiscoverySource::SearchSeed,
        action: WorthGraphAuthorityAction::QueryGap,
        authority_claim: "gap",
        replacement_or_blocker: "",
        qa_evidence: "negative test",
    });

    let violation = certify_worth_graph_authority_gate(
        inventory,
        current_worth_graph_authority_deletion_ledger(),
        current_worth_touched_graph_authority_inventory(),
        current_worth_touched_graph_deletion_ledger(),
        touched_graph_static_authority::current_worth_touched_graph_static_authority_entries(),
        touched_graph_facade_audit::current_worth_touched_graph_ordinary_public_facade_exports(),
        current_worth_graph_authority_discovery_records(),
        current_worth_lower_authority_promotion_guard_plan(),
        &current_worth_graph_authority_audited_source_paths(),
    )
    .expect_err("query gap without blocker must fail");

    assert_eq!(
        violation,
        WorthGraphAuthorityGateViolation::QueryGapWithoutNamedBlocker("bad.query-gap")
    );
}

#[test]
fn inventory_rows_require_replacement_or_blocker() {
    let mut inventory = current_worth_graph_authority_inventory();
    inventory.push(WorthGraphAuthorityInventoryRow {
        source_id: "bad.missing-replacement",
        source_path: "crates/forge-query/src/missing_replacement.rs",
        source_scope: WorthGraphAuthoritySourceScope::ExactSource,
        owner: WorthGraphAuthorityOwner::ForgeQuery,
        row_class: WorthGraphAuthorityRowClass::RootAuthority,
        deletion_target: WorthGraphAuthorityDeletionTarget::None,
        discovery_source: WorthGraphAuthorityDiscoverySource::SearchSeed,
        action: WorthGraphAuthorityAction::Keep,
        authority_claim: "claim without a replacement surface",
        replacement_or_blocker: "",
        qa_evidence: "negative test",
    });

    let violation = certify_worth_graph_authority_gate(
        inventory,
        current_worth_graph_authority_deletion_ledger(),
        current_worth_touched_graph_authority_inventory(),
        current_worth_touched_graph_deletion_ledger(),
        touched_graph_static_authority::current_worth_touched_graph_static_authority_entries(),
        touched_graph_facade_audit::current_worth_touched_graph_ordinary_public_facade_exports(),
        current_worth_graph_authority_discovery_records(),
        current_worth_lower_authority_promotion_guard_plan(),
        &current_worth_graph_authority_audited_source_paths(),
    )
    .expect_err("inventory rows without replacement or blocker must fail");

    assert_eq!(
        violation,
        WorthGraphAuthorityGateViolation::EmptyInventoryField("bad.missing-replacement")
    );
}

#[test]
fn deletion_ledger_source_outside_inventory_fails_certification() {
    let mut deletion_ledger = current_worth_graph_authority_deletion_ledger();
    deletion_ledger.push(WorthGraphAuthorityDeletionLedgerRow {
        target_id: "bad.outside-inventory",
        source_path: "crates/worth-kernel/src/not_in_inventory",
        owner: WorthGraphAuthorityOwner::Kernel,
        deletion_target: WorthGraphAuthorityDeletionTarget::DuplicateSupportReport,
        action: WorthGraphAuthorityAction::Collapse,
        replacement_or_blocker: "collapse through an inventory row",
        distinct_authority_proof: "",
        residue_owner: "",
        residue_cap: "",
        introduced_phase: "",
        removal_trigger: "",
        qa_evidence: "negative test",
    });

    let violation = certify_worth_graph_authority_gate(
        current_worth_graph_authority_inventory(),
        deletion_ledger,
        current_worth_touched_graph_authority_inventory(),
        current_worth_touched_graph_deletion_ledger(),
        touched_graph_static_authority::current_worth_touched_graph_static_authority_entries(),
        touched_graph_facade_audit::current_worth_touched_graph_ordinary_public_facade_exports(),
        current_worth_graph_authority_discovery_records(),
        current_worth_lower_authority_promotion_guard_plan(),
        &current_worth_graph_authority_audited_source_paths(),
    )
    .expect_err("deletion ledger paths outside inventory coverage must fail");

    assert_eq!(
        violation,
        WorthGraphAuthorityGateViolation::DeletionLedgerSourceOutsideInventory(
            "bad.outside-inventory"
        )
    );
}
