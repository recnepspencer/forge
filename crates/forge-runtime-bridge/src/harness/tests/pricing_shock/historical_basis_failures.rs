use super::support::*;

#[test]
fn pricing_shock_conflicting_historical_basis_is_detectable_against_independent_oracle() {
    let scenario = generated_pricing_scenario();
    let runtime = build_pricing_runtime(
        pricing_reference_source_with_conflicting_shock_snapshot(),
        RecordingSignalBridgeSink::default(),
    );

    let historical_cost = runtime
        .evaluate(
            BridgeTruthViewEvaluationRequest::for_historical_commit(
                TruthBranchIdentity::new("pricing-shock"),
                crate::facade::TruthCommitIdentity::new("commit:rubber-shock"),
            )
            .with_read_packet(pricing_component_read_packet("rubber")),
        )
        .expect("conflicting historical basis should still materialize as retained truth");
    let historical_provenance = runtime
        .evaluate(
            BridgeTruthViewEvaluationRequest::for_historical_commit(
                TruthBranchIdentity::new("pricing-shock"),
                crate::facade::TruthCommitIdentity::new("commit:rubber-shock"),
            )
            .with_read_packet(pricing_provenance_read_packet("rubber")),
        )
        .expect("conflicting historical basis should materialize provenance packet");
    let provenance_texts = read_pricing_provenance_aspect_text_packet(&historical_provenance);

    assert_eq!(
        historical_cost.snapshot_identity().as_str(),
        "snapshot:pricing-main"
    );
    assert_eq!(
        historical_provenance.snapshot_identity().as_str(),
        "snapshot:pricing-main"
    );
    assert_eq!(
        read_single_money_cents(&historical_cost),
        scenario.main_rubber_cost
    );
    assert_ne!(
        read_single_money_cents(&historical_cost),
        scenario.speculative_rubber_cost
    );
    assert_ne!(
        provenance_texts.shock_delta_text(),
        scenario
            .commit_attributions
            .get("commit:rubber-shock")
            .expect("generated scenario should retain shock attribution")
            .shock_delta_microunits
            .to_string()
    );
}

#[test]
fn pricing_shock_branch_head_and_snapshot_basis_mutation_sweep_is_detectable() {
    for (label, source, branch) in [
        (
            "speculative-branch-head-points-at-main",
            pricing_reference_source_with_branch_head_pointing_to(
                "pricing-shock",
                "commit:rubber-main",
            ),
            "pricing-shock",
        ),
        (
            "main-branch-head-points-at-speculative",
            pricing_reference_source_with_branch_head_pointing_to("main", "commit:rubber-shock"),
            "main",
        ),
    ] {
        let runtime = build_pricing_runtime(source, RecordingSignalBridgeSink::default());
        let error = runtime
            .evaluate(
                BridgeTruthViewEvaluationRequest::for_branch_head(TruthBranchIdentity::new(branch))
                    .with_read_packet(pricing_component_read_packet("rubber")),
            )
            .err()
            .unwrap_or_else(|| panic!("{label} should fail closed under branch-head mutation"));
        assert!(!error.to_string().is_empty());
    }

    let missing_snapshot_runtime = build_pricing_runtime(
        pricing_reference_source_with_missing_branch_head_snapshot(
            "pricing-shock",
            "commit:rubber-shock-missing-snapshot",
            "snapshot:pricing-shock-missing",
            "rubber",
        ),
        RecordingSignalBridgeSink::default(),
    );
    let error = missing_snapshot_runtime
        .evaluate(
            BridgeTruthViewEvaluationRequest::for_branch_head(TruthBranchIdentity::new(
                "pricing-shock",
            ))
            .with_read_packet(pricing_component_read_packet("rubber")),
        )
        .err()
        .expect("missing branch-head snapshot basis should fail closed");

    assert!(!error.to_string().is_empty());
}

#[test]
fn pricing_shock_snapshot_identity_conflict_sweep_is_detectable_against_independent_oracle() {
    let scenario = generated_pricing_scenario();

    for (label, source, selector_branch, expected_snapshot, unexpected_cost) in [
        (
            "main-snapshot-overwritten-with-speculative-meaning",
            pricing_reference_source_with_conflicting_snapshot_identity(snapshot_with_identity(
                &scenario.speculative_snapshot,
                TruthSnapshotIdentity::new("snapshot:pricing-main"),
            )),
            "main",
            "snapshot:pricing-main",
            scenario.main_rubber_cost,
        ),
        (
            "speculative-snapshot-overwritten-with-main-meaning",
            pricing_reference_source_with_conflicting_snapshot_identity(snapshot_with_identity(
                &scenario.main_snapshot,
                TruthSnapshotIdentity::new("snapshot:pricing-shock"),
            )),
            "pricing-shock",
            "snapshot:pricing-shock",
            scenario.speculative_rubber_cost,
        ),
    ] {
        let runtime = build_pricing_runtime(source, RecordingSignalBridgeSink::default());
        let evaluation = runtime
            .evaluate(
                BridgeTruthViewEvaluationRequest::for_branch_head(TruthBranchIdentity::new(
                    selector_branch,
                ))
                .with_read_packet(pricing_component_read_packet("rubber")),
            )
            .unwrap_or_else(|_| {
                panic!("{label} should still materialize the overwritten retained snapshot")
            });

        assert_eq!(
            evaluation.snapshot_identity().as_str(),
            expected_snapshot,
            "{label} should expose the conflicting retained snapshot identity"
        );
        assert_ne!(
            read_single_money_cents(&evaluation),
            unexpected_cost,
            "{label} should diverge from the independent oracle for the original branch meaning"
        );
    }
}

#[test]
fn pricing_shock_branch_head_missing_commit_sweep_fails_closed() {
    for (label, source, branch, missing_commit) in [
        (
            "main-branch-head-missing-envelope",
            pricing_reference_source_with_missing_branch_head_commit("main", "commit:missing-main"),
            "main",
            "commit:missing-main",
        ),
        (
            "speculative-branch-head-missing-envelope",
            pricing_reference_source_with_missing_branch_head_commit(
                "pricing-shock",
                "commit:missing-speculative",
            ),
            "pricing-shock",
            "commit:missing-speculative",
        ),
    ] {
        let runtime = build_pricing_runtime(source, RecordingSignalBridgeSink::default());
        let error = runtime
            .evaluate(
                BridgeTruthViewEvaluationRequest::for_branch_head(TruthBranchIdentity::new(branch))
                    .with_read_packet(pricing_component_read_packet("rubber")),
            )
            .err()
            .unwrap_or_else(|| {
                panic!("{label} should fail closed when branch head commit is missing")
            });

        let error_text = error.to_string();
        assert!(
            error_text.contains(missing_commit),
            "{label} should mention the missing retained branch-head commit"
        );
    }
}
