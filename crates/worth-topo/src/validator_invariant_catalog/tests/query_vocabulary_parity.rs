use forge_query::facade::{
    ForgeQueryGraphObligationKind, ForgeQueryGraphObligationSupportLane,
    ForgeQueryGraphObligationSupportStatus,
};

use crate::runtime_support::milestone_one_invariant_registrations;
use crate::validation::derived_topology_rule_specs;
use crate::validator_invariant_catalog::source_catalog::current_invariant_family_inputs;

use super::production_phase_two_closeout;

#[test]
fn every_family_lowers_into_query_obligation_and_support_vocabulary() {
    let closeout = production_phase_two_closeout();
    let catalog = closeout.catalog();

    assert_eq!(catalog.validator_family_count(), 5);
    assert_eq!(catalog.invariant_family_count(), 14);
    assert_eq!(
        catalog.query_projection().query_registration_count(),
        catalog.records().len()
    );
    assert_eq!(
        catalog
            .query_projection()
            .registration_projection_rows()
            .len(),
        catalog.records().len()
    );

    for registration in catalog.query_projection().query_catalog().registrations() {
        let projection_row = catalog
            .query_projection()
            .registration_projection_rows()
            .iter()
            .find(|row| row.registration_digest() == registration.registration_digest())
            .expect("every Query registration must have a Worth projection row");
        assert!(
            ForgeQueryGraphObligationKind::ALL.contains(&registration.kind()),
            "registration kind must be Query-owned vocabulary"
        );
        assert_eq!(
            registration.support_posture().lane(),
            ForgeQueryGraphObligationSupportLane::WorthTopoOperatorCatalog
        );
        assert_eq!(
            registration.support_posture().status(),
            ForgeQueryGraphObligationSupportStatus::Supported
        );
        assert!(
            !registration.rule_identity().identity_digest().is_empty(),
            "Query rule identity must be digest-bearing"
        );
        assert_eq!(
            projection_row.query_rule_identity_digest(),
            registration.rule_identity().identity_digest()
        );
        assert_eq!(
            projection_row.registration_digest(),
            registration.registration_digest()
        );
        assert_eq!(
            projection_row.touch_selector_digest(),
            registration.touch_selector().selector_digest()
        );
        assert_eq!(projection_row.query_obligation_kind(), registration.kind());
        assert_eq!(
            projection_row.support_lane(),
            ForgeQueryGraphObligationSupportLane::WorthTopoOperatorCatalog
        );
        assert_eq!(
            projection_row.support_status(),
            ForgeQueryGraphObligationSupportStatus::Supported
        );
        assert_eq!(
            projection_row.operating_world_selector(),
            "any-committed-authority"
        );
    }
}

#[test]
fn invariant_families_are_derived_from_current_runtime_registrations() {
    let runtime_family_names = milestone_one_invariant_registrations()
        .expect("runtime invariant source registrations should build")
        .iter()
        .map(|registration| {
            format!(
                "{}.{}",
                registration.rule_id().as_str(),
                registration.execution_point().diagnostic_label()
            )
        })
        .collect::<Vec<_>>();
    let catalog_family_names = current_invariant_family_inputs("phase-eight-posture")
        .expect("catalog invariant inputs should derive from runtime registrations")
        .iter()
        .map(|row| row.input.identity.name().to_string())
        .collect::<Vec<_>>();

    assert_eq!(catalog_family_names, runtime_family_names);
}

#[test]
fn validator_families_are_derived_from_current_rule_specs() {
    let closeout = production_phase_two_closeout();
    let mut catalog_validator_names = closeout
        .catalog()
        .records()
        .iter()
        .filter(|record| {
            matches!(
                record,
                crate::validator_invariant_catalog::WorthTopologyLegalityFamilyRecord::Validator(_)
            )
        })
        .map(|record| record.identity().name().to_string())
        .collect::<Vec<_>>();
    let mut registry_validator_names = derived_topology_rule_specs()
        .iter()
        .map(|spec| spec.name.to_string())
        .collect::<Vec<_>>();

    catalog_validator_names.sort();
    registry_validator_names.sort();
    assert_eq!(catalog_validator_names, registry_validator_names);
}
