use super::{
    maximum_id, validate_request, validate_selectors, MutationCampaignRequest,
    MutationCampaignScope,
};

#[test]
fn direct_campaign_selection_rejects_an_absent_catalog_id() {
    let absent = maximum_id().checked_add(1).unwrap();

    assert!(validate_selectors(MutationCampaignScope::All, Some(absent), None).is_err());
    assert!(validate_selectors(MutationCampaignScope::All, None, Some(absent)).is_err());
    assert!(validate_selectors(MutationCampaignScope::All, Some(14), None).is_err());
    assert!(validate_selectors(MutationCampaignScope::All, None, Some(14)).is_err());
}

#[test]
fn physical_work_scope_is_the_complete_phase_16_catalog() {
    let ids = MutationCampaignScope::PhysicalWork
        .mutations()
        .iter()
        .map(|mutation| mutation.id)
        .collect::<Vec<_>>();

    assert_eq!(ids, (15..=44).collect::<Vec<_>>());
    assert_eq!(ids.len(), 30);
    assert!(!MutationCampaignScope::PhysicalWork.contains(14));
}

#[test]
fn bounded_residency_scope_contains_inherited_c6_and_complete_c7_c8_corpus() {
    let ids = MutationCampaignScope::BoundedResidency
        .mutations()
        .iter()
        .map(|mutation| mutation.id)
        .collect::<Vec<_>>();

    let expected = super::catalog::physical_work_mutations()
        .iter()
        .filter(|mutation| matches!(mutation.id, 42..=44))
        .chain(super::catalog::physical_reconstruction_c6_mutations())
        .chain(super::catalog::physical_reconstruction_c7_mutations())
        .chain(super::catalog::physical_reconstruction_c8_mutations())
        .map(|mutation| mutation.id)
        .collect::<Vec<_>>();
    assert_eq!(ids, expected);
    assert!(ids.windows(2).all(|pair| pair[0] < pair[1]));
    assert_eq!(ids.last(), Some(&161));
}

#[test]
fn c7_regression_corpus_is_append_only_and_contiguous() {
    let ids = super::catalog::physical_reconstruction_c7_mutations()
        .map(|mutation| mutation.id)
        .collect::<Vec<_>>();

    assert_eq!(ids, (79..=133).collect::<Vec<_>>());
    assert!(MutationCampaignScope::All.contains(79));
    assert!(MutationCampaignScope::All.contains(118));
    assert!(MutationCampaignScope::All.contains(119));
    assert!(MutationCampaignScope::All.contains(120));
    assert!(MutationCampaignScope::All.contains(121));
    assert!(MutationCampaignScope::All.contains(122));
    assert!(MutationCampaignScope::All.contains(123));
    assert!(MutationCampaignScope::All.contains(124));
    assert!(MutationCampaignScope::All.contains(125));
    assert!(MutationCampaignScope::All.contains(126));
    assert!(MutationCampaignScope::All.contains(127));
    assert!(MutationCampaignScope::All.contains(128));
    assert!(MutationCampaignScope::All.contains(129));
    assert!(MutationCampaignScope::All.contains(130));
    assert!(MutationCampaignScope::All.contains(131));
    assert!(MutationCampaignScope::All.contains(132));
    assert!(MutationCampaignScope::All.contains(133));
}

#[test]
fn c8_regression_corpus_is_append_only_and_source_bound() {
    let ids = super::catalog::physical_reconstruction_c8_mutations()
        .iter()
        .map(|mutation| mutation.id)
        .collect::<Vec<_>>();

    assert_eq!(ids, (134..=161).collect::<Vec<_>>());
    assert!(ids.windows(2).all(|pair| pair[0] < pair[1]));
    for id in ids {
        assert!(MutationCampaignScope::BoundedResidency.contains(id));
    }
}

#[test]
fn c8_closure_scope_is_the_complete_catalog() {
    let ids = MutationCampaignScope::C8Closure
        .mutations()
        .iter()
        .map(|mutation| mutation.id)
        .collect::<Vec<_>>();

    assert_eq!(ids, (134..=161).collect::<Vec<_>>());
    assert_eq!(
        ids,
        super::catalog::c8_closure_mutations()
            .iter()
            .map(|mutation| mutation.id)
            .collect::<Vec<_>>()
    );
}

#[test]
fn ci_certification_preserves_the_required_six_mutation_categories() {
    let predicates = MutationCampaignScope::BoundedResidency
        .mutations()
        .iter()
        .map(|mutation| mutation.predicate)
        .collect::<std::collections::BTreeSet<_>>();
    for required in [
        "whole-store-allocation",
        "pinned-eviction",
        "writeback-clean-without-exact-receipt",
        "duplicate-source-load",
        "speculative-kind-budget-bypass",
        "physical-work-topology-bypass",
    ] {
        assert!(
            predicates.contains(required),
            "CI mutation floor omitted `{required}`"
        );
    }
}

#[test]
fn report_publication_requires_a_complete_bounded_scope() {
    let report = std::path::Path::new("phase16.json");
    let all = MutationCampaignRequest {
        scope: MutationCampaignScope::All,
        list: false,
        preflight: false,
        selected: None,
        first: None,
        report: Some(report),
    };
    assert!(validate_request(&all).is_err());

    let partial = MutationCampaignRequest {
        scope: MutationCampaignScope::PhysicalWork,
        selected: Some(15),
        ..all
    };
    assert!(validate_request(&partial).is_err());

    let complete = MutationCampaignRequest {
        scope: MutationCampaignScope::PhysicalWork,
        selected: None,
        ..all
    };
    assert!(validate_request(&complete).is_ok());
    assert!(validate_request(&MutationCampaignRequest {
        scope: MutationCampaignScope::BoundedResidency,
        ..complete
    })
    .is_ok());
    assert!(validate_request(&MutationCampaignRequest {
        scope: MutationCampaignScope::C8Closure,
        ..complete
    })
    .is_ok());
}

#[test]
fn preflight_is_a_complete_non_executing_scope_mode() {
    let complete = MutationCampaignRequest {
        scope: MutationCampaignScope::BoundedResidency,
        list: false,
        preflight: true,
        selected: None,
        first: None,
        report: None,
    };
    assert!(validate_request(&complete).is_ok());

    for invalid in [
        MutationCampaignRequest {
            list: true,
            ..complete
        },
        MutationCampaignRequest {
            selected: Some(42),
            ..complete
        },
        MutationCampaignRequest {
            report: Some(std::path::Path::new("report.json")),
            ..complete
        },
    ] {
        assert!(validate_request(&invalid).is_err());
    }
}

#[cfg(feature = "physical-work-evidence")]
#[test]
fn retained_c8_closure_report_is_current_and_complete() {
    let workspace = crate::workspace_root();
    let root = workspace
        .parent()
        .and_then(std::path::Path::parent)
        .unwrap()
        .to_path_buf();
    let report = root.join("_docs/worth-store/physical-reconstruction-c8-phase-8-mutants.json");
    let retained = super::load_c8_closure_record(&report, &workspace).unwrap();
    assert_eq!(retained.observation_count(), 28);
    assert_eq!(
        retained.identities().collect::<Vec<_>>(),
        (134..=161).collect::<Vec<_>>()
    );
    assert_eq!(retained.source_closure_sha256().len(), 64);
}
