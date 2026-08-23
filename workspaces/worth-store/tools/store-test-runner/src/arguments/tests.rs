use super::{help_requested, Arguments};
use crate::mutation_campaign::MutationCampaignScope;
use crate::product::{CourtroomSelection, TestProduct};

#[test]
fn owner_requires_a_package() {
    let error = Arguments::parse(["owner".into()]).unwrap_err();
    assert!(error.contains("requires -p"));
}

#[test]
fn shard_arguments_are_a_pair() {
    let error = Arguments::parse([
        "ci".into(),
        "--partition".into(),
        "scenario".into(),
        "--shard-index".into(),
        "0".into(),
    ])
    .unwrap_err();
    assert!(error.contains("supplied together"));
}

#[test]
fn parses_owner_options() {
    let parsed = Arguments::parse([
        "owner".into(),
        "-p".into(),
        "worth-store".into(),
        "--list".into(),
    ])
    .unwrap();
    assert!(parsed.list);
    assert!(!parsed.preflight);
    assert_eq!(
        parsed.product,
        TestProduct::Owner {
            package: "worth-store".into()
        }
    );
}

#[test]
fn parses_mutation_campaign() {
    let parsed = Arguments::parse(["mutants".into(), "--list".into()]).unwrap();
    assert!(parsed.list);
    assert_eq!(parsed.product, TestProduct::Mutants);
    assert_eq!(parsed.mutation_scope, MutationCampaignScope::All);
    assert_eq!(parsed.mutant, None);
    assert_eq!(parsed.first_mutant, None);
    assert_eq!(parsed.report, None);
}

#[test]
fn parses_each_reportable_mutation_scope() {
    let parsed = Arguments::parse([
        "mutants".into(),
        "--mutation-scope".into(),
        "physical-work".into(),
        "--report".into(),
        "phase16.json".into(),
    ])
    .unwrap();

    assert_eq!(parsed.mutation_scope, MutationCampaignScope::PhysicalWork);
    assert_eq!(parsed.report, Some("phase16.json".into()));

    let bounded = Arguments::parse([
        "mutants".into(),
        "--mutation-scope".into(),
        "bounded-residency".into(),
        "--report".into(),
        "bounded.json".into(),
    ])
    .unwrap();
    assert_eq!(
        bounded.mutation_scope,
        MutationCampaignScope::BoundedResidency
    );
    assert_eq!(bounded.report, Some("bounded.json".into()));

    let c8 = Arguments::parse([
        "mutants".into(),
        "--mutation-scope".into(),
        "c8-closure".into(),
        "--report".into(),
        "c8.json".into(),
    ])
    .unwrap();
    assert_eq!(c8.mutation_scope, MutationCampaignScope::C8Closure);
    assert_eq!(c8.report, Some("c8.json".into()));
}

#[test]
fn help_is_only_the_exact_global_request() {
    assert!(help_requested(&["--help".into()]));
    assert!(help_requested(&["-h".into()]));
    assert!(!help_requested(&["courtrooms".into(), "--help".into()]));
}

#[test]
fn mutation_campaign_accepts_one_bounded_selector_mode() {
    let maximum = crate::mutation_campaign::maximum_id();
    let selected = Arguments::parse(["mutants".into(), "--mutant".into(), "13".into()]).unwrap();
    assert_eq!(selected.mutant, Some(13));
    assert_eq!(selected.first_mutant, None);

    let resumed =
        Arguments::parse(["mutants".into(), "--from-mutant".into(), "11".into()]).unwrap();
    assert_eq!(resumed.mutant, None);
    assert_eq!(resumed.first_mutant, Some(11));

    for invalid in [
        vec!["mutants".to_owned(), "--mutant".to_owned(), "0".to_owned()],
        vec![
            "mutants".to_owned(),
            "--from-mutant".to_owned(),
            maximum.checked_add(1).unwrap().to_string(),
        ],
    ] {
        assert!(Arguments::parse(invalid).is_err());
    }
    assert!(Arguments::parse([
        "mutants".into(),
        "--mutant".into(),
        "1".into(),
        "--from-mutant".into(),
        "2".into(),
    ])
    .is_err());
}

#[test]
fn courtroom_execution_requires_both_machine_reports() {
    let error = Arguments::parse([
        "courtrooms".into(),
        "--courtroom".into(),
        "b".into(),
        "--report".into(),
        "courtroom-b.json".into(),
    ])
    .unwrap_err();
    assert!(error.contains("--mutant-report"), "{error}");

    let parsed = Arguments::parse([
        "courtrooms".into(),
        "--courtroom".into(),
        "b".into(),
        "--mutant-report".into(),
        "mutants.json".into(),
        "--report".into(),
        "courtroom-b.json".into(),
    ])
    .unwrap();
    assert_eq!(
        parsed.product,
        TestProduct::Courtrooms {
            courtroom: CourtroomSelection::B
        }
    );
}

#[test]
fn courtroom_listing_is_side_effect_free() {
    let parsed = Arguments::parse([
        "courtrooms".into(),
        "--courtroom".into(),
        "b".into(),
        "--list".into(),
    ])
    .unwrap();
    assert!(parsed.list);
    assert!(parsed.report.is_none());
    assert!(parsed.mutant_report.is_none());
}

