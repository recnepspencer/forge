use std::path::PathBuf;

use crate::mutation_campaign::MutationCampaignScope;
use crate::product::TestProduct;

mod parsing;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct Arguments {
    pub(crate) product: TestProduct,
    pub(crate) list: bool,
    pub(crate) target_root: Option<PathBuf>,
    pub(crate) report: Option<PathBuf>,
    pub(crate) mutant_report: Option<PathBuf>,
    pub(crate) mutation_scope: MutationCampaignScope,
    pub(crate) mutant: Option<u8>,
    pub(crate) first_mutant: Option<u8>,
}

impl Arguments {
    pub(crate) fn parse(arguments: impl IntoIterator<Item = String>) -> Result<Self, String> {
        parsing::parse(arguments)
    }
}

pub(super) fn help_requested(arguments: &[String]) -> bool {
    matches!(arguments, [argument] if argument == "-h" || argument == "--help")
}

pub(super) fn usage() -> String {
    "usage: store-test-runner <owner -p PACKAGE|smoke|ui|mutants|courtrooms --courtroom a|b|c|ci --partition LANE> \
     [--shard-index N --shard-count N] [--mutation-scope all|physical-work] \
     [--mutant N|--from-mutant N] [--mutant-report PATH] \
     [--list] [--target-root PATH] [--report PATH]"
        .into()
}

#[cfg(test)]
mod tests {
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
    fn parses_physical_work_mutation_scope() {
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
        let selected =
            Arguments::parse(["mutants".into(), "--mutant".into(), "13".into()]).unwrap();
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
        ])
        .unwrap();
        assert_eq!(
            parsed.product,
            TestProduct::Courtrooms {
                courtroom: CourtroomSelection::C
            }
        );
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
}
