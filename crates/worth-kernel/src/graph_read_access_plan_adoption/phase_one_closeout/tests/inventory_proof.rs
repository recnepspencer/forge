use std::collections::BTreeMap;

use super::super::current_worth_graph_read_access_plan_adoption_phase_one_closeout;
use crate::graph_read_access_plan_adoption::test_fixtures::production_milestone_eight_seed;
use crate::graph_read_access_plan_adoption::WorthGraphReadAccessPlanAdoptionExecutionFolkloreDisposition;

#[test]
fn execution_folklore_inventory_has_no_keep_rows() {
    let seed = production_milestone_eight_seed();
    let closeout = current_worth_graph_read_access_plan_adoption_phase_one_closeout(&seed)
        .expect("Milestone 8 seed should admit");
    let rows = closeout.execution_folklore_inventory().rows();

    assert!(!rows.is_empty());
    assert!(rows
        .iter()
        .all(|row| row.disposition().is_terminal_or_follow_on()));
    assert!(rows.iter().any(|row| {
        row.disposition() == WorthGraphReadAccessPlanAdoptionExecutionFolkloreDisposition::Migrate
    }));
    assert!(rows.iter().any(|row| {
        row.disposition() == WorthGraphReadAccessPlanAdoptionExecutionFolkloreDisposition::Delete
    }));
    assert!(rows.iter().any(|row| {
        row.disposition() == WorthGraphReadAccessPlanAdoptionExecutionFolkloreDisposition::QueryGap
    }));
}

#[test]
fn read_family_and_requirement_handoffs_create_migration_rows() {
    let seed = production_milestone_eight_seed();
    let closeout = current_worth_graph_read_access_plan_adoption_phase_one_closeout(&seed)
        .expect("Milestone 8 seed should admit");
    let rows = closeout.execution_folklore_inventory().rows();

    for identity in seed.read_family_identities() {
        assert!(rows.iter().any(|row| {
            row.disposition()
                == WorthGraphReadAccessPlanAdoptionExecutionFolkloreDisposition::Migrate
                && row.source_path().contains(identity.identity_digest())
                && row.current_caller() == identity.touched_authority_input()
                && row.migration_target()
                    == "graph_read_access_plan_adoption/phase_two_parallel_adoption_lane"
                && row.displacement_target() == "Query graph-read access-plan admission"
                && row.blocker().is_none()
        }));
    }
    for requirement_row in seed.requirement_row_evidence() {
        assert!(rows.iter().any(|row| {
            row.disposition()
                == WorthGraphReadAccessPlanAdoptionExecutionFolkloreDisposition::Migrate
                && row
                    .source_path()
                    .contains(requirement_row.requirement_row_digest())
                && row.current_caller() == requirement_row.source_requirement_record_digest()
                && row.migration_target()
                    == "graph_read_access_plan_adoption/phase_two_parallel_adoption_lane"
                && row.displacement_target() == "Query graph-read access-plan requirement admission"
                && row.blocker().is_none()
        }));
    }
}

#[test]
fn inventory_rows_preserve_source_identity() {
    let seed = production_milestone_eight_seed();
    let closeout = current_worth_graph_read_access_plan_adoption_phase_one_closeout(&seed)
        .expect("Milestone 8 seed should admit");
    let mut rows_by_source = BTreeMap::new();
    for row in closeout.execution_folklore_inventory().rows() {
        rows_by_source.insert(row.source_path(), row.row_digest());
    }

    assert_eq!(
        rows_by_source.len(),
        closeout.execution_folklore_inventory().rows().len()
    );
    assert!(rows_by_source.len() > 1);
    let mut seen_digests = BTreeMap::new();
    for (source_path, row_digest) in rows_by_source {
        assert!(
            seen_digests.insert(row_digest, source_path).is_none(),
            "distinct inventory sources must not collapse to one row digest"
        );
    }
}

#[test]
fn phase_one_counters_match_seed_and_inventory() {
    let seed = production_milestone_eight_seed();
    let closeout = current_worth_graph_read_access_plan_adoption_phase_one_closeout(&seed)
        .expect("Milestone 8 seed should admit");
    let inventory = closeout.execution_folklore_inventory();

    assert_eq!(
        closeout.counters().read_family_identity_count(),
        seed.read_family_identities().len()
    );
    assert_eq!(
        closeout.counters().requirement_row_evidence_count(),
        seed.requirement_row_evidence().len()
    );
    assert_eq!(
        closeout.counters().execution_folklore_row_count(),
        inventory.rows().len()
    );
    assert_eq!(
        closeout.counters().migrate_row_count(),
        seed.read_family_identities().len() + seed.requirement_row_evidence().len()
    );
    assert_eq!(
        closeout.counters().delete_row_count()
            + closeout.counters().capped_residue_row_count()
            + closeout.counters().query_gap_row_count()
            + closeout.counters().migrate_row_count(),
        inventory.rows().len()
    );
}