#[test]
fn parses_bounded_residency_siege_courtroom() {
    let parsed = Arguments::parse([
        "courtrooms".into(),
        "--courtroom".into(),
        "c".into(),
        "--mutant-report".into(),
        "mutants.json".into(),
        "--report".into(),
        "courtroom-c.json".into(),
        "--schedule-seed".into(),
        "18446744073709551615".into(),
    ])
    .unwrap();
    assert_eq!(
        parsed.product,
        TestProduct::Courtrooms {
            courtroom: CourtroomSelection::C
        }
    );
    assert_eq!(parsed.schedule_seed, Some(u64::MAX));
}

#[test]
fn parses_complete_mutation_source_preflight() {
    let parsed = Arguments::parse([
        "mutants".into(),
        "--mutation-scope".into(),
        "bounded-residency".into(),
        "--preflight".into(),
    ])
    .unwrap();

    assert!(parsed.preflight);
    assert_eq!(
        parsed.mutation_scope,
        MutationCampaignScope::BoundedResidency
    );
    for incompatible in ["--list", "--mutant", "--from-mutant", "--report"] {
        let mut arguments = vec![
            "mutants".to_owned(),
            "--mutation-scope".to_owned(),
            "bounded-residency".to_owned(),
            "--preflight".to_owned(),
            incompatible.to_owned(),
        ];
        if incompatible != "--list" {
            arguments.push(if incompatible == "--report" {
                "report.json".to_owned()
            } else {
                "42".to_owned()
            });
        }
        assert!(Arguments::parse(arguments).is_err());
    }
}

#[test]
fn explicit_c7_crash_seam_is_confined_to_courtroom_c() {
    let parsed = Arguments::parse([
        "courtrooms".into(),
        "--courtroom".into(),
        "c".into(),
        "--mutant-report".into(),
        "mutants.json".into(),
        "--report".into(),
        "courtroom-c.json".into(),
        "--schedule-seed".into(),
        "17".into(),
        "--crash-seam".into(),
        "before-wal-append".into(),
    ])
    .unwrap();
    assert_eq!(parsed.crash_seam.as_deref(), Some("before-wal-append"));

    let denied = Arguments::parse([
        "courtrooms".into(),
        "--courtroom".into(),
        "b".into(),
        "--mutant-report".into(),
        "mutants.json".into(),
        "--report".into(),
        "courtroom-b.json".into(),
        "--crash-seam".into(),
        "before-wal-append".into(),
    ])
    .unwrap_err();
    assert!(denied.contains("Courtroom C"), "{denied}");
}

#[test]
fn schedule_seed_is_exclusive_to_executing_courtroom_c() {
    for arguments in [
        vec![
            "courtrooms".to_owned(),
            "--courtroom".to_owned(),
            "b".to_owned(),
            "--mutant-report".to_owned(),
            "mutants.json".to_owned(),
            "--report".to_owned(),
            "courtroom-b.json".to_owned(),
            "--schedule-seed".to_owned(),
            "7".to_owned(),
        ],
        vec![
            "courtrooms".to_owned(),
            "--courtroom".to_owned(),
            "c".to_owned(),
            "--list".to_owned(),
            "--schedule-seed".to_owned(),
            "7".to_owned(),
        ],
    ] {
        assert!(Arguments::parse(arguments).is_err());
    }
}

#[test]
fn ci_schedule_lane_is_bounded_and_exclusive_with_replay_seed() {
    let parsed = Arguments::parse([
        "courtrooms".into(),
        "--courtroom".into(),
        "c".into(),
        "--mutant-report".into(),
        "mutants.json".into(),
        "--report".into(),
        "courtroom-c.json".into(),
        "--ci-schedule-lane".into(),
        "15".into(),
    ])
    .unwrap();
    assert_eq!(parsed.ci_schedule_lane.unwrap().index(), 15);
    for invalid in ["16", "256", "not-a-lane"] {
        let error = Arguments::parse([
            "courtrooms".into(),
            "--courtroom".into(),
            "c".into(),
            "--mutant-report".into(),
            "mutants.json".into(),
            "--report".into(),
            "courtroom-c.json".into(),
            "--ci-schedule-lane".into(),
            invalid.into(),
        ])
        .unwrap_err();
        assert!(error.contains("0 through 15"), "{error}");
    }
    let error = Arguments::parse([
        "courtrooms".into(),
        "--courtroom".into(),
        "c".into(),
        "--mutant-report".into(),
        "mutants.json".into(),
        "--report".into(),
        "courtroom-c.json".into(),
        "--schedule-seed".into(),
        "7".into(),
        "--ci-schedule-lane".into(),
        "3".into(),
    ])
    .unwrap_err();
    assert!(error.contains("mutually exclusive"), "{error}");
}

#[test]
fn parses_lifecycle_maelstrom_courtroom() {
    let parsed = Arguments::parse([
        "courtrooms".into(),
        "--courtroom".into(),
        "a".into(),
        "--mutant-report".into(),
        "mutants.json".into(),
        "--report".into(),
        "courtroom-a.json".into(),
    ])
    .unwrap();
    assert_eq!(
        parsed.product,
        TestProduct::Courtrooms {
            courtroom: CourtroomSelection::A
        }
    );
}
