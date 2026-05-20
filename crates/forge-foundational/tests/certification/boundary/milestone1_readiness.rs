use forge_foundational::{
    certify_milestone1_production_test_readiness, milestone1_compatibility_debt_inventory,
    milestone1_migration_readiness_report, milestone1_proof_seed_inventory,
    milestone1_public_api_inventory, require_milestone1_production_test_readiness,
    Milestone1ProductionReadinessCertified, Milestone1ProductionTestReadyArtifact,
};
use forge_proof::Proof;

#[test]
fn milestone_1_readiness_report_names_adoption_surfaces_and_debt() {
    let report = milestone1_migration_readiness_report();

    assert_eq!(report.public_api(), milestone1_public_api_inventory());
    assert_eq!(
        report.compatibility_debt(),
        milestone1_compatibility_debt_inventory()
    );
    assert_eq!(report.proof_seeds(), milestone1_proof_seed_inventory());

    let public_api: Vec<_> = report.public_api().iter().map(|row| row.name()).collect();
    assert_eq!(
        public_api,
        vec![
            "values",
            "aspect_contracts",
            "authoritative_state",
            "authoritative_patches",
            "aspect_common_path",
            "identity_categories",
            "locators",
            "compatibility_bridges",
            "compatibility_common_path",
            "digest_preparation",
        ]
    );

    assert!(report
        .public_api()
        .iter()
        .any(|row| row.adoption_use().contains("explicit boundaries")));
    assert!(report
        .compatibility_debt()
        .iter()
        .all(|row| row.boundary().contains("JsonCompatibilityAspectInput")));
    assert!(report
        .proof_seeds()
        .iter()
        .any(|row| row.evidence().contains("ui/digest_preparation")));
    assert!(report
        .proof_seeds()
        .iter()
        .any(|seed| seed.name() == "aspect_common_path_front_doors"));
    assert!(report
        .proof_seeds()
        .iter()
        .any(|seed| seed.name() == "compatibility_common_path_front_doors"));
}

#[test]
fn milestone_1_closeout_inventory_does_not_claim_later_milestone_surfaces() {
    let report = milestone1_migration_readiness_report();
    let public_api = report.public_api();

    assert!(public_api.iter().all(|row| !row.name().contains("profile")));
    assert!(public_api.iter().all(|row| !row.name().contains("receipt")));
    assert!(public_api
        .iter()
        .all(|row| !row.name().contains("diagnostic_ontology")));
    assert!(report
        .compatibility_debt()
        .iter()
        .all(|row| row.exit_condition().contains("native aspect-state")));
}

fn accepts_milestone1_readiness_proof(
    _: &Proof<
        Milestone1ProductionReadinessCertified,
        forge_foundational::Milestone1ProductionReadinessAuthority,
    >,
) {
}

#[test]
fn milestone_1_production_test_readiness_is_proof_bearing() {
    let readiness: Milestone1ProductionTestReadyArtifact =
        certify_milestone1_production_test_readiness();
    let report = require_milestone1_production_test_readiness(&readiness);

    accepts_milestone1_readiness_proof(readiness.proofs());
    assert_eq!(report.public_api(), milestone1_public_api_inventory());
    assert!(report
        .proof_seeds()
        .iter()
        .any(|seed| seed.name() == "digest_preparation_readiness"));
    assert!(report
        .proof_seeds()
        .iter()
        .any(|seed| seed.name() == "aspect_common_path_front_doors"));
}
