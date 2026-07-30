use super::parse;
use crate::courtroom_campaign::bounded_residency_siege::protocol::{
    BoundedResidencyGenerationCleanup, BoundedResidencyGenerationDenial,
    BoundedResidencyGenerationFenceCase, BoundedResidencyGenerationFenceEffects,
};

#[test]
fn generation_fencing_decoder_preserves_every_typed_case_and_position() {
    let parsed = parse(&accepted_lines()).unwrap();
    assert_eq!(
        parsed.read,
        expected_case(
            19,
            1,
            BoundedResidencyGenerationDenial::StaleGeneration,
            BoundedResidencyGenerationCleanup::None,
        )
    );
    assert_eq!(
        parsed.dirty,
        expected_case(
            39,
            21,
            BoundedResidencyGenerationDenial::StaleOrForeignFrame,
            BoundedResidencyGenerationCleanup::LeaseReleased,
        )
    );
    assert_eq!(
        parsed.writeback,
        expected_case(
            59,
            41,
            BoundedResidencyGenerationDenial::StaleGeneration,
            BoundedResidencyGenerationCleanup::DirtyReturned,
        )
    );
}

#[test]
fn generation_fencing_decoder_rejects_missing_duplicate_and_unknown_tokens() {
    let mut missing = accepted_lines();
    missing.remove(0);
    assert!(parse(&missing).is_err());

    let mut duplicate = accepted_lines();
    duplicate.push(duplicate[0].clone());
    assert!(parse(&duplicate).is_err());

    let mut denial = accepted_lines();
    denial[0] = denial[0].replace("stale-generation", "admission-stopped");
    assert!(parse(&denial).is_err());

    let mut cleanup = accepted_lines();
    cleanup[1] = cleanup[1].replace("lease-released", "dropped");
    assert!(parse(&cleanup).is_err());
}

#[test]
fn generation_fencing_decoder_rejects_scalar_and_cardinality_corruption() {
    let mut scalar = accepted_lines();
    scalar[2] = scalar[2].replacen(" 59 ", " fifty-nine ", 1);
    assert!(parse(&scalar).is_err());

    let mut missing_field = accepted_lines();
    missing_field[0].pop();
    assert!(parse(&missing_field).is_err());

    let mut extra_field = accepted_lines();
    extra_field[0].push_str(" 0");
    assert!(parse(&extra_field).is_err());
}

fn accepted_lines() -> Vec<String> {
    vec![
        "BOUNDED_RESIDENCY_GENERATION_READ 19 18 stale-generation 1 2 3 4 5 6 7 8 9 10 11 12 13 none",
        "BOUNDED_RESIDENCY_GENERATION_DIRTY 39 38 stale-or-foreign-frame 21 22 23 24 25 26 27 28 29 30 31 32 33 lease-released",
        "BOUNDED_RESIDENCY_GENERATION_WRITEBACK 59 58 stale-generation 41 42 43 44 45 46 47 48 49 50 51 52 53 dirty-returned",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

fn expected_case(
    current_generation: u64,
    first_effect: u64,
    denial: BoundedResidencyGenerationDenial,
    cleanup: BoundedResidencyGenerationCleanup,
) -> BoundedResidencyGenerationFenceCase {
    BoundedResidencyGenerationFenceCase {
        current_generation,
        stale_generation: current_generation - 1,
        denial,
        effects: BoundedResidencyGenerationFenceEffects {
            allocation_admissions: first_effect,
            allocation_releases: first_effect + 1,
            allocation_other: first_effect + 2,
            residency_hits: first_effect + 3,
            residency_faults: first_effect + 4,
            source_loads: first_effect + 5,
            dirty_transitions: first_effect + 6,
            writeback_attempts: first_effect + 7,
            work_declarations: first_effect + 8,
            signal_requests: first_effect + 9,
            scheduler_admissions: first_effect + 10,
            media_attempts: first_effect + 11,
        },
        mutation_invocations: first_effect + 12,
        cleanup,
    }
}
