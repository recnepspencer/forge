use crate::certification::topology_operator_closeout::report::{
    MilestoneThreeEditFalloutClass, MilestoneThreeEditReplayParityReport,
    MilestoneThreeHostileOutcomeClass, MilestoneThreeHostileScenario,
    MilestoneThreeHostileScenarioReport,
};
use crate::certification::ReplayParityStatus;
use crate::topology_operators::{
    NamingEditContinuityMatrix, TopologyEditDigest, TopologyEditNamingOutcome,
    TopologyOperatorDigest,
};
use crate::validation::{
    DerivedTopologyValidationReport, TopologyValidationInputClass, TopologyValidationPhase,
    TopologyValidationRow,
};
use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

use super::aggregate_acceptance::build_edit_fallout_breadth_rows;

#[test]
fn validation_rows_alone_do_not_imply_whole_view_fallback() {
    let report = MilestoneThreeHostileScenarioReport {
        scenario: MilestoneThreeHostileScenario::CancellationChainParity,
        primitive_family: "SheetDisk(n)".to_string(),
        primitive: MilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
        edit_families: Vec::new(),
        bowtie_adjacent_witness: None,
        ambiguous_local_rewire_witness: None,
        split_collapse_churn_witness: None,
        broken_radial_witness: None,
        topology_edit_digest: empty_edit_digest(),
        naming_edit_continuity_matrix: empty_continuity_matrix(),
        continuity_outcome_class: TopologyEditNamingOutcome::Preserved,
        continuity_rejection_class: None,
        outcome_class: MilestoneThreeHostileOutcomeClass::Accepted,
        rejection_class: None,
        rejected_edit_scope_report: None,
        derived_validation_report: Some(validation_only_report()),
        derived_materialization_fallback_class: None,
        edit_replay_parity_report: empty_replay_report(),
        detail: "synthetic accepted validation-only fallout proof".to_string(),
    };

    let rows = build_edit_fallout_breadth_rows(&[report]);

    assert_eq!(rows[0].derived_validation_row_count, 1);
    assert_eq!(
        rows[0].fallout_class,
        MilestoneThreeEditFalloutClass::Localized
    );
    assert_eq!(rows[0].fallback_count, 0);
}

fn validation_only_report() -> DerivedTopologyValidationReport {
    DerivedTopologyValidationReport {
        rows: vec![TopologyValidationRow {
            validator: "ownership".to_string(),
            phase: TopologyValidationPhase::DerivedMaterialization,
            input_class: TopologyValidationInputClass::MaterializedTopologyView,
            status: "passed".to_string(),
        }],
    }
}

fn empty_continuity_matrix() -> NamingEditContinuityMatrix {
    NamingEditContinuityMatrix {
        rows: Vec::new(),
        preserved_count: 0,
        ambiguous_count: 0,
        rejected_count: 0,
    }
}

fn empty_replay_report() -> MilestoneThreeEditReplayParityReport {
    MilestoneThreeEditReplayParityReport {
        replay_checked: true,
        parity_status: ReplayParityStatus::Match,
        mismatch_count: 0,
        step_rows: Vec::new(),
        replay_step_rows: Vec::new(),
        baseline_materialized_topology_digest: None,
        final_materialized_topology_digest: None,
        replay_final_materialized_topology_digest: None,
        returned_to_baseline: Some(true),
    }
}

fn empty_edit_digest() -> TopologyEditDigest {
    TopologyEditDigest {
        digest: TopologyOperatorDigest {
            algorithm: "fnv1a64".to_string(),
            digest_hex: "0000000000000000".to_string(),
            row_count: 0,
        },
        contract_count: 0,
        family_count: 0,
        changed_scope_count: 0,
        naming_scope_count: 0,
        derived_region_count: 0,
        fallback_policy_count: 0,
        fallback_rejection_policy_count: 0,
    }
}
